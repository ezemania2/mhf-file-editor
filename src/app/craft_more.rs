use super::*;

impl MhfdatApp {
    /// More Crafting keeps only Transmog 2. The other crafting tables live on the Shop tab.
    pub fn show_craft_more_tab(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "craft");
        ui.horizontal_wrapped(|ui| {
            let _ = ui.selectable_label(true, "Transmog 2");
        });
        ui.separator();
        self.show_forging_rows(ui, 2);
    }

    fn craft_window(&mut self, total: usize) -> (usize, usize, usize) {
        crate::app::list_window(&mut self.craft_more_page, total)
    }

    pub(in crate::app) fn show_forging_rows(&mut self, ui: &mut egui::Ui, which: usize) {
        let total = match which {
            0 => self.special_weapon_forging.len(),
            1 => self.armor_premium_forging.len(),
            _ => self.transmog_forging_2.len(),
        };
        if total == 0 {
            ui.label("No rows loaded.");
            return;
        }
        ui.label(format!("{total} rows (the terminator row after them is not touched)"));
        let (start, end, total_pages) = self.craft_window(total);
        let rows = match which {
            0 => &mut self.special_weapon_forging,
            1 => &mut self.armor_premium_forging,
            _ => &mut self.transmog_forging_2,
        };
        let mut changed = false;
        egui::ScrollArea::both().show(ui, |ui| {
            egui::Grid::new(("forging_rows_grid", which)).striped(true).show(ui, |ui| {
                for h in [
                    "Idx", "Equip type", "Purchasable", "Equip ID", "Item 1", "Qty", "Item 2", "Qty", "Item 3", "Qty", "Item 4", "Qty",
                    "HR req", "Preview", "Footer",
                ] {
                    ui.label(h);
                }
                ui.end_row();
                for idx in start..end {
                    let e = &mut rows[idx];
                    ui.label(idx.to_string());
                    packed_drag!(ui, e.equip_type, changed);
                    let mut buyable = e.purchaseable != 0;
                    if ui.checkbox(&mut buyable, "").changed() {
                        e.purchaseable = buyable as u8;
                        changed = true;
                    }
                    packed_drag!(ui, e.equip_id, changed);
                    for k in 0..4 {
                        match k {
                            0 => { packed_drag!(ui, e.material_id1, changed); packed_drag!(ui, e.material_amnt1, changed); }
                            1 => { packed_drag!(ui, e.material_id2, changed); packed_drag!(ui, e.material_amnt2, changed); }
                            2 => { packed_drag!(ui, e.material_id3, changed); packed_drag!(ui, e.material_amnt3, changed); }
                            _ => { packed_drag!(ui, e.material_id4, changed); packed_drag!(ui, e.material_amnt4, changed); }
                        }
                    }
                    packed_drag!(ui, e.hr_req, changed);
                    let mut preview = e.preview_able;
                    if ui.checkbox(&mut preview, "").changed() {
                        e.preview_able = preview;
                        changed = true;
                    }
                    packed_drag!(ui, e.footer, changed);
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.craft_more_page, total_pages);
        if changed {
            self.craft_more_modified = true;
        }
    }

    fn shop_list_is_details(&self) -> bool {
        matches!(self.view_mode.get("shop"), Some(ViewMode::Details))
    }

    fn open_shop_details(&mut self, idx: usize) {
        self.craft_more_selected = Some(idx);
        self.view_mode.insert("shop".to_string(), ViewMode::Details);
    }

    fn back_shop_list(&mut self) {
        self.view_mode.insert("shop".to_string(), ViewMode::List);
    }

    /// Special (0) and premium (1) forging. Transmog 2 stays on `show_forging_rows`.
    pub(in crate::app) fn show_fixed_forging(&mut self, ui: &mut egui::Ui, which: usize) {
        if self.shop_list_is_details() {
            self.show_fixed_forging_details(ui, which);
        } else {
            self.show_fixed_forging_list(ui, which);
        }
    }

    fn forging_len(&self, which: usize) -> usize {
        if which == 0 { self.special_weapon_forging.len() } else { self.armor_premium_forging.len() }
    }

    fn forging_equip_name(&self, which: usize, equip_type: u8, equip_id: u16) -> String {
        if which == 0 { self.get_weapon_name(equip_type, equip_id) } else { self.get_armor_name(equip_type, equip_id) }
    }

    fn show_fixed_forging_list(&mut self, ui: &mut egui::Ui, which: usize) {
        let title = if which == 0 { "Special Forging" } else { "Premium Forging" };
        let total = self.forging_len(which);
        MhfdatApp::section_header(ui, &format!("{title} (found: {total})"), |_| {});
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No rows found!");
            return;
        }
        let query = self.search_query.clone();
        let filtered: Vec<usize> = (0..total).filter(|&i| {
            let entry = if which == 0 { &self.special_weapon_forging[i] } else { &self.armor_premium_forging[i] };
            let equip_id = entry.equip_id;
            let equip_type = entry.equip_type;
            let name = self.forging_equip_name(which, equip_type, equip_id);
            query.is_empty() || crate::utils::text::contains_ci(&name, &query) || equip_id.to_string().contains(&query)
        }).collect();
        let (start, end, total_pages) = crate::app::list_window(&mut self.craft_more_page, filtered.len());
        let rows: Vec<(usize, u8, u16, String, u16)> = filtered[start..end].iter().map(|&i| {
            let entry = if which == 0 { &self.special_weapon_forging[i] } else { &self.armor_premium_forging[i] };
            let equip_type = entry.equip_type;
            let equip_id = entry.equip_id;
            let hr = entry.hr_req;
            let name = self.forging_equip_name(which, equip_type, equip_id);
            (i, equip_type, equip_id, name, hr)
        }).collect();
        MhfdatApp::entity_list(ui, &format!("fixed_forging_{which}"), "Entry", start, end, filtered.len(), |ui| {
            egui::Grid::new(("fixed_forging_list", which)).striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Type");
                ui.label("Equip ID");
                ui.label("Name");
                ui.label("HR");
                ui.end_row();
                for (i, equip_type, equip_id, name, hr) in &rows {
                    let selected = self.craft_more_selected == Some(*i);
                    if ui.selectable_label(selected, i.to_string()).clicked() {
                        self.open_shop_details(*i);
                    }
                    ui.label(equip_type.to_string());
                    ui.label(equip_id.to_string());
                    ui.label(name);
                    ui.label(hr.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.craft_more_page, total_pages);
    }

    fn show_fixed_forging_details(&mut self, ui: &mut egui::Ui, which: usize) {
        if ui.button("← Back to List").clicked() {
            self.back_shop_list();
            return;
        }
        let len = self.forging_len(which);
        let Some(idx) = self.craft_more_selected.filter(|&i| i < len) else {
            ui.label("No row selected");
            return;
        };
        let title = if which == 0 { "Special Forging" } else { "Premium Forging" };
        let (equip_type, equip_id, mat_ids) = {
            let entry = if which == 0 { &self.special_weapon_forging[idx] } else { &self.armor_premium_forging[idx] };
            (entry.equip_type, entry.equip_id, [entry.material_id1, entry.material_id2, entry.material_id3, entry.material_id4])
        };
        let equip_name = self.forging_equip_name(which, equip_type, equip_id);
        let mat_names: Vec<String> = mat_ids.iter().map(|id| self.get_item_name(*id)).collect();
        ui.heading(format!("Edit {title} #{idx}"));
        ui.separator();
        let entry = if which == 0 { &mut self.special_weapon_forging[idx] } else { &mut self.armor_premium_forging[idx] };
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label("Equip type:");
            packed_drag!(ui, entry.equip_type, changed);
        });
        ui.horizontal(|ui| {
            ui.label("Equip ID:");
            packed_drag!(ui, entry.equip_id, changed);
            ui.label(format!("({equip_name})"));
        });
        ui.horizontal(|ui| {
            ui.label("Purchaseable:");
            let mut buy = entry.purchaseable != 0;
            if ui.checkbox(&mut buy, "").changed() {
                entry.purchaseable = u8::from(buy);
                changed = true;
            }
            ui.label("Preview:");
            let mut preview = entry.preview_able;
            if ui.checkbox(&mut preview, "").changed() {
                entry.preview_able = preview;
                changed = true;
            }
        });
        ui.separator();
        ui.label("Materials:");
        for k in 0..4 {
            ui.horizontal(|ui| {
                ui.label(format!("Material {}:", k + 1));
                match k {
                    0 => { packed_drag!(ui, entry.material_id1, changed); ui.label(&mat_names[0]); ui.label("Qty:"); packed_drag!(ui, entry.material_amnt1, changed); }
                    1 => { packed_drag!(ui, entry.material_id2, changed); ui.label(&mat_names[1]); ui.label("Qty:"); packed_drag!(ui, entry.material_amnt2, changed); }
                    2 => { packed_drag!(ui, entry.material_id3, changed); ui.label(&mat_names[2]); ui.label("Qty:"); packed_drag!(ui, entry.material_amnt3, changed); }
                    _ => { packed_drag!(ui, entry.material_id4, changed); ui.label(&mat_names[3]); ui.label("Qty:"); packed_drag!(ui, entry.material_amnt4, changed); }
                }
            });
        }
        ui.separator();
        ui.horizontal(|ui| { ui.label("HR req:"); packed_drag!(ui, entry.hr_req, changed); });
        ui.horizontal(|ui| { ui.label("Footer:"); packed_drag!(ui, entry.footer, changed); });
        if changed {
            self.craft_more_modified = true;
        }
    }

    pub(in crate::app) fn show_combine_shop(&mut self, ui: &mut egui::Ui) {
        if self.shop_list_is_details() {
            self.show_combine_details(ui);
        } else {
            self.show_combine_list(ui);
        }
    }

    fn show_combine_list(&mut self, ui: &mut egui::Ui) {
        let total = self.combine_shop.len();
        MhfdatApp::section_header(ui, &format!("Combine shop (found: {total})"), |_| {});
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No combine shop rows found!");
            return;
        }
        let query = self.search_query.clone();
        let filtered: Vec<usize> = (0..total).filter(|&i| {
            let item_id = self.combine_shop[i].item_id;
            let name = self.get_item_name(item_id);
            query.is_empty() || crate::utils::text::contains_ci(&name, &query) || item_id.to_string().contains(&query)
        }).collect();
        let (start, end, total_pages) = crate::app::list_window(&mut self.craft_more_page, filtered.len());
        let rows: Vec<(usize, u16, String, u16, u16)> = filtered[start..end].iter().map(|&i| {
            let r = &self.combine_shop[i];
            let item_id = r.item_id;
            (i, item_id, self.get_item_name(item_id), r.combo_fee, r.recipe_no)
        }).collect();
        MhfdatApp::entity_list(ui, "combine_shop_scroll", "Entry", start, end, filtered.len(), |ui| {
            egui::Grid::new("combine_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Item ID");
                ui.label("Name");
                ui.label("Fee");
                ui.label("Recipe no.");
                ui.end_row();
                for (i, item_id, name, fee, recipe) in &rows {
                    if ui.selectable_label(self.craft_more_selected == Some(*i), i.to_string()).clicked() {
                        self.open_shop_details(*i);
                    }
                    ui.label(item_id.to_string());
                    ui.label(name);
                    ui.label(fee.to_string());
                    ui.label(recipe.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.craft_more_page, total_pages);
    }

    fn show_combine_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_shop_list();
            return;
        }
        let Some(sel) = self.craft_more_selected.filter(|&i| i < self.combine_shop.len()) else {
            ui.label("No row selected");
            return;
        };
        ui.heading(format!("Edit Combine shop #{sel}"));
        ui.separator();
        ui.label("A row ends the list at an item id of 0 or above 16700, so keep ids in that range.");
        let names = &self.item_names;
        let r = &mut self.combine_shop[sel];
        let mut changed = false;
        egui::Grid::new("combine_details").num_columns(2).show(ui, |ui| {
            changed |= MhfdatApp::num_row(ui, &format!("Item ID {}", MhfdatApp::item_label(names, r.item_id)), &mut r.item_id);
            changed |= MhfdatApp::num_row(ui, "Combo fee", &mut r.combo_fee);
            changed |= MhfdatApp::num_row(ui, "Recipe no.", &mut r.recipe_no);
        });
        for (i, rec) in r.recipes.iter_mut().enumerate() {
            egui::CollapsingHeader::new(format!("Recipe {i}")).id_source(("combine_recipe", sel, i)).default_open(true).show(ui, |ui| {
                egui::Grid::new(("combine_recipe_grid", sel, i)).num_columns(2).show(ui, |ui| {
                    changed |= MhfdatApp::num_row(ui, "Min", &mut rec.min);
                    changed |= MhfdatApp::num_row(ui, "Max", &mut rec.max);
                    for k in 0..3 {
                        changed |= MhfdatApp::num_row(ui, &format!("Ingredient {k} {}", MhfdatApp::item_label(names, rec.item_ids[k])), &mut rec.item_ids[k]);
                        changed |= MhfdatApp::num_row(ui, &format!("Ingredient {k} quantity"), &mut rec.item_qtys[k]);
                    }
                });
            });
        }
        if changed {
            self.craft_more_modified = true;
        }
    }

    pub(in crate::app) fn show_tower_jewels(&mut self, ui: &mut egui::Ui) {
        if self.shop_list_is_details() {
            self.show_tower_jewel_details(ui);
        } else {
            self.show_tower_jewel_list(ui);
        }
    }

    fn show_tower_jewel_list(&mut self, ui: &mut egui::Ui) {
        let total = self.tower_jewels.len();
        MhfdatApp::section_header(ui, &format!("Tower jewel recipes (found: {total})"), |_| {});
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No tower jewel recipes found!");
            return;
        }
        let query = self.search_query.clone();
        let filtered: Vec<usize> = (0..total).filter(|&i| {
            let id = self.tower_jewels[i].weapon_id;
            let name = self.get_weapon_name(0x06, id);
            query.is_empty() || crate::utils::text::contains_ci(&name, &query) || id.to_string().contains(&query)
        }).collect();
        let (start, end, total_pages) = crate::app::list_window(&mut self.craft_more_page, filtered.len());
        let rows: Vec<(usize, u16, String, u16, u16, u8)> = filtered[start..end].iter().map(|&i| {
            let j = &self.tower_jewels[i];
            let weapon_id = j.weapon_id;
            (i, weapon_id, self.get_weapon_name(0x06, weapon_id), j.level_min, j.level_max, j.equip_type)
        }).collect();
        MhfdatApp::entity_list(ui, "tower_jewels_scroll", "Recipe", start, end, filtered.len(), |ui| {
            egui::Grid::new("tower_jewels_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Weapon ID");
                ui.label("Name");
                ui.label("Lv min");
                ui.label("Lv max");
                ui.label("Equip type");
                ui.end_row();
                for (i, weapon_id, name, lo, hi, equip) in &rows {
                    if ui.selectable_label(self.craft_more_selected == Some(*i), i.to_string()).clicked() {
                        self.open_shop_details(*i);
                    }
                    ui.label(weapon_id.to_string());
                    ui.label(name);
                    ui.label(lo.to_string());
                    ui.label(hi.to_string());
                    ui.label(equip.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.craft_more_page, total_pages);
    }

    fn show_tower_jewel_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_shop_list();
            return;
        }
        let Some(idx) = self.craft_more_selected.filter(|&i| i < self.tower_jewels.len()) else {
            ui.label("No recipe selected");
            return;
        };
        ui.heading(format!("Edit Tower jewel recipe #{idx}"));
        ui.separator();
        let names = self.item_names.clone();
        let j = &mut self.tower_jewels[idx];
        let mut changed = false;
        ui.horizontal(|ui| { ui.label("Weapon ID:"); changed |= ui.add(egui::DragValue::new(&mut j.weapon_id)).changed(); });
        ui.horizontal(|ui| { ui.label("Level min:"); changed |= ui.add(egui::DragValue::new(&mut j.level_min)).changed(); });
        ui.horizontal(|ui| { ui.label("Level max:"); changed |= ui.add(egui::DragValue::new(&mut j.level_max)).changed(); });
        ui.horizontal(|ui| { ui.label("Stat ID:"); changed |= ui.add(egui::DragValue::new(&mut j.stat_id)).changed(); });
        ui.horizontal(|ui| { ui.label("Equip type:"); changed |= ui.add(egui::DragValue::new(&mut j.equip_type)).changed(); });
        ui.separator();
        ui.label("Materials:");
        for (k, m) in j.mats.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("Item {}:", k + 1));
                changed |= ui.add(egui::DragValue::new(&mut m.item)).changed();
                ui.label(MhfdatApp::item_label(&names, m.item));
                ui.label("Qty:");
                changed |= ui.add(egui::DragValue::new(&mut m.qty)).changed();
            });
        }
        if changed {
            self.craft_more_modified = true;
        }
    }

    pub(in crate::app) fn show_tower_seals(&mut self, ui: &mut egui::Ui) {
        if self.shop_list_is_details() {
            self.show_tower_seal_details(ui);
        } else {
            self.show_tower_seal_list(ui);
        }
    }

    fn show_tower_seal_list(&mut self, ui: &mut egui::Ui) {
        let total = self.sigil_tower_entries.len();
        MhfdatApp::section_header(ui, &format!("Tower seals (found: {total})"), |_| {});
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No tower seal recipes found!");
            return;
        }
        let query = self.search_query.clone();
        let filtered: Vec<usize> = (0..total).filter(|&i| {
            let item_id = self.sigil_tower_entries[i].item_id;
            let name = self.get_item_name(item_id);
            query.is_empty() || crate::utils::text::contains_ci(&name, &query) || item_id.to_string().contains(&query)
        }).collect();
        let (start, end, total_pages) = crate::app::list_window(&mut self.craft_more_page, filtered.len());
        let rows: Vec<(usize, u16, String, u16)> = filtered[start..end].iter().map(|&i| {
            let e = &self.sigil_tower_entries[i];
            let item_id = e.item_id;
            (i, item_id, self.get_item_name(item_id), e.receipt_category)
        }).collect();
        MhfdatApp::entity_list(ui, "tower_seals_scroll", "Recipe", start, end, filtered.len(), |ui| {
            egui::Grid::new("tower_seals_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Item ID");
                ui.label("Name");
                ui.label("Category");
                ui.end_row();
                for (i, item_id, name, category) in &rows {
                    if ui.selectable_label(self.craft_more_selected == Some(*i), i.to_string()).clicked() {
                        self.open_shop_details(*i);
                    }
                    ui.label(item_id.to_string());
                    ui.label(name);
                    ui.label(category.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.craft_more_page, total_pages);
    }

    fn show_tower_seal_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_shop_list();
            return;
        }
        let Some(idx) = self.craft_more_selected.filter(|&i| i < self.sigil_tower_entries.len()) else {
            ui.label("No recipe selected");
            return;
        };
        ui.heading(format!("Edit Tower seal #{idx}"));
        ui.separator();
        ui.label("The list ends on an item id of 0, so keep ids non-zero.");
        let names = self.item_names.clone();
        let e = &mut self.sigil_tower_entries[idx];
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label("Item ID:");
            packed_drag!(ui, e.item_id, changed);
            ui.label(MhfdatApp::item_label(&names, e.item_id));
        });
        ui.horizontal(|ui| { ui.label("Receipt category:"); packed_drag!(ui, e.receipt_category, changed); });
        for k in 0..4 {
            ui.horizontal(|ui| {
                ui.label(format!("Material {}:", k + 1));
                match k {
                    0 => { packed_drag!(ui, e.item_craft1, changed); ui.label(MhfdatApp::item_label(&names, e.item_craft1)); ui.label("Qty:"); packed_drag!(ui, e.item_qty1, changed); }
                    1 => { packed_drag!(ui, e.item_craft2, changed); ui.label(MhfdatApp::item_label(&names, e.item_craft2)); ui.label("Qty:"); packed_drag!(ui, e.item_qty2, changed); }
                    2 => { packed_drag!(ui, e.item_craft3, changed); ui.label(MhfdatApp::item_label(&names, e.item_craft3)); ui.label("Qty:"); packed_drag!(ui, e.item_qty3, changed); }
                    _ => { packed_drag!(ui, e.item_craft4, changed); ui.label(MhfdatApp::item_label(&names, e.item_craft4)); ui.label("Qty:"); packed_drag!(ui, e.item_qty4, changed); }
                }
                ui.label("Unlocked:");
                let mut flag = match k { 0 => e.item_unlock_flag1, 1 => e.item_unlock_flag2, 2 => e.item_unlock_flag3, _ => e.item_unlock_flag4 };
                if MhfdatApp::flag_cell(ui, &mut flag) {
                    match k { 0 => e.item_unlock_flag1 = flag, 1 => e.item_unlock_flag2 = flag, 2 => e.item_unlock_flag3 = flag, _ => e.item_unlock_flag4 = flag }
                    changed = true;
                }
            });
        }
        if changed {
            self.craft_more_modified = true;
        }
    }

    pub(in crate::app) fn show_tower_costs(&mut self, ui: &mut egui::Ui) {
        if self.shop_list_is_details() {
            self.show_tower_cost_details(ui);
        } else {
            self.show_tower_cost_list(ui);
        }
    }

    fn show_tower_cost_list(&mut self, ui: &mut egui::Ui) {
        let total = self.tower_costs.len();
        MhfdatApp::section_header(ui, &format!("Tower upgrade costs (found: {total})"), |_| {});
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No tower upgrade cost table found!");
            return;
        }
        ui.label("One block of steps per tower parameter id. Each step is ten (ok, cost) pairs and four bow bytes.");
        let (start, end, total_pages) = crate::app::list_window(&mut self.craft_more_page, total);
        let rows: Vec<(usize, u32, usize, String)> = (start..end).map(|i| {
            let b = &self.tower_costs[i];
            let note = if b.offset == 0 {
                "null".to_string()
            } else if let Some(first) = b.alias_of {
                format!("same as #{first}")
            } else {
                String::new()
            };
            (i, b.offset, b.rows.len(), note)
        }).collect();
        MhfdatApp::entity_list(ui, "tower_costs_scroll", "Parameter", start, end, total, |ui| {
            egui::Grid::new("tower_costs_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Offset");
                ui.label("Steps");
                ui.label("Note");
                ui.end_row();
                for (i, offset, steps, note) in &rows {
                    if ui.selectable_label(self.craft_more_selected == Some(*i), i.to_string()).clicked() {
                        self.open_shop_details(*i);
                    }
                    ui.label(format!("0x{offset:X}"));
                    ui.label(steps.to_string());
                    ui.label(note);
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.craft_more_page, total_pages);
    }

    fn show_tower_cost_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_shop_list();
            return;
        }
        let Some(sel) = self.craft_more_selected.filter(|&i| i < self.tower_costs.len()) else {
            ui.label("No parameter selected");
            return;
        };
        ui.heading(format!("Edit Tower upgrade costs #{sel}"));
        ui.separator();
        let b = &self.tower_costs[sel];
        if b.offset == 0 {
            ui.label("Null pointer: nothing to edit here.");
            return;
        }
        ui.label(format!("Block at file offset 0x{:X}", b.offset));
        if let Some(first) = b.alias_of {
            ui.label(format!("Same block as parameter {first}: edit it there."));
            return;
        }
        let mut changed = false;
        MhfdatApp::entity_list(ui, "tower_cost_steps_scroll", "Step", 0, self.tower_costs[sel].rows.len(), self.tower_costs[sel].rows.len(), |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                egui::Grid::new(("tower_costs_grid", sel)).striped(true).show(ui, |ui| {
                    ui.label("Step");
                    for h in ["raw", "gunner", "affinity", "element", "seal 7", "seal 5", "seal 6", "extra 5/6", "extra C", "extra B"] {
                        ui.label(format!("{h} ok"));
                        ui.label(format!("{h} cost"));
                    }
                    for k in 0..4 {
                        ui.label(format!("bow {k}"));
                    }
                    ui.end_row();
                    for (i, step) in self.tower_costs[sel].rows.iter_mut().enumerate() {
                        ui.label(i.to_string());
                        for pair in step.ok_cost.iter_mut() {
                            changed |= ui.add(egui::DragValue::new(&mut pair[0])).changed();
                            changed |= ui.add(egui::DragValue::new(&mut pair[1])).changed();
                        }
                        for v in step.bow.iter_mut() {
                            changed |= ui.add(egui::DragValue::new(v)).changed();
                        }
                        ui.end_row();
                    }
                });
            });
        });
        if changed {
            self.craft_more_modified = true;
        }
    }
}
