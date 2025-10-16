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
        // Sub-tabs: Transmog | Deco
        ui.horizontal_wrapped(|ui| {
            if ui.selectable_label(matches!(self.workshop_tab, super::WorkshopTab::Transmog), "Transmog").clicked() {
                self.workshop_tab = super::WorkshopTab::Transmog;
                self.view_mode.insert("shop".to_string(), ViewMode::List);
            }
            if ui.selectable_label(matches!(self.workshop_tab, super::WorkshopTab::Deco), "Deco").clicked() {
                self.workshop_tab = super::WorkshopTab::Deco;
                self.view_mode.insert("shop".to_string(), ViewMode::List);
            }
        });
        ui.separator();

        match self.workshop_tab {
            super::WorkshopTab::Transmog => {
                if !self.view_mode.contains_key("shop") {
                    self.view_mode.insert("shop".to_string(), ViewMode::List);
                }
                match self.view_mode.get("shop").unwrap() {
                    ViewMode::List => self.show_transmog_list(ui),
                    ViewMode::Details => self.show_transmog_details(ui),
                }
            }
            super::WorkshopTab::Deco => {
                self.show_deco_shop(ui);
            }
            _ => {}
        }
    }

    fn show_deco_shop(&mut self, ui: &mut egui::Ui) {
        // Load from pointers once into separate HR/GR/Cuff/Cuff GR buffers
        if self.deco_shop_hr_entries.is_empty()
            && self.deco_shop_gr_entries.is_empty()
            && self.cuff_shop_entries.is_empty()
            && self.cuff_gr_shop_entries.is_empty()
        {
            let read_ptr = |buf: &Vec<u8>, at: u32| -> Option<u32> {
                let off = at as usize;
                if buf.len() >= off + 4 { Some(u32::from_le_bytes([buf[off], buf[off+1], buf[off+2], buf[off+3]])) } else { None }
            };
            if let Some(off) = read_ptr(&self.buffer, crate::model::mhfdat_pointers::DECO_SHOP_PTR) {
                self.deco_shop_hr_entries = crate::core::mhfdat::read_deco_shop(&self.buffer, off as usize);
            }
            if let Some(off_g) = read_ptr(&self.buffer, crate::model::mhfdat_pointers::DECO_G_SHOP_PTR) {
                self.deco_shop_gr_entries = crate::core::mhfdat::read_deco_shop(&self.buffer, off_g as usize);
            }
            if let Some(off_c) = read_ptr(&self.buffer, crate::model::mhfdat_pointers::CUFF_SHOP_PTR) {
                self.cuff_shop_entries = crate::core::mhfdat::read_deco_shop(&self.buffer, off_c as usize);
            }
            if let Some(off_cg) = read_ptr(&self.buffer, crate::model::mhfdat_pointers::CUFF_GR_SHOP_PTR) {
                self.cuff_gr_shop_entries = crate::core::mhfdat::read_deco_shop(&self.buffer, off_cg as usize);
            }
        }

        MhfdatApp::section_header(ui, "Deco Shop", |ui| {
            if ui.button("Export current list to JSON").clicked() {
                let (name, data): (&str, &Vec<crate::model::mhfdat::DecoShop>) = match self.shop_page {
                    0 => ("deco_shop_hr.json", &self.deco_shop_hr_entries),
                    1 => ("deco_shop_gr.json", &self.deco_shop_gr_entries),
                    2 => ("cuff_shop.json", &self.cuff_shop_entries),
                    3 => ("cuff_gr_shop.json", &self.cuff_gr_shop_entries),
                    _ => ("deco_shop_hr.json", &self.deco_shop_hr_entries),
                };
                if let Ok(text) = serde_json::to_string_pretty(data) { let _ = std::fs::write(name, text); }
            }
        });

        // Ensure we have a view state for deco shop
        if !self.view_mode.contains_key("deco_shop") {
            self.view_mode.insert("deco_shop".to_string(), ViewMode::List);
        }

        // Sub-categories: Deco HR, Deco GR, Cuff, Cuff GR
        ui.horizontal_wrapped(|ui| {
            if ui.selectable_label(self.shop_page == 0, "Decoration HR").clicked() { self.shop_page = 0; }
            if ui.selectable_label(self.shop_page == 1, "Decoration GR").clicked() { self.shop_page = 1; }
            if ui.selectable_label(self.shop_page == 2, "Cuff").clicked() { self.shop_page = 2; }
            if ui.selectable_label(self.shop_page == 3, "Cuff GR").clicked() { self.shop_page = 3; }
        });
        ui.separator();

        // Route by view mode
        match self.view_mode.get("deco_shop").cloned().unwrap_or(ViewMode::List) {
            ViewMode::List => {
                // Search
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    ui.text_edit_singleline(&mut self.deco_shop_search);
                    if ui.button("Export current list to JSON").clicked() {
                        let (name, data): (&str, &Vec<crate::model::mhfdat::DecoShop>) = match self.shop_page {
                            0 => ("deco_shop_hr.json", &self.deco_shop_hr_entries),
                            1 => ("deco_shop_gr.json", &self.deco_shop_gr_entries),
                            2 => ("cuff_shop.json", &self.cuff_shop_entries),
                            3 => ("cuff_gr_shop.json", &self.cuff_gr_shop_entries),
                            _ => ("deco_shop_hr.json", &self.deco_shop_hr_entries),
                        };
                        if let Ok(text) = serde_json::to_string_pretty(data) {
                            let _ = std::fs::write(name, text);
                        }
                    }
                });

                // Add new entry to current subcategory
                ui.horizontal(|ui| {
                    if ui.button("Add New").clicked() {
                        let new_entry = crate::model::mhfdat::DecoShop::default();
                        let new_index = match self.shop_page {
                            0 => { self.deco_shop_hr_entries.push(new_entry); self.deco_shop_hr_entries.len() - 1 }
                            1 => { self.deco_shop_gr_entries.push(new_entry); self.deco_shop_gr_entries.len() - 1 }
                            2 => { self.cuff_shop_entries.push(new_entry); self.cuff_shop_entries.len() - 1 }
                            3 => { self.cuff_gr_shop_entries.push(new_entry); self.cuff_gr_shop_entries.len() - 1 }
                            _ => 0,
                        };
                        self.selected_deco_shop_index = Some(new_index);
                        self.view_mode.insert("deco_shop".to_string(), ViewMode::Details);
                        // Move to the last page to show the new item
                        let total_items = match self.shop_page {
                            0 => self.deco_shop_hr_entries.len(),
                            1 => self.deco_shop_gr_entries.len(),
                            2 => self.cuff_shop_entries.len(),
                            3 => self.cuff_gr_shop_entries.len(),
                            _ => 0,
                        };
                        let per_page = 20usize;
                        let total_pages = (total_items + per_page - 1) / per_page;
                        if total_pages > 0 { self.deco_shop_page = (total_pages - 1) as u32; }
                    }
                });

        // Choose source by sub-category (clone to avoid borrowing self during UI/mutations)
        let current_list_owned: Vec<crate::model::mhfdat::DecoShop> = match self.shop_page {
            0 => self.deco_shop_hr_entries.clone(),
            1 => self.deco_shop_gr_entries.clone(),
            2 => self.cuff_shop_entries.clone(),
            3 => self.cuff_gr_shop_entries.clone(),
            _ => self.deco_shop_hr_entries.clone(),
        };
        let lowered = self.deco_shop_search.to_lowercase();
        let entries_all: Vec<(usize, &crate::model::mhfdat::DecoShop)> = current_list_owned.iter().enumerate().collect();
        let entries: Vec<(usize, &crate::model::mhfdat::DecoShop)> = if lowered.is_empty() {
            entries_all
        } else {
            entries_all
                .into_iter()
                .filter(|(_, e)| {
                    // Match deco item name or any material name
                    let deco_name = self.get_item_name(e.deco_item_id).to_lowercase();
                    let m1 = self.get_item_name(e.item_id1).to_lowercase();
                    let m2 = self.get_item_name(e.item_id2).to_lowercase();
                    let m3 = self.get_item_name(e.item_id3).to_lowercase();
                    let m4 = self.get_item_name(e.item_id4).to_lowercase();
                    deco_name.contains(&lowered) || m1.contains(&lowered) || m2.contains(&lowered) || m3.contains(&lowered) || m4.contains(&lowered)
                })
                .collect()
        };

        // Pagination (20 rows/page)
        let per_page = 20usize;
        let total = entries.len();
        let total_pages = (total + per_page - 1) / per_page;
        let current = (self.deco_shop_page as usize).min(total_pages.saturating_sub(1));
        if current != self.deco_shop_page as usize { self.deco_shop_page = current as u32; }
        MhfdatApp::pagination_controls(ui, &mut self.deco_shop_page, total_pages);
        let start = current * per_page;
        let end = (start + per_page).min(total);

        MhfdatApp::list_scroll(ui, "deco_shop_grid_scroll", |ui| {
            egui::Grid::new("deco_shop_grid").striped(true).show(ui, |ui| {
                ui.label("Idx"); ui.label("Deco Name"); ui.label("Cat");
                ui.label("Mat1"); ui.label("Qty1");
                ui.label("Mat2"); ui.label("Qty2");
                ui.label("Mat3"); ui.label("Qty3");
                ui.label("Mat4"); ui.label("Qty4");
                ui.end_row();
                for (i, e) in entries[start..end].iter().cloned() {
                    // copy packed fields to locals to avoid unaligned references
                    let deco_item_id = e.deco_item_id;
                    let receipt_category = e.receipt_category;
                    let item_id1 = e.item_id1; let item_qty1 = e.item_qty1;
                    let item_id2 = e.item_id2; let item_qty2 = e.item_qty2;
                    let item_id3 = e.item_id3; let item_qty3 = e.item_qty3;
                    let item_id4 = e.item_id4; let item_qty4 = e.item_qty4;

                    let selected = self.selected_deco_shop_index == Some(i);
                    if ui.selectable_label(selected, format!("{}", i)).clicked() {
                        self.selected_deco_shop_index = Some(i);
                        self.view_mode.insert("deco_shop".to_string(), ViewMode::Details);
                    }
                    ui.label(self.get_item_name(deco_item_id));
                    ui.label(format!("{}", receipt_category));
                    // Show only IDs for materials, keep name for the Deco item
                    ui.label(format!("{}", item_id1)); ui.label(format!("{}", item_qty1));
                    ui.label(format!("{}", item_id2)); ui.label(format!("{}", item_qty2));
                    ui.label(format!("{}", item_id3)); ui.label(format!("{}", item_qty3));
                    ui.label(format!("{}", item_id4)); ui.label(format!("{}", item_qty4));
                    ui.end_row();
                }
            });
        });
            }
            ViewMode::Details => {
                // Back to list
                if ui.button("← Back to List").clicked() {
                    self.view_mode.insert("deco_shop".to_string(), ViewMode::List);
                    return;
                }
                // Details editor for selected entry
                if let Some(sel) = self.selected_deco_shop_index {
                    let mut entry_opt = match self.shop_page {
                        0 => self.deco_shop_hr_entries.get(sel).cloned(),
                        1 => self.deco_shop_gr_entries.get(sel).cloned(),
                        2 => self.cuff_shop_entries.get(sel).cloned(),
                        3 => self.cuff_gr_shop_entries.get(sel).cloned(),
                        _ => None,
                    };
                    if let Some(mut entry) = entry_opt.take() {
                        ui.separator();
                        ui.heading(format!("Edit {} entry #{}", match self.shop_page {0=>"Deco HR",1=>"Deco GR",2=>"Cuff",3=>"Cuff GR", _=>""}, sel));
                        ui.horizontal(|ui| {
                            ui.label("Deco Item ID:");
                            let mut v = entry.deco_item_id as i32;
                            if ui.add(egui::DragValue::new(&mut v).speed(1)).changed() {
                                entry.deco_item_id = v as u16;
                            }
                            ui.label(self.get_item_name(entry.deco_item_id));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Category:");
                            let mut v = entry.receipt_category as i32;
                            if ui.add(egui::DragValue::new(&mut v).speed(1)).changed() { entry.receipt_category = v as u16; }
                        });
                        for idx in 1..=4 {
                            ui.horizontal(|ui| {
                                ui.label(format!("Mat{}:", idx));
                                let mut id_i: i32 = match idx { 1 => entry.item_id1 as i32, 2 => entry.item_id2 as i32, 3 => entry.item_id3 as i32, _ => entry.item_id4 as i32 };
                                let mut qty_i: i32 = match idx { 1 => entry.item_qty1 as i32, 2 => entry.item_qty2 as i32, 3 => entry.item_qty3 as i32, _ => entry.item_qty4 as i32 };
                                if ui.add(egui::DragValue::new(&mut id_i).speed(1)).changed() {}
                                let name_prev = self.get_item_name(id_i as u16);
                                ui.label(name_prev);
                                ui.label("Qty:");
                                if ui.add(egui::DragValue::new(&mut qty_i).speed(1)).changed() {}
                                match idx {
                                    1 => { entry.item_id1 = id_i as u16; entry.item_qty1 = qty_i as u8; }
                                    2 => { entry.item_id2 = id_i as u16; entry.item_qty2 = qty_i as u8; }
                                    3 => { entry.item_id3 = id_i as u16; entry.item_qty3 = qty_i as u8; }
                                    _ => { entry.item_id4 = id_i as u16; entry.item_qty4 = qty_i as u8; }
                                }
                            });
                        }
                        // Write back to the correct list
                        match self.shop_page {
                            0 => if let Some(slot) = self.deco_shop_hr_entries.get_mut(sel) { *slot = entry; },
                            1 => if let Some(slot) = self.deco_shop_gr_entries.get_mut(sel) { *slot = entry; },
                            2 => if let Some(slot) = self.cuff_shop_entries.get_mut(sel) { *slot = entry; },
                            3 => if let Some(slot) = self.cuff_gr_shop_entries.get_mut(sel) { *slot = entry; },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn show_transmog_list(&mut self, ui: &mut egui::Ui) {
        MhfdatApp::section_header(ui, "Transmog Shop", |ui| {
            if ui.button("Add New").clicked() {
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

        // Search and filters
        ui.horizontal_wrapped(|ui| {
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
        MhfdatApp::list_scroll(ui, "transmog_shop_list_scroll", |ui| {
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

                        for (_i, (original_idx, entry)) in filtered_entries[start_idx..end_idx].iter().enumerate() {
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
        MhfdatApp::pagination_controls(ui, &mut self.shop_page, total_pages);
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
            let armor_name = self.get_armor_name(equip_type, equip_id);
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
            
            // no-op
        }
    }
} 