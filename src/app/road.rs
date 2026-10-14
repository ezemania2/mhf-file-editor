use super::*;

#[derive(PartialEq, Clone, Copy, Default)]
pub enum RoadTab {
    #[default]
    Points,
    FelyneShop,
    Skills,
}

/// Names of the known bits of `RoadMonsterPoints.flags` (road_points.hexpat); the others show as "bit N".
const POINT_FLAG_NAMES: [&str; 3] = ["alt variant (bit 0)", "alt variant (bit 1)", "Zenith tiers"];

impl MhfdatApp {
    /// Hunting Road tables: points per monster (header 0xB20, count at 0xB04 + 8), Felyne shop (0xB10) and skill
    /// definitions (0x930). All three can grow: the save then moves a table to the end of the file (the Felyne shop and
    /// the skills keep their place when their length does not change).
    pub fn show_road_tab(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "road");
        ui.horizontal_wrapped(|ui| {
            for (tab, label) in [
                (RoadTab::Points, "Points per monster"),
                (RoadTab::FelyneShop, "Felyne shop"),
                (RoadTab::Skills, "Road skills"),
            ] {
                if ui.selectable_label(self.road_tab == tab, label).clicked() {
                    self.road_tab = tab;
                    self.road_page = 0;
                    self.road_selected = None;
                    self.back_category_list("road");
                }
            }
        });
        ui.separator();
        match self.road_tab {
            RoadTab::Points => self.show_road_points(ui),
            RoadTab::FelyneShop => self.show_road_cat_shop(ui),
            RoadTab::Skills => self.show_road_skills(ui),
        }
    }

    fn road_window(&mut self, total: usize) -> (usize, usize, usize) {
        crate::app::list_window(&mut self.road_page, total)
    }

    /// First monster id (1..=176) that has no row yet, for a new row.
    fn free_road_monster_id(&self) -> u16 {
        (1..=crate::core::mhfdat::ROAD_MONSTER_ID_MAX)
            .find(|id| self.road.points.iter().all(|p| p.monster_id != *id))
            .unwrap_or(1)
    }

    pub(in crate::app) fn add_road_monster(&mut self) {
        let monster_id = self.free_road_monster_id();
        self.road.points.push(crate::model::mhfdat::RoadMonsterPoint { monster_id, flags: 0, points: [0; 6] });
        self.road_selected = Some(self.road.points.len() - 1);
        self.road_modified = true;
        self.open_category_details("road");
    }

    fn show_road_points(&mut self, ui: &mut egui::Ui) {
        if self.category_details("road") {
            self.show_road_point_details(ui);
            return;
        }
        let total = self.road.points.len();
        MhfdatApp::section_header(ui, &format!("Points per monster (found: {total})"), |ui| {
            if ui.button("Add New").clicked() {
                self.add_road_monster();
            }
        });
        ui.label("Base points, then the points of levels 1 to 5. The game uses the first row of a monster id. Adding or deleting a monster moves the table to the end of the file when saving.");
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search:");
            if ui.text_edit_singleline(&mut self.road_search).changed() {
                self.road_page = 0;
            }
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No Hunting Road point table found!");
            return;
        }
        let query = self.road_search.trim().to_lowercase();
        let filtered: Vec<usize> = (0..total).filter(|&i| {
            let id = self.road.points[i].monster_id;
            query.is_empty() || id.to_string() == query || crate::utils::text::contains_ci(crate::utils::monsters::monster_name(id), &query)
        }).collect();
        let (start, end, total_pages) = crate::app::list_window(&mut self.road_page, filtered.len());
        let rows: Vec<(usize, u16, u16, [u16; 6])> = filtered[start..end].iter().map(|&i| {
            let r = &self.road.points[i];
            (i, r.monster_id, r.flags, r.points)
        }).collect();
        MhfdatApp::entity_list(ui, "road_points_scroll", "Monster", start, end, filtered.len(), |ui| {
            egui::Grid::new("road_points_list").striped(true).show(ui, |ui| {
                for h in ["ID", "Monster", "Flags", "Base", "Level 1", "Level 2", "Level 3", "Level 4", "Level 5"] {
                    ui.label(h);
                }
                ui.end_row();
                for (idx, monster_id, flags, points) in &rows {
                    if ui.selectable_label(self.road_selected == Some(*idx), idx.to_string()).clicked() {
                        self.road_selected = Some(*idx);
                        self.open_category_details("road");
                    }
                    ui.label(format!("{} {}", monster_id, crate::utils::monsters::monster_name(*monster_id)));
                    ui.label(format!("0x{flags:X}"));
                    for p in points {
                        ui.label(p.to_string());
                    }
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.road_page, total_pages);
    }

    fn show_road_point_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("road");
            return;
        }
        let Some(sel) = self.road_selected.filter(|&i| i < self.road.points.len()) else {
            ui.label("No monster selected");
            return;
        };
        ui.heading(format!("Edit Road monster #{sel}"));
        if ui.button("Delete this monster").clicked() {
            self.road.points.remove(sel);
            self.road_modified = true;
            self.road_selected = None;
            self.back_category_list("road");
            return;
        }
        ui.separator();

        let monster_id = self.road.points[sel].monster_id;
        let twin = self.road.points.iter().enumerate().find(|(i, p)| *i != sel && p.monster_id == monster_id).map(|(i, _)| i);
        let mut changed = false;
        let row = &mut self.road.points[sel];
        // the page is taller than a small window: it scrolls
        egui::ScrollArea::vertical().id_source("road_point_details_scroll").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Monster:");
                egui::ComboBox::from_id_source("road_monster_combo")
                    .selected_text(format!("{} {}", row.monster_id, crate::utils::monsters::monster_name(row.monster_id)))
                    .show_ui(ui, |ui| {
                        for id in 1..=crate::core::mhfdat::ROAD_MONSTER_ID_MAX {
                            let label = format!("{} {}", id, crate::utils::monsters::monster_name(id));
                            if ui.selectable_value(&mut row.monster_id, id, label).changed() {
                                changed = true;
                            }
                        }
                    });
            });
            if let Some(other) = twin {
                ui.colored_label(egui::Color32::YELLOW, format!("Row {other} has the same monster: the game uses the first row of a monster id."));
            }
            changed |= MhfdatApp::bits_row(ui, "Flags:", &mut row.flags, &POINT_FLAG_NAMES);
            ui.label("Both alt-variant bits: the level 1 points are used when the quest roster has this monster as variant 1, 8 or 9. Zenith tiers: the levels follow the Zenith status (road_points.hexpat).");
            ui.separator();
            const LABELS: [&str; 6] = ["Base points", "Level 1 points", "Level 2 points", "Level 3 points", "Level 4 points", "Level 5 points"];
            egui::Grid::new("road_points_values").num_columns(2).show(ui, |ui| {
                for (label, p) in LABELS.iter().zip(row.points.iter_mut()) {
                    changed |= MhfdatApp::num_row(ui, label, p);
                }
            });
        });
        if changed {
            self.road_modified = true;
        }
    }

    fn show_road_cat_shop(&mut self, ui: &mut egui::Ui) {
        if self.category_details("road") {
            self.show_road_cat_details(ui);
            return;
        }
        let total = self.road.cat_shop.len();
        MhfdatApp::section_header(ui, &format!("Felyne shop (found: {total})"), |ui| {
            if ui.button("Add New").clicked() {
                // the item of the last row to start from (an item id of 0 would read as the end of the shop); 0xFFFFFFFF is the marker the file keeps in every row
                let item_id = self.road.cat_shop.last().map_or(1, |r| r.item_id).max(1);
                self.road.cat_shop.push(crate::model::mhfdat::RoadCatItem { item_id, unk02: 0xFFFF_FFFF, unk06: 0 });
                self.road_selected = Some(total);
                self.road_modified = true;
                self.open_category_details("road");
            }
            if MhfdatApp::delete_last_button(ui, total, None) {
                self.road.cat_shop.pop();
                self.road_modified = true;
            }
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No Felyne shop table found!");
            return;
        }
        let (start, end, total_pages) = self.road_window(total);
        let rows: Vec<(usize, u16, String)> = (start..end).map(|idx| {
            let id = self.road.cat_shop[idx].item_id;
            (idx, id, self.get_item_name(id))
        }).collect();
        MhfdatApp::entity_list(ui, "road_cat_scroll", "Item", start, end, total, |ui| {
            egui::Grid::new("road_cat_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Item ID");
                ui.label("Name");
                ui.end_row();
                for (idx, id, name) in &rows {
                    if ui.selectable_label(self.road_selected == Some(*idx), idx.to_string()).clicked() {
                        self.road_selected = Some(*idx);
                        self.open_category_details("road");
                    }
                    ui.label(id.to_string());
                    ui.label(name);
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.road_page, total_pages);
    }

    fn show_road_cat_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("road");
            return;
        }
        let Some(sel) = self.road_selected.filter(|&i| i < self.road.cat_shop.len()) else {
            ui.label("No row selected");
            return;
        };
        ui.heading(format!("Edit Felyne shop #{sel}"));
        ui.separator();
        let mut changed = false;
        let id = self.road.cat_shop[sel].item_id;
        let name = self.get_item_name(id);
        ui.horizontal(|ui| {
            ui.label("Item ID:");
            changed |= ui.add(egui::DragValue::new(&mut self.road.cat_shop[sel].item_id)).changed();
            ui.label(name);
        });
        if changed {
            self.road_modified = true;
        }

        // This table only holds the item id; the price is the one of the item itself (Items data).
        ui.separator();
        let id = self.road.cat_shop[sel].item_id;
        match self.items.get_mut(id as usize) {
            Some(item) => {
                let mut buy = item.buy_price;
                let mut sell = item.sell_price;
                let mut price_changed = false;
                ui.horizontal(|ui| {
                    ui.label("Buy price:");
                    price_changed |= ui.add(egui::DragValue::new(&mut buy)).changed();
                });
                ui.horizontal(|ui| {
                    ui.label("Sell price:");
                    price_changed |= ui.add(egui::DragValue::new(&mut sell)).changed();
                });
                ui.label("The prices belong to the item (Items tab): changing them changes this item in every shop.");
                if price_changed {
                    item.buy_price = buy;
                    item.sell_price = sell;
                    self.items_modified = true;
                }
            }
            None => {
                ui.label("This item id has no entry in the item data.");
            }
        }
    }

    fn show_road_skills(&mut self, ui: &mut egui::Ui) {
        if self.category_details("road") {
            self.show_road_skill_details(ui);
            return;
        }
        let total = self.road.skills.len();
        MhfdatApp::section_header(ui, &format!("Road skills (found: {total})"), |ui| {
            if ui.add_enabled(total > 0, egui::Button::new("Add New")).on_disabled_hover_text("The list is empty: nothing to copy the new skill from").clicked() {
                let skill = self.road.skills[total - 1].clone();
                self.road.skills.push(skill);
                self.road_selected = Some(total);
                self.road_modified = true;
                self.open_category_details("road");
            }
            if MhfdatApp::delete_last_button(ui, total, None) {
                self.road.skills.pop();
                self.road_modified = true;
            }
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No Hunting Road skill table found!");
            return;
        }
        let (start, end, total_pages) = self.road_window(total);
        let rows: Vec<(usize, u16, u16, u32)> = (start..end).map(|idx| {
            let d = &self.road.skills[idx];
            (idx, d.skill_id, d.level, d.cost)
        }).collect();
        MhfdatApp::entity_list(ui, "road_skills_scroll", "Skill", start, end, total, |ui| {
            egui::Grid::new("road_skills_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Skill ID");
                ui.label("Level");
                ui.label("Cost");
                ui.end_row();
                for (idx, skill_id, level, cost) in &rows {
                    if ui.selectable_label(self.road_selected == Some(*idx), idx.to_string()).clicked() {
                        self.road_selected = Some(*idx);
                        self.open_category_details("road");
                    }
                    ui.label(skill_id.to_string());
                    ui.label(level.to_string());
                    ui.label(cost.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.road_page, total_pages);
    }

    fn show_road_skill_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("road");
            return;
        }
        let Some(sel) = self.road_selected.filter(|&i| i < self.road.skills.len()) else {
            ui.label("No skill selected");
            return;
        };
        ui.heading(format!("Edit Road skill #{sel}"));
        ui.separator();
        let d = &mut self.road.skills[sel];
        let mut changed = false;
        egui::ScrollArea::vertical().id_source("road_skill_details_scroll").show(ui, |ui| {
            ui.horizontal(|ui| { ui.label("Skill ID:"); changed |= ui.add(egui::DragValue::new(&mut d.skill_id)).changed(); });
            ui.horizontal(|ui| { ui.label("Level:"); changed |= ui.add(egui::DragValue::new(&mut d.level)).changed(); });
            ui.horizontal(|ui| { ui.label("Category:"); changed |= ui.add(egui::DragValue::new(&mut d.category)).changed(); });
            ui.horizontal(|ui| { ui.label("Desc ID:"); changed |= ui.add(egui::DragValue::new(&mut d.desc_id)).changed(); });
            ui.horizontal(|ui| { ui.label("Sort key:"); changed |= ui.add(egui::DragValue::new(&mut d.sort_key)).changed(); });
            ui.horizontal(|ui| { ui.label("Cost:"); changed |= ui.add(egui::DragValue::new(&mut d.cost)).changed(); });
            ui.horizontal(|ui| { ui.label("Road rank:"); changed |= ui.add(egui::DragValue::new(&mut d.road_rank_req)).changed(); });
            ui.horizontal(|ui| { ui.label("Road points:"); changed |= ui.add(egui::DragValue::new(&mut d.road_points)).changed(); });
            ui.horizontal(|ui| { ui.label("Icon:"); changed |= ui.add(egui::DragValue::new(&mut d.icon_id)).changed(); });
            for (i, p) in d.prereqs.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Req {} skill:", i + 1));
                    changed |= ui.add(egui::DragValue::new(&mut p.skill_id)).changed();
                    ui.label("Lv:");
                    changed |= ui.add(egui::DragValue::new(&mut p.level)).changed();
                });
            }
        });
        if changed {
            self.road_modified = true;
        }
    }

}
