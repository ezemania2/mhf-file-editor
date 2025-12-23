use super::*;
use egui;
use std::fs::File;
use std::io::Write;
use serde_json;
use serde::Serialize;
use crate::utils::item_patterns::{ITEM_ICON_LIST, icon_name};
use crate::utils::skills::skill_name;
use crate::utils::weapon_patterns::zenith_skill_name;

impl MhfdatApp {
    pub fn show_items_tab(&mut self, ui: &mut egui::Ui) {
        // Sub-tabs: Items | Decorations (wrapped)
        ui.horizontal_wrapped(|ui| {
            if ui.selectable_label(self.item_subtab_is_deco() == false, "Items").clicked() {
                self.set_item_subtab(false);
            }
            if ui.selectable_label(self.item_subtab_is_deco() == true, "Decorations").clicked() {
                self.set_item_subtab(true);
            }
        });
        ui.separator();

        if self.item_subtab_is_deco() {
            // Decorations: show list or details depending on selection
            if self.selected_deco_index.is_some() {
                self.show_deco_details_view(ui);
            } else {
                self.show_deco_id_list(ui);
            }
            return;
        }

        // Initialize view mode if not present
        if !self.view_mode.contains_key("items") {
            self.view_mode.insert("items".to_string(), ViewMode::List);
        }

        match self.view_mode.get("items").unwrap() {
            ViewMode::List => self.show_items_list(ui),
            ViewMode::Details => self.show_item_details_view(ui),
        }
    }

    fn item_subtab_is_deco(&self) -> bool {
        self.view_mode.get("items_subtab").map(|v| matches!(v, ViewMode::Details)).unwrap_or(false)
    }

    fn set_item_subtab(&mut self, deco: bool) {
        if deco {
            self.view_mode.insert("items_subtab".to_string(), ViewMode::Details);
        } else {
            self.view_mode.insert("items_subtab".to_string(), ViewMode::List);
        }
    }

