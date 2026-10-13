use super::*;
use crate::core::mhfdat::preview_text_at;

#[derive(PartialEq, Clone, Copy, Default)]
pub enum CaravanTab {
    #[default]
    Voyage,
    Levels,
    Ranking,
}

/// A file-offset field (the game relocates it into a string pointer) with a read-only preview of the text it points at.
fn ptr_row(ui: &mut egui::Ui, label: &str, value: &mut u32, buffer: &[u8]) -> bool {
    ui.label(label);
    let mut changed = false;
    ui.horizontal(|ui| {
        changed = ui.add(egui::DragValue::new(value).hexadecimal(1, false, true)).changed();
        ui.label(preview_text_at(buffer, *value, 64));
    });
    ui.end_row();
    changed
}

impl MhfdatApp {
    /// Caravan tables: voyage routes (header 0x22C), the level index (0x230) and the ranking routes (0x91C).
    /// docs/reverse/caravan_routes.md. Edited in place; routes cannot be added or removed.
    pub fn show_caravan_tab(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "caravan");
        ui.horizontal(|ui| {
            for (tab, label) in [(CaravanTab::Voyage, "Voyage routes"), (CaravanTab::Levels, "Level index"), (CaravanTab::Ranking, "Ranking routes")] {
                if ui.selectable_label(self.caravan_tab == tab, label).clicked() {
                    self.caravan_tab = tab;
                    self.caravan_selected = None;
                    self.caravan_page = 0;
                    self.back_category_list("caravan");
                }
            }
        });
        ui.separator();
        match self.caravan_tab {
            CaravanTab::Voyage => self.show_voyage_routes(ui),
            CaravanTab::Levels => self.show_voyage_levels(ui),
            CaravanTab::Ranking => self.show_ranking_routes(ui),
        }
    }

    fn show_voyage_routes(&mut self, ui: &mut egui::Ui) {
        if self.category_details("caravan") {
            self.show_voyage_details(ui);
            return;
        }
        let total = self.caravan.voyage.len();
        MhfdatApp::section_header(ui, &format!("Voyage routes (found: {total})"), |_| {});
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No voyage route found!");
            return;
        }
        let (start, end, total_pages) = crate::app::list_window(&mut self.caravan_page, total);
        let rows: Vec<(usize, String, u32, u16, u16)> = (start..end).map(|idx| {
            let r = &self.caravan.voyage[idx];
            (idx, crate::core::mhfdat::decode_fixed_text(&r.title), r.fee, r.hr_to_post, r.hr_to_join)
        }).collect();
        MhfdatApp::entity_list(ui, "voyage_list_scroll", "Route", start, end, total, |ui| {
            egui::Grid::new("voyage_list").striped(true).show(ui, |ui| {
                for h in ["ID", "Title", "Fee", "HR to post", "HR to join"] {
                    ui.label(h);
                }
                ui.end_row();
                for (idx, title, fee, post, join) in &rows {
                    if ui.selectable_label(self.caravan_selected == Some(*idx), idx.to_string()).clicked() {
                        self.caravan_selected = Some(*idx);
                        self.open_category_details("caravan");
                    }
                    ui.label(title);
                    ui.label(fee.to_string());
                    ui.label(post.to_string());
                    ui.label(join.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.caravan_page, total_pages);
    }

    fn show_voyage_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("caravan");
            return;
        }
        let Some(sel) = self.caravan_selected.filter(|&i| i < self.caravan.voyage.len()) else {
            ui.label("No route selected");
            return;
        };
        ui.heading(format!("Edit Voyage route #{sel}"));
        ui.separator();
        let names = &self.item_names;
        let r = &mut self.caravan.voyage[sel];
        let mut changed = false;
        egui::Grid::new("voyage_details").num_columns(2).show(ui, |ui| {
            changed |= MhfdatApp::num_row(ui, "Fee", &mut r.fee);
            changed |= MhfdatApp::num_row(ui, "Reward money", &mut r.reward_money);
            changed |= MhfdatApp::num_row(ui, "Reward CP", &mut r.reward_cp);
            changed |= MhfdatApp::num_row(ui, &format!("Reward item {}", MhfdatApp::item_label(names, r.reward_item)), &mut r.reward_item);
            changed |= MhfdatApp::num_row(ui, "Reward item quantity", &mut r.reward_item_qty);
            changed |= MhfdatApp::num_row(ui, "Reputation", &mut r.reputation);
            changed |= MhfdatApp::num_row(ui, "HR to post", &mut r.hr_to_post);
            changed |= MhfdatApp::num_row(ui, "HR to join", &mut r.hr_to_join);
            changed |= MhfdatApp::float_row(ui, "CP bonus", &mut r.cp_bonus);
            changed |= MhfdatApp::float_row(ui, "Monster HP x", &mut r.monster_hp);
            changed |= MhfdatApp::float_row(ui, "Monster attack x", &mut r.monster_atk);
            changed |= MhfdatApp::float_row(ui, "Monster defense x", &mut r.monster_def);
            changed |= MhfdatApp::flag_row(ui, "Final trial", &mut r.final_trial);
            changed |= MhfdatApp::text_row(ui, "Title", &mut r.title, 64);
            changed |= MhfdatApp::text_row(ui, "Success text", &mut r.success, 64);
            changed |= MhfdatApp::text_row(ui, "Failure text", &mut r.failure, 64);
        });
        for (i, leg) in r.legs.iter_mut().enumerate() {
            egui::CollapsingHeader::new(format!("Leg {i}")).id_source(("voyage_leg", sel, i)).show(ui, |ui| {
                egui::Grid::new(("voyage_leg_grid", sel, i)).num_columns(2).show(ui, |ui| {
                    changed |= MhfdatApp::num_row(ui, "Quest ID", &mut leg.quest_id);
                    changed |= MhfdatApp::text_row(ui, "Map", &mut leg.map, 64);
                    changed |= MhfdatApp::text_row(ui, "Time", &mut leg.time, 64);
                    changed |= MhfdatApp::text_row(ui, "Target", &mut leg.target, 64);
                });
            });
        }
        if changed {
            self.caravan_modified = true;
        }
    }

    fn show_voyage_levels(&mut self, ui: &mut egui::Ui) {
        if self.category_details("caravan") {
            self.show_voyage_level_details(ui);
            return;
        }
        let total = self.caravan.levels.len();
        MhfdatApp::section_header(ui, &format!("Level index (found: {total})"), |_| {});
        ui.label("Each level lists up to 12 voyage route indexes; -1 = empty slot.");
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No level index found!");
            return;
        }
        let (start, end, total_pages) = crate::app::list_window(&mut self.caravan_page, total);
        MhfdatApp::entity_list(ui, "voyage_levels_scroll", "Level", start, end, total, |ui| {
            egui::Grid::new("voyage_levels_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Routes");
                ui.end_row();
                for idx in start..end {
                    let filled = self.caravan.levels[idx].routes.iter().filter(|r| **r >= 0).count();
                    if ui.selectable_label(self.caravan_selected == Some(idx), idx.to_string()).clicked() {
                        self.caravan_selected = Some(idx);
                        self.open_category_details("caravan");
                    }
                    ui.label(filled.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.caravan_page, total_pages);
    }

    fn show_voyage_level_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("caravan");
            return;
        }
        let Some(sel) = self.caravan_selected.filter(|&i| i < self.caravan.levels.len()) else {
            ui.label("No level selected");
            return;
        };
        ui.heading(format!("Edit Caravan level #{sel}"));
        ui.separator();
        let mut changed = false;
        for (slot, route) in self.caravan.levels[sel].routes.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("Route #{slot}:"));
                changed |= ui.add(egui::DragValue::new(route)).changed();
            });
        }
        if changed {
            self.caravan_modified = true;
        }
    }

    fn show_ranking_routes(&mut self, ui: &mut egui::Ui) {
        if self.category_details("caravan") {
            self.show_ranking_details(ui);
            return;
        }
        if self.caravan.ranking.is_empty() {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No ranking route table found!");
            return;
        }
        ui.horizontal_wrapped(|ui| {
            for i in 0..self.caravan.ranking.len() {
                let label = format!("Group {i} ({})", self.caravan.ranking[i].rows.len());
                if ui.selectable_label(self.caravan_group == i, label).clicked() {
                    self.caravan_group = i;
                    self.caravan_selected = None;
                    self.caravan_page = 0;
                    self.back_category_list("caravan");
                }
            }
        });
        let group = self.caravan_group.min(self.caravan.ranking.len() - 1);
        let (offset, alias_of, total) = {
            let g = &self.caravan.ranking[group];
            (g.offset, g.alias_of, g.rows.len())
        };
        MhfdatApp::section_header(ui, &format!("Ranking routes (found: {total})"), |_| {});
        if offset == 0 {
            ui.label("Unused group slot (offset 0).");
            return;
        }
        if let Some(first) = alias_of {
            ui.label(format!("Same records as group {first}: edit them there."));
            return;
        }
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No ranking routes found!");
            return;
        }
        let (start, end, total_pages) = crate::app::list_window(&mut self.caravan_page, total);
        let rows: Vec<(usize, u16, i32, i32, u16)> = (start..end).map(|idx| {
            let r = &self.caravan.ranking[group].rows[idx];
            (idx, r.route_number, r.fee, r.reward_money, r.hr_to_post)
        }).collect();
        MhfdatApp::entity_list(ui, "ranking_list_scroll", "Route", start, end, total, |ui| {
            egui::Grid::new("ranking_list").striped(true).show(ui, |ui| {
                for h in ["ID", "Route number", "Fee", "Reward money", "HR to post"] {
                    ui.label(h);
                }
                ui.end_row();
                for (idx, route_number, fee, reward, hr) in &rows {
                    if ui.selectable_label(self.caravan_selected == Some(*idx), idx.to_string()).clicked() {
                        self.caravan_selected = Some(*idx);
                        self.open_category_details("caravan");
                    }
                    ui.label(route_number.to_string());
                    ui.label(fee.to_string());
                    ui.label(reward.to_string());
                    ui.label(hr.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.caravan_page, total_pages);
    }

    fn show_ranking_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("caravan");
            return;
        }
        if self.caravan.ranking.is_empty() {
            ui.label("No ranking route table loaded.");
            return;
        }
        let group = self.caravan_group.min(self.caravan.ranking.len() - 1);
        let Some(sel) = self.caravan_selected.filter(|&i| i < self.caravan.ranking[group].rows.len()) else {
            ui.label("No route selected");
            return;
        };
        ui.heading(format!("Edit Ranking route {group}.{sel}"));
        ui.separator();
        let names = &self.item_names;
        let buffer = &self.buffer;
        let r = &mut self.caravan.ranking[group].rows[sel];
        let mut changed = false;
        egui::Grid::new("ranking_details").num_columns(2).show(ui, |ui| {
            changed |= ptr_row(ui, "Title pointer", &mut r.title_ptr, buffer);
            changed |= ptr_row(ui, "Success pointer", &mut r.success_ptr, buffer);
            changed |= ptr_row(ui, "Failure pointer", &mut r.failure_ptr, buffer);
            changed |= MhfdatApp::num_row(ui, "Route number", &mut r.route_number);
            changed |= MhfdatApp::num_row(ui, "Group number", &mut r.group_number);
            changed |= MhfdatApp::num_row(ui, "Fee", &mut r.fee);
            changed |= MhfdatApp::num_row(ui, "Reward money", &mut r.reward_money);
            changed |= MhfdatApp::num_row(ui, "Reward CP", &mut r.reward_cp);
            changed |= MhfdatApp::num_row(ui, &format!("Reward item {}", MhfdatApp::item_label(names, r.reward_item)), &mut r.reward_item);
            changed |= MhfdatApp::num_row(ui, "Reward item quantity", &mut r.reward_item_qty);
            changed |= MhfdatApp::num_row(ui, "Rank type (1 = normal, 2-4 = G)", &mut r.rank_type);
            changed |= MhfdatApp::num_row(ui, "HR to post", &mut r.hr_to_post);
            changed |= MhfdatApp::num_row(ui, "HR to join", &mut r.hr_to_join);
            changed |= MhfdatApp::float_row(ui, "CP bonus carve", &mut r.cp_bonus_carve);
            changed |= MhfdatApp::float_row(ui, "CP bonus gather", &mut r.cp_bonus_gather);
            changed |= MhfdatApp::num_row(ui, "Hunter title / reputation", &mut r.hunter_title);
            changed |= MhfdatApp::num_row(ui, "Extra mission", &mut r.extra_mission);
            changed |= MhfdatApp::float_row(ui, "Monster HP x", &mut r.monster_hp);
            changed |= MhfdatApp::float_row(ui, "Monster attack x", &mut r.monster_atk);
            changed |= MhfdatApp::float_row(ui, "Monster defense x", &mut r.monster_def);
        });
        for (i, leg) in r.legs.iter_mut().enumerate() {
            egui::CollapsingHeader::new(format!("Leg {i}")).id_source(("ranking_leg", group, sel, i)).show(ui, |ui| {
                egui::Grid::new(("ranking_leg_grid", group, sel, i)).num_columns(2).show(ui, |ui| {
                    changed |= MhfdatApp::num_row(ui, "Quest ID", &mut leg.quest_id);
                    changed |= ptr_row(ui, "Map pointer", &mut leg.map_ptr, buffer);
                    changed |= ptr_row(ui, "Time pointer", &mut leg.time_ptr, buffer);
                    changed |= ptr_row(ui, "Target pointer", &mut leg.target_ptr, buffer);
                });
            });
        }
        if changed {
            self.caravan_modified = true;
        }
    }
}
