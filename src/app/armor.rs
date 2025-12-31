use super::*;
use egui;
use std::io::{Cursor, Read, Seek, SeekFrom};
use crate::utils::skills::{skill_name, SKILL_LIST};
use crate::utils::weapon_patterns::{zenith_skill_name, ZENITH_SKILL_LIST};
use crate::utils::equip_flags::EquipableBy;
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
            if ui.selectable_label(self.armor_tab == ArmorTab::ArmorUpgrade, "Armor Upgrade").clicked() {
                self.armor_tab = ArmorTab::ArmorUpgrade;
            }
        });
        ui.separator();

        match self.armor_tab {
            ArmorTab::ArmorUpgrade => {
                let view_mode = self.view_mode.get("armor_upgrade").copied().unwrap_or(ViewMode::List);
                match view_mode {
                    ViewMode::List => self.show_armor_upgrade_list(ui),
                    ViewMode::Details => self.show_armor_upgrade_details(ui),
                }
            }
            _ => {
                match self.view_mode.get("armor").unwrap() {
                    ViewMode::List => self.show_armor_list_view(ui),
                    ViewMode::Details => self.show_armor_details_view(ui),
                }
            }
        }
    }

    pub fn show_armor_list_view(&mut self, ui: &mut egui::Ui) {
        let armor_type = match self.armor_tab {
            ArmorTab::Head => "Head",
            ArmorTab::Body => "Body",
            ArmorTab::Arms => "Arms",
            ArmorTab::Waist => "Waist",
            ArmorTab::Legs => "Legs",
            ArmorTab::ArmorUpgrade => unreachable!(),
        };
        
        // Store lengths for offset calculation before mutable borrows
        let head_len = self.head_armors.len();
        let body_len = self.body_armors.len();
        let arms_len = self.arms_armors.len();
        let waist_len = self.waist_armors.len();
        
        let armor_type_str_early = match self.armor_tab { 
            ArmorTab::Head=>"head", 
            ArmorTab::Body=>"body", 
            ArmorTab::Arms=>"arms", 
            ArmorTab::Waist=>"waist", 
            ArmorTab::Legs=>"legs",
            ArmorTab::ArmorUpgrade => unreachable!(),
        };
        
        let armor_type_str = match self.armor_tab { 
            ArmorTab::Head=>"head", 
            ArmorTab::Body=>"body", 
            ArmorTab::Arms=>"arms", 
            ArmorTab::Waist=>"waist", 
            ArmorTab::Legs=>"legs",
            ArmorTab::ArmorUpgrade => unreachable!(),
        };
        
        // Buttons row
        let mut do_export = false;
        let mut do_import = false;
        let mut do_add = false;
        ui.horizontal(|ui| {
            if ui.button("Export JSON").clicked() { do_export = true; }
            if ui.button("Import from JSON").clicked() { do_import = true; }
            if ui.button("Add New").clicked() { do_add = true; }
        });

        if do_import {
            if let Ok(Some(path)) = native_dialog::FileDialog::new()
                .add_filter("JSON", &["json"])
                .show_open_single_file() 
            {
                self.import_armor_merge_by_model_id(armor_type_str, path.to_str().unwrap_or(""));
            }
        }

        let (armors, names) = match self.armor_tab {
            ArmorTab::Head => (&mut self.head_armors, &mut self.head_armor_names),
            ArmorTab::Body => (&mut self.body_armors, &mut self.body_armor_names),
            ArmorTab::Arms => (&mut self.arms_armors, &mut self.arms_armor_names),
            ArmorTab::Waist => (&mut self.waist_armors, &mut self.waist_armor_names),
            ArmorTab::Legs => (&mut self.legs_armors, &mut self.legs_armor_names),
            ArmorTab::ArmorUpgrade => unreachable!(),
        };

        // Compter jusqu'à la première armure avec model_id_male == 0xFFFF
        let mut real_count = 0;
        for armor in armors.iter() {
            if armor.model_id_male == 0xFFFF {
                break;
            }
            real_count += 1;
        }
        
        ui.horizontal(|ui| {
            ui.heading(format!("{} Armor (found: {})", armor_type, real_count.min(armors.len())));
        });

        if do_export {
            if let Ok(Some(path)) = native_dialog::FileDialog::new()
                .add_filter("JSON", &["json"])
                .set_filename(&format!("{}_armor.json", armor_type_str))
                .show_save_single_file() 
            {
                let export_armors: Vec<ArmorExport> = armors
                    .iter()
                    .enumerate()
                    .map(|(index, armor)| {
                        let name = names.get(index).cloned().unwrap_or_default();
                        ArmorExport::from_armor_with_data(armor, &name, index)
                    })
                    .collect();
                if let Ok(json) = serde_json::to_string_pretty(&export_armors) {
                    let _ = std::fs::write(path.to_str().unwrap_or("armor.json"), json);
                }
            }
        }

        if do_add {
            // Calculate armor part offset using stored lengths
            let armor_part_offset = match self.armor_tab {
                ArmorTab::Head => 0,
                ArmorTab::Body => head_len,
                ArmorTab::Arms => head_len + body_len,
                ArmorTab::Waist => head_len + body_len + arms_len,
                ArmorTab::Legs => head_len + body_len + arms_len + waist_len,
                ArmorTab::ArmorUpgrade => unreachable!(),
            };
            let desc_index = armor_part_offset + real_count;
            
            let mut new_armor = MhfdatEquipment::default();
            let next_model_id = 0;
            new_armor.model_id_male = 0;
            new_armor.model_id_female = 0;
            new_armor.equipable_by = 0x0F; // All flags enabled by default
            new_armor.base_slots = 0;
            new_armor.max_slots = 3;
            let new_name = format!("New {} Armor", armor_type);
            
            // Insert into armors and names (limit scope of mutable borrows)
            {
                armors.insert(real_count, new_armor);
                names.insert(real_count, new_name);
            }
            
            // Add new armor description entry
            self.armor_descriptions.insert(desc_index, [String::new(), String::new(), String::new(), String::new()]);
            self.armor_descriptions_modified = true;
            
            // Marquer comme modifié selon le type
            match self.armor_tab {
                ArmorTab::Head => {
                    self.head_armors_modified = true;
                    self.head_armor_names_modified = true;
                },
                ArmorTab::Body => {
                    self.body_armors_modified = true;
                    self.body_armor_names_modified = true;
                },
                ArmorTab::Arms => {
                    self.arms_armors_modified = true;
                    self.arms_armor_names_modified = true;
                },
                ArmorTab::Waist => {
                    self.waist_armors_modified = true;
                    self.waist_armor_names_modified = true;
                },
                ArmorTab::Legs => {
                    self.legs_armors_modified = true;
                    self.legs_armor_names_modified = true;
                },
                ArmorTab::ArmorUpgrade => {}
            }
        }

        // Recreate references if they were dropped in do_add block
        let (armors, names) = match self.armor_tab {
            ArmorTab::Head => (&mut self.head_armors, &mut self.head_armor_names),
            ArmorTab::Body => (&mut self.body_armors, &mut self.body_armor_names),
            ArmorTab::Arms => (&mut self.arms_armors, &mut self.arms_armor_names),
            ArmorTab::Waist => (&mut self.waist_armors, &mut self.waist_armor_names),
            ArmorTab::Legs => (&mut self.legs_armors, &mut self.legs_armor_names),
            ArmorTab::ArmorUpgrade => unreachable!(),
        };
        
        // Update real_count after potential insertions
        let mut real_count_after = 0;
        for armor in armors.iter() {
            if armor.model_id_male == 0xFFFF {
                break;
            }
            real_count_after += 1;
        }
        let max_count = real_count_after.min(armors.len());

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
                            if ui.selectable_label(selected, format!("{}", i)).clicked() {
                                self.selected_armor_index = Some(i);
                                // Initialiser view_mode si nécessaire
                                if !self.view_mode.contains_key("armor") {
                                    self.view_mode.insert("armor".to_string(), ViewMode::List);
                                }
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
            // Initialiser view_mode si nécessaire
            if !self.view_mode.contains_key("armor") {
                self.view_mode.insert("armor".to_string(), ViewMode::List);
            }
            *self.view_mode.get_mut("armor").unwrap() = ViewMode::List;
            return;
        }

        if let Some(index) = self.selected_armor_index {
            // Calculate armor part offset before mutable borrows
            let armor_part_offset = match self.armor_tab {
                ArmorTab::Head => 0,
                ArmorTab::Body => self.head_armors.len(),
                ArmorTab::Arms => self.head_armors.len() + self.body_armors.len(),
                ArmorTab::Waist => self.head_armors.len() + self.body_armors.len() + self.arms_armors.len(),
                ArmorTab::Legs => self.head_armors.len() + self.body_armors.len() + self.arms_armors.len() + self.waist_armors.len(),
                ArmorTab::ArmorUpgrade => unreachable!(),
            };
            let desc_index = armor_part_offset + index;
            
            // Marquer comme modifié selon le type d'armure
            match self.armor_tab {
                ArmorTab::Head => self.head_armors_modified = true,
                ArmorTab::Body => self.body_armors_modified = true,
                ArmorTab::Arms => self.arms_armors_modified = true,
                ArmorTab::Waist => self.waist_armors_modified = true,
                ArmorTab::Legs => self.legs_armors_modified = true,
                ArmorTab::ArmorUpgrade => {}
            }
            
            let (armors, names) = match self.armor_tab {
                ArmorTab::Head => (&mut self.head_armors, &mut self.head_armor_names),
                ArmorTab::Body => (&mut self.body_armors, &mut self.body_armor_names),
                ArmorTab::Arms => (&mut self.arms_armors, &mut self.arms_armor_names),
                ArmorTab::Waist => (&mut self.waist_armors, &mut self.waist_armor_names),
                ArmorTab::Legs => (&mut self.legs_armors, &mut self.legs_armor_names),
                ArmorTab::ArmorUpgrade => unreachable!(),
            };

            let armor_type = match self.armor_tab {
            ArmorTab::Head => "Head",
            ArmorTab::Body => "Body",
            ArmorTab::Arms => "Arms",
            ArmorTab::Waist => "Waist",
            ArmorTab::Legs => "Legs",
            ArmorTab::ArmorUpgrade => unreachable!(),
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
                                        // Marquer le nom comme modifié selon le type
                                        match self.armor_tab {
                                            ArmorTab::Head => self.head_armor_names_modified = true,
                                            ArmorTab::Body => self.body_armor_names_modified = true,
                                            ArmorTab::Arms => self.arms_armor_names_modified = true,
                                            ArmorTab::Waist => self.waist_armor_names_modified = true,
                                            ArmorTab::Legs => self.legs_armor_names_modified = true,
                                            ArmorTab::ArmorUpgrade => {}
                                        }
                                    }
                                }
                            });
                
                // Armor descriptions
                ui.heading("Descriptions");
                if let Some(descs) = self.armor_descriptions.get_mut(desc_index) {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Description 1:");
                            if ui.add(egui::TextEdit::multiline(&mut descs[0]).desired_rows(2)).changed() {
                                self.armor_descriptions_modified = true;
                            }
                        });
                        ui.vertical(|ui| {
                            ui.label("Description 2:");
                            if ui.add(egui::TextEdit::multiline(&mut descs[1]).desired_rows(2)).changed() {
                                self.armor_descriptions_modified = true;
                            }
                        });
                        ui.vertical(|ui| {
                            ui.label("Description 3:");
                            if ui.add(egui::TextEdit::multiline(&mut descs[2]).desired_rows(2)).changed() {
                                self.armor_descriptions_modified = true;
                            }
                        });
                    });
                } else {
                    ui.label(format!("Warning: Description index {} out of range", desc_index));
                }
                
                ui.separator();
                let armor_changed = Self::render_armor_details(ui, armor, 
                    &mut self.armor_skill1_search,
                    &mut self.armor_skill2_search,
                    &mut self.armor_skill3_search,
                    &mut self.armor_skill4_search,
                    &mut self.armor_skill5_search,
                    &mut self.armor_zenith_skill_search,
                    &mut self.armor_deco_item_search,
                    &self.item_names,
                    &self.items
                );
                
                // Marquer les armures comme modifiées si des changements ont été faits
                if armor_changed {
                    match self.armor_tab {
                        ArmorTab::Head => self.head_armors_modified = true,
                        ArmorTab::Body => self.body_armors_modified = true,
                        ArmorTab::Arms => self.arms_armors_modified = true,
                        ArmorTab::Waist => self.waist_armors_modified = true,
                        ArmorTab::Legs => self.legs_armors_modified = true,
                        ArmorTab::ArmorUpgrade => {}
                    }
                }
            }
        } else {
            ui.label("No armor selected");
        }
    }

    pub     fn render_armor_details(
        ui: &mut egui::Ui, 
        armor: &mut MhfdatEquipment,
        armor_skill1_search: &mut String,
        armor_skill2_search: &mut String,
        armor_skill3_search: &mut String,
        armor_skill4_search: &mut String,
        armor_skill5_search: &mut String,
        armor_zenith_skill_search: &mut String,
        armor_deco_item_search: &mut String,
        item_names: &[String],
        items: &[MhfdatItem],
    ) -> bool {
        // Track if anything changed
        let mut changed = false;
        
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
                // Save original values for change detection
                let orig_skill_id1 = armor.skill_id1;
                let orig_skill_pts1 = armor.skill_pts1;
                let orig_skill_id2 = armor.skill_id2;
                let orig_skill_pts2 = armor.skill_pts2;
                let orig_skill_id3 = armor.skill_id3;
                let orig_skill_pts3 = armor.skill_pts3;
                let orig_skill_id4 = armor.skill_id4;
                let orig_skill_pts4 = armor.skill_pts4;
                let orig_skill_id5 = armor.skill_id5;
                let orig_skill_pts5 = armor.skill_pts5;
                let orig_zenith_skill = armor.zenith_skill;
                
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

                // Write back all skill values and check if anything changed
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
                
                // Detect if skills changed
                if skill_id1 != orig_skill_id1 || skill_pts1 != orig_skill_pts1
                    || skill_id2 != orig_skill_id2 || skill_pts2 != orig_skill_pts2
                    || skill_id3 != orig_skill_id3 || skill_pts3 != orig_skill_pts3
                    || skill_id4 != orig_skill_id4 || skill_pts4 != orig_skill_pts4
                    || skill_id5 != orig_skill_id5 || skill_pts5 != orig_skill_pts5
                    || zenith_skill != orig_zenith_skill {
                    changed = true;
                }
                            });
                    });

            // Advanced Stats
            ui.collapsing("Advanced Stats", |ui| {
                // Save original values for change detection
                let orig_coef_upgrade = armor.coef_upgrade;
                let orig_post_festi = armor.post_festi;
                let orig_show_next_level = armor.show_next_level;
                let orig_armor_type = armor.armor_type;
                let orig_weap_hiden = armor.weap_hiden;
                let orig_towerslots = armor.towerslots;
                let orig_deco_item_id = armor.deco_item_id;
                let orig_g_rank = armor.g_rank;
                let orig_app_price = armor.app_price;
                let orig_equip_id = armor.equip_id;
                let orig_zenny_cost = armor.zenny_cost;
                let orig_base_slots = armor.base_slots;
                let orig_max_slots = armor.max_slots;
                
                let mut coef_upgrade = armor.coef_upgrade;
                let mut post_festi = armor.post_festi;
                let mut show_next_level = armor.show_next_level;
                let mut armor_type = armor.armor_type;
                let mut weap_hiden = armor.weap_hiden;
                let mut towerslots = armor.towerslots;
                let mut deco_item_id = armor.deco_item_id;
                let mut g_rank = armor.g_rank;
                let mut app_price = armor.app_price;
                let mut equip_id = armor.equip_id;
                let mut zenny_cost = armor.zenny_cost;
                let mut base_slots = armor.base_slots;
                let mut max_slots = armor.max_slots;

                Self::render_editable_field(ui, "Coef Upgrade", &mut coef_upgrade);
                Self::render_editable_field(ui, "Post Festi", &mut post_festi);
                Self::render_editable_field(ui, "Show Next Level", &mut show_next_level);
                Self::render_editable_field(ui, "Armor Type", &mut armor_type);
                Self::render_editable_field(ui, "Weap Hiden", &mut weap_hiden);
                Self::render_editable_field(ui, "Tower Slots", &mut towerslots);
                
                // Deco Item ID with searchable combo box
                ui.horizontal(|ui| {
                    ui.label("Deco Item ID:");
                    ui.add(egui::TextEdit::singleline(armor_deco_item_search)
                        .hint_text("Search...").desired_width(100.0));
                    
                    let q = armor_deco_item_search.to_lowercase();
                    let current_name = if deco_item_id == 0 {
                        "None".to_string()
                    } else {
                        item_names.get(deco_item_id as usize).cloned().unwrap_or_else(|| format!("Unknown Item {}", deco_item_id))
                    };
                    
                    egui::ComboBox::from_id_source("deco_item_combo")
                        .selected_text(if deco_item_id == 0 { "None".to_string() } else { format!("{} - {}", deco_item_id, current_name) })
                        .show_ui(ui, |ui| {
                            // Option "None"
                            ui.selectable_value(&mut deco_item_id, 0, "None");
                            
                            egui::ScrollArea::vertical()
                                .max_height(300.0)
                                .show(ui, |ui| {
                                    for (item_id, item_name) in item_names.iter().enumerate() {
                                        // Filter: only show items with deco_id > 0
                                        if let Some(item) = items.get(item_id) {
                                            if item.deco_id == 0 {
                                                continue;
                                            }
                                        } else {
                                            continue;
                                        }
                                        
                                        let item_id_u16 = item_id as u16;
                                        let item_name_lower = item_name.to_lowercase();
                                        if q.is_empty() || item_name_lower.contains(&q) || item_id.to_string().contains(&q) {
                                            let display_text = if item_name.is_empty() {
                                                format!("{} - (Unnamed)", item_id)
                                            } else {
                                                format!("{} - {}", item_id, item_name)
                                            };
                                            ui.selectable_value(&mut deco_item_id, item_id_u16, display_text);
                                        }
                                    }
                                });
                        });
                });
                
                Self::render_editable_field(ui, "G Rank", &mut g_rank);
                Self::render_editable_field(ui, "App Price", &mut app_price);
                Self::render_editable_field(ui, "Equip ID", &mut equip_id);
                Self::render_editable_field(ui, "Zenny Cost", &mut zenny_cost);
                Self::render_editable_field(ui, "Base Slots", &mut base_slots);
                Self::render_editable_field(ui, "Max Slots", &mut max_slots);

                armor.coef_upgrade = coef_upgrade;
                armor.post_festi = post_festi;
                armor.show_next_level = show_next_level;
                armor.armor_type = armor_type;
                armor.weap_hiden = weap_hiden;
                armor.towerslots = towerslots;
                armor.deco_item_id = deco_item_id;
                armor.g_rank = g_rank;
                armor.app_price = app_price;
                armor.equip_id = equip_id;
                armor.zenny_cost = zenny_cost;
                armor.base_slots = base_slots;
                armor.max_slots = max_slots;
                
                if coef_upgrade != orig_coef_upgrade || post_festi != orig_post_festi
                    || show_next_level != orig_show_next_level || armor_type != orig_armor_type
                    || weap_hiden != orig_weap_hiden || towerslots != orig_towerslots
                    || deco_item_id != orig_deco_item_id || g_rank != orig_g_rank
                    || app_price != orig_app_price || equip_id != orig_equip_id
                    || zenny_cost != orig_zenny_cost || base_slots != orig_base_slots
                    || max_slots != orig_max_slots {
                    changed = true;
                }
            });
        
        // Return true only if something actually changed
        changed
    }

    fn get_equipment_flags(equipable_by: u8) -> (bool, bool, bool, bool) {
        let flags = EquipableBy::from_u8(equipable_by);
        (flags.male, flags.female, flags.blade, flags.gunner)
    }

    fn set_equipment_flags(armor: &mut MhfdatEquipment, is_male: bool, is_female: bool, is_blade: bool, is_gunner: bool) {
        let mut flags = EquipableBy::from_u8(armor.equipable_by);
        flags.male = is_male;
        flags.female = is_female;
        flags.blade = is_blade;
        flags.gunner = is_gunner;
        armor.equipable_by = flags.to_u8();
    }
}

