use super::*;
use egui;
use crate::core::mhfdat::{read_equipment_counts, write_equipment_counts};
use crate::utils::equip_flags::{EquipType, WeaponType, BulletTypes};
use crate::utils::weapon_patterns::LENGTH_LIST;
use std::fs::File;
use std::io::Write;
use serde_json;

impl MhfdatApp {
    /// Recompute weapon counts strictly from number of entries (max index + 1)
    pub(crate) fn refresh_weapon_counts_from_entries(&mut self) {
        // Vérifier que le buffer n'est pas vide avant de lire les counts
        if self.buffer.is_empty() {
            return;
        }
        
        if let Some(mut counts) = read_equipment_counts(&self.buffer) {
            counts.numMeleeW = self.melee_weapons.len() as u16;
            counts.numRangedW = self.ranged_weapons.len() as u16;
            let _ = write_equipment_counts(&mut self.buffer, &counts);
        }
    }
    /// Compute next melee weapon model_id from existing max id (not len)
    fn next_melee_weapon_model_id(&self) -> u16 {
        self.melee_weapons
            .iter()
            .map(|w| w.model_id)
            .max()
            .map(|max_id| max_id.saturating_add(1))
            .unwrap_or(0)
    }

    /// Compute next ranged weapon model_id from existing max id (not len)
    fn next_ranged_weapon_model_id(&self) -> u16 {
        self.ranged_weapons
            .iter()
            .map(|w| w.model_id)
            .max()
            .map(|max_id| max_id.saturating_add(1))
            .unwrap_or(0)
    }
    pub fn show_weapons_tab(&mut self, ui: &mut egui::Ui) {
        // Initialize view modes if not present
        if !self.view_mode.contains_key("melee_weapons") {
            self.view_mode.insert("melee_weapons".to_string(), ViewMode::List);
        }
        if !self.view_mode.contains_key("ranged_weapons") {
            self.view_mode.insert("ranged_weapons".to_string(), ViewMode::List);
        }

        // Add weapon category tabs (wrapped)
        ui.horizontal_wrapped(|ui| {
            if ui.selectable_label(self.weapon_tab == WeaponTab::Melee, "Melee Weapons").clicked() {
                self.weapon_tab = WeaponTab::Melee;
            }
            if ui.selectable_label(self.weapon_tab == WeaponTab::Ranged, "Ranged Weapons").clicked() {
                self.weapon_tab = WeaponTab::Ranged;
            }
        });
        ui.separator();

        match self.weapon_tab {
            WeaponTab::Melee => {
                match self.view_mode.get("melee_weapons").unwrap_or(&ViewMode::List) {
                    ViewMode::List => self.show_melee_weapons_list(ui),
                    ViewMode::Details => self.show_melee_weapon_details_view(ui),
                }
            }
            WeaponTab::Ranged => {
                match self.view_mode.get("ranged_weapons").unwrap_or(&ViewMode::List) {
                    ViewMode::List => self.show_ranged_weapons_list(ui),
                    ViewMode::Details => self.show_ranged_weapon_details_view(ui),
                }
            }
        }
    }

    pub fn add_single_melee_weapon(&mut self) {
        let next_model_id = self.next_melee_weapon_model_id();
        let mut new_weapon = MhfdatMeleeWeapon::default();
        new_weapon.model_id = next_model_id;
        
        // Calculer le real_count comme pour les armures
        let mut real_count = 0;
        for weapon in self.melee_weapons.iter() {
            if weapon.model_id == 0xFFFF {
                break;
            }
            real_count += 1;
        }
        
        // Utiliser insert() comme pour les armures
        self.melee_weapons.insert(real_count, new_weapon);
        self.melee_weapon_names.insert(real_count, format!("New Weapon {}", next_model_id));
        self.melee_weapon_descriptions.insert(real_count, ["".to_string(), "".to_string(), "".to_string(), "MhfY".to_string()]);
        
        // Marquer comme modifié
        self.melee_weapons_modified = true;
        self.melee_weapon_names_modified = true;
        self.melee_weapon_descriptions_modified = true;

        // Sélectionner la nouvelle arme et basculer vers la vue détail
        self.selected_melee_index = Some(real_count);
        self.melee_weapons_page = (real_count / 15) as u32;
        
        // Initialiser view_mode si nécessaire
        if !self.view_mode.contains_key("melee_weapons") {
            self.view_mode.insert("melee_weapons".to_string(), ViewMode::List);
        }
        if let Some(view_mode) = self.view_mode.get_mut("melee_weapons") {
            *view_mode = ViewMode::Details;
        }

        // Mettre à jour le compteur d'armes (nombre réel en mémoire)
        self.refresh_weapon_counts_from_entries();
    }

