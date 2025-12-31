use super::*;
use egui;
use crate::utils::automatic_skills::{auto_skill_name, AUTO_SKILL_LIST};

const PAGE_SIZE: usize = 20;
const EQ_TYPES: [(u8, &str); 7] = [
    (0x00, "Legs"),
    (0x02, "Head"),
    (0x03, "Chest"),
    (0x04, "Arms"),
    (0x05, "Waist"),
    (0x06, "Melee"),
    (0x07, "Ranged"),
];

impl MhfdatApp {
    fn get_equip_name_from_autoskill(&self, eq_type: u8, equip_id: u16) -> String {
        let idx = equip_id as usize;
        match eq_type {
            0x00 => self.legs_armor_names.get(idx),
            0x02 => self.head_armor_names.get(idx),
            0x03 => self.body_armor_names.get(idx),
            0x04 => self.arms_armor_names.get(idx),
            0x05 => self.waist_armor_names.get(idx),
            0x06 => self.melee_weapon_names.get(idx),
            0x07 => self.ranged_weapon_names.get(idx),
            _ => None,
        }.cloned().unwrap_or_else(|| "Unknown".to_string())
    }

    fn get_eq_type_name(eq_type: u8) -> &'static str {
        EQ_TYPES.iter().find(|(t, _)| *t == eq_type).map(|(_, n)| *n).unwrap_or("Unknown")
    }

    pub fn show_automatic_skills_tab(&mut self, ui: &mut egui::Ui) {
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
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_filename("automatic_skills.json")
                    .show_save_single_file() 
                {
                    if let Ok(text) = serde_json::to_string_pretty(&self.automatic_skills) {
                        let _ = std::fs::write(path.to_str().unwrap_or("automatic_skills.json"), text);
                    }
                }
            }
            if ui.button("Import from JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .show_open_single_file() 
                {
                    if let Ok(data) = std::fs::read_to_string(path.to_str().unwrap_or("")) {
                        if let Ok(imported) = serde_json::from_str::<Vec<crate::model::mhfdat::AutomaticSkill>>(&data) {
                            self.automatic_skills = imported;
                            self.automatic_skills_modified = true;
                        }
                    }
                }
            }
        });

        // Filters
        ui.horizontal_wrapped(|ui| {
            ui.label("Type:");
            let filter_text = self.automatic_skills_eq_type_filter
                .map(Self::get_eq_type_name)
                .unwrap_or("All");
            egui::ComboBox::from_id_source("auto_skills_eq_type_filter")
                .selected_text(filter_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.automatic_skills_eq_type_filter, None, "All");
                    for (id, name) in EQ_TYPES {
                        ui.selectable_value(&mut self.automatic_skills_eq_type_filter, Some(id), name);
                    }
                });

            ui.label("Search:");
            ui.text_edit_singleline(&mut self.automatic_skills_search);

            if ui.button("Add New").clicked() {
                self.automatic_skills.push(AutomaticSkill {
                    is_armor: false,
                    eq_type: 0x02,
                    equip_id: 0,
                    skill_id: 0,
                    padding: [0; 2],
                });
                self.automatic_skills_count_limiter = self.automatic_skills.len() as u16;
                self.automatic_skills_count_limiter_modified = true;
                self.selected_automatic_skill_index = Some(self.automatic_skills.len() - 1);
                self.view_mode.insert("automatic_skills".to_string(), ViewMode::Details);
                self.automatic_skills_modified = true;
            }
        });

        // Filter entries
        let search = self.automatic_skills_search.to_lowercase();
        let type_filter = self.automatic_skills_eq_type_filter;
        
        let filtered: Vec<_> = self.automatic_skills.iter().enumerate()
            .filter(|(_, e)| {
                if let Some(t) = type_filter {
                    if e.eq_type != t { return false; }
                }
                if !search.is_empty() {
                    let equip_name = self.get_equip_name_from_autoskill(e.eq_type, e.equip_id).to_lowercase();
                    let skill_name = auto_skill_name(e.skill_id).to_lowercase();
                    if !equip_name.contains(&search) && !skill_name.contains(&search) {
                        return false;
                    }
                }
                true
            })
            .map(|(i, e)| (i, e.is_armor, e.eq_type, e.equip_id, e.skill_id))
            .collect();

        // Pagination
        let total = filtered.len();
        let total_pages = (total + PAGE_SIZE - 1) / PAGE_SIZE;
        let page = (self.automatic_skills_page as usize).min(total_pages.saturating_sub(1));
        if page != self.automatic_skills_page as usize {
            self.automatic_skills_page = page as u32;
        }
        MhfdatApp::pagination_controls(ui, &mut self.automatic_skills_page, total_pages);

        let start = page * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(total);

        // Pre-compute names
        let display_data: Vec<_> = filtered[start..end].iter()
            .map(|(idx, is_armor, eq_type, equip_id, skill_id)| {
                let name = self.get_equip_name_from_autoskill(*eq_type, *equip_id);
                (*idx, *is_armor, *eq_type, *equip_id, name, *skill_id)
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

                for (idx, is_armor, eq_type, equip_id, name, skill_id) in &display_data {
                    let selected = self.selected_automatic_skill_index == Some(*idx);
                    if ui.selectable_label(selected, format!("{}", idx)).clicked() {
                        self.selected_automatic_skill_index = Some(*idx);
                        self.view_mode.insert("automatic_skills".to_string(), ViewMode::Details);
                    }
                    ui.label(format!("{}", is_armor));
                    ui.label(Self::get_eq_type_name(*eq_type));
                    ui.label(format!("{}", equip_id));
                    ui.label(name);
                    ui.label(format!("{}", skill_id));
                    ui.label(auto_skill_name(*skill_id));
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

        let Some(index) = self.selected_automatic_skill_index else { return };
        let Some(mut entry) = self.automatic_skills.get(index).cloned() else { return };

        ui.heading(format!("Edit Automatic Skill #{}", index));
        ui.separator();

        let equip_name = self.get_equip_name_from_autoskill(entry.eq_type, entry.equip_id);

        egui::Grid::new("auto_skill_edit_grid").show(ui, |ui| {
            ui.label("Is Armor:");
            let mut val = entry.is_armor;
            if ui.checkbox(&mut val, "").changed() {
                entry.is_armor = val;
            }
            ui.end_row();

            ui.label("Equipment Type:");
            egui::ComboBox::from_id_source("edit_auto_skill_eq_type")
                .selected_text(Self::get_eq_type_name(entry.eq_type))
                .show_ui(ui, |ui| {
                    for (id, name) in EQ_TYPES {
                        ui.selectable_value(&mut entry.eq_type, id, name);
                    }
                });
            ui.end_row();

            ui.label("Equipment ID:");
            ui.horizontal(|ui| {
                let mut id = entry.equip_id as i32;
                if ui.add(egui::DragValue::new(&mut id).speed(1)).changed() {
                    entry.equip_id = id as u16;
                }
                ui.label(format!("({})", equip_name));
            });
            ui.end_row();

            ui.label("Skill:");
            ui.horizontal(|ui| {
                let mut skill_id = entry.skill_id;
                ui.add(egui::TextEdit::singleline(&mut self.automatic_skills_skill_search)
                    .hint_text("Search...").desired_width(100.0));

                let q = self.automatic_skills_skill_search.to_lowercase();
                egui::ComboBox::from_id_source("edit_auto_skill_id")
                    .selected_text(format!("{} ({})", skill_id, auto_skill_name(skill_id)))
                    .show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for (id, name) in AUTO_SKILL_LIST {
                                if q.is_empty() || name.to_lowercase().contains(&q) {
                                    ui.selectable_value(&mut skill_id, *id, format!("{} - {}", id, name));
                                }
                            }
                        });
                    });
                entry.skill_id = skill_id;
            });
            ui.end_row();
        });

        ui.separator();
        if ui.button("Delete this entry").clicked() {
            self.automatic_skills.remove(index);
            self.selected_automatic_skill_index = None;
            self.view_mode.insert("automatic_skills".to_string(), ViewMode::List);
            self.automatic_skills_modified = true;
            return;
        }

        if let Some(slot) = self.automatic_skills.get_mut(index) {
            *slot = entry;
            self.automatic_skills_modified = true;
        }
    }
}