fn armor_type_name(equipable_by: u8) -> String {
    let flags = EquipableBy::from_u8(equipable_by);
    let mut parts = Vec::new();
    
    if flags.male && !flags.female {
        parts.push("Male");
    } else if !flags.male && flags.female {
        parts.push("Female");
    } else if flags.male && flags.female {
        parts.push("Both");
    }
    
    if flags.blade && !flags.gunner {
        parts.push("Blade");
    } else if !flags.blade && flags.gunner {
        parts.push("Gunner");
    } else if flags.blade && flags.gunner {
        parts.push("Both");
    }
    
    if flags.sp {
        parts.push("SP");
    }
    
    if parts.is_empty() {
        "None".to_string()
    } else {
        parts.join("/")
    }
}

impl MhfdatApp {
    fn show_armor_upgrade_list(&mut self, ui: &mut egui::Ui) {
        let table_count = self.armor_upgrade_mats.tables.len();
        
        MhfdatApp::section_header(ui, &format!("Armor Upgrade Materials ({} tables)", table_count), |ui| {
            if ui.button("Add Table").clicked() {
                self.armor_upgrade_mats.tables.push(crate::model::mhfdat::ArmorUpgradeTable { rows: Vec::new() });
                self.armor_upgrade_mats_modified = true;
            }
            if ui.button("Export to JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_filename("armor_upgrade_mats.json")
                    .show_save_single_file() 
                {
                    if let Ok(json) = serde_json::to_string_pretty(&self.armor_upgrade_mats) {
                        let _ = std::fs::write(path.to_str().unwrap_or("armor_upgrade_mats.json"), json);
                    }
                }
            }
            if ui.button("Import from JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .show_open_single_file() 
                {
                    if let Ok(json) = std::fs::read_to_string(path.to_str().unwrap_or("")) {
                        if let Ok(data) = serde_json::from_str(&json) {
                            self.armor_upgrade_mats = data;
                            self.armor_upgrade_mats_modified = true;
                        }
                    }
                }
            }
        });
        
        if table_count == 0 {
            ui.label("No armor upgrade materials loaded.");
            return;
        }
        
        // List of tables
        ui.heading("Tables");
        
        let tables_page_size = 50usize;
        let tables_total_pages = (table_count + tables_page_size - 1) / tables_page_size;
        let tables_page = self.armor_upgrade_tables_page.min(tables_total_pages.saturating_sub(1) as u32);
        self.armor_upgrade_tables_page = tables_page;
        
        let tables_start = (tables_page as usize) * tables_page_size;
        let tables_end = (tables_start + tables_page_size).min(table_count);
        
        MhfdatApp::list_scroll(ui, "armor_upgrade_tables_scroll", |ui| {
            egui::Grid::new("armor_upgrade_tables_grid")
                .striped(true)
                .num_columns(3)
                .show(ui, |ui| {
                    ui.label("Table");
                    ui.label("Rows");
                    ui.label("Actions");
                    ui.end_row();
                    
                    for idx in tables_start..tables_end {
                        if let Some(table) = self.armor_upgrade_mats.tables.get(idx) {
                            let selected = self.selected_armor_upgrade_table_index == Some(idx);
                            if ui.selectable_label(selected, format!("Table {}", idx)).clicked() {
                                self.selected_armor_upgrade_table_index = Some(idx);
                                self.armor_upgrade_page = 0;
                                self.view_mode.insert("armor_upgrade".to_string(), ViewMode::Details);
                            }
                            ui.label(format!("{}", table.rows.len()));
                            ui.label("");
                            ui.end_row();
                        }
                    }
                });
        });
        
        MhfdatApp::pagination_controls(ui, &mut self.armor_upgrade_tables_page, tables_total_pages);
    }
    