    pub fn show_melee_weapons_list(&mut self, ui: &mut egui::Ui) {
        let count = self.melee_weapons.len();
        use crate::model::mhfdat_pointers::MELEE_WEAPONS_PTR;
        let melee_offset = if let Some((melee_offset, _)) = read_mhfdat_offsets(&self.buffer) {
            melee_offset as usize
        } else {
            MELEE_WEAPONS_PTR as usize
        };

        MhfdatApp::section_header(ui, &format!("Melee Weapons (found: {})", count), |ui| {
            if ui.button("Export to JSON").clicked() {
                // Convert weapons to export format with decomposed bitfields
                let export_weapons: Vec<MeleeWeaponExport> = self.melee_weapons
                    .iter()
                    .enumerate()
                    .map(|(index, weapon)| {
                        let name = self.melee_weapon_names.get(index).cloned().unwrap_or_default();
                        let descriptions = self.melee_weapon_descriptions.get(index).cloned().unwrap_or_default();
                        MeleeWeaponExport::from_weapon_with_data(weapon, &name, &descriptions, index)
                    })
                    .collect();
                if let Ok(json) = serde_json::to_string_pretty(&export_weapons) {
                    if let Ok(mut file) = File::create("melee_weapons.json") {
                        let _ = file.write_all(json.as_bytes());
                    }
                }
            }
            if ui.button("Extract Names").clicked() {
                // Extract only weapon names to a text file
                let mut names_content = String::new();
                names_content.push_str("# Noms d'armes de melee extraits\n");
                names_content.push_str(&format!("# Total: {} armes\n", self.melee_weapons.len()));
                names_content.push_str("# Format: ID: Nom\n\n");
                
                for (index, name) in self.melee_weapon_names.iter().enumerate() {
                    names_content.push_str(&format!("{:5}: {}\n", index, name));
                }
                
                if let Ok(mut file) = File::create("melee_weapon_names.txt") {
                    if file.write_all(names_content.as_bytes()).is_ok() {
                        self.error_message = Some(format!("Noms extraits: {} armes sauvées dans 'melee_weapon_names.txt'", self.melee_weapons.len()));
                    } else {
                        self.error_message = Some("Erreur lors de l'écriture du fichier de noms".to_string());
                    }
                } else {
                    self.error_message = Some("Erreur lors de la création du fichier de noms".to_string());
                }
            }
            if ui.button("Add New").clicked() { self.add_single_melee_weapon(); }
        });

        // Search and filters
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Type:");
            egui::ComboBox::from_id_source("class_id_filter_combo")
                .selected_text(self.class_id_filter.map(class_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.class_id_filter, None, "All").clicked() {}
                    for (id, name) in CLASS_ID_LIST {
                        if ui.selectable_value(&mut self.class_id_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Element:");
            egui::ComboBox::from_id_source("element_filter_combo")
                .selected_text(self.element_filter.map(element_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.element_filter, None, "All").clicked() {}
                    for (id, name) in ELEMENT_ID_LIST {
                        if ui.selectable_value(&mut self.element_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Ailment:");
            egui::ComboBox::from_id_source("ailment_filter_combo")
                .selected_text(self.ailment_filter.map(ailment_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.ailment_filter, None, "All").clicked() {}
                    for (id, name) in AILMENT_ID_LIST {
                        if ui.selectable_value(&mut self.ailment_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Equip:");
            egui::ComboBox::from_id_source("equip_type_filter_combo")
                .selected_text(self.equip_type_filter.map(equip_type_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.equip_type_filter, None, "All").clicked() {}
                    for (id, name) in EQUIP_TYPE_LIST {
                        if ui.selectable_value(&mut self.equip_type_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("WeaponType:");
            egui::ComboBox::from_id_source("weapon_type_filter_combo")
                .selected_text(self.weapon_type_filter.map(weapon_type_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.weapon_type_filter, None, "All").clicked() {}
                    for (id, name) in WEAPON_TYPE_LIST {
                        if ui.selectable_value(&mut self.weapon_type_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Zenith:");
            egui::ComboBox::from_id_source("zenith_skill_filter_combo")
                .selected_text(self.zenith_skill_filter.map(zenith_skill_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.zenith_skill_filter, None, "All").clicked() {}
                    for (id, name) in ZENITH_SKILL_LIST {
                        if ui.selectable_value(&mut self.zenith_skill_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
            ui.checkbox(&mut self.show_dummy_weapons, "Show Dummy Weapons");
        });

        // filters

        if count == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No melee weapons found at the expected offset!");
        } else {

            // Get filtered weapons
            let query = self.search_query.to_lowercase();
            let filtered_weapons: Vec<(usize, &MhfdatMeleeWeapon)> = self.melee_weapons.iter()
                .enumerate()
                .filter(|(i, weapon)| {
                    // Copy fields to local variables to avoid unaligned references
                    let model_id = weapon.model_id;
                    let rarity = weapon.rarity;
                    let raw_damage = weapon.raw_damage;
                    let affinity = weapon.affinity;
                    let element_id = weapon.element_id;
                    let slots = weapon.slots;
                    let weapon_type = weapon.weapon_type;
                    let weapon_name = self.melee_weapon_names.get(*i).cloned().unwrap_or_default();

                    // Dummy weapon detection
                    let is_dummy = model_id == 0x05AD
                        && rarity == 0x00
                        && weapon.class_id == 0x00
                        && weapon.zenny_cost == 0x00000528
                        && weapon.sharpness_id == 0x00
                        && weapon.sharpness_max == 0x00
                        && raw_damage == 0x003C
                        && weapon.defense == 0x0000
                        && affinity == 0x00
                        && element_id == 0x00
                        && weapon.ele_damage == 0x00
                        && weapon.ailment_id == 0x00
                        && weapon.ail_damage == 0x00
                        && slots == 0x00
                        && weapon.weapon_attribute == 0x00
                        && weapon.unk15 == 0x00
                        && weapon.upgrade_path == 0x00FF
                        && weapon.other_model == 0x0000
                        && weapon.equip_type == 0x00
                        && weapon.unk1b == 0x00
                        && weapon.length == 0x00000000
                        && weapon_type == 0x00000000
                        && weapon.visual_effects == 0x0000
                        && weapon.tower_g50_param_id == 0x0000
                        && weapon.g_rank == 0x00
                        && weapon.unk29 == 0x00
                        && weapon.unk2a == 0x00
                        && weapon.zero_f == 0x0F
                        && weapon.unk2c == 0x00000000
                        && weapon.zenith_skill == 0x0000;

                    if self.show_dummy_weapons {
                        if !is_dummy { return false; }
                    } else {
                        if is_dummy { return false; }
                    }

                    // Apply filters
                    if let Some(class_id) = self.class_id_filter {
                        if weapon.class_id != class_id { return false; }
                    }
                    if let Some(element_id_filter) = self.element_filter {
                        if element_id != element_id_filter { return false; }
                    }
                    if let Some(ailment_id) = self.ailment_filter {
                        if weapon.ailment_id != ailment_id { return false; }
                    }
                    if let Some(equip_type_id) = self.equip_type_filter {
                        if weapon.equip_type != equip_type_id { return false; }
                    }
                    if let Some(weapon_type_id) = self.weapon_type_filter {
                        if weapon_type != weapon_type_id { return false; }
                    }
                    if let Some(zenith_skill_id) = self.zenith_skill_filter {
                        if weapon.zenith_skill != zenith_skill_id { return false; }
                    }

                    if !query.is_empty() && !weapon_name.to_lowercase().contains(&query) {
                        return false;
                    }

                    true
                })
                .collect();

            // Calculate pagination
            let entries_per_page = 15;
            let total_pages = (filtered_weapons.len() + entries_per_page - 1) / entries_per_page;
            let current_page = self.melee_weapons_page as usize;
            let start_idx = current_page * entries_per_page;
            let end_idx = (start_idx + entries_per_page).min(filtered_weapons.len());

            // Weapon list
            egui::CollapsingHeader::new(format!("Weapon List (showing {}-{} of {})", 
                if filtered_weapons.is_empty() { 0 } else { start_idx + 1 }, 
                end_idx, 
                filtered_weapons.len()))
                .default_open(true)
                .show(ui, |ui| {
                    MhfdatApp::list_scroll(ui, "weapon_list_scroll", |ui| {
                            egui::Grid::new("weapon_list_grid")
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label("ID");
                                    ui.label("Model ID");
                                    ui.label("Name");
                                    ui.label("Rarity");
                                    ui.label("Damage");
                                    ui.label("Affinity");
                                    ui.label("Element");
                                    ui.label("Slots");
                                    ui.label("Type");
                                    ui.end_row();

                                    for (original_idx, weapon) in filtered_weapons[start_idx..end_idx].iter() {
                                        let i = *original_idx;
                                        
                                        // Copy fields to local variables to avoid unaligned references
                                        let model_id = weapon.model_id;
                                        let rarity = weapon.rarity;
                                        let raw_damage = weapon.raw_damage;
                                        let affinity = weapon.affinity;
                                        let element_id = weapon.element_id;
                                        let slots = weapon.slots;
                                        let weapon_name = self.melee_weapon_names.get(i).cloned().unwrap_or_default();

                                        let selected = self.selected_melee_index == Some(i);
                                        if ui.selectable_label(selected, format!("{}", i)).clicked() {
                                            self.selected_melee_index = Some(i);
                                            // Initialiser view_mode si nécessaire
                                            if !self.view_mode.contains_key("melee_weapons") {
                                                self.view_mode.insert("melee_weapons".to_string(), ViewMode::List);
                                            }
                                            if let Some(view_mode) = self.view_mode.get_mut("melee_weapons") {
                                                *view_mode = ViewMode::Details;
                                            }
                                        }
                                        ui.label(format!("{}", model_id));
                                        ui.label(&weapon_name);
                                        ui.label(format!("{}", rarity + 1));
                                        ui.label(format!("{}", raw_damage));
                                        ui.label(format!("{}", affinity));
                                        ui.label(format!("{}", element_name(element_id)));
                                        ui.label(format!("{}", slots));
                                        ui.label(weapon.weapon_type_string());
                                        ui.end_row();
                                    }
                                });
                        });
                });

            // Pagination controls
            MhfdatApp::pagination_controls(ui, &mut self.melee_weapons_page, total_pages);
        }
    }

    pub fn show_ranged_weapons_list(&mut self, ui: &mut egui::Ui) {
        let count = self.ranged_weapons.len();
        let ranged_offset = if let Some((_, ranged_offset)) = read_mhfdat_offsets(&self.buffer) {
            ranged_offset as usize
        } else {
            0 // Default offset if not found
        };

        MhfdatApp::section_header(ui, &format!("Ranged Weapons (found: {})", count), |ui| {
            if ui.button("Export to JSON").clicked() {
                // Convert weapons to export format with decomposed bitfields
                let export_weapons: Vec<RangedWeaponExport> = self.ranged_weapons
                    .iter()
                    .enumerate()
                    .map(|(index, weapon)| {
                        let name = self.ranged_weapon_names.get(index).cloned().unwrap_or_default();
                        let descriptions = self.ranged_weapon_descriptions.get(index).cloned().unwrap_or_default();
                        RangedWeaponExport::from_weapon_with_data(weapon, &name, &descriptions, index)
                    })
                    .collect();
                if let Ok(json) = serde_json::to_string_pretty(&export_weapons) {
                    if let Ok(mut file) = File::create("ranged_weapons.json") {
                        let _ = file.write_all(json.as_bytes());
                    }
                }
            }
        
            if ui.button("Add New").clicked() {
                let next_model_id = self.next_ranged_weapon_model_id();
                let mut new_weapon = MhfdatRangedWeapon::default();
                new_weapon.model_id = next_model_id;
                
                // Calculer le real_count comme pour les armures
                let mut real_count = 0;
                for weapon in self.ranged_weapons.iter() {
                    if weapon.model_id == 0xFFFF {
                        break;
                    }
                    real_count += 1;
                }
                
                // Utiliser insert() comme pour les armures
                self.ranged_weapons.insert(real_count, new_weapon);
                self.ranged_weapon_names.insert(real_count, format!("New Ranged Weapon {}", next_model_id));
                self.ranged_weapon_descriptions.insert(real_count, ["".to_string(), "".to_string(), "".to_string(), "MhfY".to_string()]);
                
                // Marquer comme modifié
                self.ranged_weapons_modified = true;
                self.ranged_weapon_names_modified = true;
                self.ranged_weapon_descriptions_modified = true;
                
                self.selected_ranged_index = Some(real_count);
                self.ranged_weapons_page = (real_count / 15) as u32;
                
                // Initialiser view_mode si nécessaire
                if !self.view_mode.contains_key("ranged_weapons") {
                    self.view_mode.insert("ranged_weapons".to_string(), ViewMode::List);
                }
                if let Some(view_mode) = self.view_mode.get_mut("ranged_weapons") {
                    *view_mode = ViewMode::Details;
                }
                self.refresh_weapon_counts_from_entries();
            }
        });

        // Search and filters
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Type:");
            egui::ComboBox::from_id_source("class_id_filter_combo")
                .selected_text(self.class_id_filter.map(class_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.class_id_filter, None, "All").clicked() {}
                    for (id, name) in CLASS_ID_LIST {
                        if ui.selectable_value(&mut self.class_id_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Element:");
            egui::ComboBox::from_id_source("element_filter_combo")
                .selected_text(self.element_filter.map(element_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.element_filter, None, "All").clicked() {}
                    for (id, name) in ELEMENT_ID_LIST {
                        if ui.selectable_value(&mut self.element_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Equip:");
            egui::ComboBox::from_id_source("equip_type_filter_combo")
                .selected_text(self.equip_type_filter.map(equip_type_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.equip_type_filter, None, "All").clicked() {}
                    for (id, name) in EQUIP_TYPE_LIST {
                        if ui.selectable_value(&mut self.equip_type_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("WeaponType:");
            egui::ComboBox::from_id_source("weapon_type_filter_combo")
                .selected_text(self.weapon_type_filter.map(weapon_type_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.weapon_type_filter, None, "All").clicked() {}
                    for (id, name) in WEAPON_TYPE_LIST {
                        if ui.selectable_value(&mut self.weapon_type_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Zenith:");
            egui::ComboBox::from_id_source("zenith_skill_filter_combo")
                .selected_text(self.zenith_skill_filter.map(zenith_skill_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.zenith_skill_filter, None, "All").clicked() {}
                    for (id, name) in ZENITH_SKILL_LIST {
                        if ui.selectable_value(&mut self.zenith_skill_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
            ui.checkbox(&mut self.show_dummy_ranged_weapons, "Show Dummy Weapons");
        });

        // filters

        if count == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No ranged weapons found at the expected offset!");
        } else {

            // Get filtered weapons
            let query = self.search_query.to_lowercase();
            let filtered_weapons: Vec<(usize, &MhfdatRangedWeapon)> = self.ranged_weapons.iter()
                .enumerate()
                .filter(|(i, weapon)| {
                    // Copy fields to local variables to avoid unaligned references
                    let model_id = weapon.model_id;
                    let rarity = weapon.rarity;
                    let raw_damage = weapon.raw_damage;
                    let affinity = weapon.affinity;
                    let element_id = weapon.element_id;
                    let slots = weapon.slots;
                    let weapon_type = weapon.weapon_type;
                    let weapon_name = self.ranged_weapon_names.get(*i).cloned().unwrap_or_default();

                    // Dummy weapon detection using the pattern
                    let is_dummy = model_id == 0x00E0
                        && rarity == 0x05
                        && weapon.class_id == 0x0A
                        && weapon.zenny_cost == 0x00035B60
                        && raw_damage == 0x00DC
                        && weapon.defense == 0x0000
                        && affinity == 0x0F
                        && element_id == 0x00
                        && weapon.ele_damage == 0x00
                        && slots == 0x01
                        && weapon.weapon_attribute == 0x19
                        && weapon.equip_type == 0x00
                        && weapon_type == 0x00000000
                        && weapon.bullet == 0x0000001A
                        && weapon.recoil == 0x00
                        && weapon.reload == 0x00
                        && weapon.tower_g50_param_id == 0x0000
                        && weapon.g_rank == 0x00
                        && weapon.zenith_skill == 0x0000;

                    if self.show_dummy_ranged_weapons {
                        if !is_dummy { return false; }
                    } else {
                        if is_dummy { return false; }
                    }

                    // Apply filters
                    if let Some(class_id) = self.class_id_filter {
                        if weapon.class_id != class_id { return false; }
                    }
                    if let Some(element_id_filter) = self.element_filter {
                        if element_id != element_id_filter { return false; }
                    }
                    if let Some(equip_type_id) = self.equip_type_filter {
                        if weapon.equip_type != equip_type_id { return false; }
                    }
                    if let Some(weapon_type_id) = self.weapon_type_filter {
                        if weapon_type != weapon_type_id { return false; }
                    }
                    if let Some(zenith_skill_id) = self.zenith_skill_filter {
                        if weapon.zenith_skill != zenith_skill_id { return false; }
                    }

                    if !query.is_empty() && !weapon_name.to_lowercase().contains(&query) {
                        return false;
                    }

                    true
                })
                .collect();

            // Calculate pagination
            let entries_per_page = 15;
            let total_pages = (filtered_weapons.len() + entries_per_page - 1) / entries_per_page;
            let current_page = self.ranged_weapons_page as usize;
            let start_idx = current_page * entries_per_page;
            let end_idx = (start_idx + entries_per_page).min(filtered_weapons.len());

            // Weapon list
            egui::CollapsingHeader::new(format!("Weapon List (showing {}-{} of {})", 
                if filtered_weapons.is_empty() { 0 } else { start_idx + 1 }, 
                end_idx, 
                filtered_weapons.len()))
                .default_open(true)
                .show(ui, |ui| {
                    MhfdatApp::list_scroll(ui, "ranged_weapon_list_scroll", |ui| {
                            egui::Grid::new("ranged_weapon_list_grid")
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label("ID");
                                    ui.label("Model ID");
                                    ui.label("Name");
                                    ui.label("Rarity");
                                    ui.label("Damage");
                                    ui.label("Affinity");
                                    ui.label("Element");
                                    ui.label("Slots");
                                    ui.label("Type");
                                    ui.end_row();

                                    for (original_idx, weapon) in filtered_weapons[start_idx..end_idx].iter() {
                                        let i = *original_idx;
                                        
                                        // Copy fields to local variables to avoid unaligned references
                                        let model_id = weapon.model_id;
                                        let rarity = weapon.rarity;
                                        let raw_damage = weapon.raw_damage;
                                        let affinity = weapon.affinity;
                                        let element_id = weapon.element_id;
                                        let slots = weapon.slots;
                                        let weapon_name = self.ranged_weapon_names.get(i).cloned().unwrap_or_default();

                                        let selected = self.selected_ranged_index == Some(i);
                                        if ui.selectable_label(selected, format!("{}", i)).clicked() {
                                            self.selected_ranged_index = Some(i);
                                            // Initialiser view_mode si nécessaire
                                            if !self.view_mode.contains_key("ranged_weapons") {
                                                self.view_mode.insert("ranged_weapons".to_string(), ViewMode::List);
                                            }
                                            if let Some(view_mode) = self.view_mode.get_mut("ranged_weapons") {
                                                *view_mode = ViewMode::Details;
                                            }
                                        }
                                        ui.label(format!("{}", model_id));
                                        ui.label(&weapon_name);
                                        ui.label(format!("{}", rarity + 1));
                                        ui.label(format!("{}", raw_damage));
                                        ui.label(format!("{}", affinity));
                                        ui.label(format!("{}", element_name(element_id)));
                                        ui.label(format!("{}", slots));
                                        ui.label(weapon.weapon_type_string());
                                        ui.end_row();
                                    }
                                });
                        });
                });

            // Pagination controls
            MhfdatApp::pagination_controls(ui, &mut self.ranged_weapons_page, total_pages);
        }
    }

    pub fn render_melee_weapon_details(ui: &mut egui::Ui, weapon: &mut MhfdatMeleeWeapon, zenith_skill_search: &mut String) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Basic Stats
            ui.collapsing("Basic Stats", |ui| {
                let mut model_id = weapon.model_id;
                let mut rarity = weapon.rarity + 1;
                let mut class_id = weapon.class_id;
                let mut zenny_cost = weapon.zenny_cost;

                Self::render_editable_field(ui, "Model ID", &mut model_id);
                Self::render_editable_field(ui, "Rarity", &mut rarity);
                Self::render_combo_field(ui, "Class", &mut class_id, CLASS_ID_LIST);
                Self::render_editable_field(ui, "Zenny Cost", &mut zenny_cost);

                weapon.model_id = model_id;
                weapon.rarity = rarity.saturating_sub(1);
                weapon.class_id = class_id;
                weapon.zenny_cost = zenny_cost;
            });

            // Combat Stats
            ui.collapsing("Combat Stats", |ui| {
                let mut raw_damage = weapon.raw_damage;
                let mut defense = weapon.defense;
                let mut affinity = weapon.affinity;
                let mut sharpness_id = weapon.sharpness_id;
                let mut sharpness_max = weapon.sharpness_max;

                Self::render_editable_field(ui, "Raw Damage", &mut raw_damage);
                Self::render_editable_field(ui, "Defense", &mut defense);
                Self::render_editable_field(ui, "Affinity", &mut affinity);
                Self::render_editable_u8_field_with_max(ui, "Sharpness ID", &mut sharpness_id, 128);
                Self::render_editable_u8_field_with_max(ui, "Sharpness Max", &mut sharpness_max, 4);

                weapon.raw_damage = raw_damage;
                weapon.defense = defense;
                weapon.affinity = affinity;
                weapon.sharpness_id = sharpness_id;
                weapon.sharpness_max = sharpness_max;
            });

            // Element & Status
            ui.collapsing("Element & Status", |ui| {
                let mut element_id = weapon.element_id;
                let mut ele_damage = weapon.ele_damage;
                let mut ailment_id = weapon.ailment_id;
                let mut ail_damage = weapon.ail_damage;

                Self::render_combo_field(ui, "Element", &mut element_id, ELEMENT_ID_LIST);
                Self::render_editable_u8_field(ui, "Element Damage", &mut ele_damage);
                Self::render_combo_field(ui, "Ailment", &mut ailment_id, AILMENT_ID_LIST);
                Self::render_editable_u8_field(ui, "Ailment Damage", &mut ail_damage);

                weapon.element_id = element_id;
                weapon.ele_damage = ele_damage;
                weapon.ailment_id = ailment_id;
                weapon.ail_damage = ail_damage;
            });

            // Weapon Properties
            ui.collapsing("Weapon Properties", |ui| {
                let mut slots = weapon.slots;
                let mut weapon_attribute = weapon.weapon_attribute;
                let mut upgrade_path = weapon.upgrade_path;
                let mut other_model = weapon.other_model;
                
                // Use bitfield editors for equip_type and weapon_type
                let mut equip_type = weapon.get_equip_type();
                let mut weapon_type = weapon.get_weapon_type();

                Self::render_editable_u8_field(ui, "Slots", &mut slots);
                Self::render_editable_u8_field(ui, "Weapon Attribute", &mut weapon_attribute);
                Self::render_equip_type_field(ui, "Equipment Type", &mut equip_type);
                Self::render_weapon_type_field(ui, "Weapon Type", &mut weapon_type);
                Self::render_editable_field(ui, "Upgrade Path", &mut upgrade_path);
                Self::render_editable_field(ui, "Other Model", &mut other_model);

                weapon.slots = slots;
                weapon.weapon_attribute = weapon_attribute;
                weapon.set_equip_type(equip_type);
                weapon.set_weapon_type(weapon_type);
                weapon.upgrade_path = upgrade_path;
                weapon.other_model = other_model;
            });

            // Advanced Properties
            ui.collapsing("Advanced Properties", |ui| {
                let mut length = weapon.length;
                let mut visual_effects = weapon.visual_effects;
                let mut tower_g50_param_id = weapon.tower_g50_param_id;
                let mut g_rank = weapon.g_rank;
                let mut zenith_skill = weapon.zenith_skill;

                Self::render_combo_field(ui, "Length", &mut length, LENGTH_LIST);
                Self::render_editable_field(ui, "Visual Effects", &mut visual_effects);
                Self::render_editable_field(ui, "Tower G50 Param ID", &mut tower_g50_param_id);
                Self::render_combo_field(ui, "G Rank", &mut g_rank, &[(0, "Non-G"), (1, "G-Rank")]);
                
                // Zenith Skill with search
                ui.horizontal(|ui| {
                    ui.label("Zenith Skill:");
                    ui.add(egui::TextEdit::singleline(zenith_skill_search)
                        .hint_text("Search...").desired_width(100.0));
                    
                    let q = zenith_skill_search.to_lowercase();
                    let current_text = ZENITH_SKILL_LIST.iter()
                        .find(|(v, _)| *v == zenith_skill)
                        .map(|(_, name)| *name)
                        .unwrap_or("Unknown");
                    
                    egui::ComboBox::from_id_source("melee_zenith_skill_combo")
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

                weapon.length = length;
                weapon.visual_effects = visual_effects;
                weapon.tower_g50_param_id = tower_g50_param_id;
                weapon.g_rank = g_rank;
                weapon.zenith_skill = zenith_skill;
            });
        });
    }

    pub fn render_ranged_weapon_details(ui: &mut egui::Ui, weapon: &mut MhfdatRangedWeapon, zenith_skill_search: &mut String) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Basic Stats
            ui.collapsing("Basic Stats", |ui| {
                let mut model_id = weapon.model_id;
                let mut rarity = weapon.rarity + 1;
                let mut class_id = weapon.class_id;
                let mut zenny_cost = weapon.zenny_cost;

                Self::render_editable_field(ui, "Model ID", &mut model_id);
                Self::render_editable_field(ui, "Rarity", &mut rarity);
                Self::render_combo_field(ui, "Class", &mut class_id, CLASS_ID_LIST);
                Self::render_editable_field(ui, "Zenny Cost", &mut zenny_cost);

                weapon.model_id = model_id;
                weapon.rarity = rarity.saturating_sub(1);
                weapon.class_id = class_id;
                weapon.zenny_cost = zenny_cost;
            });

            // Combat Stats
            ui.collapsing("Combat Stats", |ui| {
                let mut raw_damage = weapon.raw_damage;
                let mut defense = weapon.defense;
                let mut affinity = weapon.affinity;
                let mut recoil = weapon.recoil;
                let mut reload = weapon.reload;

                Self::render_editable_field(ui, "Raw Damage", &mut raw_damage);
                Self::render_editable_field(ui, "Defense", &mut defense);
                Self::render_editable_field(ui, "Affinity", &mut affinity);
                Self::render_combo_field(ui, "Recoil", &mut recoil, RECOIL_LIST);
                Self::render_combo_field(ui, "Reload", &mut reload, RELOAD_LIST);

                weapon.raw_damage = raw_damage;
                weapon.defense = defense;
                weapon.affinity = affinity;
                weapon.recoil = recoil;
                weapon.reload = reload;
            });

            // Element & Status
            ui.collapsing("Element & Status", |ui| {
                let mut element_id = weapon.element_id;
                let mut ele_damage = weapon.ele_damage;

                Self::render_combo_field(ui, "Element", &mut element_id, ELEMENT_ID_LIST);
                Self::render_editable_u8_field(ui, "Element Damage", &mut ele_damage);

                weapon.element_id = element_id;
                weapon.ele_damage = ele_damage;
            });

            // Weapon Properties
            ui.collapsing("Weapon Properties", |ui| {
                let mut slots = weapon.slots;
                let mut weapon_attribute = weapon.weapon_attribute;
                
                // Use bitfield editors for equip_type, weapon_type, and bullet types
                let mut equip_type = weapon.get_equip_type();
                let mut weapon_type = weapon.get_weapon_type();
                let mut bullet_types = weapon.get_bullet_types();

                Self::render_editable_u8_field(ui, "Slots", &mut slots);
                Self::render_editable_u8_field(ui, "Weapon Attribute", &mut weapon_attribute);
                Self::render_equip_type_field(ui, "Equipment Type", &mut equip_type);
                Self::render_weapon_type_field(ui, "Weapon Type", &mut weapon_type);

                weapon.slots = slots;
                weapon.weapon_attribute = weapon_attribute;
                weapon.set_equip_type(equip_type);
                weapon.set_weapon_type(weapon_type);
                weapon.set_bullet_types(bullet_types);
            });
            
            // Bullet Types Box - All ammo types with checkboxes
            ui.collapsing("Ammo Types", |ui| {
                let mut bullet_types = weapon.get_bullet_types();
                
                // All ammo types displayed with checkboxes
                ui.horizontal_wrapped(|ui| {
                    ui.checkbox(&mut bullet_types.normal_lv1, "Normal Lv1");
                    ui.checkbox(&mut bullet_types.normal_lv2, "Normal Lv2");
                    ui.checkbox(&mut bullet_types.normal_lv3, "Normal Lv3");
                    ui.checkbox(&mut bullet_types.pierce_lv1, "Pierce Lv1");
                    ui.checkbox(&mut bullet_types.pierce_lv2, "Pierce Lv2");
                    ui.checkbox(&mut bullet_types.pierce_lv3, "Pierce Lv3");
                    ui.checkbox(&mut bullet_types.spread_lv1, "Spread Lv1");
                    ui.checkbox(&mut bullet_types.spread_lv2, "Spread Lv2");
                    ui.checkbox(&mut bullet_types.spread_lv3, "Spread Lv3");
                    ui.checkbox(&mut bullet_types.crag_lv1, "Crag Lv1");
                    ui.checkbox(&mut bullet_types.crag_lv2, "Crag Lv2");
                    ui.checkbox(&mut bullet_types.crag_lv3, "Crag Lv3");
                    ui.checkbox(&mut bullet_types.cluster_lv1, "Cluster Lv1");
                    ui.checkbox(&mut bullet_types.cluster_lv2, "Cluster Lv2");
                    ui.checkbox(&mut bullet_types.cluster_lv3, "Cluster Lv3");
                    ui.checkbox(&mut bullet_types.fire, "Fire");
                    ui.checkbox(&mut bullet_types.water, "Water");
                    ui.checkbox(&mut bullet_types.thunder, "Thunder");
                    ui.checkbox(&mut bullet_types.ice, "Ice");
                    ui.checkbox(&mut bullet_types.dragon, "Dragon");
                    ui.checkbox(&mut bullet_types.recovery_lv1, "Recovery Lv1");
                    ui.checkbox(&mut bullet_types.recovery_lv2, "Recovery Lv2");
                    ui.checkbox(&mut bullet_types.poison_lv1, "Poison Lv1");
                    ui.checkbox(&mut bullet_types.poison_lv2, "Poison Lv2");
                    ui.checkbox(&mut bullet_types.paralysis_lv1, "Paralysis Lv1");
                    ui.checkbox(&mut bullet_types.paralysis_lv2, "Paralysis Lv2");
                    ui.checkbox(&mut bullet_types.sleep_lv1, "Sleep Lv1");
                    ui.checkbox(&mut bullet_types.sleep_lv2, "Sleep Lv2");
                    ui.checkbox(&mut bullet_types.tranquilizer, "Tranquilizer");
                    ui.checkbox(&mut bullet_types.paint, "Paint");
                    ui.checkbox(&mut bullet_types.demon, "Demon");
                    ui.checkbox(&mut bullet_types.armor, "Armor");
                });
                
                // Update the weapon's bullet types
                weapon.set_bullet_types(bullet_types);
            });

            // Advanced Properties
            ui.collapsing("Advanced Properties", |ui| {
                let mut tower_g50_param_id = weapon.tower_g50_param_id;
                let mut g_rank = weapon.g_rank;
                let mut zenith_skill = weapon.zenith_skill;
                let mut sort_order = weapon.sort_order_maybe;
                let mut max_slots = weapon.max_slots_maybe;

                Self::render_editable_field(ui, "Tower G50 Param ID", &mut tower_g50_param_id);
                Self::render_combo_field(ui, "G Rank", &mut g_rank, &[(0, "Non-G"), (1, "G-Rank")]);
                
                // Zenith Skill with search
                ui.horizontal(|ui| {
                    ui.label("Zenith Skill:");
                    ui.add(egui::TextEdit::singleline(zenith_skill_search)
                        .hint_text("Search...").desired_width(100.0));
                    
                    let q = zenith_skill_search.to_lowercase();
                    let current_text = ZENITH_SKILL_LIST.iter()
                        .find(|(v, _)| *v == zenith_skill)
                        .map(|(_, name)| *name)
                        .unwrap_or("Unknown");
                    
                    egui::ComboBox::from_id_source("ranged_zenith_skill_combo")
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
                
                Self::render_editable_u8_field(ui, "Sort Order", &mut sort_order);
                Self::render_editable_u8_field(ui, "Max Slots", &mut max_slots);

                weapon.tower_g50_param_id = tower_g50_param_id;
                weapon.g_rank = g_rank;
                weapon.zenith_skill = zenith_skill;
                weapon.sort_order_maybe = sort_order;
                weapon.max_slots_maybe = max_slots;
            });
        });
    }

    pub fn render_editable_field<T: eframe::emath::Numeric + Copy + PartialEq>(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut T
    ) {
        ui.horizontal(|ui| {
            ui.label(label);
            let mut temp = *value;
            if ui.add(egui::DragValue::new(&mut temp)).changed() {
                *value = temp;
            }
        });
    }
    
    pub fn render_editable_u8_field(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut u8
    ) {
        ui.horizontal(|ui| {
            ui.label(label);
            let mut temp = *value as u32;
            if ui.add(egui::DragValue::new(&mut temp)).changed() {
                *value = temp.min(255) as u8;
            }
        });
    }
    
    pub fn render_editable_u8_field_with_max(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut u8,
        max_value: u8
    ) {
        ui.horizontal(|ui| {
            ui.label(label);
            let mut temp = *value as u32;
            if ui.add(egui::DragValue::new(&mut temp)).changed() {
                *value = temp.min(max_value as u32) as u8;
            }
        });
    }

    pub fn render_combo_field<T: Copy + PartialEq>(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut T,
        options: &[(T, &str)]
    ) {
        ui.horizontal(|ui| {
            ui.label(label);
            let current_text = options.iter()
                .find(|(v, _)| *v == *value)
                .map(|(_, name)| *name)
                .unwrap_or("Unknown");
            
            egui::ComboBox::from_id_source(format!("{}_{}", label, std::any::type_name::<T>()))
                .selected_text(current_text)
                .show_ui(ui, |ui| {
                    for (id, name) in options {
                        ui.selectable_value(value, *id, *name);
                    }
                });
        });
    }
    
    /// Render a bitfield editor for EquipType
    pub fn render_equip_type_field(
        ui: &mut egui::Ui,
        label: &str,
        equip_type: &mut EquipType
    ) {
        ui.vertical(|ui| {
            ui.label(label);
            ui.horizontal_wrapped(|ui| {
                ui.checkbox(&mut equip_type.sp, "SP");
                ui.checkbox(&mut equip_type.gou, "Gou");
                ui.checkbox(&mut equip_type.evolution, "Evolution");
                ui.checkbox(&mut equip_type.hc, "HC");
                ui.checkbox(&mut equip_type.random_weapon, "Random Weapon");
                ui.checkbox(&mut equip_type.ravi, "Ravi");
                ui.checkbox(&mut equip_type.g50, "G50");
                ui.checkbox(&mut equip_type.unk_7, "Unknown_7");
            });
            ui.label(format!("Preview: {}", equip_type.to_string()));
        });
    }
    
    /// Render a bitfield editor for WeaponType
    pub fn render_weapon_type_field(
        ui: &mut egui::Ui,
        label: &str,
        weapon_type: &mut WeaponType
    ) {
        ui.vertical(|ui| {
            ui.label(label);
            ui.horizontal_wrapped(|ui| {
                ui.checkbox(&mut weapon_type.finess_base, "Finess Base");
                ui.checkbox(&mut weapon_type.gou_hr1, "Gou HR1");
                ui.checkbox(&mut weapon_type.gou_hr2, "Gou HR2");
                ui.checkbox(&mut weapon_type.gr_gunner, "GR Gunner");
                ui.checkbox(&mut weapon_type.gou_gr1, "Gou GR1");
                ui.checkbox(&mut weapon_type.gou_gr2, "Gou GR2");
                ui.checkbox(&mut weapon_type.finess_ext, "Finess Ext");
                ui.checkbox(&mut weapon_type.tower, "Tower");
            });
            ui.horizontal_wrapped(|ui| {
                ui.checkbox(&mut weapon_type.gou_gr3, "Gou GR3");
                ui.checkbox(&mut weapon_type.exotique, "Exotic");
                ui.checkbox(&mut weapon_type.ravi_z, "Ravi Z");
                ui.checkbox(&mut weapon_type.prayer_base, "Prayer Base");
                ui.checkbox(&mut weapon_type.zenith, "Zenith");
                ui.checkbox(&mut weapon_type.ravi_gr_plus, "Ravi GR+");
                ui.checkbox(&mut weapon_type.gr_simple_upgrade, "GR Simple upgrade");
            });
            ui.label(format!("Preview: {}", weapon_type.to_string()));
        });
    }
    
    /// Render a bitfield editor for BulletTypes
    pub fn render_bullet_types_field(
        ui: &mut egui::Ui,
        label: &str,
        bullet_types: &mut BulletTypes
    ) {
        ui.vertical(|ui| {
            ui.label(label);
            
            ui.collapsing("Normal Shots", |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut bullet_types.normal_lv1, "Normal Lv1");
                    ui.checkbox(&mut bullet_types.normal_lv2, "Normal Lv2");
                    ui.checkbox(&mut bullet_types.normal_lv3, "Normal Lv3");
                });
            });
            
            ui.collapsing("Pierce Shots", |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut bullet_types.pierce_lv1, "Pierce Lv1");
                    ui.checkbox(&mut bullet_types.pierce_lv2, "Pierce Lv2");
                    ui.checkbox(&mut bullet_types.pierce_lv3, "Pierce Lv3");
                });
            });
            
            ui.collapsing("Spread Shots", |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut bullet_types.spread_lv1, "Spread Lv1");
                    ui.checkbox(&mut bullet_types.spread_lv2, "Spread Lv2");
                    ui.checkbox(&mut bullet_types.spread_lv3, "Spread Lv3");
                });
            });
            
            ui.collapsing("Explosive Shots", |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.checkbox(&mut bullet_types.crag_lv1, "Crag Lv1");
                    ui.checkbox(&mut bullet_types.crag_lv2, "Crag Lv2");
                    ui.checkbox(&mut bullet_types.crag_lv3, "Crag Lv3");
                    ui.checkbox(&mut bullet_types.cluster_lv1, "Cluster Lv1");
                    ui.checkbox(&mut bullet_types.cluster_lv2, "Cluster Lv2");
                    ui.checkbox(&mut bullet_types.cluster_lv3, "Cluster Lv3");
                });
            });
            
            ui.collapsing("Elemental Shots", |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.checkbox(&mut bullet_types.fire, "Fire");
                    ui.checkbox(&mut bullet_types.water, "Water");
                    ui.checkbox(&mut bullet_types.thunder, "Thunder");
                    ui.checkbox(&mut bullet_types.ice, "Ice");
                    ui.checkbox(&mut bullet_types.dragon, "Dragon");
                });
            });
            
            ui.collapsing("Support Shots", |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.checkbox(&mut bullet_types.recovery_lv1, "Recovery Lv1");
                    ui.checkbox(&mut bullet_types.recovery_lv2, "Recovery Lv2");
                    ui.checkbox(&mut bullet_types.poison_lv1, "Poison Lv1");
                    ui.checkbox(&mut bullet_types.poison_lv2, "Poison Lv2");
                    ui.checkbox(&mut bullet_types.paralysis_lv1, "Paralysis Lv1");
                    ui.checkbox(&mut bullet_types.paralysis_lv2, "Paralysis Lv2");
                    ui.checkbox(&mut bullet_types.sleep_lv1, "Sleep Lv1");
                    ui.checkbox(&mut bullet_types.sleep_lv2, "Sleep Lv2");
                });
            });
            
            ui.collapsing("Special Shots", |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.checkbox(&mut bullet_types.tranquilizer, "Tranquilizer");
                    ui.checkbox(&mut bullet_types.paint, "Paint");
                    ui.checkbox(&mut bullet_types.demon, "Demon");
                    ui.checkbox(&mut bullet_types.armor, "Armor");
                });
            });
            
