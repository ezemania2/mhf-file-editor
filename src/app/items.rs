use super::*;
use egui;
use std::fs::File;
use std::io::Write;
use serde_json;
use crate::utils::item_patterns::{ITEM_ICON_LIST, icon_name, equip_type_name, item_type_name, icon_color_name};

impl MhfdatApp {
    pub fn show_items_tab(&mut self, ui: &mut egui::Ui) {
        // Initialize view mode if not present
        if !self.view_mode.contains_key("items") {
            self.view_mode.insert("items".to_string(), ViewMode::List);
        }

        match self.view_mode.get("items").unwrap() {
            ViewMode::List => self.show_items_list(ui),
            ViewMode::Details => self.show_item_details_view(ui),
        }
    }

    pub fn show_items_list(&mut self, ui: &mut egui::Ui) {
                 ui.heading(format!("Items (found: {})", self.items.len()));

        // Search and filters
        ui.horizontal(|ui| {
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
             egui::ScrollArea::vertical().show(ui, |ui| {
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
            ui.horizontal(|ui| {
                let can_go_previous = current_page > 0 && filtered_count > 0;
                let can_go_next = current_page < total_pages.saturating_sub(1) && filtered_count > 0;
                
                if ui.button("← Previous").clicked() && can_go_previous {
                    self.item_page = (current_page.saturating_sub(1)) as u32;
                }
                if filtered_count > 0 {
                    ui.label(format!("Page {} of {}", current_page + 1, total_pages));
                } else {
                    ui.label("No results found");
                }
                if ui.button("Next →").clicked() && can_go_next {
                    self.item_page = (current_page + 1) as u32;
                }
            });
        }
    }

    pub fn show_item_details_view(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("← Back to List").clicked() {
                if let Some(view_mode) = self.view_mode.get_mut("items") {
                    *view_mode = ViewMode::List;
                }
            }
        });

        if let Some(index) = self.selected_item_index {
            if let Some(item) = self.items.get(index) {
                ui.heading("Item Details");
                
                let item_name = self.item_names.get(index).cloned().unwrap_or_default();
                let item_description = self.item_descriptions.get(index).cloned().unwrap_or_default();
                
                // Ensure we have valid data for this index
                if index >= self.item_names.len() {
                    self.item_names.resize(index + 1, String::new());
                }
                if index >= self.item_descriptions.len() {
                    self.item_descriptions.resize(index + 1, String::new());
                }
                
                                 ui.horizontal(|ui| {
                     ui.label("Name:");
                     ui.text_edit_singleline(&mut self.item_names[index]);
                 });
                 ui.horizontal(|ui| {
                     ui.label("Description:");
                     ui.text_edit_multiline(&mut self.item_descriptions[index]);
                 });
                 
                 ui.add_space(10.0);
                 

                 
                 ui.add_space(10.0);
                 ui.separator();
                 ui.add_space(10.0);
                
                // Copy fields to local variables to avoid unaligned references
                let unk00 = item.unk00;
                let unk01 = item.unk01;
                let rarity = item.rarity;
                let max_stack = item.max_stack;
                let unk04 = item.unk04;
                let icon = item.icon;
                let icon_color = item.icon_color;
                let unk07 = item.unk07;
                let bottle = item.bottle;
                let unk0A = item.unk0A;
                let buy_price = item.buy_price;
                let sell_price = item.sell_price;
                let item_type = item.item_type;
                let deco_id = item.deco_id;
                let unk18 = item.unk18;
                let unk1A = item.unk1A;
                let unk1B = item.unk1B;
                let equip_type = item.equip_type;
                let is_gz = item.is_gz;
                let unk1F = item.unk1F;
                let unk20 = item.unk20;
                let unk22 = item.unk22;
                
                                 // Copy fields to local variables to avoid unaligned references
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
                 
                 egui::Grid::new("item_details_grid")
                     .num_columns(2)
                     .show(ui, |ui| {
                         ui.label("Interaction Type:");
                         ui.add(egui::DragValue::new(&mut unk00).speed(1.0));
                         ui.end_row();
                         
                         ui.label("Is Usable:");
                         ui.add(egui::DragValue::new(&mut unk01).speed(1.0));
                         ui.end_row();
                         
                         ui.label("Rarity:");
                         ui.add(egui::DragValue::new(&mut rarity).speed(1.0).clamp_range(0..=9));
                         ui.end_row();
                         
                         ui.label("Max Stack:");
                         ui.add(egui::DragValue::new(&mut max_stack).speed(1.0));
                         ui.end_row();
                         
                         ui.label("Data Bitset:");
                         ui.add(egui::DragValue::new(&mut unk04).speed(1.0));
                         ui.end_row();
                         
                         ui.label("Icon:");
                         egui::ComboBox::from_id_source("icon_combo")
                             .selected_text(icon_name(icon))
                             .show_ui(ui, |ui| {
                                 for (id, name) in ITEM_ICON_LIST {
                                     if ui.selectable_value(&mut icon, *id, *name).clicked() {}
                                 }
                             });
                         ui.end_row();
                         
                         ui.label("Icon Color:");
                         ui.add(egui::DragValue::new(&mut icon_color).speed(1.0));
                         ui.end_row();
                         
                         ui.label("Bottle:");
                         ui.add(egui::DragValue::new(&mut bottle).speed(1.0));
                         ui.end_row();
                         
                         ui.label("Buy Price:");
                         ui.add(egui::DragValue::new(&mut buy_price).speed(1.0));
                         ui.end_row();
                         
                         ui.label("Sell Price:");
                         ui.add(egui::DragValue::new(&mut sell_price).speed(1.0));
                         ui.end_row();
                         
                         ui.label("Item Type:");
                         ui.add(egui::DragValue::new(&mut item_type).speed(1.0));
                         ui.end_row();
                         
                         ui.label("Deco ID:");
                         ui.add(egui::DragValue::new(&mut deco_id).speed(1.0));
                         ui.end_row();
                         
                         ui.label("Equip Type:");
                         ui.add(egui::DragValue::new(&mut equip_type).speed(1.0));
                         ui.end_row();
                         
                         ui.label("Is GZ:");
                         ui.add(egui::DragValue::new(&mut is_gz).speed(1.0));
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
            }
        }
    }
}