use eframe::egui;
use super::{MhfdatApp, QuestTab, ViewMode};
use crate::model::mhfdat::QuestItem;

const HR_RANKS: &[&str] = &["HR1", "HR2", "HR3", "HR4", "HR5", "HR6"];
const GR_RANKS: &[&str] = &["G1", "G2", "G3", "G4", "G5", "G6", "G7"];

impl MhfdatApp {
    pub fn show_quests_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (tab, label) in [
                (QuestTab::HR, "HR Quests"),
                (QuestTab::GR, "GR Quests"),
                (QuestTab::GRMenus, "GR menus G1-G10"),
                (QuestTab::Beginner, "Beginner quests"),
            ] {
                if ui.selectable_label(self.quest_tab == tab, label).clicked() {
                    self.quest_tab = tab;
                    self.selected_quest_index = None;
                    self.selected_quest_rank = 0;
                    self.quest_page = 0;
                    self.back_category_list("gr_menu");
                    self.back_category_list("beginner");
                }
            }
            ui.separator();
            if ui.button("Export HR to JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .set_filename("hr_quests.json")
                    .add_filter("JSON", &["json"])
                    .show_save_single_file()
                {
                    if let Ok(json) = serde_json::to_string_pretty(&self.hr_quests) {
                        let _ = std::fs::write(path, json);
                    }
                }
            }
            if ui.button("Import HR from JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .show_open_single_file()
                {
                    if let Ok(data) = std::fs::read_to_string(&path) {
                        if let Ok(imported) = serde_json::from_str::<crate::model::mhfdat::HRQuests>(&data) {
                            self.hr_quests = imported;
                            self.hr_quests_modified = true;
                        }
                    }
                }
            }
            if ui.button("Export GR to JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .set_filename("gr_quests.json")
                    .add_filter("JSON", &["json"])
                    .show_save_single_file()
                {
                    if let Ok(json) = serde_json::to_string_pretty(&self.gr_quests) {
                        let _ = std::fs::write(path, json);
                    }
                }
            }
            if ui.button("Import GR from JSON").clicked() {
                if let Ok(Some(path)) = native_dialog::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .show_open_single_file()
                {
                    if let Ok(data) = std::fs::read_to_string(&path) {
                        if let Ok(imported) = serde_json::from_str::<crate::model::mhfdat::GRQuests>(&data) {
                            self.gr_quests = imported;
                            self.gr_quests_modified = true;
                        }
                    }
                }
            }
        });
        ui.separator();

        self.show_table_problems(ui, "quests");
        match self.quest_tab {
            QuestTab::GRMenus => return self.show_gr_menus(ui),
            QuestTab::Beginner => return self.show_beginner_quests(ui),
            QuestTab::HR | QuestTab::GR => {}
        }

        let is_hr = self.quest_tab == QuestTab::HR;
        self.show_quest_rank_selector(ui, if is_hr { HR_RANKS } else { GR_RANKS });

        let view_key = self.quest_view_key(is_hr);
        match self.view_mode.get(&view_key).cloned().unwrap_or(ViewMode::List) {
            ViewMode::List => self.show_quest_list(ui, is_hr),
            ViewMode::Details => self.show_quest_details(ui, is_hr),
        }
    }

    fn quest_view_key(&self, is_hr: bool) -> String {
        format!("{}_quests_{}", if is_hr { "hr" } else { "gr" }, self.selected_quest_rank)
    }

    fn get_quests(&self, is_hr: bool, rank: usize) -> Option<&Vec<QuestItem>> {
        if is_hr {
            match rank {
                0 => Some(&self.hr_quests.one_star),
                1 => Some(&self.hr_quests.two_stars),
                2 => Some(&self.hr_quests.three_stars),
                3 => Some(&self.hr_quests.four_stars),
                4 => Some(&self.hr_quests.five_stars),
                5 => Some(&self.hr_quests.six_stars),
                _ => None,
            }
        } else {
            match rank {
                0 => Some(&self.gr_quests.g1),
                1 => Some(&self.gr_quests.g2),
                2 => Some(&self.gr_quests.g3),
                3 => Some(&self.gr_quests.g4),
                4 => Some(&self.gr_quests.g5),
                5 => Some(&self.gr_quests.g6),
                6 => Some(&self.gr_quests.g7),
                _ => None,
            }
        }
    }

    fn get_quests_mut(&mut self, is_hr: bool, rank: usize) -> Option<&mut Vec<QuestItem>> {
        if is_hr {
            match rank {
                0 => Some(&mut self.hr_quests.one_star),
                1 => Some(&mut self.hr_quests.two_stars),
                2 => Some(&mut self.hr_quests.three_stars),
                3 => Some(&mut self.hr_quests.four_stars),
                4 => Some(&mut self.hr_quests.five_stars),
                5 => Some(&mut self.hr_quests.six_stars),
                _ => None,
            }
        } else {
            match rank {
                0 => Some(&mut self.gr_quests.g1),
                1 => Some(&mut self.gr_quests.g2),
                2 => Some(&mut self.gr_quests.g3),
                3 => Some(&mut self.gr_quests.g4),
                4 => Some(&mut self.gr_quests.g5),
                5 => Some(&mut self.gr_quests.g6),
                6 => Some(&mut self.gr_quests.g7),
                _ => None,
            }
        }
    }

    fn set_quests_modified(&mut self, is_hr: bool) {
        if is_hr {
            self.hr_quests_modified = true;
        } else {
            self.gr_quests_modified = true;
        }
    }

    fn show_quest_rank_selector(&mut self, ui: &mut egui::Ui, ranks: &[&str]) {
        ui.horizontal(|ui| {
            for (i, name) in ranks.iter().enumerate() {
                if ui.selectable_label(self.selected_quest_rank == i, *name).clicked() {
                    self.selected_quest_rank = i;
                    self.selected_quest_index = None;
                    self.quest_page = 0;
                }
            }
        });
        ui.separator();
    }

    fn show_quest_list(&mut self, ui: &mut egui::Ui, is_hr: bool) {
        let total = self.get_quests(is_hr, self.selected_quest_rank)
            .map(|q| q.len())
            .unwrap_or(0);
        
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No quests found!");
            return;
        }

        let (start, end, total_pages) = crate::app::list_window(&mut self.quest_page, total);
        let rank_name = if is_hr { HR_RANKS } else { GR_RANKS };
        let rank_label = rank_name.get(self.selected_quest_rank).copied().unwrap_or("?");
        MhfdatApp::section_header(ui, &format!("{rank_label} Quests (found: {total})"), |ui| {
            if ui.button("Add New").clicked() {
                if let Some(list) = self.get_quests_mut(is_hr, self.selected_quest_rank) {
                    list.push(QuestItem::default());
                }
                self.set_quests_modified(is_hr);
            }
        });

        let view_key = self.quest_view_key(is_hr);
        let quest_data: Vec<_> = self.get_quests(is_hr, self.selected_quest_rank)
            .map(|q| q.iter().skip(start).take(end - start)
                .map(|quest| (quest.quest_id, quest.quest_number, quest.key_quest, quest.urgent_quest))
                .collect())
            .unwrap_or_default();

        MhfdatApp::entity_list(ui, &format!("{}_quest_scroll", if is_hr { "hr" } else { "gr" }), "Quest", start, end, total, |ui| {
            egui::Grid::new(format!("{}_quest_grid", if is_hr { "hr" } else { "gr" }))
                .striped(true)
                .num_columns(5)
                .show(ui, |ui| {
                    ui.label("ID");
                    ui.label("Quest ID");
                    ui.label("Quest Num");
                    ui.label("Key");
                    ui.label("Urgent");
                    ui.end_row();

                    for (i, (quest_id, quest_number, key_quest, urgent_quest)) in quest_data.iter().enumerate() {
                        let idx = start + i;
                        if ui.selectable_label(self.selected_quest_index == Some(idx), format!("{}", idx)).clicked() {
                            self.selected_quest_index = Some(idx);
                            self.view_mode.insert(view_key.clone(), ViewMode::Details);
                        }
                        ui.label(format!("{}", quest_id));
                        ui.label(format!("{}", quest_number));
                        ui.label(if *key_quest != 0 { "Yes" } else { "No" });
                        ui.label(if *urgent_quest != 0 { "Yes" } else { "No" });
                        ui.end_row();
                    }
                });
        });
        MhfdatApp::pagination_controls(ui, &mut self.quest_page, total_pages);
    }

    fn show_quest_details(&mut self, ui: &mut egui::Ui, is_hr: bool) {
        let view_key = self.quest_view_key(is_hr);

        if ui.button("← Back to List").clicked() {
            self.view_mode.insert(view_key, ViewMode::List);
            self.selected_quest_index = None;
            return;
        }
        ui.separator();

        let idx = match self.selected_quest_index {
            Some(i) => i,
            None => return,
        };

        let quest = match self.get_quests_mut(is_hr, self.selected_quest_rank).and_then(|q| q.get_mut(idx)) {
            Some(q) => q,
            None => return,
        };

        let mut quest_id = quest.quest_id;
        let mut quest_number = quest.quest_number;
        let mut key_quest = quest.key_quest;
        let mut urgent_quest = quest.urgent_quest;
        let mut changed = false;

        egui::Grid::new("quest_details_grid").show(ui, |ui| {
            ui.label("Quest ID:");
            changed |= ui.add(egui::DragValue::new(&mut quest_id)).changed();
            ui.end_row();

            ui.label("Quest Number:");
            changed |= ui.add(egui::DragValue::new(&mut quest_number)).changed();
            ui.end_row();

            ui.label("Key Quest:");
            changed |= ui.add(egui::DragValue::new(&mut key_quest).clamp_range(0..=1)).changed();
            ui.end_row();

            ui.label("Urgent Quest:");
            changed |= ui.add(egui::DragValue::new(&mut urgent_quest).clamp_range(0..=1)).changed();
            ui.end_row();
        });

        if changed {
            quest.quest_id = quest_id;
            quest.quest_number = quest_number;
            quest.key_quest = key_quest;
            quest.urgent_quest = urgent_quest;
            self.set_quests_modified(is_hr);
        }
    }
}

