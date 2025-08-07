use super::*;
use egui;

impl MhfdatApp {
    fn get_armor_name(&self, equip_type: u8, equip_id: u16) -> String {
        match equip_type {
            0x02 => self.head_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
            0x03 => self.body_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
            0x04 => self.arms_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
            0x05 => self.waist_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
            0x00 => self.legs_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
            _ => "Unknown".to_string(),
        }
    }

    fn get_item_name(&self, item_id: u16) -> String {
        if item_id == 0 {
            "None".to_string()
        } else {
            self.item_names.get(item_id as usize).cloned().unwrap_or_else(|| format!("Unknown Item {}", item_id))
        }
    }

    pub fn show_shop_tab(&mut self, ui: &mut egui::Ui) {
        if !self.view_mode.contains_key("shop") {
            self.view_mode.insert("shop".to_string(), ViewMode::List);
        }

        match self.view_mode.get("shop").unwrap() {
            ViewMode::List => self.show_transmog_list(ui),
            ViewMode::Details => self.show_transmog_details(ui),
        }
    }

    fn show_transmog_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Transmog Shop");

        // Search and filters
        ui.horizontal(|ui| {
            ui.label("Type:");
            egui::ComboBox::from_id_source("shop_armor_type_filter_combo")
                .selected_text(match self.shop_equip_type_filter {
                    Some(0x02) => "Head",
                    Some(0x03) => "Body",
                    Some(0x04) => "Arms",
                    Some(0x05) => "Waist",
                    Some(0x00) => "Legs",
                    _ => "All",
                })
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.shop_equip_type_filter, None, "All").clicked() {}
                    if ui.selectable_value(&mut self.shop_equip_type_filter, Some(0x02), "Head").clicked() {}
                    if ui.selectable_value(&mut self.shop_equip_type_filter, Some(0x03), "Body").clicked() {}
                    if ui.selectable_value(&mut self.shop_equip_type_filter, Some(0x04), "Arms").clicked() {}
                    if ui.selectable_value(&mut self.shop_equip_type_filter, Some(0x05), "Waist").clicked() {}
                    if ui.selectable_value(&mut self.shop_equip_type_filter, Some(0x00), "Legs").clicked() {}
                });

            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);

            if ui.button("Add New Transmog").clicked() {
                self.transmog_entries.push(ShopEntry {
                    equip_type: 0x02, // Default to Head
                    equip_id: 0,
                    material_id1: 0,
                    material_amnt1: 0,
                    material_id2: 0,
                    material_amnt2: 0,
                    material_id3: 0,
                    material_amnt3: 0,
                    material_id4: 0,
                    material_amnt4: 0,
                    hr_req: 0,
                    ..Default::default()
                });
                self.selected_transmog_index = Some(self.transmog_entries.len() - 1);
                *self.view_mode.get_mut("shop").unwrap() = ViewMode::Details;
            }
        });

        // Get filtered entries
        let filtered_entries: Vec<(usize, &ShopEntry)> = self.transmog_entries.iter()
            .enumerate()
            .filter(|(_, entry)| {
                if let Some(filter_type) = self.shop_equip_type_filter {
                    if entry.equip_type != filter_type { return false; }
                }
                if !self.search_query.is_empty() {
                    let armor_name = self.get_armor_name(entry.equip_type, entry.equip_id);
                    if !armor_name.to_lowercase().contains(&self.search_query.to_lowercase()) {
                        return false;
                    }
                }
                true
            })
            .collect();

        // Calculate pagination
        let entries_per_page = 15;
        let total_pages = (filtered_entries.len() + entries_per_page - 1) / entries_per_page;
        let current_page = self.shop_page as usize;
        let start_idx = current_page * entries_per_page;
        let end_idx = (start_idx + entries_per_page).min(filtered_entries.len());

        let mut selected_idx = None;
        let mut should_switch_view = false;

        // Transmog list
        egui::ScrollArea::vertical()
            .id_source("transmog_shop_list_scroll")
            .max_height(600.0)
            .show(ui, |ui| {
                egui::Grid::new("transmog_shop_list_grid")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("ID");
                        ui.label("Type");
                        ui.label("Name");
                        ui.label("Material 1");
                        ui.label("Amount 1");
                        ui.label("Material 2");
                        ui.label("Amount 2");
                        ui.label("Material 3");
                        ui.label("Amount 3");
                        ui.label("Material 4");
                        ui.label("Amount 4");
                        ui.label("HR Req");
                        ui.end_row();

                        for (i, (original_idx, entry)) in filtered_entries[start_idx..end_idx].iter().enumerate() {
                            let equip_type = entry.equip_type;
                            let equip_id = entry.equip_id;
                            let material_id1 = entry.material_id1;
                            let material_amnt1 = entry.material_amnt1;
                            let material_id2 = entry.material_id2;
                            let material_amnt2 = entry.material_amnt2;
                            let material_id3 = entry.material_id3;
                            let material_amnt3 = entry.material_amnt3;
                            let material_id4 = entry.material_id4;
                            let material_amnt4 = entry.material_amnt4;
                            let hr_req = entry.hr_req;
                            let armor_name = self.get_armor_name(equip_type, equip_id);

                            let selected = self.selected_transmog_index == Some(*original_idx);
                            if ui.selectable_label(selected, format!("{}", original_idx)).clicked() {
                                selected_idx = Some(*original_idx);
                                should_switch_view = true;
                            }
                            ui.label(match equip_type {
                                0x02 => "Head",
                                0x03 => "Body",
                                0x04 => "Arms",
                                0x05 => "Waist",
                                0x00 => "Legs",
                                _ => "Unknown",
                            });
                            ui.label(armor_name);
                            ui.label(format!("{}", material_id1));
                            ui.label(format!("{}", material_amnt1));
                            ui.label(format!("{}", material_id2));
                            ui.label(format!("{}", material_amnt2));
                            ui.label(format!("{}", material_id3));
                            ui.label(format!("{}", material_amnt3));
                            ui.label(format!("{}", material_id4));
                            ui.label(format!("{}", material_amnt4));
                            ui.label(format!("{}", hr_req));
                            ui.end_row();
                        }
                    });
            });

        if should_switch_view {
            if let Some(idx) = selected_idx {
                self.selected_transmog_index = Some(idx);
                *self.view_mode.get_mut("shop").unwrap() = ViewMode::Details;
            }
        }

        // Pagination controls
        ui.horizontal(|ui| {
            if ui.button("← Previous").clicked() && current_page > 0 {
                self.shop_page = (current_page - 1) as u32;
            }
            ui.label(format!("Page {} of {}", current_page + 1, total_pages));
            if ui.button("Next →").clicked() && current_page < total_pages - 1 {
                self.shop_page = (current_page + 1) as u32;
            }
        });
    }

    fn show_transmog_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            *self.view_mode.get_mut("shop").unwrap() = ViewMode::List;
            return;
        }

        if let Some(index) = self.selected_transmog_index {
            let entry = &self.transmog_entries[index];
            let equip_type = entry.equip_type;
            let equip_id = entry.equip_id;
            let mut armor_name = self.get_armor_name(equip_type, equip_id);
            let mut new_type = equip_type;
            let mut new_id = equip_id;
            
            // Get item names before getting mutable reference
            let item_name1 = self.get_item_name(entry.material_id1);
            let item_name2 = self.get_item_name(entry.material_id2);
            let item_name3 = self.get_item_name(entry.material_id3);
            let item_name4 = self.get_item_name(entry.material_id4);
            
            if let Some(entry) = self.transmog_entries.get_mut(index) {
                
                ui.heading(format!("Edit Transmog #{}", index));
                ui.separator();

                // Type selection
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    egui::ComboBox::from_id_source("edit_armor_type")
                        .selected_text(match entry.equip_type {
                            0x02 => "Head",
                            0x03 => "Body",
                            0x04 => "Arms",
                            0x05 => "Waist",
                            0x01 => "Legs",
                            _ => "Unknown",
                        })
                        .show_ui(ui, |ui| {
                            if ui.selectable_value(&mut entry.equip_type, 0x02, "Head").clicked() {}
                            if ui.selectable_value(&mut entry.equip_type, 0x03, "Body").clicked() {}
                            if ui.selectable_value(&mut entry.equip_type, 0x04, "Arms").clicked() {}
                            if ui.selectable_value(&mut entry.equip_type, 0x05, "Waist").clicked() {}
                            if ui.selectable_value(&mut entry.equip_type, 0x00, "Legs").clicked() {}
                        });
                    new_type = entry.equip_type;
                });

                // Equipment ID
                ui.horizontal(|ui| {
                    ui.label("Equipment ID:");
                    let mut equip_id = entry.equip_id as i32;
                    if ui.add(egui::DragValue::new(&mut equip_id).speed(1)).changed() {
                        entry.equip_id = equip_id as u16;
                        new_id = entry.equip_id;
                    }
                    ui.label(format!("({})", armor_name));
                });

                // Materials
                ui.separator();
                ui.heading("Materials");
                
                // Material 1
                ui.horizontal(|ui| {
                    ui.label("Material 1:");
                    let mut id = entry.material_id1 as i32;
                    let mut amount = entry.material_amnt1 as i32;
                    if ui.add(egui::DragValue::new(&mut id).speed(1)).changed() {
                        entry.material_id1 = id as u16;
                    }
                    ui.label(format!("({})", item_name1));
                    ui.label("Amount:");
                    if ui.add(egui::DragValue::new(&mut amount).speed(1)).changed() {
                        entry.material_amnt1 = amount as u16;
                    }
                });

                // Material 2
                ui.horizontal(|ui| {
                    ui.label("Material 2:");
                    let mut id = entry.material_id2 as i32;
                    let mut amount = entry.material_amnt2 as i32;
                    if ui.add(egui::DragValue::new(&mut id).speed(1)).changed() {
                        entry.material_id2 = id as u16;
                    }
                    ui.label(format!("({})", item_name2));
                    ui.label("Amount:");
                    if ui.add(egui::DragValue::new(&mut amount).speed(1)).changed() {
                        entry.material_amnt2 = amount as u16;
                    }
                });

                // Material 3
                ui.horizontal(|ui| {
                    ui.label("Material 3:");
                    let mut id = entry.material_id3 as i32;
                    let mut amount = entry.material_amnt3 as i32;
                    if ui.add(egui::DragValue::new(&mut id).speed(1)).changed() {
                        entry.material_id3 = id as u16;
                    }
                    ui.label(format!("({})", item_name3));
                    ui.label("Amount:");
                    if ui.add(egui::DragValue::new(&mut amount).speed(1)).changed() {
                        entry.material_amnt3 = amount as u16;
                    }
                });

                // Material 4
                ui.horizontal(|ui| {
                    ui.label("Material 4:");
                    let mut id = entry.material_id4 as i32;
                    let mut amount = entry.material_amnt4 as i32;
                    if ui.add(egui::DragValue::new(&mut id).speed(1)).changed() {
                        entry.material_id4 = id as u16;
                    }
                    ui.label(format!("({})", item_name4));
                    ui.label("Amount:");
                    if ui.add(egui::DragValue::new(&mut amount).speed(1)).changed() {
                        entry.material_amnt4 = amount as u16;
                    }
                });

                // HR Requirement
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("HR Requirement:");
                    let mut hr_req = entry.hr_req as i32;
                    if ui.add(egui::DragValue::new(&mut hr_req).speed(1)).changed() {
                        entry.hr_req = hr_req as u16;
                    }
                });
            }
            
            if new_type != equip_type || new_id != equip_id {
                armor_name = self.get_armor_name(new_type, new_id);
            }
        }
    }
} 