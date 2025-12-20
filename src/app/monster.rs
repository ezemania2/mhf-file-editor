use super::*;
use crate::model::mhfdat::{CarveDrop, CarveDropTable, PartBreakDrop, PartBreakDropTable};
use egui;

impl MhfdatApp {
    pub fn show_monster_tab(&mut self, ui: &mut egui::Ui) {
        use crate::app::{ViewMode, MonsterTab};
        
        // Add monster category tabs
        ui.horizontal_wrapped(|ui| {
            if ui.selectable_label(self.monster_tab == MonsterTab::CarveParts, "Carve Parts").clicked() {
                self.monster_tab = MonsterTab::CarveParts;
            }
            if ui.selectable_label(self.monster_tab == MonsterTab::PartBreakDrops, "Part Break Drops").clicked() {
                self.monster_tab = MonsterTab::PartBreakDrops;
            }
            if ui.selectable_label(self.monster_tab == MonsterTab::MonsterDescriptions, "Monster Descriptions").clicked() {
                self.monster_tab = MonsterTab::MonsterDescriptions;
            }
        });
        ui.separator();
        
        match self.monster_tab {
            MonsterTab::CarveParts => {
                // Initialize view mode if not present
                if !self.view_mode.contains_key("carve_parts") {
                    self.view_mode.insert("carve_parts".to_string(), ViewMode::List);
                }
                
                let view_mode = self.view_mode.get("carve_parts").copied().unwrap_or(ViewMode::List);
                
                match view_mode {
                    ViewMode::List => {
                        self.show_carve_parts_list(ui);
                    }
                    ViewMode::Details => {
                        self.show_carve_parts_details(ui);
                    }
                }
            }
            MonsterTab::PartBreakDrops => {
                // Initialize view mode if not present
                if !self.view_mode.contains_key("part_break_parts") {
                    self.view_mode.insert("part_break_parts".to_string(), ViewMode::List);
                }
                
                let view_mode = self.view_mode.get("part_break_parts").copied().unwrap_or(ViewMode::List);
                
                match view_mode {
                    ViewMode::List => {
                        self.show_part_break_parts_list(ui);
                    }
                    ViewMode::Details => {
                        self.show_part_break_parts_details(ui);
                    }
                }
            }
            MonsterTab::MonsterDescriptions => {
                // Initialize view mode if not present
                if !self.view_mode.contains_key("monster_descriptions") {
                    self.view_mode.insert("monster_descriptions".to_string(), ViewMode::List);
                }
                
                let view_mode = self.view_mode.get("monster_descriptions").copied().unwrap_or(ViewMode::List);
                
                match view_mode {
                    ViewMode::List => {
                        self.show_monster_descriptions_list(ui);
                    }
                    ViewMode::Details => {
                        self.show_monster_descriptions_details(ui);
                    }
                }
            }
        }
    }
    
    fn show_carve_parts_list(&mut self, ui: &mut egui::Ui) {
        let table_count = self.carve_parts.tables.len();
        
        MhfdatApp::section_header(ui, &format!("Carve Parts ({} carve tables)", table_count), |ui| {
            if ui.button("Add Table").clicked() {
                self.carve_parts.tables.push(CarveDropTable { carves: Vec::new() });
                self.carve_parts_modified = true;
                self.carve_parts_count = self.carve_parts.tables.len() as u16;
                self.carve_parts_count_modified = true;
            }
            if ui.button("Export to JSON").clicked() {
                if let Ok(json) = serde_json::to_string_pretty(&self.carve_parts) {
                    let _ = std::fs::write("carve_parts.json", json);
                }
            }
            if ui.button("Import from JSON").clicked() {
                if let Ok(json) = std::fs::read_to_string("carve_parts.json") {
                    if let Ok(data) = serde_json::from_str(&json) {
                        self.carve_parts = data;
                        self.carve_parts_modified = true;
                        self.carve_parts_count = self.carve_parts.tables.len() as u16;
                        self.carve_parts_count_modified = true;
                    }
                }
            }
        });
        
        if table_count == 0 {
            ui.label("No carve parts loaded.");
            return;
        }
        
        // List of carve tables
        ui.heading("Carve Tables");
        
        let tables_page_size = 50usize;
        let tables_total_pages = (table_count + tables_page_size - 1) / tables_page_size;
        let tables_page = self.carve_parts_tables_page.min(tables_total_pages.saturating_sub(1) as u32);
        self.carve_parts_tables_page = tables_page;
        
        let tables_start = (tables_page as usize) * tables_page_size;
        let tables_end = (tables_start + tables_page_size).min(table_count);
        
        MhfdatApp::list_scroll(ui, "carve_parts_tables_scroll", |ui| {
            egui::Grid::new("carve_parts_tables_grid")
                .striped(true)
                .num_columns(3)
                .show(ui, |ui| {
                    ui.label("Carve Table");
                    ui.label("Carves");
                    ui.label("Actions");
                    ui.end_row();
                    
                    for idx in tables_start..tables_end {
                        if let Some(table) = self.carve_parts.tables.get(idx) {
                            let selected = self.selected_carve_parts_table_index == Some(idx);
                            if ui.selectable_label(selected, format!("Carve Table {}", idx)).clicked() {
                                self.selected_carve_parts_table_index = Some(idx);
                                self.carve_parts_page = 0;
                                self.view_mode.insert("carve_parts".to_string(), ViewMode::Details);
                            }
                            ui.label(format!("{}", table.carves.len()));
                            ui.label("");
                            ui.end_row();
                        }
                    }
                });
        });
        
        MhfdatApp::pagination_controls(ui, &mut self.carve_parts_tables_page, tables_total_pages);
    }
    
