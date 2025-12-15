use super::*;
use egui;
use crate::utils::automatic_skills::auto_skill_name;

impl MhfdatApp {
    /// Get equipment name based on eq_type and equip_id
    fn get_equip_name_from_autoskill(&self, eq_type: u8, equip_id: u16) -> String {
        match eq_type {
            0x00 => self.legs_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
            0x02 => self.head_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
            0x03 => self.body_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
            0x04 => self.arms_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
            0x05 => self.waist_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
            0x06 => self.melee_weapon_names.get(equip_id as usize).cloned().unwrap_or_default(),
            0x07 => self.ranged_weapon_names.get(equip_id as usize).cloned().unwrap_or_default(),
            _ => "Unknown".to_string(),
        }
    }
    
    /// Get equipment type name
    fn get_eq_type_name(eq_type: u8) -> &'static str {
        match eq_type {
            0x00 => "Legs",
            0x02 => "Head",
            0x03 => "Chest",
            0x04 => "Arms",
            0x05 => "Waist",
            0x06 => "Melee",
            0x07 => "Ranged",
            _ => "Unknown",
        }
    }
    
    pub fn show_automatic_skills_tab(&mut self, ui: &mut egui::Ui) {
        // Initialize view mode if not present
        if !self.view_mode.contains_key("automatic_skills") {
            self.view_mode.insert("automatic_skills".to_string(), ViewMode::List);
        }
        
        match self.view_mode.get("automatic_skills").cloned().unwrap_or(ViewMode::List) {
            ViewMode::List => self.show_automatic_skills_list(ui),
            ViewMode::Details => self.show_automatic_skills_details(ui),
        }
    }
    
    fn show_automatic_skills_list(&mut self, ui: &mut egui::Ui) {
        MhfdatApp::section_header(ui, "Automatic Skills Table", |ui| {
            if ui.button("Export to JSON").clicked() {
                if let Ok(text) = serde_json::to_string_pretty(&self.automatic_skills) {
                    let _ = std::fs::write("automatic_skills.json", text);
                }
            }
        });
        
        // Filters and search
        ui.horizontal_wrapped(|ui| {
            ui.label("Type:");
            egui::ComboBox::from_id_source("auto_skills_eq_type_filter")
                .selected_text(match self.automatic_skills_eq_type_filter {
                    Some(0x00) => "Legs",
                    Some(0x02) => "Head",
                    Some(0x03) => "Chest",
                    Some(0x04) => "Arms",
                    Some(0x05) => "Waist",
                    Some(0x06) => "Melee",
                    Some(0x07) => "Ranged",
                    _ => "All",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.automatic_skills_eq_type_filter, None, "All");
                    ui.selectable_value(&mut self.automatic_skills_eq_type_filter, Some(0x00), "Legs");
                    ui.selectable_value(&mut self.automatic_skills_eq_type_filter, Some(0x02), "Head");
                    ui.selectable_value(&mut self.automatic_skills_eq_type_filter, Some(0x03), "Chest");
                    ui.selectable_value(&mut self.automatic_skills_eq_type_filter, Some(0x04), "Arms");
                    ui.selectable_value(&mut self.automatic_skills_eq_type_filter, Some(0x05), "Waist");
                    ui.selectable_value(&mut self.automatic_skills_eq_type_filter, Some(0x06), "Melee");
                    ui.selectable_value(&mut self.automatic_skills_eq_type_filter, Some(0x07), "Ranged");
                });
            
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.automatic_skills_search);
            
            if ui.button("Add New").clicked() {
                let new_entry = AutomaticSkill {
                    unk00: 0,
                    eq_type: 0x02, // Default to Head
                    equip_id: 0,
                    skill_id: 0,
                    padding: [0; 2],
                };
                self.automatic_skills.push(new_entry);
                self.selected_automatic_skill_index = Some(self.automatic_skills.len() - 1);
                self.view_mode.insert("automatic_skills".to_string(), ViewMode::Details);
                self.automatic_skills_modified = true;
            }
        });
        
        // Filter entries
        let lowered = self.automatic_skills_search.to_lowercase();
        let entries: Vec<(usize, &AutomaticSkill)> = self.automatic_skills.iter()
            .enumerate()
            .filter(|(_, e)| {
                // Type filter
                if let Some(filter_type) = self.automatic_skills_eq_type_filter {
                    if e.eq_type != filter_type { return false; }
                }
                // Name filter (search in both equipment name and skill name)
                if !lowered.is_empty() {
                    let equip_name = self.get_equip_name_from_autoskill(e.eq_type, e.equip_id).to_lowercase();
                    let skill_name = auto_skill_name(e.skill_id).to_lowercase();
                    if !equip_name.contains(&lowered) && !skill_name.contains(&lowered) { 
                        return false; 
                    }
                }
                true
            })
            .collect();
        
        // Pagination
        let per_page = 20usize;
        let total = entries.len();
        let total_pages = (total + per_page - 1) / per_page;
        let current = (self.automatic_skills_page as usize).min(total_pages.saturating_sub(1));
        if current != self.automatic_skills_page as usize { self.automatic_skills_page = current as u32; }
        MhfdatApp::pagination_controls(ui, &mut self.automatic_skills_page, total_pages);
        let start = current * per_page;
        let end = (start + per_page).min(total);
        
        // Pre-compute names to avoid borrowing self in closure
        let entries_with_names: Vec<_> = entries[start..end].iter()
            .map(|(idx, entry)| {
                let eq_type = entry.eq_type;
                let equip_id = entry.equip_id;
                let name = self.get_equip_name_from_autoskill(eq_type, equip_id);
                (*idx, entry, name)
            })
            .collect();
        
        MhfdatApp::list_scroll(ui, "automatic_skills_scroll", |ui| {
            egui::Grid::new("automatic_skills_grid").striped(true).show(ui, |ui| {
                ui.label("Index");
                ui.label("Unk00");
                ui.label("Type");
                ui.label("Equip ID");
                ui.label("Equip Name");
                ui.label("Skill ID");
                ui.label("Skill Name");
                ui.end_row();
                
                for (idx, entry, name) in entries_with_names.iter() {
                    let selected = self.selected_automatic_skill_index == Some(*idx);
                    if ui.selectable_label(selected, format!("{}", idx)).clicked() {
                        self.selected_automatic_skill_index = Some(*idx);
                        self.view_mode.insert("automatic_skills".to_string(), ViewMode::Details);
                    }
                    
                    // Copy packed fields to local variables
                    let unk00 = entry.unk00;
                    let eq_type = entry.eq_type;
                    let equip_id = entry.equip_id;
                    let skill_id = entry.skill_id;
                    
                    ui.label(format!("{}", unk00));
                    ui.label(Self::get_eq_type_name(eq_type));
                    ui.label(format!("{}", equip_id));
                    ui.label(name);
                    ui.label(format!("{}", skill_id));
                    ui.label(auto_skill_name(skill_id));
                    ui.end_row();
                }
            });
        });
    }
    
    fn show_automatic_skills_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert("automatic_skills".to_string(), ViewMode::List);
            return;
        }
        
        if let Some(index) = self.selected_automatic_skill_index {
            if let Some(mut entry) = self.automatic_skills.get(index).cloned() {
                ui.heading(format!("Edit Automatic Skill #{}", index));
                ui.separator();
                
                // Get name before we mutate
                let equip_name = self.get_equip_name_from_autoskill(entry.eq_type, entry.equip_id);
                
                // Unk00
                ui.horizontal(|ui| {
                    ui.label("Unk00:");
                    let mut val = entry.unk00 as i32;
                    if ui.add(egui::DragValue::new(&mut val).speed(1)).changed() {
                        entry.unk00 = val as u8;
                    }
                });
                
                // Equipment type
                ui.horizontal(|ui| {
                    ui.label("Equipment Type:");
                    egui::ComboBox::from_id_source("edit_auto_skill_eq_type")
                        .selected_text(Self::get_eq_type_name(entry.eq_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut entry.eq_type, 0x00, "Legs");
                            ui.selectable_value(&mut entry.eq_type, 0x02, "Head");
                            ui.selectable_value(&mut entry.eq_type, 0x03, "Chest");
                            ui.selectable_value(&mut entry.eq_type, 0x04, "Arms");
                            ui.selectable_value(&mut entry.eq_type, 0x05, "Waist");
                            ui.selectable_value(&mut entry.eq_type, 0x06, "Melee");
                            ui.selectable_value(&mut entry.eq_type, 0x07, "Ranged");
                        });
                });
                
                // Equipment ID
                ui.horizontal(|ui| {
                    ui.label("Equipment ID:");
                    let mut id = entry.equip_id as i32;
                    if ui.add(egui::DragValue::new(&mut id).speed(1)).changed() {
                        entry.equip_id = id as u16;
                    }
                    ui.label(format!("({})", equip_name));
                });
                
                // Skill ID with ComboBox
                ui.horizontal(|ui| {
                    ui.label("Skill:");
                    let mut skill_id = entry.skill_id; // Copy to avoid packed field reference
                    
                    // Search field outside the combo
                    ui.add(egui::TextEdit::singleline(&mut self.automatic_skills_skill_search)
                        .hint_text("Search...").desired_width(100.0));
                    
                    let q = self.automatic_skills_skill_search.to_lowercase();
                    egui::ComboBox::from_id_source("edit_auto_skill_id")
                        .selected_text(format!("{} ({})", skill_id, auto_skill_name(skill_id)))
                        .show_ui(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                    for (id, name) in crate::utils::automatic_skills::AUTO_SKILL_LIST {
                                        if q.is_empty() || name.to_lowercase().contains(&q) {
                                            if ui.selectable_value(&mut skill_id, *id, format!("{} - {}", id, name)).clicked() {}
                                        }
                                    }
                                });
                        });
                    entry.skill_id = skill_id; // Write back
                });
                
                // Delete button
                ui.separator();
                if ui.button("Delete this entry").clicked() {
                    self.automatic_skills.remove(index);
                    self.selected_automatic_skill_index = None;
                    self.view_mode.insert("automatic_skills".to_string(), ViewMode::List);
                    self.automatic_skills_modified = true;
                    return;
                }
                
                // Write back
                if let Some(slot) = self.automatic_skills.get_mut(index) {
                    *slot = entry;
                    self.automatic_skills_modified = true;
                }
            }
        }
    }
}

