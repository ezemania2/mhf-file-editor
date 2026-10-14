use super::*;
use crate::core::mhfdat::preview_text_at;

#[derive(PartialEq, Clone, Copy, Default)]
pub enum CaravanTab {
    #[default]
    Voyage,
    Levels,
    Ranking,
    GemSkills,
    GemLists,
    GemTiers,
}

/// A file-offset field (the game relocates it into a string pointer) with a read-only preview of the text it points at.
fn ptr_row(ui: &mut egui::Ui, label: &str, value: &mut u32, buffer: &[u8]) -> bool {
    ui.label(label);
    let mut changed = false;
    ui.horizontal(|ui| {
        changed = ui.add(egui::DragValue::new(value).hexadecimal(1, false, true)).changed();
        ui.add(egui::Label::new(preview_text_at(buffer, *value, 64)).truncate(true));
    });
    ui.end_row();
    changed
}

/// Number of columns a details page can use: one per `min_width` pixels, at most `max`.
fn page_columns(ui: &egui::Ui, min_width: f32, max: usize) -> usize {
    ((ui.available_width() / min_width) as usize).clamp(1, max)
}

/// Lays `sections` blocks over `columns` columns, left to right; when there are fewer columns the last one stacks the rest.
fn in_columns(ui: &mut egui::Ui, columns: usize, sections: usize, mut add: impl FnMut(&mut egui::Ui, usize)) {
    ui.columns(columns, |cols| {
        for section in 0..sections {
            add(&mut cols[section.min(columns - 1)], section);
        }
    });
}