            ui.label(format!("Supported ammo: {}", bullet_types.to_string()));
        });
    }

    pub fn show_melee_weapon_details_view(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            // Initialiser view_mode si nécessaire
            if !self.view_mode.contains_key("melee_weapons") {
                self.view_mode.insert("melee_weapons".to_string(), ViewMode::List);
            }
            if let Some(view_mode) = self.view_mode.get_mut("melee_weapons") {
                *view_mode = ViewMode::List;
            }
            return;
        }

        if let Some(index) = self.selected_melee_index {
            // Marquer comme modifié dès qu'on édite une arme
            self.melee_weapons_modified = true;
            
            if let Some(weapon) = self.melee_weapons.get_mut(index) {
                let name = self.melee_weapon_names.get(index).cloned().unwrap_or_default();
                let descriptions = self.melee_weapon_descriptions.get(index).cloned().unwrap_or_default();
                
                ui.heading(format!("Edit Melee Weapon #{}", index));
                ui.separator();
                
                // Editable name
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    let mut name_edit = name.clone();
                    if ui.text_edit_singleline(&mut name_edit).changed() {
                        if let Some(name_ref) = self.melee_weapon_names.get_mut(index) {
                            *name_ref = name_edit;
                            self.melee_weapon_names_modified = true;
                        }
                    }
                });
                
                // Editable descriptions
                ui.horizontal(|ui| {
                    ui.label("Description 1:");
                    let mut desc1 = descriptions[0].clone();
                    if ui.text_edit_singleline(&mut desc1).changed() {
                        if let Some(descs) = self.melee_weapon_descriptions.get_mut(index) {
                            descs[0] = desc1;
                            self.melee_weapon_descriptions_modified = true;
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Description 2:");
                    let mut desc2 = descriptions[1].clone();
                    if ui.text_edit_singleline(&mut desc2).changed() {
                        if let Some(descs) = self.melee_weapon_descriptions.get_mut(index) {
                            descs[1] = desc2;
                            self.melee_weapon_descriptions_modified = true;
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Description 3:");
                    let mut desc3 = descriptions[2].clone();
                    if ui.text_edit_singleline(&mut desc3).changed() {
                        if let Some(descs) = self.melee_weapon_descriptions.get_mut(index) {
                            descs[2] = desc3;
                            self.melee_weapon_descriptions_modified = true;
                        }
                    }
                });
                
                ui.separator();
                Self::render_melee_weapon_details(ui, weapon, &mut self.zenith_skill_search);

                // Upgrades (index-aligned: weapon i ↔ upgrade i)
                ui.separator();
                ui.collapsing("Upgrade Path", |ui| {
                    if index < self.mw_upgrade_entries.len() {
                        // Copy to locals
                        let (mut mat1, mut qty1, mut mat2, mut qty2, mut mat3, mut qty3, mut to1, mut to2, mut to3, mut to4) = {
                            let up = &self.mw_upgrade_entries[index];
                            (up.upgrade_material1, up.num_material1,
                             up.upgrade_material2, up.num_material2,
                             up.upgrade_material3, up.num_material3,
                             up.upgrades_to1, up.upgrades_to2, up.upgrades_to3, up.upgrades_to4)
                        };

                        let item_name = |id: u16| -> String {
                            self.item_names.get(id as usize).cloned().unwrap_or_default()
                        };
                        let melee_name = |id: usize| -> String {
                            self.melee_weapon_names.get(id).cloned().unwrap_or_default()
                        };

                        let mut upgrade_changed = false;
                        ui.label("Required Materials:");
                        egui::Grid::new("mw_upgrade_path_grid").striped(true).show(ui, |ui| {
                            ui.label("Material ID"); ui.label("Material Name"); ui.label("Qty"); ui.end_row();
                            
                            if ui.add(egui::DragValue::new(&mut mat1)).changed() {
                                upgrade_changed = true;
                            }
                            ui.label(format!("{}", item_name(mat1)));
                            if ui.add(egui::DragValue::new(&mut qty1)).changed() {
                                upgrade_changed = true;
                            }
                            ui.end_row();

                            if ui.add(egui::DragValue::new(&mut mat2)).changed() {
                                upgrade_changed = true;
                            }
                            ui.label(format!("{}", item_name(mat2)));
                            if ui.add(egui::DragValue::new(&mut qty2)).changed() {
                                upgrade_changed = true;
                            }
                            ui.end_row();

                            if ui.add(egui::DragValue::new(&mut mat3)).changed() {
                                upgrade_changed = true;
                            }
                            ui.label(format!("{}", item_name(mat3)));
                            if ui.add(egui::DragValue::new(&mut qty3)).changed() {
                                upgrade_changed = true;
                            }
                            ui.end_row();
                        });

                        ui.separator();
                        ui.label("Upgrades To (Melee):");
                        egui::Grid::new("mw_upgrade_targets_grid").striped(true).show(ui, |ui| {
                            ui.label("Weapon ID"); ui.label("Weapon Name"); ui.end_row();
                            if ui.add(egui::DragValue::new(&mut to1)).changed() { upgrade_changed = true; }
                            ui.label(format!("{}", melee_name(to1 as usize))); ui.end_row();
                            if ui.add(egui::DragValue::new(&mut to2)).changed() { upgrade_changed = true; }
                            ui.label(format!("{}", melee_name(to2 as usize))); ui.end_row();
                            if ui.add(egui::DragValue::new(&mut to3)).changed() { upgrade_changed = true; }
                            ui.label(format!("{}", melee_name(to3 as usize))); ui.end_row();
                            if ui.add(egui::DragValue::new(&mut to4)).changed() { upgrade_changed = true; }
                            ui.label(format!("{}", melee_name(to4 as usize))); ui.end_row();
                        });

                        // Write back
                        if let Some(entry) = self.mw_upgrade_entries.get_mut(index) {
                            entry.upgrade_material1 = mat1;
                            entry.num_material1 = qty1;
                            entry.upgrade_material2 = mat2;
                            entry.num_material2 = qty2;
                            entry.upgrade_material3 = mat3;
                            entry.num_material3 = qty3;
                            entry.upgrades_to1 = to1;
                            entry.upgrades_to2 = to2;
                            entry.upgrades_to3 = to3;
                            entry.upgrades_to4 = to4;
                        }
                        
                        // Marquer comme modifié si des changements ont été faits
                        if upgrade_changed {
                            self.mw_upgrades_modified = true;
                        }
                    } else {
                        ui.label("No upgrade path entry for this weapon index.");
                    }
                });
            }
        } else {
            ui.label("No weapon selected");
        }
    }

    pub fn show_ranged_weapon_details_view(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            // Initialiser view_mode si nécessaire
            if !self.view_mode.contains_key("ranged_weapons") {
                self.view_mode.insert("ranged_weapons".to_string(), ViewMode::List);
            }
            if let Some(view_mode) = self.view_mode.get_mut("ranged_weapons") {
                *view_mode = ViewMode::List;
            }
            return;
        }

        if let Some(index) = self.selected_ranged_index {
            // Marquer comme modifié dès qu'on édite une arme
            self.ranged_weapons_modified = true;
            
            if let Some(weapon) = self.ranged_weapons.get_mut(index) {
                let name = self.ranged_weapon_names.get(index).cloned().unwrap_or_default();
                let descriptions = self.ranged_weapon_descriptions.get(index).cloned().unwrap_or_default();
                
                ui.heading(format!("Edit Ranged Weapon #{}", index));
                ui.separator();
                
                // Editable name
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    let mut name_edit = name.clone();
                    if ui.text_edit_singleline(&mut name_edit).changed() {
                        if let Some(name_ref) = self.ranged_weapon_names.get_mut(index) {
                            *name_ref = name_edit;
                            self.ranged_weapon_names_modified = true;
                        }
                    }
                });
                
                // Editable descriptions
                ui.horizontal(|ui| {
                    ui.label("Description 1:");
                    let mut desc1 = descriptions[0].clone();
                    if ui.text_edit_singleline(&mut desc1).changed() {
                        if let Some(descs) = self.ranged_weapon_descriptions.get_mut(index) {
                            descs[0] = desc1;
                            self.ranged_weapon_descriptions_modified = true;
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Description 2:");
                    let mut desc2 = descriptions[1].clone();
                    if ui.text_edit_singleline(&mut desc2).changed() {
                        if let Some(descs) = self.ranged_weapon_descriptions.get_mut(index) {
                            descs[1] = desc2;
                            self.ranged_weapon_descriptions_modified = true;
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Description 3:");
                    let mut desc3 = descriptions[2].clone();
                    if ui.text_edit_singleline(&mut desc3).changed() {
                        if let Some(descs) = self.ranged_weapon_descriptions.get_mut(index) {
                            descs[2] = desc3;
                            self.ranged_weapon_descriptions_modified = true;
                        }
                    }
                });
                
                ui.separator();
                Self::render_ranged_weapon_details(ui, weapon, &mut self.zenith_skill_search);

                // Upgrades (index-aligned: weapon i ↔ upgrade i)
                ui.separator();
                ui.collapsing("Upgrade Path", |ui| {
                    if index < self.rw_upgrade_entries.len() {
                        // Copy to locals
                        let (mut mat1, mut qty1, mut mat2, mut qty2, mut mat3, mut qty3, mut to1, mut to2, mut to3, mut to4) = {
                            let up = &self.rw_upgrade_entries[index];
                            (up.upgrade_material1, up.num_material1,
                             up.upgrade_material2, up.num_material2,
                             up.upgrade_material3, up.num_material3,
                             up.upgrades_to1, up.upgrades_to2, up.upgrades_to3, up.upgrades_to4)
                        };

                        let item_name = |id: u16| -> String {
                            self.item_names.get(id as usize).cloned().unwrap_or_default()
                        };
                        let ranged_name = |id: usize| -> String {
                            self.ranged_weapon_names.get(id).cloned().unwrap_or_default()
                        };

                        let mut upgrade_changed = false;
                        ui.label("Required Materials:");
                        egui::Grid::new("rw_upgrade_path_grid").striped(true).show(ui, |ui| {
                            ui.label("Material ID"); ui.label("Material Name"); ui.label("Qty"); ui.end_row();
                            
                            if ui.add(egui::DragValue::new(&mut mat1)).changed() {
                                upgrade_changed = true;
                            }
                            ui.label(format!("{}", item_name(mat1)));
                            if ui.add(egui::DragValue::new(&mut qty1)).changed() {
                                upgrade_changed = true;
                            }
                            ui.end_row();

                            if ui.add(egui::DragValue::new(&mut mat2)).changed() {
                                upgrade_changed = true;
                            }
                            ui.label(format!("{}", item_name(mat2)));
                            if ui.add(egui::DragValue::new(&mut qty2)).changed() {
                                upgrade_changed = true;
                            }
                            ui.end_row();

                            if ui.add(egui::DragValue::new(&mut mat3)).changed() {
                                upgrade_changed = true;
                            }
                            ui.label(format!("{}", item_name(mat3)));
                            if ui.add(egui::DragValue::new(&mut qty3)).changed() {
                                upgrade_changed = true;
                            }
                            ui.end_row();
                        });

                        ui.separator();
                        ui.label("Upgrades To (Ranged):");
                        egui::Grid::new("rw_upgrade_targets_grid").striped(true).show(ui, |ui| {
                            ui.label("Weapon ID"); ui.label("Weapon Name"); ui.end_row();
                            if ui.add(egui::DragValue::new(&mut to1)).changed() { upgrade_changed = true; }
                            ui.label(format!("{}", ranged_name(to1 as usize))); ui.end_row();
                            if ui.add(egui::DragValue::new(&mut to2)).changed() { upgrade_changed = true; }
                            ui.label(format!("{}", ranged_name(to2 as usize))); ui.end_row();
                            if ui.add(egui::DragValue::new(&mut to3)).changed() { upgrade_changed = true; }
                            ui.label(format!("{}", ranged_name(to3 as usize))); ui.end_row();
                            if ui.add(egui::DragValue::new(&mut to4)).changed() { upgrade_changed = true; }
                            ui.label(format!("{}", ranged_name(to4 as usize))); ui.end_row();
                        });

                        // Write back
                        if let Some(entry) = self.rw_upgrade_entries.get_mut(index) {
                            entry.upgrade_material1 = mat1;
                            entry.num_material1 = qty1;
                            entry.upgrade_material2 = mat2;
                            entry.num_material2 = qty2;
                            entry.upgrade_material3 = mat3;
                            entry.num_material3 = qty3;
                            entry.upgrades_to1 = to1;
                            entry.upgrades_to2 = to2;
                            entry.upgrades_to3 = to3;
                            entry.upgrades_to4 = to4;
                        }
                        
                        // Marquer comme modifié si des changements ont été faits
                        if upgrade_changed {
                            self.rw_upgrades_modified = true;
                        }
                    } else {
                        ui.label("No upgrade path entry for this weapon index.");
                    }
                });
            }
        } else {
            ui.label("No weapon selected");
        }
    }
} 