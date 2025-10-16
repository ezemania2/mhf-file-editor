use super::*;
use egui;
use std::io::{Cursor, Read, Seek, SeekFrom};
use crate::utils::skills::{skill_name, SKILL_LIST};
use crate::utils::weapon_patterns::{zenith_skill_name, ZENITH_SKILL_LIST};
use std::fs::File;
use std::io::Write;
use serde_json;
use crate::core::mhfdat::{read_equipment_counts, write_equipment_counts};

impl MhfdatApp {
    /// Compute next armor model_id (male/female) from existing entries up to real_count
    fn next_armor_model_id_from(entries: &[MhfdatEquipment], real_count: usize) -> u16 {
        entries
            .iter()
            .take(real_count)
            .map(|a| a.model_id_male.max(a.model_id_female))
            .max()
            .map(|max_id| max_id.saturating_add(1))
            .unwrap_or(0)
    }

    /// Recompute EquipmentCounts strictly from number of entries (max index + 1) for each armor category
    pub(crate) fn refresh_equipment_counts_from_entries(&mut self) {
        if let Some(mut counts) = read_equipment_counts(&self.buffer) {
            counts.numHeadA = self.head_armors.len() as u16;
            counts.numBodyA = self.body_armors.len() as u16;
            counts.numArmA = self.arms_armors.len() as u16;
            counts.numWaistA = self.waist_armors.len() as u16;
            counts.numLegA = self.legs_armors.len() as u16;
            let _ = write_equipment_counts(&mut self.buffer, &counts);
            self.equipment_counts = Some(counts);
        }
    }

    pub fn show_armor_tab(&mut self, ui: &mut egui::Ui) {
        // Initialize view modes if not present
        if !self.view_mode.contains_key("armor") {
            self.view_mode.insert("armor".to_string(), ViewMode::List);
        }

        // Add armor category tabs (wrapped)
        ui.horizontal_wrapped(|ui| {
            if ui.selectable_label(self.armor_tab == ArmorTab::Head, "Head").clicked() {
                self.armor_tab = ArmorTab::Head;
            }
            if ui.selectable_label(self.armor_tab == ArmorTab::Body, "Body").clicked() {
                self.armor_tab = ArmorTab::Body;
            }
            if ui.selectable_label(self.armor_tab == ArmorTab::Arms, "Arms").clicked() {
                self.armor_tab = ArmorTab::Arms;
            }
            if ui.selectable_label(self.armor_tab == ArmorTab::Waist, "Waist").clicked() {
                self.armor_tab = ArmorTab::Waist;
            }
            if ui.selectable_label(self.armor_tab == ArmorTab::Legs, "Legs").clicked() {
                self.armor_tab = ArmorTab::Legs;
            }
        });
        ui.separator();

        match self.view_mode.get("armor").unwrap() {
            ViewMode::List => self.show_armor_list_view(ui),
            ViewMode::Details => self.show_armor_details_view(ui),
        }
    }