impl MhfdatApp {
    /// The GR quest menus G1-G6 and G8-G10 (header slots 0x6C8..0x6DC and 0x740..0x748), each an 8-byte QuestItem
    /// list that is rewritten where it is: no quest can be added or removed, so nothing after the list is touched.
    fn show_gr_menus(&mut self, ui: &mut egui::Ui) {
        if self.category_details("gr_menu") {
            self.show_gr_menu_details(ui);
            return;
        }
        if self.gr_menus.is_empty() {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No GR menu could be read!");
            return;
        }
        ui.horizontal_wrapped(|ui| {
            for i in 0..self.gr_menus.len() {
                let label = format!("{} ({})", self.gr_menus[i].name, self.gr_menus[i].rows.len());
                if ui.selectable_label(self.selected_gr_menu == i, label).clicked() {
                    self.selected_gr_menu = i;
                    self.gr_menu_page = 0;
                    self.back_category_list("gr_menu");
                }
            }
        });
        let sel = self.selected_gr_menu.min(self.gr_menus.len() - 1);
        let (offset, alias_of, total) = {
            let b = &self.gr_menus[sel];
            (b.offset, b.alias_of, b.rows.len())
        };
        MhfdatApp::section_header(ui, &format!("{} (found: {total})", self.gr_menus[sel].name), |_| {});
        if offset == 0 {
            ui.label("This menu has no quest list in this file.");
            return;
        }
        if let Some(first) = alias_of {
            ui.label(format!("Same list as {}: edit it there.", self.gr_menus[first].name));
            return;
        }
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: Empty list!");
            return;
        }
        let (start, end, total_pages) = crate::app::list_window(&mut self.gr_menu_page, total);
        let rows: Vec<(usize, u16, u16, u8, u8)> = (start..end).map(|idx| {
            let q = &self.gr_menus[sel].rows[idx];
            (idx, q.quest_id, q.quest_number, q.key_quest, q.urgent_quest)
        }).collect();
        MhfdatApp::entity_list(ui, "gr_menu_scroll", "Quest", start, end, total, |ui| {
            egui::Grid::new("gr_menu_grid").striped(true).show(ui, |ui| {
                for h in ["ID", "Quest ID", "Quest number", "Key", "Urgent"] {
                    ui.label(h);
                }
                ui.end_row();
                for (idx, quest_id, quest_number, key_quest, urgent_quest) in &rows {
                    if ui.selectable_label(self.selected_quest_index == Some(*idx), idx.to_string()).clicked() {
                        self.selected_quest_index = Some(*idx);
                        self.open_category_details("gr_menu");
                    }
                    ui.label(quest_id.to_string());
                    ui.label(quest_number.to_string());
                    ui.label(if *key_quest != 0 { "Yes" } else { "No" });
                    ui.label(if *urgent_quest != 0 { "Yes" } else { "No" });
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.gr_menu_page, total_pages);
    }

    fn show_gr_menu_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("gr_menu");
            return;
        }
        if self.gr_menus.is_empty() {
            ui.label("No GR menu could be read.");
            return;
        }
        let menu = self.selected_gr_menu.min(self.gr_menus.len() - 1);
        let Some(idx) = self.selected_quest_index.filter(|&i| i < self.gr_menus[menu].rows.len()) else {
            ui.label("No quest selected");
            return;
        };
        ui.heading(format!("Edit {} quest #{idx}", self.gr_menus[menu].name));
        ui.separator();
        let q = &mut self.gr_menus[menu].rows[idx];
        let mut changed = false;
        ui.horizontal(|ui| { ui.label("Quest ID:"); packed_drag!(ui, q.quest_id, changed); });
        ui.horizontal(|ui| { ui.label("Quest number:"); packed_drag!(ui, q.quest_number, changed); });
        ui.horizontal(|ui| {
            ui.label("Key quest:");
            let mut key = q.key_quest != 0;
            if ui.checkbox(&mut key, "").changed() { q.key_quest = u8::from(key); changed = true; }
        });
        ui.horizontal(|ui| {
            ui.label("Urgent:");
            let mut urgent = q.urgent_quest != 0;
            if ui.checkbox(&mut urgent, "").changed() { q.urgent_quest = u8::from(urgent); changed = true; }
        });
        if changed { self.gr_menus_modified = true; }
    }

    fn show_beginner_quests(&mut self, ui: &mut egui::Ui) {
        if self.category_details("beginner") {
            if ui.button("← Back to List").clicked() {
                self.back_category_list("beginner");
                return;
            }
            let Some(idx) = self.selected_quest_index.filter(|&i| i < self.beginner_quests.len()) else {
                ui.label("No quest selected");
                return;
            };
            ui.heading(format!("Edit Beginner quest #{idx}"));
            ui.separator();
            let mut changed = false;
            ui.horizontal(|ui| {
                ui.label("Quest ID:");
                changed |= ui.add(egui::DragValue::new(&mut self.beginner_quests[idx])).changed();
            });
            if changed { self.beginner_quests_modified = true; }
            return;
        }
        let total = self.beginner_quests.len();
        MhfdatApp::section_header(ui, &format!("Beginner quests (found: {total})"), |_| {});
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No beginner quest ids found!");
            return;
        }
        let (start, end, total_pages) = crate::app::list_window(&mut self.quest_page, total);
        let rows: Vec<(usize, u16)> = (start..end).map(|idx| (idx, self.beginner_quests[idx])).collect();
        MhfdatApp::entity_list(ui, "beginner_quests_scroll", "Quest", start, end, total, |ui| {
            egui::Grid::new("beginner_quests_grid").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Quest ID");
                ui.end_row();
                for (idx, quest_id) in &rows {
                    if ui.selectable_label(self.selected_quest_index == Some(*idx), idx.to_string()).clicked() {
                        self.selected_quest_index = Some(*idx);
                        self.open_category_details("beginner");
                    }
                    ui.label(quest_id.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.quest_page, total_pages);
    }
}
