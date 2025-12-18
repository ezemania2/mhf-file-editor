use eframe::egui;
use super::{MhfdatApp, QuestTab, ViewMode};
use crate::model::mhfdat::QuestItem;

impl MhfdatApp {
    pub fn show_quests_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.selectable_label(self.quest_tab == QuestTab::HR, "HR Quests").clicked() {
                self.quest_tab = QuestTab::HR;
                self.selected_quest_index = None;
                self.selected_quest_rank = 0;
            }
            if ui.selectable_label(self.quest_tab == QuestTab::GR, "GR Quests").clicked() {
                self.quest_tab = QuestTab::GR;
                self.selected_quest_index = None;
                self.selected_quest_rank = 0;
            }
        });
        ui.separator();

        match self.quest_tab {
            QuestTab::HR => self.show_hr_quests(ui),
            QuestTab::GR => self.show_gr_quests(ui),
        }
    }

    fn show_hr_quests(&mut self, ui: &mut egui::Ui) {
        let ranks = ["HR1", "HR2", "HR3", "HR4", "HR5", "HR6"];
        
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

        let view_key = format!("hr_quests_{}", self.selected_quest_rank);
        let view_mode = self.view_mode.get(&view_key).cloned().unwrap_or(ViewMode::List);

        match view_mode {
            ViewMode::List => self.show_hr_quest_list(ui),
            ViewMode::Details => self.show_hr_quest_details(ui),
        }
    }

    fn show_hr_quest_list(&mut self, ui: &mut egui::Ui) {
        let quests = match self.selected_quest_rank {
            0 => &self.hr_quests.one_star,
            1 => &self.hr_quests.two_stars,
            2 => &self.hr_quests.three_stars,
            3 => &self.hr_quests.four_stars,
            4 => &self.hr_quests.five_stars,
            5 => &self.hr_quests.six_stars,
            _ => return,
        };

        let total = quests.len();
        let page_size = 15;
        let total_pages = (total + page_size - 1) / page_size;
        let page = (self.quest_page as usize).min(total_pages.saturating_sub(1));

        ui.label(format!("{} quests", total));

        if ui.button("Add New Quest").clicked() {
            let new_quest = QuestItem::default();
            match self.selected_quest_rank {
                0 => self.hr_quests.one_star.push(new_quest),
                1 => self.hr_quests.two_stars.push(new_quest),
                2 => self.hr_quests.three_stars.push(new_quest),
                3 => self.hr_quests.four_stars.push(new_quest),
                4 => self.hr_quests.five_stars.push(new_quest),
                5 => self.hr_quests.six_stars.push(new_quest),
                _ => {}
            }
            self.hr_quests_modified = true;
        }

        MhfdatApp::pagination_controls(ui, &mut self.quest_page, total_pages);
        ui.separator();

        let start = page * page_size;
        let end = (start + page_size).min(total);

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("hr_quest_grid")
                .striped(true)
                .num_columns(5)
                .show(ui, |ui| {
                    ui.label("ID");
                    ui.label("Quest ID");
                    ui.label("Quest Num");
                    ui.label("Key");
                    ui.label("Urgent");
                    ui.end_row();

                    let quests = match self.selected_quest_rank {
                        0 => &self.hr_quests.one_star,
                        1 => &self.hr_quests.two_stars,
                        2 => &self.hr_quests.three_stars,
                        3 => &self.hr_quests.four_stars,
                        4 => &self.hr_quests.five_stars,
                        5 => &self.hr_quests.six_stars,
                        _ => return,
                    };

                    for idx in start..end {
                        let quest = &quests[idx];
                        let quest_id = quest.quest_id;
                        let quest_number = quest.quest_number;
                        let key_quest = quest.key_quest;
                        let urgent_quest = quest.urgent_quest;

                        if ui.selectable_label(false, format!("{}", idx)).clicked() {
                            self.selected_quest_index = Some(idx);
                            let view_key = format!("hr_quests_{}", self.selected_quest_rank);
                            self.view_mode.insert(view_key, ViewMode::Details);
                        }
                        ui.label(format!("{}", quest_id));
                        ui.label(format!("{}", quest_number));
                        ui.label(if key_quest != 0 { "Yes" } else { "No" });
                        ui.label(if urgent_quest != 0 { "Yes" } else { "No" });
                        ui.end_row();
                    }
                });
        });
    }

    fn show_hr_quest_details(&mut self, ui: &mut egui::Ui) {
        let view_key = format!("hr_quests_{}", self.selected_quest_rank);
        
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert(view_key.clone(), ViewMode::List);
            self.selected_quest_index = None;
            return;
        }
        ui.separator();

        let idx = match self.selected_quest_index {
            Some(i) => i,
            None => return,
        };

        let quest = match self.selected_quest_rank {
            0 => self.hr_quests.one_star.get_mut(idx),
            1 => self.hr_quests.two_stars.get_mut(idx),
            2 => self.hr_quests.three_stars.get_mut(idx),
            3 => self.hr_quests.four_stars.get_mut(idx),
            4 => self.hr_quests.five_stars.get_mut(idx),
            5 => self.hr_quests.six_stars.get_mut(idx),
            _ => None,
        };

        let quest = match quest {
            Some(q) => q,
            None => return,
        };

        let mut changed = false;
        let mut quest_id = quest.quest_id;
        let mut quest_number = quest.quest_number;
        let mut key_quest = quest.key_quest;
        let mut urgent_quest = quest.urgent_quest;
        let mut unknown = quest.unknown;

        egui::Grid::new("quest_details_grid").show(ui, |ui| {
            ui.label("Quest ID:");
            if ui.add(egui::DragValue::new(&mut quest_id)).changed() {
                changed = true;
            }
            ui.end_row();

            ui.label("Quest Number:");
            if ui.add(egui::DragValue::new(&mut quest_number)).changed() {
                changed = true;
            }
            ui.end_row();

            ui.label("Key Quest:");
            if ui.add(egui::DragValue::new(&mut key_quest).clamp_range(0..=1)).changed() {
                changed = true;
            }
            ui.end_row();

            ui.label("Urgent Quest:");
            if ui.add(egui::DragValue::new(&mut urgent_quest).clamp_range(0..=1)).changed() {
                changed = true;
            }
            ui.end_row();

            ui.label("Unknown:");
            if ui.add(egui::DragValue::new(&mut unknown)).changed() {
                changed = true;
            }
            ui.end_row();
        });

        if changed {
            quest.quest_id = quest_id;
            quest.quest_number = quest_number;
            quest.key_quest = key_quest;
            quest.urgent_quest = urgent_quest;
            quest.unknown = unknown;
            self.hr_quests_modified = true;
        }
    }

    fn show_gr_quests(&mut self, ui: &mut egui::Ui) {
        let ranks = ["G1", "G2", "G3", "G4", "G5", "G6", "G7"];
        
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

        let view_key = format!("gr_quests_{}", self.selected_quest_rank);
        let view_mode = self.view_mode.get(&view_key).cloned().unwrap_or(ViewMode::List);

        match view_mode {
            ViewMode::List => self.show_gr_quest_list(ui),
            ViewMode::Details => self.show_gr_quest_details(ui),
        }
    }

    fn show_gr_quest_list(&mut self, ui: &mut egui::Ui) {
        let quests = match self.selected_quest_rank {
            0 => &self.gr_quests.g1,
            1 => &self.gr_quests.g2,
            2 => &self.gr_quests.g3,
            3 => &self.gr_quests.g4,
            4 => &self.gr_quests.g5,
            5 => &self.gr_quests.g6,
            6 => &self.gr_quests.g7,
            _ => return,
        };

        let total = quests.len();
        let page_size = 15;
        let total_pages = (total + page_size - 1) / page_size;
        let page = (self.quest_page as usize).min(total_pages.saturating_sub(1));

        ui.label(format!("{} quests", total));

        if ui.button("Add New Quest").clicked() {
            let new_quest = QuestItem::default();
            match self.selected_quest_rank {
                0 => self.gr_quests.g1.push(new_quest),
                1 => self.gr_quests.g2.push(new_quest),
                2 => self.gr_quests.g3.push(new_quest),
                3 => self.gr_quests.g4.push(new_quest),
                4 => self.gr_quests.g5.push(new_quest),
                5 => self.gr_quests.g6.push(new_quest),
                6 => self.gr_quests.g7.push(new_quest),
                _ => {}
            }
            self.gr_quests_modified = true;
        }

        MhfdatApp::pagination_controls(ui, &mut self.quest_page, total_pages);
        ui.separator();

        let start = page * page_size;
        let end = (start + page_size).min(total);

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("gr_quest_grid")
                .striped(true)
                .num_columns(5)
                .show(ui, |ui| {
                    ui.label("ID");
                    ui.label("Quest ID");
                    ui.label("Quest Num");
                    ui.label("Key");
                    ui.label("Urgent");
                    ui.end_row();

                    let quests = match self.selected_quest_rank {
                        0 => &self.gr_quests.g1,
                        1 => &self.gr_quests.g2,
                        2 => &self.gr_quests.g3,
                        3 => &self.gr_quests.g4,
                        4 => &self.gr_quests.g5,
                        5 => &self.gr_quests.g6,
                        6 => &self.gr_quests.g7,
                        _ => return,
                    };

                    for idx in start..end {
                        let quest = &quests[idx];
                        let quest_id = quest.quest_id;
                        let quest_number = quest.quest_number;
                        let key_quest = quest.key_quest;
                        let urgent_quest = quest.urgent_quest;

                        if ui.selectable_label(false, format!("{}", idx)).clicked() {
                            self.selected_quest_index = Some(idx);
                            let view_key = format!("gr_quests_{}", self.selected_quest_rank);
                            self.view_mode.insert(view_key, ViewMode::Details);
                        }
                        ui.label(format!("{}", quest_id));
                        ui.label(format!("{}", quest_number));
                        ui.label(if key_quest != 0 { "Yes" } else { "No" });
                        ui.label(if urgent_quest != 0 { "Yes" } else { "No" });
                        ui.end_row();
                    }
                });
        });
    }

    fn show_gr_quest_details(&mut self, ui: &mut egui::Ui) {
        let view_key = format!("gr_quests_{}", self.selected_quest_rank);
        
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert(view_key.clone(), ViewMode::List);
            self.selected_quest_index = None;
            return;
        }
        ui.separator();

        let idx = match self.selected_quest_index {
            Some(i) => i,
            None => return,
        };

        let quest = match self.selected_quest_rank {
            0 => self.gr_quests.g1.get_mut(idx),
            1 => self.gr_quests.g2.get_mut(idx),
            2 => self.gr_quests.g3.get_mut(idx),
            3 => self.gr_quests.g4.get_mut(idx),
            4 => self.gr_quests.g5.get_mut(idx),
            5 => self.gr_quests.g6.get_mut(idx),
            6 => self.gr_quests.g7.get_mut(idx),
            _ => None,
        };

        let quest = match quest {
            Some(q) => q,
            None => return,
        };

        let mut changed = false;
        let mut quest_id = quest.quest_id;
        let mut quest_number = quest.quest_number;
        let mut key_quest = quest.key_quest;
        let mut urgent_quest = quest.urgent_quest;
        let mut unknown = quest.unknown;

        egui::Grid::new("gr_quest_details_grid").show(ui, |ui| {
            ui.label("Quest ID:");
            if ui.add(egui::DragValue::new(&mut quest_id)).changed() {
                changed = true;
            }
            ui.end_row();

            ui.label("Quest Number:");
            if ui.add(egui::DragValue::new(&mut quest_number)).changed() {
                changed = true;
            }
            ui.end_row();

            ui.label("Key Quest:");
            if ui.add(egui::DragValue::new(&mut key_quest).clamp_range(0..=1)).changed() {
                changed = true;
            }
            ui.end_row();

            ui.label("Urgent Quest:");
            if ui.add(egui::DragValue::new(&mut urgent_quest).clamp_range(0..=1)).changed() {
                changed = true;
            }
            ui.end_row();

            ui.label("Unknown:");
            if ui.add(egui::DragValue::new(&mut unknown)).changed() {
                changed = true;
            }
            ui.end_row();
        });

        if changed {
            quest.quest_id = quest_id;
            quest.quest_number = quest_number;
            quest.key_quest = key_quest;
            quest.urgent_quest = urgent_quest;
            quest.unknown = unknown;
            self.gr_quests_modified = true;
        }
    }
}

