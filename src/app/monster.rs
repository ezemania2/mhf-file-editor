use super::*;
use crate::model::mhfdat::{CarveDrop, CarveDropTable};
use egui;

impl MhfdatApp {
    pub fn show_monster_tab(&mut self, ui: &mut egui::Ui) {
        use crate::app::ViewMode;
        
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
}
