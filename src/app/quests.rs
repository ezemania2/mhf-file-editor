use eframe::egui;
use super::{MhfdatApp, QuestTab, ViewMode};
use crate::model::mhfdat::QuestItem;

const PAGE_SIZE: usize = 15;
const HR_RANKS: &[&str] = &["HR1", "HR2", "HR3", "HR4", "HR5", "HR6"];
const GR_RANKS: &[&str] = &["G1", "G2", "G3", "G4", "G5", "G6", "G7"];

impl MhfdatApp {
    pub fn show_quests_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (tab, label) in [(QuestTab::HR, "HR Quests"), (QuestTab::GR, "GR Quests")] {
                if ui.selectable_label(self.quest_tab == tab, label).clicked() {
                    self.quest_tab = tab;
                    self.selected_quest_index = None;
                    self.selected_quest_rank = 0;
                    self.quest_page = 0;
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
            ui.label("No quests");
            return;
        }

        let total_pages = (total + PAGE_SIZE - 1) / PAGE_SIZE;
        let page = (self.quest_page as usize).min(total_pages.saturating_sub(1));
        let start = page * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(total);

        ui.label(format!("{} quests", total));

        if ui.button("Add New Quest").clicked() {
            if let Some(list) = self.get_quests_mut(is_hr, self.selected_quest_rank) {
                list.push(QuestItem::default());
            }
            self.set_quests_modified(is_hr);
        }

        MhfdatApp::pagination_controls(ui, &mut self.quest_page, total_pages);
        ui.separator();

        let view_key = self.quest_view_key(is_hr);
        let quest_data: Vec<_> = self.get_quests(is_hr, self.selected_quest_rank)
            .map(|q| q.iter().skip(start).take(end - start)
                .map(|quest| (quest.quest_id, quest.quest_number, quest.key_quest, quest.urgent_quest))
                .collect())
            .unwrap_or_default();

        egui::ScrollArea::vertical().show(ui, |ui| {
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
                        if ui.selectable_label(false, format!("{}", idx)).clicked() {
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

        if changed {
            quest.quest_id = quest_id;
            quest.quest_number = quest_number;
            quest.key_quest = key_quest;
            quest.urgent_quest = urgent_quest;
            quest.unknown = unknown;
            self.set_quests_modified(is_hr);
        }
    }
}
