use super::*;
use crate::core::mhfdat::{SKILL_TREE_ACTIVE_MAX, SKILL_TREE_ID_MAX};
use crate::model::mhfdat::SkillTreeRow;
use crate::utils::automatic_skills::auto_skill_name;
use crate::utils::skills::skill_name;

/// Default thresholds of a new tree (the ones the table's end row carries).
const NEW_TREE_THRESHOLDS: [i16; 8] = [-20, -15, -10, 10, 15, 20, 25, 30];

fn series_name(tree_id: i16) -> &'static str {
    u8::try_from(tree_id).map_or("?", skill_name)
}

fn active_name(id: i16) -> &'static str {
    match id {
        0 => "none",
        _ => u16::try_from(id).map_or("?", auto_skill_name),
    }
}

impl MhfdatApp {
    /// First skill tree id (1 to 224) that no row uses.
    fn free_skill_tree_id(&self) -> Option<i16> {
        (1..=SKILL_TREE_ID_MAX).find(|id| self.skill_tree.iter().all(|r| r.tree_id != *id))
    }

    /// Skill tree (header 0xEC): turns the points of a skill series into active skills. Rows can be edited, added (while a skill
    /// tree id is free) and deleted; the save moves the table to the end of the file when its length changed.
    pub(in crate::app) fn show_skill_tree(&mut self, ui: &mut egui::Ui) {
        if self.category_details("skill_tree") {
            self.show_skill_tree_details(ui);
            return;
        }
        let total = self.skill_tree.len();
        let free = self.free_skill_tree_id();
        MhfdatApp::section_header(ui, &format!("Skill tree (found: {total})"), |ui| {
            if ui.add_enabled(free.is_some(), egui::Button::new("Add New")).on_disabled_hover_text("Every skill tree id (1 to 224) is already used").clicked() {
                if let Some(tree_id) = free {
                    self.skill_tree.push(SkillTreeRow { tree_id, thresholds: NEW_TREE_THRESHOLDS, active_skills: [0; 8] });
                    self.skill_tree_modified = true;
                    self.skill_tree_selected = Some(total);
                    self.open_category_details("skill_tree");
                }
            }
        });
        ui.label("Each row turns the points a player has in one skill series into active skills: up to three on the negative side (points at or under a threshold) and five on the positive side (points at or over a threshold).");
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search:");
            if ui.text_edit_singleline(&mut self.skill_tree_search).changed() {
                self.skill_tree_page = 0;
            }
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No skill tree found!");
            return;
        }
        let query = self.skill_tree_search.trim().to_lowercase();
        let shown: Vec<usize> = (0..total)
            .filter(|&i| {
                let id = self.skill_tree[i].tree_id;
                query.is_empty() || id.to_string() == query || crate::utils::text::contains_ci(series_name(id), &query)
            })
            .collect();
        let (start, end, total_pages) = crate::app::list_window(&mut self.skill_tree_page, shown.len());
        let rows: Vec<(usize, i16, String, String)> = shown[start..end].iter().map(|&i| {
            let r = &self.skill_tree[i];
            let thresholds = r.thresholds.iter().map(|t| t.to_string()).collect::<Vec<_>>().join(" / ");
            let skills = r.active_skills.iter().filter(|a| **a != 0).map(|a| active_name(*a)).collect::<Vec<_>>().join(", ");
            (i, r.tree_id, thresholds, skills)
        }).collect();
        MhfdatApp::entity_list(ui, "skill_tree_scroll", "Tree", start, end, shown.len(), |ui| {
            egui::Grid::new("skill_tree_list").striped(true).show(ui, |ui| {
                for h in ["ID", "Tree ID", "Series", "Thresholds", "Active skills"] {
                    ui.label(h);
                }
                ui.end_row();
                for (idx, tree_id, thresholds, skills) in &rows {
                    if ui.selectable_label(self.skill_tree_selected == Some(*idx), idx.to_string()).clicked() {
                        self.skill_tree_selected = Some(*idx);
                        self.open_category_details("skill_tree");
                    }
                    ui.label(tree_id.to_string());
                    ui.label(series_name(*tree_id));
                    ui.label(thresholds);
                    ui.label(skills);
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.skill_tree_page, total_pages);
    }

    fn show_skill_tree_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("skill_tree");
            return;
        }
        let Some(idx) = self.skill_tree_selected.filter(|&i| i < self.skill_tree.len()) else {
            ui.label("No skill tree selected");
            return;
        };
        ui.heading(format!("Edit Skill tree #{idx} ({})", series_name(self.skill_tree[idx].tree_id)));
        if ui.button("Delete this row").clicked() {
            self.skill_tree.remove(idx);
            self.skill_tree_modified = true;
            self.skill_tree_selected = None;
            self.back_category_list("skill_tree");
            return;
        }
        ui.separator();
        let tree_id = self.skill_tree[idx].tree_id;
        let twin = self.skill_tree.iter().enumerate().find(|(i, r)| *i != idx && r.tree_id == tree_id).map(|(i, _)| i);
        let mut changed = false;
        let row = &mut self.skill_tree[idx];
        egui::ScrollArea::vertical().id_source("skill_tree_details_scroll").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tree ID:");
                changed |= ui.add(egui::DragValue::new(&mut row.tree_id).clamp_range(1..=SKILL_TREE_ID_MAX)).changed();
                ui.label(series_name(row.tree_id));
            });
            if let Some(other) = twin {
                ui.colored_label(egui::Color32::YELLOW, format!("Row {other} has the same tree id: the game keeps only one row per id, the save is refused."));
            }
            if row.tree_id == 0xA1 {
                ui.label("Tree 0xA1 is looked up as a cascade: the highest threshold that is reached wins.");
            }
            ui.label("Negative side: tier 1 to 3 give their skill while the points are at or under the threshold (checked in order). Positive side: tier 4 to 8 give their skill once the points reach the threshold (highest first). Nothing is granted between tiers 3 and 4.");
            ui.separator();
            egui::Grid::new(("skill_tree_tiers", idx)).num_columns(4).striped(true).show(ui, |ui| {
                for h in ["Tier", "Points", "Active skill", ""] {
                    ui.label(h);
                }
                ui.end_row();
                for k in 0..8 {
                    ui.label(format!("{} {}", k + 1, if k < 3 { "(at or under)" } else { "(at or over)" }));
                    changed |= ui.add(egui::DragValue::new(&mut row.thresholds[k])).changed();
                    changed |= ui.add(egui::DragValue::new(&mut row.active_skills[k]).clamp_range(0..=SKILL_TREE_ACTIVE_MAX)).changed();
                    ui.label(active_name(row.active_skills[k]));
                    ui.end_row();
                }
            });
        });
        if changed {
            self.skill_tree_modified = true;
        }
    }
}