    pub fn show_deco_id_list(&mut self, ui: &mut egui::Ui) {
        MhfdatApp::section_header(ui, "Decorations (DecoID)", |ui| {
            if ui.button("Add New Deco").clicked() {
                // Add a new deco entry
                let new_deco = crate::model::mhfdat::MhfdatDecoId::default();
                self.deco_ids.push(new_deco);
                // Update the limiter with the count (len() gives the actual count)
                self.deco_id_count_limiter = self.deco_ids.len() as u16;
                self.deco_id_count_limiter_modified = true;
                self.deco_ids_modified = true;
                // Select the new entry
                self.selected_deco_index = Some(self.deco_ids.len() - 1);
            }
            if ui.button("Export JSON").clicked() {
                #[derive(Serialize)]
                struct DecoExport {
                    idx: usize,
                    name: String,
                    slot_nb: u8,
                    flags: u16,
                    price: u32,
                    skill_id1: u8,
                    skill_pts1: i8,
                    skill_id2: u8,
                    skill_pts2: i8,
                    skill_id3: u8,
                    skill_pts3: i8,
                    skill_id4: u8,
                    skill_pts4: i8,
                    special_flags: u16,
                    zenith_skill: u16,
                }
                let export_rows: Vec<DecoExport> = self.deco_ids.iter().enumerate().map(|(i, d)| {
                    let name = self.items.iter().position(|it| it.deco_id as usize == i)
                        .and_then(|idx| self.item_names.get(idx)).cloned().unwrap_or_default();
                    DecoExport {
                        idx: i,
                        name,
                        slot_nb: d.slot_nb,
                        flags: d.flags,
                        price: d.price,
                        skill_id1: d.skill_id1,
                        skill_pts1: d.skill_pts1,
                        skill_id2: d.skill_id2,
                        skill_pts2: d.skill_pts2,
                        skill_id3: d.skill_id3,
                        skill_pts3: d.skill_pts3,
                        skill_id4: d.skill_id4,
                        skill_pts4: d.skill_pts4,
                        special_flags: d.special_flags,
                        zenith_skill: d.zenith_skill,
                    }
                }).collect();
                if let Ok(json) = serde_json::to_string_pretty(&export_rows) {
                    let _ = std::fs::write("decorations.json", json);
                }
            }
            if ui.button("Import from JSON").clicked() {
                if let Ok(data) = std::fs::read_to_string("deco_ids.json") {
                    if let Ok(imported) = serde_json::from_str::<Vec<crate::model::mhfdat::MhfdatDecoId>>(&data) {
                        self.deco_ids = imported;
                        self.deco_id_count_limiter = self.deco_ids.len() as u16;
                        self.deco_id_count_limiter_modified = true;
                        self.deco_ids_modified = true;
                    }
                }
            }
        });
        // Filters
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search name:");
            ui.text_edit_singleline(&mut self.deco_search);
            ui.separator();
            ui.label("Skill filter:");
            ui.text_edit_singleline(&mut self.deco_skill_search);
            let mut selected_skill = self.deco_skill_filter.unwrap_or(0xFF);
            egui::ComboBox::from_id_source("deco_skill_filter")
                .selected_text(if selected_skill == 0xFF { "Any".to_string() } else { crate::utils::skills::skill_name(selected_skill).to_string() })
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut selected_skill, 0xFF, "Any").clicked() {}
                    let q = self.deco_skill_search.to_lowercase();
                    for (id, name) in crate::utils::skills::SKILL_LIST {
                        if q.is_empty() || name.to_lowercase().contains(&q) {
                            if ui.selectable_value(&mut selected_skill, *id, *name).clicked() {}
                        }
                    }
                });
            self.deco_skill_filter = if selected_skill == 0xFF { None } else { Some(selected_skill) };
            ui.separator();
            ui.label("Zenith skill:");
            ui.text_edit_singleline(&mut self.deco_zenith_search);
            let mut selected_zenith = self.deco_zenith_filter.unwrap_or(0xFFFF);
            egui::ComboBox::from_id_source("deco_zenith_filter")
                .selected_text(if selected_zenith == 0xFFFF { "Any".to_string() } else { crate::utils::weapon_patterns::zenith_skill_name(selected_zenith).to_string() })
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut selected_zenith, 0xFFFF, "Any").clicked() {}
                    let qz = self.deco_zenith_search.to_lowercase();
                    for (id, name) in crate::utils::weapon_patterns::ZENITH_SKILL_LIST {
                        if qz.is_empty() || name.to_lowercase().contains(&qz) {
                            if ui.selectable_value(&mut selected_zenith, *id, *name).clicked() {}
                        }
                    }
                });
            self.deco_zenith_filter = if selected_zenith == 0xFFFF { None } else { Some(selected_zenith) };
        });

        // Build filtered indices
        let mut indices: Vec<usize> = (0..self.deco_ids.len()).collect();
        if !self.deco_search.is_empty() {
            let q = self.deco_search.to_lowercase();
            indices.retain(|&i| {
                if let Some(item_idx) = self.items.iter().position(|it| it.deco_id as usize == i) {
                    let name = self.item_names.get(item_idx).cloned().unwrap_or_default();
                    name.to_lowercase().contains(&q)
                } else { false }
            });
        }
        if let Some(skill) = self.deco_skill_filter {
            indices.retain(|&i| {
                let d = &self.deco_ids[i];
                d.skill_id1 == skill || d.skill_id2 == skill || d.skill_id3 == skill || d.skill_id4 == skill
            });
        }
        if let Some(zen) = self.deco_zenith_filter {
            indices.retain(|&i| self.deco_ids[i].zenith_skill == zen);
        }

        // Paging controls (20 rows per page)
        let items_per_page = 20usize;
        let total = indices.len();
        let total_pages = (total + items_per_page - 1) / items_per_page;
        let current_page = (self.deco_page as usize).min(total_pages.saturating_sub(1));
        if current_page != self.deco_page as usize { self.deco_page = current_page as u32; }

        MhfdatApp::pagination_controls(ui, &mut self.deco_page, total_pages);

        let start_idx = current_page * items_per_page;
        let end_idx = (start_idx + items_per_page).min(total);

        MhfdatApp::list_scroll(ui, "deco_id_grid_scroll", |ui| {
            egui::Grid::new("deco_id_grid")
                .num_columns(12)
                .striped(true)
                .show(ui, |ui| {
                ui.label("Idx");
                ui.label("Deco Name");
                ui.label("Slots");
                ui.label("Flags");
                ui.label("Price");
                ui.label("Skill1"); ui.label("Pts1");
                ui.label("Skill2"); ui.label("Pts2");
                ui.label("Skill3"); ui.label("Pts3");
                ui.label("Skill4"); ui.label("Pts4");
                ui.label("Zenith");
                ui.end_row();
                if self.deco_ids.is_empty() {
                    ui.colored_label(egui::Color32::YELLOW, "No decorations loaded.");
                    ui.end_row();
                }
                for slice_i in start_idx..end_idx {
                    let i = indices[slice_i];
                    let d = &self.deco_ids[i];
                    // Copy packed fields to locals to avoid unaligned references
                    let slot_nb = d.slot_nb;
                    let flags = d.flags;
                    let price = d.price;
                    let skill_id1 = d.skill_id1; let skill_pts1 = d.skill_pts1;
                    let skill_id2 = d.skill_id2; let skill_pts2 = d.skill_pts2;
                    let skill_id3 = d.skill_id3; let skill_pts3 = d.skill_pts3;
                    let skill_id4 = d.skill_id4; let skill_pts4 = d.skill_pts4;
                    let zen = d.zenith_skill;

                    let selected = self.selected_deco_index == Some(i);
                    if ui.selectable_label(selected, format!("{}", i)).clicked() {
                        self.selected_deco_index = Some(i);
                    }
                    // Link DecoID to item name if possible (items[i].deco_id == i)
                    if let Some(item_idx) = self.items.iter().position(|it| it.deco_id as usize == i) {
                        let name = self.item_names.get(item_idx).cloned().unwrap_or_default();
                        if name.is_empty() { ui.label(format!("{}", i)); } else { ui.label(name); }
                    } else {
                        ui.label(format!("{}", i));
                    }
                    ui.label(format!("{}", slot_nb));
                    ui.label(format!("0x{:04X}", flags));
                    ui.label(format!("{}", price));
                    ui.label(skill_name(skill_id1)); ui.label(format!("{}", skill_pts1));
                    ui.label(skill_name(skill_id2)); ui.label(format!("{}", skill_pts2));
                    ui.label(skill_name(skill_id3)); ui.label(format!("{}", skill_pts3));
                    ui.label(skill_name(skill_id4)); ui.label(format!("{}", skill_pts4));
                    ui.label(zenith_skill_name(zen));
                    ui.end_row();
                }
            });
        });
    }

    pub fn show_items_list(&mut self, ui: &mut egui::Ui) {
                 MhfdatApp::section_header(ui, &format!("Items (found: {})", self.items.len()), |ui| {
                     if ui.button("Export JSON").clicked() {
                         // Convert items to export format
                         let export_items: Vec<ItemExport> = self.items
                             .iter()
                             .enumerate()
                             .map(|(index, item)| {
                                 let name = if index < self.item_names.len() { self.item_names[index].clone() } else { String::new() };
                                 let description = if index < self.item_descriptions.len() { self.item_descriptions[index].clone() } else { String::new() };
                                 ItemExport::from_item_with_data(item, &name, &description, index)
                             })
                             .collect();
                         if let Ok(json) = serde_json::to_string_pretty(&export_items) { let _ = std::fs::write("items.json", json); }
                     }
                     if ui.button("Import from JSON").clicked() {
                         // Try export format first (with names and descriptions)
                         if let Ok(data) = std::fs::read_to_string("items.json") {
                             if let Ok(imported_export) = serde_json::from_str::<Vec<crate::model::mhfdat::ItemExport>>(&data) {
                                 let imported: Vec<crate::model::mhfdat::MhfdatItem> = imported_export.iter().map(|e| e.to_item()).collect();
                                 self.items = imported;
                                 self.items_modified = true;
                                 return;
                             }
                         }
                         // Fallback to raw format
                         if let Ok(data) = std::fs::read_to_string("items_raw.json") {
                             if let Ok(imported) = serde_json::from_str::<Vec<crate::model::mhfdat::MhfdatItem>>(&data) {
                                 self.items = imported;
                                 self.items_modified = true;
                             }
                         }
                     }
                 });

        // Search and filters
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
        });

        // Export buttons
        ui.horizontal(|ui| {
            if ui.button("Export Items to JSON").clicked() {
                // Convert items to export format
                let export_items: Vec<ItemExport> = self.items
                    .iter()
                    .enumerate()
                    .map(|(index, item)| {
                        let name = if index < self.item_names.len() {
                            self.item_names[index].clone()
                        } else {
                            String::new()
                        };
                        let description = if index < self.item_descriptions.len() {
                            self.item_descriptions[index].clone()
                        } else {
                            String::new()
                        };
                        ItemExport::from_item_with_data(item, &name, &description, index)
                    })
                    .collect();
                
                if let Ok(json) = serde_json::to_string_pretty(&export_items) {
                    if let Ok(mut file) = File::create("items.json") {
                        let _ = file.write_all(json.as_bytes());
                    }
                }
            }
            
        });

        if self.items.is_empty() {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No items found or not yet loaded!");
        } else {
            // Apply filters
            let query = self.search_query.to_lowercase();
            let filtered_items: Vec<(usize, &MhfdatItem)> = self.items.iter()
                .enumerate()
                .filter(|(index, _item)| {
                    if query.is_empty() {
                        true
                    } else {
                        let item_name = if *index < self.item_names.len() {
                            self.item_names[*index].clone()
                        } else {
                            String::new()
                        };
                        item_name.to_lowercase().contains(&query)
                    }
                })
                .collect();

            let filtered_count = filtered_items.len();
            let items_per_page = 20;
            let total_pages = (filtered_count + items_per_page - 1) / items_per_page;
            let current_page = (self.item_page as usize).min(total_pages.saturating_sub(1));
            
            // Reset page if out of bounds
            if current_page != self.item_page as usize {
                self.item_page = current_page as u32;
            }
            
            let start_idx = current_page * items_per_page;
            let end_idx = (start_idx + items_per_page).min(filtered_count);

                         // Show items table
            MhfdatApp::list_scroll(ui, "items_grid_scroll", |ui| {
                                  egui::Grid::new("items_grid")
                      .num_columns(7)
                      .striped(true)
                      .show(ui, |ui| {
                          // Header
                          ui.label("ID");
                          ui.label("Icon");
                          ui.label("Name");
                          ui.label("Rarity");
                          ui.label("Max Stack");
                          ui.label("Buy Price");
                          ui.label("Sell Price");
                          ui.end_row();

                                                 // Items
                         for (original_idx, item) in filtered_items[start_idx..end_idx].iter() {
                             // Ensure we have valid data for this index
                             let item_name = if *original_idx < self.item_names.len() {
                                 self.item_names[*original_idx].clone()
                             } else {
                                 String::new()
                             };
                             let selected = self.selected_item_index == Some(*original_idx);
                             
                             // Copy fields to local variables to avoid unaligned references
                             let icon = item.icon;
                             let rarity = item.rarity;
                             let max_stack = item.max_stack;
                             let buy_price = item.buy_price;
                             let sell_price = item.sell_price;
                             
                                                           if ui.selectable_label(selected, format!("{}", original_idx)).clicked() {
                                  self.selected_item_index = Some(*original_idx);
                                  if let Some(view_mode) = self.view_mode.get_mut("items") {
                                      *view_mode = ViewMode::Details;
                                  }
                              }
                              ui.label(icon_name(icon));
                             ui.label(&item_name);
                             
                             ui.label(format!("{}", rarity + 1)); // Add 1 to display rarity
                             ui.label(format!("{}", max_stack));
                             ui.label(format!("{}", buy_price));
                             ui.label(format!("{}", sell_price));
                             ui.end_row();
                         }
                    });
            });

            // Pagination controls
            MhfdatApp::pagination_controls(ui, &mut self.item_page, total_pages);
        }
    }

    pub fn show_item_details_view(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("← Back to List").clicked() {
                if let Some(view_mode) = self.view_mode.get_mut("items") {
                    *view_mode = ViewMode::List;
                }
                // also clear deco selection when returning
                self.selected_deco_index = None;
            }
        });

        if let Some(index) = self.selected_item_index {
            if let Some(item) = self.items.get(index) {
                ui.heading("Item Details");
                
                // Ensure we have valid data for this index
                if index >= self.item_names.len() {
                    self.item_names.resize(index + 1, String::new());
                }
                if index >= self.item_descriptions.len() {
                    self.item_descriptions.resize(index + 1, String::new());
                }
                
                                 ui.horizontal(|ui| {
                     ui.label("Name:");
                     if ui.text_edit_singleline(&mut self.item_names[index]).changed() {
                         self.item_names_modified = true;
                     }
                 });
                 ui.horizontal(|ui| {
                     ui.label("Description:");
                     if ui.text_edit_multiline(&mut self.item_descriptions[index]).changed() {
                         self.item_descriptions_modified = true;
                     }
                 });
                 
                 ui.add_space(10.0);
                 

                 
                 ui.add_space(10.0);
                 ui.separator();
                 ui.add_space(10.0);
                
                 let mut unk00 = item.unk00;
                 let mut unk01 = item.unk01;
                 let mut rarity = item.rarity;
                 let mut max_stack = item.max_stack;
                 let mut unk04 = item.unk04;
                 let mut icon = item.icon;
                 let mut icon_color = item.icon_color;
                 let mut bottle = item.bottle;
                 let mut buy_price = item.buy_price;
                 let mut sell_price = item.sell_price;
                 let mut item_type = item.item_type;
                 let mut deco_id = item.deco_id;
                 let mut equip_type = item.equip_type;
                 let mut is_gz = item.is_gz;
                 
                 let mut items_changed = false;
                 egui::Grid::new("item_details_grid")
                     .num_columns(2)
                     .show(ui, |ui| {
                         ui.label("Interaction Type:");
                         if ui.add(egui::DragValue::new(&mut unk00).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Is Usable:");
                         if ui.add(egui::DragValue::new(&mut unk01).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Rarity:");
                         if ui.add(egui::DragValue::new(&mut rarity).speed(1.0).clamp_range(0..=9)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Max Stack:");
                         if ui.add(egui::DragValue::new(&mut max_stack).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Data Bitset:");
                         if ui.add(egui::DragValue::new(&mut unk04).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Icon:");
                         egui::ComboBox::from_id_source("icon_combo")
                             .selected_text(icon_name(icon))
                             .show_ui(ui, |ui| {
                                 for (id, name) in ITEM_ICON_LIST {
                                     if ui.selectable_value(&mut icon, *id, *name).clicked() {
                                         items_changed = true;
                                     }
                                 }
                             });
                         ui.end_row();
                         
                         ui.label("Icon Color:");
                         if ui.add(egui::DragValue::new(&mut icon_color).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Bottle:");
                         if ui.add(egui::DragValue::new(&mut bottle).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Buy Price:");
                         if ui.add(egui::DragValue::new(&mut buy_price).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Sell Price:");
                         if ui.add(egui::DragValue::new(&mut sell_price).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Item Type:");
                         if ui.add(egui::DragValue::new(&mut item_type).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Deco ID:");
                         if ui.add(egui::DragValue::new(&mut deco_id).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Equip Type:");
                         if ui.add(egui::DragValue::new(&mut equip_type).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                         
                         ui.label("Is GZ:");
                         if ui.add(egui::DragValue::new(&mut is_gz).speed(1.0)).changed() {
                             items_changed = true;
                         }
                         ui.end_row();
                     });
                 
                 // Update the original item with edited values
                 self.items[index].unk00 = unk00;
                 self.items[index].unk01 = unk01;
                 self.items[index].rarity = rarity;
                 self.items[index].max_stack = max_stack;
                 self.items[index].unk04 = unk04;
                 self.items[index].icon = icon;
                 self.items[index].icon_color = icon_color;
                 self.items[index].bottle = bottle;
                 self.items[index].buy_price = buy_price;
                 self.items[index].sell_price = sell_price;
                 self.items[index].item_type = item_type;
                 self.items[index].deco_id = deco_id;
                 self.items[index].equip_type = equip_type;
                 self.items[index].is_gz = is_gz;
                 
                 // Marquer les items comme modifiés si des changements ont été faits
                 if items_changed {
                     self.items_modified = true;
                 }
            }
        } else if let Some(deco_idx) = self.selected_deco_index {
            if deco_idx < self.deco_ids.len() {
                // Read packed struct safely
                let d: crate::model::mhfdat::MhfdatDecoId = unsafe { 
                    std::ptr::read_unaligned(&self.deco_ids[deco_idx] as *const _) 
                };
                ui.heading(format!("Decoration Details (Idx {})", deco_idx));
                // Local editable copies
                let mut slot_nb = d.slot_nb;
                let mut flags = d.flags;
                let mut price = d.price;
                let mut skill_id1 = d.skill_id1; let mut skill_pts1 = d.skill_pts1;
                let mut skill_id2 = d.skill_id2; let mut skill_pts2 = d.skill_pts2;
                let mut skill_id3 = d.skill_id3; let mut skill_pts3 = d.skill_pts3;
                let mut skill_id4 = d.skill_id4; let mut skill_pts4 = d.skill_pts4;
                let mut zen = d.zenith_skill;

                // Name preview (linked item)
                if let Some(item_idx) = self.items.iter().position(|it| it.deco_id as usize == deco_idx) {
                    let name = self.item_names.get(item_idx).cloned().unwrap_or_default();
                    if !name.is_empty() { ui.label(format!("Deco Name: {}", name)); }
                }

                egui::Grid::new("deco_details_grid").num_columns(3).show(ui, |ui| {
                    ui.label("Slots:"); ui.add(egui::DragValue::new(&mut slot_nb).clamp_range(0..=3)); ui.end_row();
                    ui.label("Flags:"); ui.add(egui::DragValue::new(&mut flags)); ui.end_row();
                    ui.label("Price:"); ui.add(egui::DragValue::new(&mut price)); ui.end_row();

                });

                // Skills section with search (like armor)
                ui.horizontal(|ui| {
                    ui.label("Skill1:");
                    ui.add(egui::TextEdit::singleline(&mut self.deco_detail_skill_search[0]).hint_text("Search...").desired_width(100.0));
                    let q1 = self.deco_detail_skill_search[0].to_lowercase();
                    egui::ComboBox::from_id_source("deco_s1").selected_text(skill_name(skill_id1)).show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for (id, name) in crate::utils::skills::SKILL_LIST { 
                                if q1.is_empty() || name.to_lowercase().contains(&q1) {
                                    ui.selectable_value(&mut skill_id1, *id, format!("{} - {}", id, name));
                                }
                            }
                        });
                    });
                    ui.label("Pts:"); ui.add(egui::DragValue::new(&mut skill_pts1));
                });

                ui.horizontal(|ui| {
                    ui.label("Skill2:");
                    ui.add(egui::TextEdit::singleline(&mut self.deco_detail_skill_search[1]).hint_text("Search...").desired_width(100.0));
                    let q2 = self.deco_detail_skill_search[1].to_lowercase();
                    egui::ComboBox::from_id_source("deco_s2").selected_text(skill_name(skill_id2)).show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for (id, name) in crate::utils::skills::SKILL_LIST { 
                                if q2.is_empty() || name.to_lowercase().contains(&q2) {
                                    ui.selectable_value(&mut skill_id2, *id, format!("{} - {}", id, name));
                                }
                            }
                        });
                    });
                    ui.label("Pts:"); ui.add(egui::DragValue::new(&mut skill_pts2));
                });

                ui.horizontal(|ui| {
                    ui.label("Skill3:");
                    ui.add(egui::TextEdit::singleline(&mut self.deco_detail_skill_search[2]).hint_text("Search...").desired_width(100.0));
                    let q3 = self.deco_detail_skill_search[2].to_lowercase();
                    egui::ComboBox::from_id_source("deco_s3").selected_text(skill_name(skill_id3)).show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for (id, name) in crate::utils::skills::SKILL_LIST { 
                                if q3.is_empty() || name.to_lowercase().contains(&q3) {
                                    ui.selectable_value(&mut skill_id3, *id, format!("{} - {}", id, name));
                                }
                            }
                        });
                    });
                    ui.label("Pts:"); ui.add(egui::DragValue::new(&mut skill_pts3));
                });

                ui.horizontal(|ui| {
                    ui.label("Skill4:");
                    ui.add(egui::TextEdit::singleline(&mut self.deco_detail_skill_search[3]).hint_text("Search...").desired_width(100.0));
                    let q4 = self.deco_detail_skill_search[3].to_lowercase();
                    egui::ComboBox::from_id_source("deco_s4").selected_text(skill_name(skill_id4)).show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for (id, name) in crate::utils::skills::SKILL_LIST { 
                                if q4.is_empty() || name.to_lowercase().contains(&q4) {
                                    ui.selectable_value(&mut skill_id4, *id, format!("{} - {}", id, name));
                                }
                            }
                        });
                    });
                    ui.label("Pts:"); ui.add(egui::DragValue::new(&mut skill_pts4));
                });

                ui.horizontal(|ui| {
                    ui.label("Zenith:");
                    ui.add(egui::TextEdit::singleline(&mut self.deco_detail_skill_search[4]).hint_text("Search...").desired_width(100.0));
                    let qz = self.deco_detail_skill_search[4].to_lowercase();
                    egui::ComboBox::from_id_source("deco_zen").selected_text(crate::utils::weapon_patterns::zenith_skill_name(zen)).show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for (id, name) in crate::utils::weapon_patterns::ZENITH_SKILL_LIST { 
                                if qz.is_empty() || name.to_lowercase().contains(&qz) {
                                    ui.selectable_value(&mut zen, *id, format!("{} - {}", id, name));
                                }
                            }
                        });
                    });
                });

                // Write back edited values using full struct replacement
                let new_deco = crate::model::mhfdat::MhfdatDecoId {
                    slot_nb,
                    flags,
                    price,
                    _pad0: d._pad0,
                    skill_id1, skill_pts1,
                    skill_id2, skill_pts2,
                    skill_id3, skill_pts3,
                    skill_id4, skill_pts4,
                    special_flags: d.special_flags,
                    zenith_skill: zen,
                };
                self.deco_ids[deco_idx] = new_deco;
                self.deco_ids_modified = true;
            }
        }
    }

    pub fn show_deco_details_view(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("← Back to List").clicked() {
                self.selected_deco_index = None;
            }
        });

        if let Some(deco_idx) = self.selected_deco_index {
            if deco_idx < self.deco_ids.len() {
                // Read packed struct safely
                let d: crate::model::mhfdat::MhfdatDecoId = unsafe { 
                    std::ptr::read_unaligned(&self.deco_ids[deco_idx] as *const _) 
                };
                // Local editable copies
                let mut slot_nb = d.slot_nb;
                let mut flags = d.flags;
                let mut price = d.price;
                let mut skill_id1 = d.skill_id1; let mut skill_pts1 = d.skill_pts1;
                let mut skill_id2 = d.skill_id2; let mut skill_pts2 = d.skill_pts2;
                let mut skill_id3 = d.skill_id3; let mut skill_pts3 = d.skill_pts3;
                let mut skill_id4 = d.skill_id4; let mut skill_pts4 = d.skill_pts4;
                let mut zen = d.zenith_skill;

                // Name preview (linked item)
                if let Some(item_idx) = self.items.iter().position(|it| it.deco_id as usize == deco_idx) {
                    let name = self.item_names.get(item_idx).cloned().unwrap_or_default();
                    if !name.is_empty() { ui.label(format!("Deco Name: {}", name)); }
                }

                egui::Grid::new("deco_details_grid2").num_columns(2).show(ui, |ui| {
                    ui.label("Slots:"); ui.add(egui::DragValue::new(&mut slot_nb).clamp_range(0..=3)); ui.end_row();
                    ui.label("Flags:"); ui.add(egui::DragValue::new(&mut flags)); ui.end_row();
                    ui.label("Price:"); ui.add(egui::DragValue::new(&mut price)); ui.end_row();
                });

                // Skills section with search (like armor)
                ui.horizontal(|ui| {
                    ui.label("Skill1:");
                    ui.add(egui::TextEdit::singleline(&mut self.deco_detail_skill_search[0]).hint_text("Search...").desired_width(100.0));
                    let q1 = self.deco_detail_skill_search[0].to_lowercase();
                    egui::ComboBox::from_id_source("deco_s1_2").selected_text(skill_name(skill_id1)).show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for (id, name) in crate::utils::skills::SKILL_LIST { 
                                if q1.is_empty() || name.to_lowercase().contains(&q1) {
                                    ui.selectable_value(&mut skill_id1, *id, format!("{} - {}", id, name));
                                }
                            }
                        });
                    });
                    ui.label("Pts:"); ui.add(egui::DragValue::new(&mut skill_pts1));
                });

                ui.horizontal(|ui| {
                    ui.label("Skill2:");
                    ui.add(egui::TextEdit::singleline(&mut self.deco_detail_skill_search[1]).hint_text("Search...").desired_width(100.0));
                    let q2 = self.deco_detail_skill_search[1].to_lowercase();
                    egui::ComboBox::from_id_source("deco_s2_2").selected_text(skill_name(skill_id2)).show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for (id, name) in crate::utils::skills::SKILL_LIST { 
                                if q2.is_empty() || name.to_lowercase().contains(&q2) {
                                    ui.selectable_value(&mut skill_id2, *id, format!("{} - {}", id, name));
                                }
                            }
                        });
                    });
                    ui.label("Pts:"); ui.add(egui::DragValue::new(&mut skill_pts2));
                });

                ui.horizontal(|ui| {
                    ui.label("Skill3:");
                    ui.add(egui::TextEdit::singleline(&mut self.deco_detail_skill_search[2]).hint_text("Search...").desired_width(100.0));
                    let q3 = self.deco_detail_skill_search[2].to_lowercase();
                    egui::ComboBox::from_id_source("deco_s3_2").selected_text(skill_name(skill_id3)).show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for (id, name) in crate::utils::skills::SKILL_LIST { 
                                if q3.is_empty() || name.to_lowercase().contains(&q3) {
                                    ui.selectable_value(&mut skill_id3, *id, format!("{} - {}", id, name));
                                }
                            }
                        });
                    });
                    ui.label("Pts:"); ui.add(egui::DragValue::new(&mut skill_pts3));
                });

                ui.horizontal(|ui| {
                    ui.label("Skill4:");
                    ui.add(egui::TextEdit::singleline(&mut self.deco_detail_skill_search[3]).hint_text("Search...").desired_width(100.0));
                    let q4 = self.deco_detail_skill_search[3].to_lowercase();
                    egui::ComboBox::from_id_source("deco_s4_2").selected_text(skill_name(skill_id4)).show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for (id, name) in crate::utils::skills::SKILL_LIST { 
                                if q4.is_empty() || name.to_lowercase().contains(&q4) {
                                    ui.selectable_value(&mut skill_id4, *id, format!("{} - {}", id, name));
                                }
                            }
                        });
                    });
                    ui.label("Pts:"); ui.add(egui::DragValue::new(&mut skill_pts4));
                });

                ui.horizontal(|ui| {
                    ui.label("Zenith:");
                    ui.add(egui::TextEdit::singleline(&mut self.deco_detail_skill_search[4]).hint_text("Search...").desired_width(100.0));
                    let qz = self.deco_detail_skill_search[4].to_lowercase();
                    egui::ComboBox::from_id_source("deco_zen_2").selected_text(crate::utils::weapon_patterns::zenith_skill_name(zen)).show_ui(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                            for (id, name) in crate::utils::weapon_patterns::ZENITH_SKILL_LIST { 
                                if qz.is_empty() || name.to_lowercase().contains(&qz) {
                                    ui.selectable_value(&mut zen, *id, format!("{} - {}", id, name));
                                }
                            }
                        });
                    });
                });

                // Write back using full struct replacement
                let new_deco = crate::model::mhfdat::MhfdatDecoId {
                    slot_nb,
                    flags,
                    price,
                    _pad0: d._pad0,
                    skill_id1, skill_pts1,
                    skill_id2, skill_pts2,
                    skill_id3, skill_pts3,
                    skill_id4, skill_pts4,
                    special_flags: d.special_flags,
                    zenith_skill: zen,
                };
                self.deco_ids[deco_idx] = new_deco;
                self.deco_ids_modified = true;
            }
        }
    }
}