impl MhfdatApp {
    /// Caravan tables: voyage routes (header 0x22C), the level index (0x230), the ranking routes (0x91C) and the
    /// caravan gems: gem skills (0xF0), their lists per tier (0xF4) and the tiers (0x234).
    /// docs/reverse/caravan_routes.md. Voyage routes, ranking routes (12 per group), gem skills and the skills of a gem
    /// list can grow (the table is moved to the end of the file on save); the level index and the gem tiers cannot.
    pub fn show_caravan_tab(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "caravan");
        ui.horizontal(|ui| {
            for (tab, label) in [
                (CaravanTab::Voyage, "Voyage routes"),
                (CaravanTab::Levels, "Level index"),
                (CaravanTab::Ranking, "Ranking routes"),
                (CaravanTab::GemSkills, "Gem skills"),
                (CaravanTab::GemLists, "Gem skill lists"),
                (CaravanTab::GemTiers, "Gem tiers"),
            ] {
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
            CaravanTab::GemSkills => self.show_gem_skills(ui),
            CaravanTab::GemLists => self.show_gem_lists(ui),
            CaravanTab::GemTiers => self.show_gem_tiers(ui),
        }
    }

    fn show_voyage_routes(&mut self, ui: &mut egui::Ui) {
        if self.category_details("caravan") {
            self.show_voyage_details(ui);
            return;
        }
        let total = self.caravan.voyage.len();
        let used_by_level = self.caravan.levels.iter().any(|l| l.routes.contains(&(total as i32 - 1)));
        MhfdatApp::section_header(ui, &format!("Voyage routes (found: {total})"), |ui| {
            if ui.add_enabled(total > 0, egui::Button::new("Add New")).on_disabled_hover_text("The list is empty: nothing to copy the new route from").clicked() {
                let route = self.caravan.voyage[total - 1].clone();
                self.caravan.voyage.push(route);
                self.caravan_modified = true;
                self.caravan_selected = Some(total);
                self.open_category_details("caravan");
            }
            let blocked = used_by_level.then_some("A slot of the Level index still points at this route");
            if MhfdatApp::delete_last_button(ui, total, blocked) {
                self.caravan.voyage.pop();
                self.caravan_modified = true;
            }
        });
        ui.label("Add New copies the last route. A route is offered by the game once a slot of the Level index points at its number.");
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
        egui::ScrollArea::vertical().id_source("voyage_details_scroll").show(ui, |ui| {
            // rewards | rules | texts side by side when the window is wide enough, stacked otherwise
            let columns = page_columns(ui, 420.0, 3);
            in_columns(ui, columns, 3, |ui, section| match section {
                0 => {
                    ui.strong("Rewards");
                    egui::Grid::new(("voyage_rewards", sel)).num_columns(2).show(ui, |ui| {
                        changed |= MhfdatApp::num_row(ui, "Fee", &mut r.fee);
                        changed |= MhfdatApp::num_row(ui, "Reward money", &mut r.reward_money);
                        changed |= MhfdatApp::num_row(ui, "Reward CP", &mut r.reward_cp);
                        changed |= MhfdatApp::num_row(ui, &format!("Reward item {}", MhfdatApp::item_label(names, r.reward_item)), &mut r.reward_item);
                        changed |= MhfdatApp::num_row(ui, "Reward item quantity", &mut r.reward_item_qty);
                        changed |= MhfdatApp::num_row(ui, "Reputation", &mut r.reputation);
                        changed |= MhfdatApp::float_row(ui, "CP bonus", &mut r.cp_bonus);
                    });
                }
                1 => {
                    ui.strong("Requirements and difficulty");
                    egui::Grid::new(("voyage_rules", sel)).num_columns(2).show(ui, |ui| {
                        changed |= MhfdatApp::num_row(ui, "HR to post", &mut r.hr_to_post);
                        changed |= MhfdatApp::num_row(ui, "HR to join", &mut r.hr_to_join);
                        changed |= MhfdatApp::float_row(ui, "Monster HP x", &mut r.monster_hp);
                        changed |= MhfdatApp::float_row(ui, "Monster attack x", &mut r.monster_atk);
                        changed |= MhfdatApp::float_row(ui, "Monster defense x", &mut r.monster_def);
                        changed |= MhfdatApp::flag_row(ui, "Final trial", &mut r.final_trial);
                    });
                }
                _ => {
                    ui.strong("Texts");
                    egui::Grid::new(("voyage_texts", sel)).num_columns(2).show(ui, |ui| {
                        changed |= MhfdatApp::text_row(ui, "Title", &mut r.title, 64);
                        changed |= MhfdatApp::text_row(ui, "Success text", &mut r.success, 64);
                        changed |= MhfdatApp::text_row(ui, "Failure text", &mut r.failure, 64);
                    });
                }
            });
            ui.separator();
            ui.strong("Legs");
            // the legs side by side, unused ones (quest id 0) start folded
            let columns = page_columns(ui, 400.0, 4);
            ui.columns(columns, |cols| {
                for (i, leg) in r.legs.iter_mut().enumerate() {
                    let ui = &mut cols[i % columns];
                    egui::CollapsingHeader::new(format!("Leg {i}")).id_source(("voyage_leg", sel, i)).default_open(leg.quest_id != 0).show(ui, |ui| {
                        egui::Grid::new(("voyage_leg_grid", sel, i)).num_columns(2).show(ui, |ui| {
                            changed |= MhfdatApp::num_row(ui, "Quest ID", &mut leg.quest_id);
                            changed |= MhfdatApp::text_row(ui, "Map", &mut leg.map, 64);
                            changed |= MhfdatApp::text_row(ui, "Time", &mut leg.time, 64);
                            changed |= MhfdatApp::text_row(ui, "Target", &mut leg.target, 64);
                        });
                    });
                }
            });
        });
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
        // a group holds 12 routes at most; the first row of a new group is copied from the last route of any group
        let template = self.caravan.ranking[group].rows.last().or_else(|| self.caravan.ranking.iter().rev().find_map(|g| g.rows.last())).cloned();
        let editable = offset != 0 && alias_of.is_none();
        MhfdatApp::section_header(ui, &format!("Ranking routes (found: {total})"), |ui| {
            let can_add = editable && total < 12 && template.is_some();
            if ui.add_enabled(can_add, egui::Button::new("Add New")).on_disabled_hover_text("A group holds 12 routes at most, and a group slot that is unused or shared cannot take new ones").clicked() {
                if let Some(route) = template.clone() {
                    self.caravan.ranking[group].rows.push(route);
                    self.caravan_modified = true;
                    self.caravan_selected = Some(total);
                    self.open_category_details("caravan");
                }
            }
            if MhfdatApp::delete_last_button(ui, if editable { total } else { 0 }, None) {
                self.caravan.ranking[group].rows.pop();
                self.caravan_modified = true;
            }
        });
        if offset == 0 {
            ui.label("Unused group slot (offset 0).");
            return;
        }
        if let Some(first) = alias_of {
            ui.label(format!("Same records as group {first}: edit them there."));
            return;
        }
        ui.label("A group ends on the first route without a title pointer, so every route needs one.");
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
        egui::ScrollArea::vertical().id_source("ranking_details_scroll").show(ui, |ui| {
            let columns = page_columns(ui, 420.0, 3);
            in_columns(ui, columns, 3, |ui, section| match section {
                0 => {
                    ui.strong("Rewards");
                    egui::Grid::new(("ranking_rewards", group, sel)).num_columns(2).show(ui, |ui| {
                        changed |= MhfdatApp::num_row(ui, "Fee", &mut r.fee);
                        changed |= MhfdatApp::num_row(ui, "Reward money", &mut r.reward_money);
                        changed |= MhfdatApp::num_row(ui, "Reward CP", &mut r.reward_cp);
                        changed |= MhfdatApp::num_row(ui, &format!("Reward item {}", MhfdatApp::item_label(names, r.reward_item)), &mut r.reward_item);
                        changed |= MhfdatApp::num_row(ui, "Reward item quantity", &mut r.reward_item_qty);
                        changed |= MhfdatApp::float_row(ui, "CP bonus carve", &mut r.cp_bonus_carve);
                        changed |= MhfdatApp::float_row(ui, "CP bonus gather", &mut r.cp_bonus_gather);
                        changed |= MhfdatApp::num_row(ui, "Hunter title / reputation", &mut r.hunter_title);
                    });
                }
                1 => {
                    ui.strong("Requirements and difficulty");
                    egui::Grid::new(("ranking_rules", group, sel)).num_columns(2).show(ui, |ui| {
                        changed |= MhfdatApp::num_row(ui, "Route number", &mut r.route_number);
                        changed |= MhfdatApp::num_row(ui, "Group number", &mut r.group_number);
                        changed |= MhfdatApp::num_row(ui, "Rank type (1 = normal, 2-4 = G)", &mut r.rank_type);
                        changed |= MhfdatApp::num_row(ui, "HR to post", &mut r.hr_to_post);
                        changed |= MhfdatApp::num_row(ui, "HR to join", &mut r.hr_to_join);
                        changed |= MhfdatApp::num_row(ui, "Extra mission", &mut r.extra_mission);
                        changed |= MhfdatApp::float_row(ui, "Monster HP x", &mut r.monster_hp);
                        changed |= MhfdatApp::float_row(ui, "Monster attack x", &mut r.monster_atk);
                        changed |= MhfdatApp::float_row(ui, "Monster defense x", &mut r.monster_def);
                    });
                }
                _ => {
                    ui.strong("Texts (file offsets)");
                    egui::Grid::new(("ranking_texts", group, sel)).num_columns(2).show(ui, |ui| {
                        changed |= ptr_row(ui, "Title pointer", &mut r.title_ptr, buffer);
                        changed |= ptr_row(ui, "Success pointer", &mut r.success_ptr, buffer);
                        changed |= ptr_row(ui, "Failure pointer", &mut r.failure_ptr, buffer);
                    });
                }
            });
            ui.separator();
            ui.strong("Legs");
            let columns = page_columns(ui, 400.0, 4);
            ui.columns(columns, |cols| {
                for (i, leg) in r.legs.iter_mut().enumerate() {
                    let ui = &mut cols[i % columns];
                    egui::CollapsingHeader::new(format!("Leg {i}")).id_source(("ranking_leg", group, sel, i)).default_open(leg.quest_id != 0).show(ui, |ui| {
                        egui::Grid::new(("ranking_leg_grid", group, sel, i)).num_columns(2).show(ui, |ui| {
                            changed |= MhfdatApp::num_row(ui, "Quest ID", &mut leg.quest_id);
                            changed |= ptr_row(ui, "Map pointer", &mut leg.map_ptr, buffer);
                            changed |= ptr_row(ui, "Time pointer", &mut leg.time_ptr, buffer);
                            changed |= ptr_row(ui, "Target pointer", &mut leg.target_ptr, buffer);
                        });
                    });
                }
            });
        });
        if changed {
            self.caravan_modified = true;
        }
    }
}

impl MhfdatApp {
    fn show_gem_skills(&mut self, ui: &mut egui::Ui) {
        if self.category_details("caravan") {
            self.show_gem_skill_details(ui);
            return;
        }
        let total = self.caravan.gem_skills.len();
        MhfdatApp::section_header(ui, &format!("Gem skills (found: {total})"), |ui| {
            if ui.add_enabled(total > 0, egui::Button::new("Add New")).on_disabled_hover_text("The list is empty: nothing to copy the new gem skill from").clicked() {
                let gem = self.caravan.gem_skills[total - 1].clone();
                self.caravan.gem_skills.push(gem);
                self.caravan_modified = true;
                self.caravan_selected = Some(total);
                self.open_category_details("caravan");
            }
            if MhfdatApp::delete_last_button(ui, total, None) {
                self.caravan.gem_skills.pop();
                self.caravan_modified = true;
            }
        });
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search:");
            if ui.text_edit_singleline(&mut self.gem_skills_search).changed() {
                self.caravan_page = 0;
            }
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No gem skill found!");
            return;
        }
        let query = self.gem_skills_search.trim().to_string();
        let filtered: Vec<usize> = (0..total)
            .filter(|&i| query.is_empty() || self.caravan.gem_skills[i].skill_id.to_string().contains(&query))
            .collect();
        let (start, end, total_pages) = crate::app::list_window(&mut self.caravan_page, filtered.len());
        let rows: Vec<(usize, i16, i16)> = filtered[start..end].iter().map(|&i| {
            let g = &self.caravan.gem_skills[i];
            (i, g.skill_id, g.gem_level & 0xFF)
        }).collect();
        MhfdatApp::entity_list(ui, "gem_skills_scroll", "Gem skill", start, end, filtered.len(), |ui| {
            egui::Grid::new("gem_skills_list").striped(true).show(ui, |ui| {
                for h in ["ID", "Skill ID", "Gem level"] {
                    ui.label(h);
                }
                ui.end_row();
                for (idx, skill_id, level) in &rows {
                    if ui.selectable_label(self.caravan_selected == Some(*idx), idx.to_string()).clicked() {
                        self.caravan_selected = Some(*idx);
                        self.open_category_details("caravan");
                    }
                    ui.label(skill_id.to_string());
                    ui.label(level.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.caravan_page, total_pages);
    }

    fn show_gem_skill_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("caravan");
            return;
        }
        let Some(sel) = self.caravan_selected.filter(|&i| i < self.caravan.gem_skills.len()) else {
            ui.label("No gem skill selected");
            return;
        };
        ui.heading(format!("Edit Gem skill #{sel}"));
        ui.separator();
        let g = &mut self.caravan.gem_skills[sel];
        let mut changed = false;
        egui::ScrollArea::vertical().id_source("gem_skill_details_scroll").show(ui, |ui| {
            egui::Grid::new("gem_skill_details").num_columns(2).show(ui, |ui| {
                // -1 ends the list in the file, so it cannot be typed here
                ui.label("Skill ID:");
                changed |= ui.add(egui::DragValue::new(&mut g.skill_id).clamp_range(0..=i16::MAX)).changed();
                ui.end_row();
                // the game reads the low byte only; the high byte stays as it is
                ui.label("Gem level:");
                let mut level = (g.gem_level & 0xFF) as u8;
                if ui.add(egui::DragValue::new(&mut level)).changed() {
                    g.gem_level = ((g.gem_level as u16 & 0xFF00) | level as u16) as i16;
                    changed = true;
                }
                ui.end_row();
                for (k, threshold) in g.thresholds.iter_mut().enumerate() {
                    changed |= MhfdatApp::num_row(ui, &format!("Counter {k} threshold"), threshold);
                }
            });
        });
        if changed {
            self.caravan_modified = true;
        }
    }

    fn show_gem_lists(&mut self, ui: &mut egui::Ui) {
        if self.category_details("caravan") {
            self.show_gem_list_details(ui);
            return;
        }
        let total = self.caravan.gem_lists.len();
        MhfdatApp::section_header(ui, &format!("Gem skill lists (found: {total})"), |_| {});
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No gem skill list found!");
            return;
        }
        let array = crate::core::mhfdat::read_header_pointer(&self.buffer, crate::model::mhfdat_pointers::HUNTER_PEARL_SKILL_LISTS_PTR).unwrap_or(0);
        let (start, end, total_pages) = crate::app::list_window(&mut self.caravan_page, total);
        let rows: Vec<(usize, u32, String)> = (start..end).map(|idx| {
            let b = &self.caravan.gem_lists[idx];
            let count = match b.alias_of {
                _ if b.offset == 0 => "-".to_string(),
                Some(first) => format!("same as {first}"),
                None => b.rows.len().to_string(),
            };
            (idx, b.offset, count)
        }).collect();
        MhfdatApp::entity_list(ui, "gem_lists_scroll", "Gem skill list", start, end, total, |ui| {
            egui::Grid::new("gem_lists_list").striped(true).show(ui, |ui| {
                for h in ["ID", "Pointer at", "List at", "Skills"] {
                    ui.label(h);
                }
                ui.end_row();
                for (idx, offset, count) in &rows {
                    if ui.selectable_label(self.caravan_selected == Some(*idx), idx.to_string()).clicked() {
                        self.caravan_selected = Some(*idx);
                        self.open_category_details("caravan");
                    }
                    ui.label(format!("0x{:X}", array as usize + idx * 4));
                    ui.label(format!("0x{:X}", offset));
                    ui.label(count);
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.caravan_page, total_pages);
    }

    fn show_gem_list_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("caravan");
            return;
        }
        let Some(sel) = self.caravan_selected.filter(|&i| i < self.caravan.gem_lists.len()) else {
            ui.label("No gem skill list selected");
            return;
        };
        ui.heading(format!("Edit Gem skill list #{sel}"));
        ui.separator();
        let (offset, alias_of) = {
            let b = &self.caravan.gem_lists[sel];
            (b.offset, b.alias_of)
        };
        if offset == 0 {
            ui.label("Null pointer: nothing to edit here.");
            return;
        }
        ui.label(format!("List at file offset 0x{offset:X}. It ends on a skill id of -1, which is not shown."));
        if let Some(first) = alias_of {
            ui.label(format!("Same list as tier {first}: edit it there."));
            return;
        }
        let mut changed = false;
        // a new skill: the first gem skill this list does not carry yet
        let next = self.caravan.gem_skills.iter().map(|g| g.skill_id)
            .find(|id| *id >= 0 && !self.caravan.gem_lists[sel].rows.contains(id)).unwrap_or(0);
        ui.horizontal(|ui| {
            if ui.button("Add skill").clicked() {
                self.caravan.gem_lists[sel].rows.push(next);
                if self.gem_list_later_tiers {
                    // a later list that is another tier's list is shared with it and already has the skill
                    for t in sel + 1..self.caravan.gem_lists.len() {
                        let b = &mut self.caravan.gem_lists[t];
                        if b.offset != 0 && b.alias_of.is_none() && !b.rows.contains(&next) {
                            b.rows.push(next);
                        }
                    }
                }
                changed = true;
            }
            ui.checkbox(&mut self.gem_list_later_tiers, "also add it to the later tiers");
            if MhfdatApp::delete_last_button(ui, self.caravan.gem_lists[sel].rows.len(), None) {
                self.caravan.gem_lists[sel].rows.pop();
                changed = true;
            }
        });
        egui::ScrollArea::vertical().id_source("gem_list_details_scroll").show(ui, |ui| {
            egui::Grid::new(("gem_list_details", sel)).num_columns(2).show(ui, |ui| {
                for (k, id) in self.caravan.gem_lists[sel].rows.iter_mut().enumerate() {
                    ui.label(format!("Skill #{k}:"));
                    changed |= ui.add(egui::DragValue::new(id).clamp_range(0..=i16::MAX)).changed();
                    ui.end_row();
                }
            });
        });
        if changed {
            self.caravan_modified = true;
        }
    }

    /// The tiers the game can reach (CARAVAN_TIER_COUNT); the row after them only carries the +0x0F byte of the last one.
    fn gem_tier_count(&self) -> usize {
        self.caravan.gem_tiers.len().saturating_sub(1)
    }

    fn show_gem_tiers(&mut self, ui: &mut egui::Ui) {
        if self.category_details("caravan") {
            self.show_gem_tier_details(ui);
            return;
        }
        let total = self.gem_tier_count();
        MhfdatApp::section_header(ui, &format!("Gem tiers (found: {total})"), |_| {});
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No gem tier found!");
            return;
        }
        let (start, end, total_pages) = crate::app::list_window(&mut self.caravan_page, total);
        let rows: Vec<(usize, u16, u8)> = (start..end).map(|idx| {
            let r = &self.caravan.gem_tiers[idx];
            (idx, r.pts_cap, r.percent)
        }).collect();
        MhfdatApp::entity_list(ui, "gem_tiers_scroll", "Gem tier", start, end, total, |ui| {
            egui::Grid::new("gem_tiers_list").striped(true).show(ui, |ui| {
                for h in ["ID", "Point cap", "Percent"] {
                    ui.label(h);
                }
                ui.end_row();
                for (idx, cap, percent) in &rows {
                    if ui.selectable_label(self.caravan_selected == Some(*idx), idx.to_string()).clicked() {
                        self.caravan_selected = Some(*idx);
                        self.open_category_details("caravan");
                    }
                    ui.label(cap.to_string());
                    ui.label(percent.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.caravan_page, total_pages);
    }

    fn show_gem_tier_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("caravan");
            return;
        }
        let Some(sel) = self.caravan_selected.filter(|&i| i < self.gem_tier_count()) else {
            ui.label("No gem tier selected");
            return;
        };
        ui.heading(format!("Edit Gem tier #{sel}"));
        ui.separator();
        let mut changed = false;
        let mut unlock = self.caravan.tier_unlock(sel);
        egui::Grid::new("gem_tier_details").num_columns(2).show(ui, |ui| {
            let r = &mut self.caravan.gem_tiers[sel];
            changed |= MhfdatApp::num_row(ui, "Point cap (+0x04)", &mut r.pts_cap);
            changed |= MhfdatApp::num_row(ui, "Percent (+0x06)", &mut r.percent);
            if MhfdatApp::num_row(ui, "Unlock requirement (+0x0F)", &mut unlock) {
                changed = true;
            }
        });
        // +0x0F of tier t is the last byte of the next 8-byte row
        self.caravan.set_tier_unlock(sel, unlock);
        if changed {
            self.caravan_modified = true;
        }
    }
}
