use super::*;

#[derive(PartialEq, Clone, Copy, Default)]
pub enum RoadTab {
    #[default]
    Points,
    FelyneShop,
    FallbackShop,
    Skills,
    Weight,
}

/// Names of the known bits of `RoadMonsterPoints.flags` (road_points.hexpat); the others show as "bit N".
const POINT_FLAG_NAMES: [&str; 3] = ["alt variant (bit 0)", "alt variant (bit 1)", "Zenith tiers"];

impl MhfdatApp {
    /// Hunting Road tables: points per monster (header 0xB20), Felyne shop (0xB10), fallback shop (special shops ->
    /// ptRoadShop), skill definitions (0x930) and the spawn weight window (0x928 + 0x8C4). Edited in place; rows
    /// cannot be added or removed.
    pub fn show_road_tab(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "road");
        ui.horizontal_wrapped(|ui| {
            for (tab, label) in [
                (RoadTab::Points, "Points per monster"),
                (RoadTab::FelyneShop, "Felyne shop"),
                (RoadTab::FallbackShop, "Fallback shop"),
                (RoadTab::Skills, "Road skills"),
                (RoadTab::Weight, "Weight window"),
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
            RoadTab::FallbackShop => self.show_road_shop(ui),
            RoadTab::Skills => self.show_road_skills(ui),
            RoadTab::Weight => self.show_road_weight(ui),
        }
    }

    fn road_window(&mut self, total: usize) -> (usize, usize, usize) {
        crate::app::list_window(&mut self.road_page, total)
    }

    fn show_road_points(&mut self, ui: &mut egui::Ui) {
        if self.category_details("road") {
            self.show_road_point_details(ui);
            return;
        }
        let total = self.road.points.len();
        MhfdatApp::section_header(ui, &format!("Points per monster (found: {total})"), |_| {});
        ui.label("points[0] = normal, points[1..4] = Zenith tiers 1-4 when the Zenith flag is set.");
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No Hunting Road point table found!");
            return;
        }
        let (start, end, total_pages) = self.road_window(total);
        let rows: Vec<(usize, u16, u16)> = (start..end).map(|idx| {
            let r = &self.road.points[idx];
            (idx, r.monster_id, r.points[0])
        }).collect();
        MhfdatApp::entity_list(ui, "road_points_scroll", "Monster", start, end, total, |ui| {
            egui::Grid::new("road_points_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Monster ID");
                ui.label("points[0]");
                ui.end_row();
                for (idx, monster_id, points0) in &rows {
                    if ui.selectable_label(self.road_selected == Some(*idx), idx.to_string()).clicked() {
                        self.road_selected = Some(*idx);
                        self.open_category_details("road");
                    }
                    ui.label(monster_id.to_string());
                    ui.label(points0.to_string());
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
            ui.label("No row selected");
            return;
        };
        ui.heading(format!("Edit Road points #{sel}"));
        ui.separator();
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label("Monster ID:");
            changed |= ui.add(egui::DragValue::new(&mut self.road.points[sel].monster_id)).changed();
        });
        egui::Grid::new("road_points_flags").show(ui, |ui| {
            changed |= MhfdatApp::bits_row(ui, "Flags", &mut self.road.points[sel].flags, &POINT_FLAG_NAMES);
        });
        for (i, p) in self.road.points[sel].points.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("points[{i}]:"));
                changed |= ui.add(egui::DragValue::new(p)).changed();
            });
        }
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
        MhfdatApp::section_header(ui, &format!("Felyne shop (found: {total})"), |_| {});
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
    }

    fn show_road_shop(&mut self, ui: &mut egui::Ui) {
        if self.category_details("road") {
            if ui.button("← Back to List").clicked() {
                self.back_category_list("road");
                return;
            }
            ui.heading("Fallback shop");
            ui.label("This row has no named fields yet.");
            return;
        }
        let total = self.road.shop.len();
        MhfdatApp::section_header(ui, &format!("Fallback shop (found: {total})"), |_| {});
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No fallback shop table found!");
            return;
        }
        let (start, end, total_pages) = self.road_window(total);
        MhfdatApp::entity_list(ui, "road_shop_scroll", "Row", start, end, total, |ui| {
            egui::Grid::new("road_shop_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.end_row();
                for idx in start..end {
                    if ui.selectable_label(self.road_selected == Some(idx), idx.to_string()).clicked() {
                        self.road_selected = Some(idx);
                        self.open_category_details("road");
                    }
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.road_page, total_pages);
    }

    fn show_road_skills(&mut self, ui: &mut egui::Ui) {
        if self.category_details("road") {
            self.show_road_skill_details(ui);
            return;
        }
        let total = self.road.skills.len();
        MhfdatApp::section_header(ui, &format!("Road skills (found: {total})"), |_| {});
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
        if changed {
            self.road_modified = true;
        }
    }

    fn show_road_weight(&mut self, ui: &mut egui::Ui) {
        if self.category_details("road") {
            self.show_road_weight_details(ui);
            return;
        }
        let total = self.road.weight_window.len();
        MhfdatApp::section_header(ui, &format!("Weight window (found: {total})"), |_| {});
        ui.label("Spawn weight bonus by Road skill level.");
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No weight window found!");
            return;
        }
        let (start, end, total_pages) = self.road_window(total);
        let rows: Vec<(usize, u32)> = (start..end).map(|i| (i, self.road.weight_window[i])).collect();
        MhfdatApp::entity_list(ui, "road_weight_scroll", "Weight", start, end, total, |ui| {
            egui::Grid::new("road_weight_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Weight");
                ui.end_row();
                for (i, v) in &rows {
                    if ui.selectable_label(self.road_selected == Some(*i), i.to_string()).clicked() {
                        self.road_selected = Some(*i);
                        self.open_category_details("road");
                    }
                    ui.label(v.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.road_page, total_pages);
    }

    fn show_road_weight_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("road");
            return;
        }
        let Some(sel) = self.road_selected.filter(|&i| i < self.road.weight_window.len()) else {
            ui.label("No row selected");
            return;
        };
        ui.heading(format!("Edit Weight #{sel}"));
        ui.separator();
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label("Weight:");
            changed |= ui.add(egui::DragValue::new(&mut self.road.weight_window[sel])).changed();
        });
        if changed {
            self.road_modified = true;
        }
    }
}
