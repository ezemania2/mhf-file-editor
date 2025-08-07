use super::*;
use egui;
use std::io::{Cursor, Read, Seek, SeekFrom};
use crate::utils::skills::{skill_name, SKILL_LIST};
use crate::utils::weapon_patterns::{zenith_skill_name, ZENITH_SKILL_LIST};
use std::fs::File;
use std::io::Write;
use serde_json;

impl MhfdatApp {
    pub fn show_armor_tab(&mut self, ui: &mut egui::Ui) {
        // Add armor category tabs
        ui.horizontal(|ui| {
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

        let armor_tab = &mut self.armor_tab;
        let (armors, names) = match *armor_tab {
            ArmorTab::Head => (&mut self.head_armors, &mut self.head_armor_names),
            ArmorTab::Body => (&mut self.body_armors, &mut self.body_armor_names),
            ArmorTab::Arms => (&mut self.arms_armors, &mut self.arms_armor_names),
            ArmorTab::Waist => (&mut self.waist_armors, &mut self.waist_armor_names),
            ArmorTab::Legs => (&mut self.legs_armors, &mut self.legs_armor_names),
        };

        let search_query = &mut self.search_query;
        let class_id_filter = self.class_id_filter;
        let element_filter = self.element_filter;
        let equip_type_filter = self.equip_type_filter;
        let selected_armor_index = &mut self.selected_armor_index;
        let show_dummy_armor = &mut self.show_dummy_weapons;

        // Compter jusqu'à la première armure avec model_id_male == 0xFFFF
        let mut real_count = 0;
        for armor in armors.iter() {
            if armor.model_id_male == 0xFFFF {
                break;
            }
            real_count += 1;
        }
        let max_count = real_count.min(armors.len());
        let mut all_armors = armors.iter().take(max_count).cloned().collect::<Vec<_>>();
        let mut all_names = names.iter().take(max_count).cloned().collect::<Vec<_>>();

        // Filter armors based on search and filters first
        let mut filtered_indices = Vec::new();
        for i in 0..max_count {
            if let Some(armor) = all_armors.get(i) {
                let armor_name = all_names.get(i).cloned().unwrap_or_default();
                
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

                // Dummy armor detection
                let is_dummy = model_id_male == 0x0000
                    && model_id_female == 0x0000
                    && rarity == 0x00
                    && equipable_by == 0x00
                    && armor.zenny_cost == 0x00000000
                    && base_defense == 0x0000
                    && base_slots == 0x00
                    && max_slots == 0x00
                    && fire_res == 0x00
                    && water_res == 0x00
                    && thunder_res == 0x00
                    && dragon_res == 0x00
                    && ice_res == 0x00
                    && armor.zenith_skill == 0x0000;

                // Apply dummy filter
                if *show_dummy_armor {
                    if !is_dummy { continue; }
                } else {
                    if is_dummy { continue; }
                }

                // Apply other filters
                if let Some(class_id) = class_id_filter {
                    if armor.equipable_by != class_id { continue; }
                }
                if let Some(element_id_filter) = element_filter {
                    let has_element = match element_id_filter {
                        1 => fire_res > 0,
                        2 => water_res > 0,
                        3 => thunder_res > 0,
                        4 => dragon_res > 0,
                        5 => ice_res > 0,
                        _ => false,
                    };
                    if !has_element { continue; }
                }
                if let Some(equip_type_id) = equip_type_filter {
                    if armor.equipable_by != equip_type_id { continue; }
                }

                // Apply search filter
                if !search_query.is_empty() && !armor_name.to_lowercase().contains(&search_query.to_lowercase()) {
                    continue;
                }

                filtered_indices.push(i);
            }
        }

        // Calculate pagination based on filtered results
        let entries_per_page = 15;
        let filtered_count = filtered_indices.len();
        let total_pages = if filtered_count > 0 { 
            ((filtered_count + entries_per_page - 1) / entries_per_page).max(1)
        } else { 
            1 
        };
        
        // Reset to first page when starting a new search or when search results change dramatically
        if !search_query.is_empty() && !filtered_indices.is_empty() && (self.armor_page as usize) >= total_pages {
            self.armor_page = 0;
        }
        // Also reset to first page when search is cleared
        if search_query.is_empty() && self.armor_page as usize >= total_pages {
            self.armor_page = 0;
        }
        
        let current_page = (self.armor_page as usize).min(total_pages.saturating_sub(1));
        
        let start_idx = current_page * entries_per_page;
        let end_idx = (start_idx + entries_per_page).min(filtered_count);

        // Create slices for the filtered results
        let page_indices = if filtered_count == 0 {
            &[]
        } else if start_idx < filtered_count && end_idx > start_idx {
            let safe_end = end_idx.min(filtered_count);
            &filtered_indices[start_idx..safe_end]
        } else {
            &[]
        };

        // Create references to all armors (we'll use indices to access filtered ones)
        let armors_ref = &mut all_armors;
        let names_ref = &mut all_names;

        Self::show_armor_list(
            ui,
            armors_ref,
            names_ref,
            &mut self.armor_descriptions,
            search_query,
            class_id_filter,
            element_filter,
            equip_type_filter,
            selected_armor_index,
            show_dummy_armor,
            armor_tab,
            page_indices,
            current_page,
            total_pages,
            filtered_count,
        );

        // Add "Add a new piece" and export buttons
        ui.horizontal(|ui| {
            let armor_type = match *armor_tab {
                ArmorTab::Head => "head",
                ArmorTab::Body => "body", 
                ArmorTab::Arms => "arms",
                ArmorTab::Waist => "waist",
                ArmorTab::Legs => "legs",
            };
            
            if ui.button(format!("Add a new {} piece", armor_type)).clicked() {
                // Create a new default armor piece
                let mut new_armor = MhfdatEquipment::default();
                new_armor.model_id_male = 0x0000;
                new_armor.model_id_female = 0x0000;
                new_armor.equipable_by = 0x0F; // All flags enabled by default
                new_armor.base_slots = 0;
                new_armor.max_slots = 3;
                
                // Add to appropriate armor list and print data
                match *armor_tab {
                    ArmorTab::Head => {
                        self.head_armors.insert(real_count, new_armor.clone());
                        let name = "New Head Armor".to_string();
                        self.head_armor_names.insert(real_count, name.clone());
                        // Increment EquipmentCounts
                        if let Some(counts) = &mut self.equipment_counts {
                            counts.numHeadA += 1;
                        }
                        // Sélectionner la nouvelle pièce
                        self.selected_armor_index = Some(real_count);
                        self.armor_page = (real_count / 15) as u32; // Update page to show new armor

                    },
                    ArmorTab::Body => {
                        self.body_armors.insert(real_count, new_armor.clone());
                        let name = "New Body Armor".to_string();
                        self.body_armor_names.insert(real_count, name.clone());
                        if let Some(counts) = &mut self.equipment_counts {
                            counts.numBodyA += 1;
                        }
                        self.selected_armor_index = Some(real_count);
                        self.armor_page = (real_count / 15) as u32;

                    },
                    ArmorTab::Arms => {
                        self.arms_armors.insert(real_count, new_armor.clone());
                        let name = "New Arms Armor".to_string();
                        self.arms_armor_names.insert(real_count, name.clone());
                        if let Some(counts) = &mut self.equipment_counts {
                            counts.numArmA += 1;
                        }
                        self.selected_armor_index = Some(real_count);
                        self.armor_page = (real_count / 15) as u32;

                    },
                    ArmorTab::Waist => {
                        self.waist_armors.insert(real_count, new_armor.clone());
                        let name = "New Waist Armor".to_string();
                        self.waist_armor_names.insert(real_count, name.clone());
                        if let Some(counts) = &mut self.equipment_counts {
                            counts.numWaistA += 1;
                        }
                        self.selected_armor_index = Some(real_count);
                        self.armor_page = (real_count / 15) as u32;

                    },
                    ArmorTab::Legs => {
                        self.legs_armors.insert(real_count, new_armor.clone());
                        let name = "New Legs Armor".to_string();
                        self.legs_armor_names.insert(real_count, name.clone());
                        if let Some(counts) = &mut self.equipment_counts {
                            counts.numLegA += 1;
                        }
                        self.selected_armor_index = Some(real_count);
                        self.armor_page = (real_count / 15) as u32;

                    },
                }
            }
        });

        // Pagination controls
        ui.horizontal(|ui| {
            let can_go_previous = current_page > 0 && filtered_count > 0;
            let can_go_next = current_page < total_pages.saturating_sub(1) && filtered_count > 0;
            
            if ui.button("← Previous").clicked() && can_go_previous {
                self.armor_page = (current_page.saturating_sub(1)) as u32;
            }
            if filtered_count > 0 {
                ui.label(format!("Page {} of {}", current_page + 1, total_pages));
            } else {
                ui.label("No results found");
            }
            if ui.button("Next →").clicked() && can_go_next {
                self.armor_page = (current_page + 1) as u32;
            }
        });
    }

    pub fn show_armor_list(
        ui: &mut egui::Ui,
        armors: &mut [MhfdatEquipment],
        names: &mut [String],
        descriptions: &mut Vec<[String; 3]>,
        search_query: &mut String,
        class_id_filter: Option<u8>,
        element_filter: Option<u8>,
        equip_type_filter: Option<u8>,
        selected_armor_index: &mut Option<usize>,
        show_dummy_armor: &mut bool,
        armor_tab: &mut ArmorTab,
        page_indices: &[usize],
        current_page: usize,
        total_pages: usize,
        filtered_count: usize,
    ) {
        let armor_type = match armor_tab {
            ArmorTab::Head => "Head",
            ArmorTab::Body => "Body",
            ArmorTab::Arms => "Arms",
            ArmorTab::Waist => "Waist",
            ArmorTab::Legs => "Legs",
        };

        ui.heading(format!("{} Armor (found: {})", armor_type, filtered_count));

        // Search and filters
        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(search_query);
            ui.checkbox(show_dummy_armor, "Show Dummy Armor");
        });

        // Export buttons
        ui.horizontal(|ui| {
            if ui.button(format!("Export {} Armor to JSON", armor_type)).clicked() {
                // Convert armor to export format with decomposed bitfields
                let export_armors: Vec<ArmorExport> = armors
                    .iter()
                    .enumerate()
                    .map(|(index, armor)| {
                        let name = names.get(index).cloned().unwrap_or_default();
                        ArmorExport::from_armor_with_data(armor, &name, index)
                    })
                    .collect();
                
                let filename = format!("{}_armor.json", armor_type.to_lowercase());
                if let Ok(json) = serde_json::to_string_pretty(&export_armors) {
                    if let Ok(mut file) = File::create(&filename) {
                        let _ = file.write_all(json.as_bytes());
                    }
                }
            }
        });

        if filtered_count == 0 {
            ui.colored_label(egui::Color32::YELLOW, format!("Warning: No {} armor found!", armor_type));
        } else {
            // Show selected armor details if any
            if let Some(index) = selected_armor_index {
                if let Some(armor) = armors.get_mut(*index) {
                    let name = names.get(*index).cloned().unwrap_or_default();
                    let mut current_descriptions = descriptions.get(*index).cloned().unwrap_or_default();
                    let mut should_return = false;
                    
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading(format!("Editing {} Armor #{}", armor_type, *index));
                            if ui.button("← Back to List").clicked() {
                                should_return = true;
                            }
                        });
                        
                        ui.add_space(10.0);
                        
                        // Basic Info
                        ui.group(|ui| {
                            ui.heading("Basic Information");
                            ui.horizontal(|ui| {
                                ui.label("Name:");
                                let mut name_edit = name.clone();
                                if ui.text_edit_singleline(&mut name_edit).changed() {
                                    if let Some(name_ref) = names.get_mut(*index) {
                                        *name_ref = name_edit;
                                    }
                                }
                            });
                        });

                        ui.add_space(10.0);

                        // Main layout with two columns
                        ui.horizontal(|ui| {
                            // Left column - Equipment Flags and Stats
                            ui.vertical(|ui| {
                                ui.group(|ui| {
                                    ui.heading("Equipment Flags");
                                    
                                    let (is_male, is_female, is_blade, is_gunner) = Self::get_equipment_flags(armor.equipable_by);
                                    let mut male = is_male;
                                    let mut female = is_female;
                                    let mut blade = is_blade;
                                    let mut gunner = is_gunner;

                                    // Copy packed fields to local variables
                                    let model_id_male = armor.model_id_male;
                                    let model_id_female = armor.model_id_female;
                                    let rarity = armor.rarity;
                                    let max_level = armor.max_level;
                                    let base_defense = armor.base_defense;
                                    let fire_res = armor.fire_res;
                                    let water_res = armor.water_res;
                                    let thunder_res = armor.thunder_res;
                                    let dragon_res = armor.dragon_res;
                                    let ice_res = armor.ice_res;

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

                                    ui.add_space(5.0);

                                    ui.horizontal(|ui| {
                                        ui.label("Model ID Male:");
                                        let mut male_id = model_id_male;
                                        if ui.add(egui::DragValue::new(&mut male_id).speed(1.0)).changed() {
                                            armor.model_id_male = male_id;
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("Model ID Female:");
                                        let mut female_id = model_id_female;
                                        if ui.add(egui::DragValue::new(&mut female_id).speed(1.0)).changed() {
                                            armor.model_id_female = female_id;
                                        }
                                    });

                                    ui.add_space(5.0);

                                    ui.horizontal(|ui| {
                                        ui.label("Rarity:");
                                        let mut rarity_val = rarity;
                                        if ui.add(egui::DragValue::new(&mut rarity_val).speed(1.0).clamp_range(0..=11)).changed() {
                                            armor.rarity = rarity_val;
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("Max Level:");
                                        let mut level = max_level;
                                        if ui.add(egui::DragValue::new(&mut level).speed(1.0).clamp_range(1..=7)).changed() {
                                            armor.max_level = level;
                                        }
                                    });

                                    ui.add_space(5.0);

                                    // Defense section
                                    ui.horizontal(|ui| {
                                        ui.label("Base Defense:");
                                        let mut def = base_defense;
                                        if ui.add(egui::DragValue::new(&mut def).speed(1.0)).changed() {
                                            armor.base_defense = def;
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("Fire Res:");
                                        let mut res = fire_res;
                                        if ui.add(egui::DragValue::new(&mut res).speed(1.0)).changed() {
                                            armor.fire_res = res;
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("Water Res:");
                                        let mut res = water_res;
                                        if ui.add(egui::DragValue::new(&mut res).speed(1.0)).changed() {
                                            armor.water_res = res;
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("Thunder Res:");
                                        let mut res = thunder_res;
                                        if ui.add(egui::DragValue::new(&mut res).speed(1.0)).changed() {
                                            armor.thunder_res = res;
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("Dragon Res:");
                                        let mut res = dragon_res;
                                        if ui.add(egui::DragValue::new(&mut res).speed(1.0)).changed() {
                                            armor.dragon_res = res;
                                        }
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("Ice Res:");
                                        let mut res = ice_res;
                                        if ui.add(egui::DragValue::new(&mut res).speed(1.0)).changed() {
                                            armor.ice_res = res;
                                        }
                                    });
                                });
                            });

                            ui.add_space(20.0);

                            // Right column - Skills and Talents
                            ui.vertical(|ui| {
                                ui.group(|ui| {
                                    ui.heading("Skills");
                                    

                                    


                                    // Skill 1
                                    ui.horizontal(|ui| {
                                        ui.label("Skill 1:");
                                        let mut skill_id1 = armor.skill_id1;
                                        let mut skill_pts1 = armor.skill_pts1;
                                        egui::ComboBox::from_id_source("skill1_combo")
                                            .selected_text(skill_name(skill_id1))
                                            .show_ui(ui, |ui| {
                                                for (id, name) in SKILL_LIST {
                                                    if ui.selectable_value(&mut skill_id1, *id, *name).clicked() {}
                                                }
                                            });
                                        
                                        ui.label("Points:");
                                        ui.add(egui::DragValue::new(&mut skill_pts1).speed(1.0).clamp_range(-10..=10));
                                        armor.skill_id1 = skill_id1;
                                        armor.skill_pts1 = skill_pts1;
                                    });

                                    // Skill 2
                                    ui.horizontal(|ui| {
                                        ui.label("Skill 2:");
                                        let mut skill_id = armor.skill_id2;
                                        let mut skill_pts = armor.skill_pts2;
                                        egui::ComboBox::from_id_source("skill2_combo")
                                            .selected_text(skill_name(skill_id))
                                            .show_ui(ui, |ui| {
                                                for (id, name) in SKILL_LIST {
                                                    if ui.selectable_value(&mut skill_id, *id, *name).clicked() {}
                                                }
                                            });
                                        
                                        ui.label("Points:");
                                        ui.add(egui::DragValue::new(&mut skill_pts).speed(1.0).clamp_range(-10..=10));
                                        armor.skill_id2 = skill_id;
                                        armor.skill_pts2 = skill_pts;
                                    });

                                    // Skill 3
                                    ui.horizontal(|ui| {
                                        ui.label("Skill 3:");
                                        let mut skill_id = armor.skill_id3;
                                        let mut skill_pts = armor.skill_pts3;
                                        egui::ComboBox::from_id_source("skill3_combo")
                                            .selected_text(skill_name(skill_id))
                                            .show_ui(ui, |ui| {
                                                for (id, name) in SKILL_LIST {
                                                    if ui.selectable_value(&mut skill_id, *id, *name).clicked() {}
                                                }
                                            });
                                        
                                        ui.label("Points:");
                                        ui.add(egui::DragValue::new(&mut skill_pts).speed(1.0).clamp_range(-10..=10));
                                        armor.skill_id3 = skill_id;
                                        armor.skill_pts3 = skill_pts;
                                    });

                                    // Skill 4
                                    ui.horizontal(|ui| {
                                        ui.label("Skill 4:");
                                        let mut skill_id = armor.skill_id4;
                                        let mut skill_pts = armor.skill_pts4;
                                        egui::ComboBox::from_id_source("skill4_combo")
                                            .selected_text(skill_name(skill_id))
                                            .show_ui(ui, |ui| {
                                                for (id, name) in SKILL_LIST {
                                                    if ui.selectable_value(&mut skill_id, *id, *name).clicked() {}
                                                }
                                            });
                                        
                                        ui.label("Points:");
                                        ui.add(egui::DragValue::new(&mut skill_pts).speed(1.0).clamp_range(-10..=10));
                                        armor.skill_id4 = skill_id;
                                        armor.skill_pts4 = skill_pts;
                                    });

                                    // Skill 5
                                    ui.horizontal(|ui| {
                                        ui.label("Skill 5:");
                                        let mut skill_id = armor.skill_id5;
                                        let mut skill_pts = armor.skill_pts5;
                                        egui::ComboBox::from_id_source("skill5_combo")
                                            .selected_text(skill_name(skill_id))
                                            .show_ui(ui, |ui| {
                                                for (id, name) in SKILL_LIST {
                                                    if ui.selectable_value(&mut skill_id, *id, *name).clicked() {}
                                                }
                                            });
                                        
                                        ui.label("Points:");
                                        ui.add(egui::DragValue::new(&mut skill_pts).speed(1.0).clamp_range(-10..=10));
                                        armor.skill_id5 = skill_id;
                                        armor.skill_pts5 = skill_pts;
                                    });

                                    ui.add_space(10.0);

                                    // Zenith Skill
                                    ui.horizontal(|ui| {
                                        ui.label("Zenith Skill:");
                                        let mut zenith = armor.zenith_skill;
                                        egui::ComboBox::from_id_source("zenith_skill_combo")
                                            .selected_text(zenith_skill_name(zenith))
                                            .show_ui(ui, |ui| {
                                                for (id, name) in ZENITH_SKILL_LIST {
                                                    if ui.selectable_value(&mut zenith, *id, *name).clicked() {}
                                                }
                                            });
                                        armor.zenith_skill = zenith;
                                    });
                                });
                            });
                        });
                    });
                    
                    if should_return {
                        *selected_armor_index = None;
                    }
                }
            } else {
                // Armor list view
                egui::ScrollArea::vertical()
                    .id_source("armor_list_scroll")
                    .show(ui, |ui| {
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

                                for &i in page_indices {
                                    if let Some(armor) = armors.get_mut(i) {
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

                                        let selected = *selected_armor_index == Some(i);
                                        if ui.selectable_label(selected, format!("{}", i + 1)).clicked() {
                                            *selected_armor_index = Some(i);
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
                                }
                            });
                    });
            }
        }
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