    fn show_carve_parts_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert("carve_parts".to_string(), ViewMode::List);
            self.selected_carve_parts_row_index = None;
            return;
        }
        ui.separator();
        
        let table_idx = self.selected_carve_parts_table_index.unwrap_or(0);
        let Some(table) = self.carve_parts.tables.get_mut(table_idx) else {
            ui.label("No table loaded");
            return;
        };
        
        let total = table.carves.len();
        
        ui.heading(format!("Carve Table {} ({} carves)", table_idx, total));
        ui.separator();
        
        // Add carve button
        if ui.button("Add Carve").clicked() {
            table.carves.push(CarveDrop {
                percentage: 0,
                item_id: 0,
            });
            self.carve_parts_modified = true;
        }
        ui.separator();
        
        let page_size = 50usize;
        let total_pages = (total + page_size - 1) / page_size;
        let page = self.carve_parts_page.min(total_pages.saturating_sub(1) as u32);
        self.carve_parts_page = page;
        
        let start = (page as usize) * page_size;
        let end = (start + page_size).min(total);
        
        MhfdatApp::list_scroll(ui, "carve_parts_details_scroll", |ui| {
            egui::Grid::new("carve_parts_details_grid")
                .striped(true)
                .num_columns(3)
                .show(ui, |ui| {
                    ui.label("Index");
                    ui.label("Percentage");
                    ui.label("Item");
                    ui.end_row();
                    
                    for i in start..end {
                        if let Some(carve) = table.carves.get_mut(i) {
                            ui.label(format!("{}", i));
                            
                            // Percentage (editable)
                            let mut percentage = carve.percentage;
                            if ui.add(egui::DragValue::new(&mut percentage).clamp_range(0..=65535)).changed() {
                                carve.percentage = percentage;
                                self.carve_parts_modified = true;
                            }
                            
                            // Item ID (editable)
                            let mut item_id = carve.item_id;
                            let item_name = self.item_names.get(item_id as usize).cloned().unwrap_or_default();
                            ui.horizontal(|ui| {
                                if ui.add(egui::DragValue::new(&mut item_id).clamp_range(0..=65535)).changed() {
                                    carve.item_id = item_id;
                                    self.carve_parts_modified = true;
                                }
                                ui.label(format!("({})", item_name));
                            });
                            
                            ui.end_row();
                        }
                    }
                });
        });
        
        MhfdatApp::pagination_controls(ui, &mut self.carve_parts_page, total_pages);
    }
    
    fn show_part_break_parts_list(&mut self, ui: &mut egui::Ui) {
        let table_count = self.part_break_parts.tables.len();
        
        MhfdatApp::section_header(ui, &format!("Part Break Drops ({} part break tables)", table_count), |ui| {
            if ui.button("Add Table").clicked() {
                self.part_break_parts.tables.push(PartBreakDropTable { break_drops: Vec::new() });
                self.part_break_parts_modified = true;
                self.part_break_parts_count = self.part_break_parts.tables.len() as u16;
                self.part_break_parts_count_modified = true;
            }
            if ui.button("Export to JSON").clicked() {
                if let Ok(json) = serde_json::to_string_pretty(&self.part_break_parts) {
                    let _ = std::fs::write("part_break_parts.json", json);
                }
            }
            if ui.button("Import from JSON").clicked() {
                if let Ok(json) = std::fs::read_to_string("part_break_parts.json") {
                    if let Ok(data) = serde_json::from_str(&json) {
                        self.part_break_parts = data;
                        self.part_break_parts_modified = true;
                        self.part_break_parts_count = self.part_break_parts.tables.len() as u16;
                        self.part_break_parts_count_modified = true;
                    }
                }
            }
        });
        
        if table_count == 0 {
            ui.label("No part break parts loaded.");
            return;
        }
        
        // List of part break tables
        ui.heading("Part Break Tables");
        
        let tables_page_size = 50usize;
        let tables_total_pages = (table_count + tables_page_size - 1) / tables_page_size;
        let tables_page = self.part_break_parts_tables_page.min(tables_total_pages.saturating_sub(1) as u32);
        self.part_break_parts_tables_page = tables_page;
        
        let tables_start = (tables_page as usize) * tables_page_size;
        let tables_end = (tables_start + tables_page_size).min(table_count);
        
        MhfdatApp::list_scroll(ui, "part_break_parts_tables_scroll", |ui| {
            egui::Grid::new("part_break_parts_tables_grid")
                .striped(true)
                .num_columns(3)
                .show(ui, |ui| {
                    ui.label("Part Break Table");
                    ui.label("Drops");
                    ui.label("Actions");
                    ui.end_row();
                    
                    for idx in tables_start..tables_end {
                        if let Some(table) = self.part_break_parts.tables.get(idx) {
                            let selected = self.selected_part_break_parts_table_index == Some(idx);
                            if ui.selectable_label(selected, format!("Part Break Table {}", idx)).clicked() {
                                self.selected_part_break_parts_table_index = Some(idx);
                                self.part_break_parts_page = 0;
                                self.view_mode.insert("part_break_parts".to_string(), ViewMode::Details);
                            }
                            ui.label(format!("{}", table.break_drops.len()));
                            ui.label("");
                            ui.end_row();
                        }
                    }
                });
        });
        
        MhfdatApp::pagination_controls(ui, &mut self.part_break_parts_tables_page, tables_total_pages);
    }
    
    fn show_part_break_parts_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert("part_break_parts".to_string(), ViewMode::List);
            self.selected_part_break_parts_row_index = None;
            return;
        }
        ui.separator();
        
        let table_idx = self.selected_part_break_parts_table_index.unwrap_or(0);
        let Some(table) = self.part_break_parts.tables.get_mut(table_idx) else {
            ui.label("No table loaded");
            return;
        };
        
        let total = table.break_drops.len();
        
        ui.heading(format!("Part Break Table {} ({} drops)", table_idx, total));
        ui.separator();
        
        // Add drop button
        if ui.button("Add Drop").clicked() {
            table.break_drops.push(PartBreakDrop {
                percentage: 0,
                item_id: 0,
                number: 0,
            });
            self.part_break_parts_modified = true;
        }
        ui.separator();
        
        let page_size = 50usize;
        let total_pages = (total + page_size - 1) / page_size;
        let page = self.part_break_parts_page.min(total_pages.saturating_sub(1) as u32);
        self.part_break_parts_page = page;
        
        let start = (page as usize) * page_size;
        let end = (start + page_size).min(total);
        
        MhfdatApp::list_scroll(ui, "part_break_parts_details_scroll", |ui| {
            egui::Grid::new("part_break_parts_details_grid")
                .striped(true)
                .num_columns(4)
                .show(ui, |ui| {
                    ui.label("Index");
                    ui.label("Percentage");
                    ui.label("Item");
                    ui.label("Number");
                    ui.end_row();
                    
                    for i in start..end {
                        if let Some(drop) = table.break_drops.get_mut(i) {
                            ui.label(format!("{}", i));
                            
                            // Percentage (editable)
                            let mut percentage = drop.percentage;
                            if ui.add(egui::DragValue::new(&mut percentage).clamp_range(0..=65535)).changed() {
                                drop.percentage = percentage;
                                self.part_break_parts_modified = true;
                            }
                            
                            // Item ID (editable)
                            let mut item_id = drop.item_id;
                            let item_name = self.item_names.get(item_id as usize).cloned().unwrap_or_default();
                            ui.horizontal(|ui| {
                                if ui.add(egui::DragValue::new(&mut item_id).clamp_range(0..=65535)).changed() {
                                    drop.item_id = item_id;
                                    self.part_break_parts_modified = true;
                                }
                                ui.label(format!("({})", item_name));
                            });
                            
                            // Number (editable)
                            let mut number = drop.number;
                            if ui.add(egui::DragValue::new(&mut number).clamp_range(0..=65535)).changed() {
                                drop.number = number;
                                self.part_break_parts_modified = true;
                            }
                            
                            ui.end_row();
                        }
                    }
                });
        });
        
        MhfdatApp::pagination_controls(ui, &mut self.part_break_parts_page, total_pages);
    }
    
    fn show_monster_descriptions_list(&mut self, ui: &mut egui::Ui) {
        let desc_count = self.monster_descriptions.len();
        
        MhfdatApp::section_header(ui, &format!("Monster Descriptions ({} entries)", desc_count), |ui| {
            if ui.button("Add Description").clicked() {
                self.monster_descriptions.push(String::new());
                self.monster_descriptions_modified = true;
                self.monster_descriptions_count = self.monster_descriptions.len() as u16;
                self.monster_descriptions_count_modified = true;
            }
            if ui.button("Export to JSON").clicked() {
                if let Ok(json) = serde_json::to_string_pretty(&self.monster_descriptions) {
                    let _ = std::fs::write("monster_descriptions.json", json);
                }
            }
            if ui.button("Import from JSON").clicked() {
                if let Ok(json) = std::fs::read_to_string("monster_descriptions.json") {
                    if let Ok(data) = serde_json::from_str::<Vec<String>>(&json) {
                        self.monster_descriptions = data;
                        self.monster_descriptions_modified = true;
                        self.monster_descriptions_count = self.monster_descriptions.len() as u16;
                        self.monster_descriptions_count_modified = true;
                    }
                }
            }
        });
        
        if desc_count == 0 {
            ui.label("No monster descriptions loaded.");
            return;
        }
        
        ui.heading("Monster Descriptions");
        
        let page_size = 15usize;
        let total_pages = (desc_count + page_size - 1) / page_size;
        let page = self.monster_descriptions_page.min(total_pages.saturating_sub(1) as u32);
        self.monster_descriptions_page = page;
        
        let start = (page as usize) * page_size;
        let end = (start + page_size).min(desc_count);
        
        MhfdatApp::list_scroll(ui, "monster_descriptions_list_scroll", |ui| {
            egui::Grid::new("monster_descriptions_list_grid")
                .striped(true)
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Index");
                    ui.label("Description (preview)");
                    ui.end_row();
                    
                    for idx in start..end {
                        let selected = self.selected_monster_description_index == Some(idx);
                        let preview = if let Some(desc) = self.monster_descriptions.get(idx) {
                            if desc.len() > 50 {
                                format!("{}...", &desc[..50])
                            } else {
                                desc.clone()
                            }
                        } else {
                            String::new()
                        };
                        
                        if ui.selectable_label(selected, format!("{}", idx)).clicked() {
                            self.selected_monster_description_index = Some(idx);
                            self.view_mode.insert("monster_descriptions".to_string(), ViewMode::Details);
                        }
                        ui.label(preview);
                        ui.end_row();
                    }
                });
        });
        
        MhfdatApp::pagination_controls(ui, &mut self.monster_descriptions_page, total_pages);
    }
    
    fn show_monster_descriptions_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert("monster_descriptions".to_string(), ViewMode::List);
            self.selected_monster_description_index = None;
            return;
        }
        ui.separator();
        
        let idx = self.selected_monster_description_index.unwrap_or(0);
        
        ui.heading(format!("Monster Description #{}", idx));
        ui.separator();
        
        // Ensure we have valid data for this index
        if idx >= self.monster_descriptions.len() {
            self.monster_descriptions.resize(idx + 1, String::new());
        }
        
        ui.horizontal(|ui| {
            ui.label("Description:");
        });
        if ui.text_edit_multiline(&mut self.monster_descriptions[idx]).changed() {
            self.monster_descriptions_modified = true;
        }
    }
}