    fn show_armor_upgrade_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert("armor_upgrade".to_string(), ViewMode::List);
            self.selected_armor_upgrade_row_index = None;
            return;
        }
        ui.separator();
        
        let table_idx = self.selected_armor_upgrade_table_index.unwrap_or(0);
        let Some(table) = self.armor_upgrade_mats.tables.get_mut(table_idx) else {
            ui.label("No table loaded");
            return;
        };
        
        let total = table.rows.len();
        if total == 0 {
            ui.label("Table has no rows.");
            return;
        }
        
        ui.heading(format!("Armor Upgrade Materials - Table {} ({} items)", table_idx, total));
        ui.separator();
        
        let page_size = 50usize;
        let total_pages = (total + page_size - 1) / page_size;
        let page = self.armor_upgrade_page.min(total_pages.saturating_sub(1) as u32);
        self.armor_upgrade_page = page;
        
        let start = (page as usize) * page_size;
        let end = (start + page_size).min(total);
        
        MhfdatApp::list_scroll(ui, "armor_upgrade_details_scroll", |ui| {
            egui::Grid::new("armor_upgrade_details_grid")
                .striped(true)
                .num_columns(8)
                .show(ui, |ui| {
                    ui.label("Item");
                    ui.label("Lv1");
                    ui.label("Lv2");
                    ui.label("Lv3");
                    ui.label("Lv4");
                    ui.label("Lv5");
                    ui.label("Lv6");
                    ui.label("Lv7");
                    ui.end_row();
                    
                    for i in start..end {
                        if let Some(row) = table.rows.get_mut(i) {
                            let item_name = self.item_names.get(row.item_id as usize).cloned().unwrap_or_default();
                            
                            // Item ID (editable)
                            let mut item_id = row.item_id;
                            ui.horizontal(|ui| {
                                if ui.add(egui::DragValue::new(&mut item_id).clamp_range(0..=65535)).changed() {
                                    row.item_id = item_id;
                                    self.armor_upgrade_mats_modified = true;
                                }
                                ui.label(format!("({})", item_name));
                            });
                            
                            // Level upgrade values (editable)
                            let mut lv1 = row.lv1_upgrade;
                            if ui.add(egui::DragValue::new(&mut lv1).clamp_range(0..=65535)).changed() {
                                row.lv1_upgrade = lv1;
                                self.armor_upgrade_mats_modified = true;
                            }
                            
                            let mut lv2 = row.lv2_upgrade;
                            if ui.add(egui::DragValue::new(&mut lv2).clamp_range(0..=65535)).changed() {
                                row.lv2_upgrade = lv2;
                                self.armor_upgrade_mats_modified = true;
                            }
                            
                            let mut lv3 = row.lv3_upgrade;
                            if ui.add(egui::DragValue::new(&mut lv3).clamp_range(0..=65535)).changed() {
                                row.lv3_upgrade = lv3;
                                self.armor_upgrade_mats_modified = true;
                            }
                            
                            let mut lv4 = row.lv4_upgrade;
                            if ui.add(egui::DragValue::new(&mut lv4).clamp_range(0..=65535)).changed() {
                                row.lv4_upgrade = lv4;
                                self.armor_upgrade_mats_modified = true;
                            }
                            
                            let mut lv5 = row.lv5_upgrade;
                            if ui.add(egui::DragValue::new(&mut lv5).clamp_range(0..=65535)).changed() {
                                row.lv5_upgrade = lv5;
                                self.armor_upgrade_mats_modified = true;
                            }
                            
                            let mut lv6 = row.lv6_upgrade;
                            if ui.add(egui::DragValue::new(&mut lv6).clamp_range(0..=65535)).changed() {
                                row.lv6_upgrade = lv6;
                                self.armor_upgrade_mats_modified = true;
                            }
                            
                            let mut lv7 = row.lv7_upgrade;
                            if ui.add(egui::DragValue::new(&mut lv7).clamp_range(0..=65535)).changed() {
                                row.lv7_upgrade = lv7;
                                self.armor_upgrade_mats_modified = true;
                            }
                            
                            ui.end_row();
                        }
                    }
                });
        });
        
        MhfdatApp::pagination_controls(ui, &mut self.armor_upgrade_page, total_pages);
    }
    
    // Import functions for armor
    fn import_armor_replace_all(&mut self, _armor_type_str: &str, file_path: &str) {
        let armor_tab = self.armor_tab;
        
        // Try export format first
        if let Ok(data) = std::fs::read_to_string(file_path) {
            if let Ok(imported_export) = serde_json::from_str::<Vec<ArmorExport>>(&data) {
                let imported: Vec<MhfdatEquipment> = imported_export.iter().map(|e| e.to_armor()).collect();
                match armor_tab {
                    ArmorTab::Head => { self.head_armors = imported; self.head_armors_modified = true; }
                    ArmorTab::Body => { self.body_armors = imported; self.body_armors_modified = true; }
                    ArmorTab::Arms => { self.arms_armors = imported; self.arms_armors_modified = true; }
                    ArmorTab::Waist => { self.waist_armors = imported; self.waist_armors_modified = true; }
                    ArmorTab::Legs => { self.legs_armors = imported; self.legs_armors_modified = true; }
                    ArmorTab::ArmorUpgrade => {},
                }
                return;
            }
            // Fallback to raw format
            if let Ok(imported) = serde_json::from_str::<Vec<MhfdatEquipment>>(&data) {
                match armor_tab {
                    ArmorTab::Head => { self.head_armors = imported; self.head_armors_modified = true; }
                    ArmorTab::Body => { self.body_armors = imported; self.body_armors_modified = true; }
                    ArmorTab::Arms => { self.arms_armors = imported; self.arms_armors_modified = true; }
                    ArmorTab::Waist => { self.waist_armors = imported; self.waist_armors_modified = true; }
                    ArmorTab::Legs => { self.legs_armors = imported; self.legs_armors_modified = true; }
                    ArmorTab::ArmorUpgrade => {},
                }
            }
        }
    }
    
    fn import_armor_merge_by_model_id(&mut self, _armor_type_str: &str, file_path: &str) {
        let armor_tab = self.armor_tab;
        
        // Try export format first
        if let Ok(data) = std::fs::read_to_string(file_path) {
            if let Ok(imported_export) = serde_json::from_str::<Vec<ArmorExport>>(&data) {
                eprintln!("[DEBUG] Importing {} armors from export format", imported_export.len());
                for export in imported_export.iter() {
                    let armor = export.to_armor();
                    let model_id_male = armor.model_id_male;
                    eprintln!("[DEBUG] Processing armor with model_id_male: {}", model_id_male);
                    
                    // Get the appropriate armor vector based on tab
                    let armor_vec = match armor_tab {
                        ArmorTab::Head => &mut self.head_armors,
                        ArmorTab::Body => &mut self.body_armors,
                        ArmorTab::Arms => &mut self.arms_armors,
                        ArmorTab::Waist => &mut self.waist_armors,
                        ArmorTab::Legs => &mut self.legs_armors,
                        ArmorTab::ArmorUpgrade => continue,
                    };
                    
                    // Find existing armor with same model_id_male
                    if let Some(existing) = armor_vec.iter_mut().find(|a| a.model_id_male == model_id_male) {
                        // Update existing armor
                        eprintln!("[DEBUG] Updating existing armor with model_id_male: {}", model_id_male);
                        *existing = armor;
                    } else {
                        // Add new armor
                        eprintln!("[DEBUG] Adding new armor with model_id_male: {}", model_id_male);
                        armor_vec.push(armor);
                    }
                }
                
                // Mark as modified based on tab
                match armor_tab {
                    ArmorTab::Head => self.head_armors_modified = true,
                    ArmorTab::Body => self.body_armors_modified = true,
                    ArmorTab::Arms => self.arms_armors_modified = true,
                    ArmorTab::Waist => self.waist_armors_modified = true,
                    ArmorTab::Legs => self.legs_armors_modified = true,
                    ArmorTab::ArmorUpgrade => {},
                }
                eprintln!("[DEBUG] Armor import completed");
                return;
            }
            // Fallback to raw format
            if let Ok(imported) = serde_json::from_str::<Vec<MhfdatEquipment>>(&data) {
                eprintln!("[DEBUG] Importing {} armors from raw format", imported.len());
                for armor in imported {
                    let model_id_male = armor.model_id_male;
                    eprintln!("[DEBUG] Processing armor with model_id_male: {}", model_id_male);
                    
                    let armor_vec = match armor_tab {
                        ArmorTab::Head => &mut self.head_armors,
                        ArmorTab::Body => &mut self.body_armors,
                        ArmorTab::Arms => &mut self.arms_armors,
                        ArmorTab::Waist => &mut self.waist_armors,
                        ArmorTab::Legs => &mut self.legs_armors,
                        ArmorTab::ArmorUpgrade => continue,
                    };
                    
                    if let Some(existing) = armor_vec.iter_mut().find(|a| a.model_id_male == model_id_male) {
                        eprintln!("[DEBUG] Updating existing armor with model_id_male: {}", model_id_male);
                        *existing = armor;
                    } else {
                        eprintln!("[DEBUG] Adding new armor with model_id_male: {}", model_id_male);
                        armor_vec.push(armor);
                    }
                }
                
                match armor_tab {
                    ArmorTab::Head => self.head_armors_modified = true,
                    ArmorTab::Body => self.body_armors_modified = true,
                    ArmorTab::Arms => self.arms_armors_modified = true,
                    ArmorTab::Waist => self.waist_armors_modified = true,
                    ArmorTab::Legs => self.legs_armors_modified = true,
                    ArmorTab::ArmorUpgrade => {},
                }
                eprintln!("[DEBUG] Armor import completed");
            }
        }
    }
    
}