    pub fn show_armor_list_view(&mut self, ui: &mut egui::Ui) {
        let (armors, names) = match self.armor_tab {
            ArmorTab::Head => (&mut self.head_armors, &mut self.head_armor_names),
            ArmorTab::Body => (&mut self.body_armors, &mut self.body_armor_names),
            ArmorTab::Arms => (&mut self.arms_armors, &mut self.arms_armor_names),
            ArmorTab::Waist => (&mut self.waist_armors, &mut self.waist_armor_names),
            ArmorTab::Legs => (&mut self.legs_armors, &mut self.legs_armor_names),
        };

        let armor_type = match self.armor_tab {
            ArmorTab::Head => "Head",
            ArmorTab::Body => "Body",
            ArmorTab::Arms => "Arms",
            ArmorTab::Waist => "Waist",
            ArmorTab::Legs => "Legs",
        };

        // Compter jusqu'à la première armure avec model_id_male == 0xFFFF
        let mut real_count = 0;
        for armor in armors.iter() {
            if armor.model_id_male == 0xFFFF {
                break;
            }
            real_count += 1;
        }
        let max_count = real_count.min(armors.len());

        let armor_type_str = match self.armor_tab { 
            ArmorTab::Head=>"head", 
            ArmorTab::Body=>"body", 
            ArmorTab::Arms=>"arms", 
            ArmorTab::Waist=>"waist", 
            ArmorTab::Legs=>"legs" 
        };
        
        MhfdatApp::section_header(ui, &format!("{} Armor (found: {})", armor_type, max_count), |ui| {
            if ui.button("Export JSON").clicked() {
                // This will be handled after the closure
            }
            if ui.button("Add New").clicked() {
                // This will be handled after the closure
            }
        });

        // Handle Export JSON button click
            if ui.button("Export JSON").clicked() {
            let export_armors: Vec<ArmorExport> = armors
                    .iter()
                    .enumerate()
                    .map(|(index, armor)| {
                    let name = names.get(index).cloned().unwrap_or_default();
                        ArmorExport::from_armor_with_data(armor, &name, index)
                    })
                    .collect();
            let filename = format!("{}_armor.json", armor_type_str);
                if let Ok(json) = serde_json::to_string_pretty(&export_armors) {
                    let _ = std::fs::write(&filename, json);
                }
            }

        // Handle Add New button click
        if ui.button("Add New").clicked() {
            let mut new_armor = MhfdatEquipment::default();
            let next_model_id = 0;
            new_armor.model_id_male = 0;
            new_armor.model_id_female = 0;
            new_armor.equipable_by = 0x0F; // All flags enabled by default
            new_armor.base_slots = 0;
            new_armor.max_slots = 3;
            
            armors.insert(real_count, new_armor);
            names.insert(real_count, format!("New {} Armor", armor_type));
            // Note: Other self modifications will be handled after the function scope
        }

        // Search and filters
        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
            ui.checkbox(&mut self.show_dummy_weapons, "Show Dummy Armor");
        });

        if max_count == 0 {
            ui.colored_label(egui::Color32::YELLOW, format!("Warning: No {} armor found!", armor_type));
        } else {
            // Filter armors
            let query = self.search_query.to_lowercase();
            let filtered_armors: Vec<(usize, &MhfdatEquipment)> = armors.iter()
                .enumerate()
                .take(max_count)
                .filter(|(i, armor)| {
                    let armor_name = names.get(*i).cloned().unwrap_or_default();
                    
                    // Copy fields to local variables to avoid unaligned references
                    let model_id_male = armor.model_id_male;
                    let model_id_female = armor.model_id_female;
                    let rarity = armor.rarity;
                    let base_defense = armor.base_defense;
                    let equipable_by = armor.equipable_by;

                    // Dummy armor detection
                    let is_dummy = model_id_male == 0x0000
                        && model_id_female == 0x0000
                        && rarity == 0x00
                        && equipable_by == 0x00
                        && armor.zenny_cost == 0x00000000
                        && base_defense == 0x0000
                        && armor.zenith_skill == 0x0000;

                    // Apply dummy filter
                    if self.show_dummy_weapons {
                        if !is_dummy { return false; }
                    } else {
                        if is_dummy { return false; }
                    }

                    // Apply search filter
                    if !query.is_empty() && !armor_name.to_lowercase().contains(&query) {
                        return false;
                    }

                    true
                })
                .collect();

            // Calculate pagination
            let entries_per_page = 15;
            let total_pages = if filtered_armors.is_empty() { 1 } else { 
                ((filtered_armors.len() + entries_per_page - 1) / entries_per_page).max(1)
            };
            let current_page = self.armor_page as usize;
            let start_idx = current_page * entries_per_page;
            let end_idx = (start_idx + entries_per_page).min(filtered_armors.len());

            // Armor list
            MhfdatApp::list_scroll(ui, "armor_list_scroll", |ui| {
                egui::Grid::new("armor_list_grid")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("ID");
                        ui.label("Name");
                        ui.label("Rarity");
                        ui.label("Defense");
                        ui.label("Element");
                        ui.label("Slots");
                        ui.label("Type");
                        ui.end_row();

                        for (original_idx, armor) in filtered_armors[start_idx..end_idx].iter() {
                            let i = *original_idx;
                            let armor_name = names.get(i).cloned().unwrap_or_default();

                            // Copy fields to local variables to avoid unaligned references
                            let model_id_male = armor.model_id_male;
                            let model_id_female = armor.model_id_female;
                            let rarity = armor.rarity;
                            let base_defense = armor.base_defense;
                            let base_slots = armor.base_slots;
                            let max_slots = armor.max_slots;
                            let fire_res = armor.fire_res;
                            let water_res = armor.water_res;
                            let thunder_res = armor.thunder_res;
                            let dragon_res = armor.dragon_res;
                            let ice_res = armor.ice_res;
                            let equipable_by = armor.equipable_by;

                            let selected = self.selected_armor_index == Some(i);
                            if ui.selectable_label(selected, format!("{}", i + 1)).clicked() {
                                self.selected_armor_index = Some(i);
                                *self.view_mode.get_mut("armor").unwrap() = ViewMode::Details;
                            }
                            ui.label(&armor_name);
                            ui.label(format!("{}", rarity + 1));
                            ui.label(format!("{}", base_defense));
                            ui.label(format!("Fire:{} Water:{} Thunder:{} Dragon:{} Ice:{}", 
                                fire_res, water_res, thunder_res, 
                                dragon_res, ice_res));
                            ui.label(format!("{}/{}", base_slots, max_slots));
                            ui.label(armor_type_name(equipable_by));
                            ui.end_row();
                        }
                    });
            });

        // Pagination controls
            MhfdatApp::pagination_controls(ui, &mut self.armor_page, total_pages);
        }
    }

    pub fn show_armor_details_view(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            *self.view_mode.get_mut("armor").unwrap() = ViewMode::List;
            return;
        }

        if let Some(index) = self.selected_armor_index {
            let (armors, names) = match self.armor_tab {
                ArmorTab::Head => (&mut self.head_armors, &mut self.head_armor_names),
                ArmorTab::Body => (&mut self.body_armors, &mut self.body_armor_names),
                ArmorTab::Arms => (&mut self.arms_armors, &mut self.arms_armor_names),
                ArmorTab::Waist => (&mut self.waist_armors, &mut self.waist_armor_names),
                ArmorTab::Legs => (&mut self.legs_armors, &mut self.legs_armor_names),
            };

            let armor_type = match self.armor_tab {
            ArmorTab::Head => "Head",
            ArmorTab::Body => "Body",
            ArmorTab::Arms => "Arms",
            ArmorTab::Waist => "Waist",
            ArmorTab::Legs => "Legs",
        };

            if let Some(armor) = armors.get_mut(index) {
                let name = names.get(index).cloned().unwrap_or_default();
                
                ui.heading(format!("Edit {} Armor #{}", armor_type, index));
                ui.separator();
                
                // Editable name
                            ui.horizontal(|ui| {
                                ui.label("Name:");
                                let mut name_edit = name.clone();
                                if ui.text_edit_singleline(&mut name_edit).changed() {
                        if let Some(name_ref) = names.get_mut(index) {
                                        *name_ref = name_edit;
                                    }
                                }
                            });
                
                ui.separator();
                Self::render_armor_details(ui, armor, 
                    &mut self.armor_skill1_search,
                    &mut self.armor_skill2_search,
                    &mut self.armor_skill3_search,
                    &mut self.armor_skill4_search,
                    &mut self.armor_skill5_search,
                    &mut self.armor_zenith_skill_search
                );
            }
        } else {
            ui.label("No armor selected");
        }
    }

    pub fn render_armor_details(
        ui: &mut egui::Ui, 
        armor: &mut MhfdatEquipment,
        armor_skill1_search: &mut String,
        armor_skill2_search: &mut String,
        armor_skill3_search: &mut String,
        armor_skill4_search: &mut String,
        armor_skill5_search: &mut String,
        armor_zenith_skill_search: &mut String,
    ) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Basic Stats
            ui.collapsing("Basic Stats", |ui| {
                let mut model_id_male = armor.model_id_male;
                let mut model_id_female = armor.model_id_female;
                let mut rarity = armor.rarity;
                let mut max_level = armor.max_level;
                let mut base_defense = armor.base_defense;

                Self::render_editable_field(ui, "Model ID Male", &mut model_id_male);
                Self::render_editable_field(ui, "Model ID Female", &mut model_id_female);
                Self::render_editable_field(ui, "Rarity", &mut rarity);
                Self::render_editable_field(ui, "Max Level", &mut max_level);
                Self::render_editable_field(ui, "Base Defense", &mut base_defense);

                armor.model_id_male = model_id_male;
                armor.model_id_female = model_id_female;
                armor.rarity = rarity;
                armor.max_level = max_level;
                armor.base_defense = base_defense;
            });

            // Equipment Flags
            ui.collapsing("Equipment Flags", |ui| {
                                    let (is_male, is_female, is_blade, is_gunner) = Self::get_equipment_flags(armor.equipable_by);
                                    let mut male = is_male;
                                    let mut female = is_female;
                                    let mut blade = is_blade;
                                    let mut gunner = is_gunner;

                                    ui.horizontal(|ui| {
                                        ui.label("Gender:");
                                        if ui.checkbox(&mut male, "Male").changed() || 
                                           ui.checkbox(&mut female, "Female").changed() {
                                            Self::set_equipment_flags(armor, male, female, blade, gunner);
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("Role:");
                                        if ui.checkbox(&mut blade, "Blademaster").changed() || 
                                           ui.checkbox(&mut gunner, "Gunner").changed() {
                                            Self::set_equipment_flags(armor, male, female, blade, gunner);
                                        }
                                    });
            });

            // Resistances
            ui.collapsing("Resistances", |ui| {
                let mut fire_res = armor.fire_res;
                let mut water_res = armor.water_res;
                let mut thunder_res = armor.thunder_res;
                let mut dragon_res = armor.dragon_res;
                let mut ice_res = armor.ice_res;

                Self::render_editable_field(ui, "Fire Res", &mut fire_res);
                Self::render_editable_field(ui, "Water Res", &mut water_res);
                Self::render_editable_field(ui, "Thunder Res", &mut thunder_res);
                Self::render_editable_field(ui, "Dragon Res", &mut dragon_res);
                Self::render_editable_field(ui, "Ice Res", &mut ice_res);

                armor.fire_res = fire_res;
                armor.water_res = water_res;
                armor.thunder_res = thunder_res;
                armor.dragon_res = dragon_res;
                armor.ice_res = ice_res;
            });

            // Skills
            ui.collapsing("Skills", |ui| {
                let mut skill_id1 = armor.skill_id1;
                let mut skill_pts1 = armor.skill_pts1;
                let mut skill_id2 = armor.skill_id2;
                let mut skill_pts2 = armor.skill_pts2;
                let mut skill_id3 = armor.skill_id3;
                let mut skill_pts3 = armor.skill_pts3;
                let mut skill_id4 = armor.skill_id4;
                let mut skill_pts4 = armor.skill_pts4;
                let mut skill_id5 = armor.skill_id5;
                let mut skill_pts5 = armor.skill_pts5;
                let mut zenith_skill = armor.zenith_skill;

                                    // Skill 1
                                    ui.horizontal(|ui| {
                                        ui.label("Skill 1:");
                    ui.add(egui::TextEdit::singleline(armor_skill1_search)
                        .hint_text("Search...").desired_width(100.0));
                    
                    let q = armor_skill1_search.to_lowercase();
                    let current_text = SKILL_LIST.iter()
                        .find(|(v, _)| *v == skill_id1)
                        .map(|(_, name)| *name)
                        .unwrap_or("Unknown");
                    
                    egui::ComboBox::from_id_source("skill1_combo")
                        .selected_text(current_text)
                                            .show_ui(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                                for (id, name) in SKILL_LIST {
                                        if q.is_empty() || name.to_lowercase().contains(&q) {
                                            ui.selectable_value(&mut skill_id1, *id, format!("{} - {}", id, name));
                                        }
                                                }
                                            });
                        });
                                        
                                        ui.label("Points:");
                    ui.add(egui::DragValue::new(&mut skill_pts1).speed(1.0).clamp_range(-10..=10));
                                    });

                                    // Skill 2
                                    ui.horizontal(|ui| {
                                        ui.label("Skill 2:");
                    ui.add(egui::TextEdit::singleline(armor_skill2_search)
                        .hint_text("Search...").desired_width(100.0));
                    
                    let q = armor_skill2_search.to_lowercase();
                    let current_text = SKILL_LIST.iter()
                        .find(|(v, _)| *v == skill_id2)
                        .map(|(_, name)| *name)
                        .unwrap_or("Unknown");
                    
                    egui::ComboBox::from_id_source("skill2_combo")
                        .selected_text(current_text)
                                            .show_ui(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                                for (id, name) in SKILL_LIST {
                                        if q.is_empty() || name.to_lowercase().contains(&q) {
                                            ui.selectable_value(&mut skill_id2, *id, format!("{} - {}", id, name));
                                        }
                                                }
                                            });
                        });
                                        
                                        ui.label("Points:");
                    ui.add(egui::DragValue::new(&mut skill_pts2).speed(1.0).clamp_range(-10..=10));
                                    });

                                    // Skill 3
                                    ui.horizontal(|ui| {
                                        ui.label("Skill 3:");
                    ui.add(egui::TextEdit::singleline(armor_skill3_search)
                        .hint_text("Search...").desired_width(100.0));
                    
                    let q = armor_skill3_search.to_lowercase();
                    let current_text = SKILL_LIST.iter()
                        .find(|(v, _)| *v == skill_id3)
                        .map(|(_, name)| *name)
                        .unwrap_or("Unknown");
                    
                    egui::ComboBox::from_id_source("skill3_combo")
                        .selected_text(current_text)
                                            .show_ui(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                                for (id, name) in SKILL_LIST {
                                        if q.is_empty() || name.to_lowercase().contains(&q) {
                                            ui.selectable_value(&mut skill_id3, *id, format!("{} - {}", id, name));
                                        }
                                                }
                                            });
                        });
                                        
                                        ui.label("Points:");
                    ui.add(egui::DragValue::new(&mut skill_pts3).speed(1.0).clamp_range(-10..=10));
                                    });

                                    // Skill 4
                                    ui.horizontal(|ui| {
                                        ui.label("Skill 4:");
                    ui.add(egui::TextEdit::singleline(armor_skill4_search)
                        .hint_text("Search...").desired_width(100.0));
                    
                    let q = armor_skill4_search.to_lowercase();
                    let current_text = SKILL_LIST.iter()
                        .find(|(v, _)| *v == skill_id4)
                        .map(|(_, name)| *name)
                        .unwrap_or("Unknown");
                    
                    egui::ComboBox::from_id_source("skill4_combo")
                        .selected_text(current_text)
                                            .show_ui(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                                for (id, name) in SKILL_LIST {
                                        if q.is_empty() || name.to_lowercase().contains(&q) {
                                            ui.selectable_value(&mut skill_id4, *id, format!("{} - {}", id, name));
                                        }
                                                }
                                            });
                        });
                                        
                                        ui.label("Points:");
                    ui.add(egui::DragValue::new(&mut skill_pts4).speed(1.0).clamp_range(-10..=10));
                                    });

                                    // Skill 5
                                    ui.horizontal(|ui| {
                                        ui.label("Skill 5:");
                    ui.add(egui::TextEdit::singleline(armor_skill5_search)
                        .hint_text("Search...").desired_width(100.0));
                    
                    let q = armor_skill5_search.to_lowercase();
                    let current_text = SKILL_LIST.iter()
                        .find(|(v, _)| *v == skill_id5)
                        .map(|(_, name)| *name)
                        .unwrap_or("Unknown");
                    
                    egui::ComboBox::from_id_source("skill5_combo")
                        .selected_text(current_text)
                                            .show_ui(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                                for (id, name) in SKILL_LIST {
                                        if q.is_empty() || name.to_lowercase().contains(&q) {
                                            ui.selectable_value(&mut skill_id5, *id, format!("{} - {}", id, name));
                                        }
                                                }
                                            });
                        });
                                        
                                        ui.label("Points:");
                    ui.add(egui::DragValue::new(&mut skill_pts5).speed(1.0).clamp_range(-10..=10));
                                    });

                                    // Zenith Skill
                                    ui.horizontal(|ui| {
                                        ui.label("Zenith Skill:");
                    ui.add(egui::TextEdit::singleline(armor_zenith_skill_search)
                        .hint_text("Search...").desired_width(100.0));
                    
                    let q = armor_zenith_skill_search.to_lowercase();
                    let current_text = ZENITH_SKILL_LIST.iter()
                        .find(|(v, _)| *v == zenith_skill)
                        .map(|(_, name)| *name)
                        .unwrap_or("Unknown");
                    
                    egui::ComboBox::from_id_source("zenith_skill_combo")
                        .selected_text(current_text)
                                            .show_ui(ui, |ui| {
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                                for (id, name) in ZENITH_SKILL_LIST {
                                        if q.is_empty() || name.to_lowercase().contains(&q) {
                                            ui.selectable_value(&mut zenith_skill, *id, format!("{} - {}", id, name));
                                                }
                                        }
                            });
                        });
                    });
                    
                // Write back all skill values
                armor.skill_id1 = skill_id1;
                armor.skill_pts1 = skill_pts1;
                armor.skill_id2 = skill_id2;
                armor.skill_pts2 = skill_pts2;
                armor.skill_id3 = skill_id3;
                armor.skill_pts3 = skill_pts3;
                armor.skill_id4 = skill_id4;
                armor.skill_pts4 = skill_pts4;
                armor.skill_id5 = skill_id5;
                armor.skill_pts5 = skill_pts5;
                armor.zenith_skill = zenith_skill;
                            });
                    });
    }

    fn get_equipment_flags(equipable_by: u8) -> (bool, bool, bool, bool) {
        match equipable_by {
            0 => (true, true, true, true),  // All
            1 => (true, false, true, true), // Male only
            2 => (false, true, true, true), // Female only
            _ => (true, true, true, true),  // Default to All
        }
    }

    fn set_equipment_flags(armor: &mut MhfdatEquipment, is_male: bool, is_female: bool, is_blade: bool, is_gunner: bool) {
        let mut bitfield = 0u8;
        if is_male { bitfield |= 1 << 0; }
        if is_female { bitfield |= 1 << 1; }
        if is_blade { bitfield |= 1 << 2; }
        if is_gunner { bitfield |= 1 << 3; }
        armor.equipable_by = bitfield;
    }
}

fn armor_type_name(equipable_by: u8) -> &'static str {
    match equipable_by {
        0 => "All",
        1 => "Male",
        2 => "Female",
        _ => "Unknown",
    }
}
