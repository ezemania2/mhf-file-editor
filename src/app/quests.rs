use eframe::egui;
use super::{MhfdatApp, QuestTab, ViewMode};
use crate::model::mhfdat::QuestItem;

const HR_RANKS: &[&str] = &["HR1", "HR2", "HR3", "HR4", "HR5", "HR6"];
const GR_RANKS: &[&str] = &["G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8", "G9", "G10"];

impl MhfdatApp {
    pub fn show_quests_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (tab, label) in [
                (QuestTab::HR, "HR Quests"),
                (QuestTab::GR, "GR Quests"),
                (QuestTab::Beginner, "Beginner quests"),
            ] {
                if ui.selectable_label(self.quest_tab == tab, label).clicked() {
                    self.quest_tab = tab;
                    self.selected_quest_index = None;
                    self.selected_quest_rank = 0;
                    self.quest_page = 0;
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
                    if let Ok(json) = serde_json::to_string_pretty(&self.gr_quests_json()) {
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
                            self.import_gr_quests_json(imported);
                        }
                    }
                }
            }
        });
        ui.separator();

        self.show_table_problems(ui, "quests");
        if self.quest_tab == QuestTab::Beginner {
            self.show_beginner_quests(ui);
            return;
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

    /// The GR lists as the JSON of the old GR tab (G1 to G7, plus G8 to G10).
    fn gr_quests_json(&self) -> crate::model::mhfdat::GRQuests {
        let rows = |i: usize| self.gr_rows(i).cloned().unwrap_or_default();
        crate::model::mhfdat::GRQuests {
            g1: rows(0), g2: rows(1), g3: rows(2), g4: rows(3), g5: rows(4), g6: rows(5), g7: rows(6),
            g8: rows(7), g9: rows(8), g10: rows(9),
        }
    }

    fn import_gr_quests_json(&mut self, imported: crate::model::mhfdat::GRQuests) {
        let lists = [imported.g1, imported.g2, imported.g3, imported.g4, imported.g5, imported.g6, imported.g7, imported.g8, imported.g9, imported.g10];
        for (i, list) in lists.into_iter().enumerate() {
            // an empty list in the file leaves the list of the game alone; a list that has no place in this file is skipped
            if list.is_empty() {
                continue;
            }
            if let Some(rows) = self.gr_rows_mut(i) {
                *rows = list;
                self.gr_menus_modified = true;
            }
        }
    }

    /// Index of the block that owns the quests of GR list `rank` (a list that shares its pointer with an earlier one is
    /// edited there).
    fn gr_owner(&self, rank: usize) -> Option<usize> {
        let b = self.gr_menus.get(rank)?;
        if b.offset == 0 {
            return None;
        }
        Some(b.alias_of.unwrap_or(rank))
    }

    fn gr_rows(&self, rank: usize) -> Option<&Vec<QuestItem>> {
        self.gr_owner(rank).map(|i| &self.gr_menus[i].rows)
    }

    fn gr_rows_mut(&mut self, rank: usize) -> Option<&mut Vec<QuestItem>> {
        let i = self.gr_owner(rank)?;
        Some(&mut self.gr_menus[i].rows)
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
            self.gr_rows(rank)
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
            self.gr_rows_mut(rank)
        }
    }

    fn set_quests_modified(&mut self, is_hr: bool) {
        if is_hr {
            self.hr_quests_modified = true;
        } else {
            self.gr_menus_modified = true;
        }
    }

    fn show_quest_rank_selector(&mut self, ui: &mut egui::Ui, ranks: &[&str]) {
        ui.horizontal_wrapped(|ui| {
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
        let rank = self.selected_quest_rank;
        let rank_name = if is_hr { HR_RANKS } else { GR_RANKS };
        let rank_label = rank_name.get(rank).copied().unwrap_or("?");
        let usable = self.get_quests(is_hr, rank).is_some();
        let total = self.get_quests(is_hr, rank).map(|q| q.len()).unwrap_or(0);

        let (start, end, total_pages) = crate::app::list_window(&mut self.quest_page, total);
        MhfdatApp::section_header(ui, &format!("{rank_label} Quests (found: {total})"), |ui| {
            if ui.add_enabled(usable, egui::Button::new("Add New")).on_disabled_hover_text("This list has no place in this file").clicked() {
                if let Some(list) = self.get_quests_mut(is_hr, rank) {
                    list.push(QuestItem::default());
                    let last = list.len() - 1;
                    self.selected_quest_index = Some(last);
                    self.view_mode.insert(self.quest_view_key(is_hr), ViewMode::Details);
                }
                self.set_quests_modified(is_hr);
            }
        });
        if !usable {
            ui.colored_label(egui::Color32::YELLOW, "This list is not in the file (null or unreadable pointer).");
            return;
        }
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "This list is empty: Add New creates its first quest.");
            return;
        }

        let view_key = self.quest_view_key(is_hr);
        let quest_data: Vec<_> = self.get_quests(is_hr, rank)
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
        let rank = self.selected_quest_rank;
        let len = self.get_quests(is_hr, rank).map(|q| q.len()).unwrap_or(0);

        // only the last quest of a list can go: the others would shift
        let is_last = idx + 1 == len;
        if ui
            .add_enabled(is_last, egui::Button::new("Delete this quest"))
            .on_disabled_hover_text("Only the last quest of a list can be deleted: the others would move")
            .clicked()
        {
            if let Some(list) = self.get_quests_mut(is_hr, rank) {
                list.pop();
            }
            self.set_quests_modified(is_hr);
            self.view_mode.insert(view_key, ViewMode::List);
            self.selected_quest_index = None;
            return;
        }

        let quest = match self.get_quests_mut(is_hr, rank).and_then(|q| q.get_mut(idx)) {
            Some(q) => q,
            None => return,
        };

        let mut quest_id = quest.quest_id;
        let mut quest_number = quest.quest_number;
        let mut key_quest = quest.key_quest;
        let mut urgent_quest = quest.urgent_quest;
        let mut unknown = quest.unknown;
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

            ui.label("Unknown:");
            changed |= ui.add(egui::DragValue::new(&mut unknown)).changed();
            ui.end_row();
        });
        if quest_id == 0 {
            ui.colored_label(egui::Color32::YELLOW, "A quest id of 0 ends the list: set an id before saving.");
        }

        if changed {
            quest.quest_id = quest_id;
            quest.quest_number = quest_number;
            quest.key_quest = key_quest;
            quest.urgent_quest = urgent_quest;
            quest.unknown = unknown;
            self.set_quests_modified(is_hr);
        }
    }

    /// Beginner quest ids (header 0x1B8, u16 each, count = important_nums + 0x3C). Quests can be added at the end.
    fn show_beginner_quests(&mut self, ui: &mut egui::Ui) {
        let total = self.beginner_quests.len();
        MhfdatApp::section_header(ui, &format!("Beginner quests (found: {total})"), |ui| {
            if ui.button("Add New").clicked() {
                let id = self.beginner_quests.last().copied().unwrap_or(0);
                self.beginner_quests.push(id);
                self.beginner_quests_modified = true;
            }
            if ui.add_enabled(total > 0, egui::Button::new("Delete last")).clicked() {
                self.beginner_quests.pop();
                self.beginner_quests_modified = true;
            }
        });
        if self.beginner_quests.is_empty() {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No beginner quest ids found!");
            return;
        }
        let mut changed = false;
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("beginner_quests_grid").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Quest ID");
                ui.end_row();
                for (idx, id) in self.beginner_quests.iter_mut().enumerate() {
                    ui.label(idx.to_string());
                    changed |= ui.add(egui::DragValue::new(id)).changed();
                    ui.end_row();
                }
            });
        });
        if changed {
            self.beginner_quests_modified = true;
        }
    }
}
