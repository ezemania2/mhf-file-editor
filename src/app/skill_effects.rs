use super::*;
use crate::utils::automatic_skills::auto_skill_name;

/// Name of an active skill id (the ids of the effect rows and of the skills they grant).
fn active_skill_name(id: u32) -> &'static str {
    u16::try_from(id).map_or("?", auto_skill_name)
}

/// Effect id with the skill it names: types 1 to 4 grant that skill and type 6 disables it; type 5 is a leaf effect id, not a skill.
fn effect_target_label(effect_type: u32, effect_id: u32) -> String {
    match effect_type {
        1..=4 | 6 => format!("{effect_id} {}", active_skill_name(effect_id)),
        _ => effect_id.to_string(),
    }
}

/// docs/reverse/skill_cargo.md: the meaning of `effect_type`
fn effect_type_name(t: u32) -> &'static str {
    match t {
        1 => "cargo (always)",
        2 => "grant if equipped skills <= 1",
        3 => "grant if equipped skills <= 2",
        4 => "grant if equipped skills <= 3",
        5 => "leaf (value / range)",
        6 => "disable list",
        _ => "?",
    }
}

#[derive(PartialEq, Clone, Copy, Default)]
pub enum SkillTab {
    #[default]
    Effects,
    Tree,
    Vampirism,
    CritConversion,
    Rush,
    Obscurity,
}

impl MhfdatApp {
    /// Skill effect / cargo records (header 0x594, 0x1C bytes, count = u16 at +2 of the block behind header 0x988).
    /// Rows can be added and deleted. The client builds its per-skill index from the table, so the rows of one skill have
    /// to stay together: a new row goes at the end of the run of its skill, and the save refuses a split run.
    /// The Skills tab: skill effects (0x594), the skill tree (0xEC) and the skill parameter tables (Vampirism 0x7A0, Crit
    /// Conversion 0xA1C, Rush and Obscurity).
    pub fn show_skill_effects_tab(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "skills");
        ui.horizontal_wrapped(|ui| {
            for (tab, label) in [
                (SkillTab::Effects, "Skill effects"),
                (SkillTab::Tree, "Skill tree"),
                (SkillTab::Vampirism, "Vampirism"),
                (SkillTab::CritConversion, "Crit Conversion"),
                (SkillTab::Rush, "Rush"),
                (SkillTab::Obscurity, "Obscurity"),
            ] {
                if ui.selectable_label(self.skill_tab == tab, label).clicked() {
                    self.skill_tab = tab;
                    self.back_category_list("skill_effects");
                    self.back_category_list("skill_tree");
                }
            }
        });
        ui.separator();
        match self.skill_tab {
            SkillTab::Effects => {
                if self.category_details("skill_effects") {
                    self.show_skill_effect_details(ui);
                } else {
                    self.show_skill_effect_list(ui);
                }
            }
            SkillTab::Tree => self.show_skill_tree(ui),
            SkillTab::Vampirism => self.show_vampirism(ui),
            SkillTab::CritConversion => self.show_crit_conversion(ui),
            SkillTab::Rush => self.show_rush(ui),
            SkillTab::Obscurity => self.show_obscurity(ui),
        }
    }

    fn show_skill_effect_list(&mut self, ui: &mut egui::Ui) {
        let total = self.skill_effects.len();
        MhfdatApp::section_header(ui, &format!("Skill effects (found: {total})"), |ui| {
            ui.label("Skill ID:");
            ui.add(egui::DragValue::new(&mut self.skill_effects_new_skill).clamp_range(0..=crate::core::mhfdat::SKILL_ID_COUNT - 1));
            ui.label(active_skill_name(self.skill_effects_new_skill));
            if ui.button("Add New").on_hover_text("Adds a row to this skill, after the rows it already has").clicked() {
                let skill_id = self.skill_effects_new_skill;
                // after the last row of the skill when it has some (a copy of that row), otherwise at the end of the table
                let (at, row) = match self.skill_effects.iter().rposition(|e| e.skill_id == skill_id) {
                    Some(last) => (last + 1, self.skill_effects[last].clone()),
                    None => (total, crate::model::mhfdat::SkillEffect { skill_id, effect_type: 5, ..Default::default() }),
                };
                self.skill_effects.insert(at, row);
                self.skill_effects_modified = true;
                self.skill_effects_selected = Some(at);
                self.open_category_details("skill_effects");
            }
        });
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search:");
            if ui.text_edit_singleline(&mut self.skill_effects_search).changed() {
                self.skill_effects_page = 0;
            }
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No skill effect table found!");
            return;
        }
        // a number matches a skill id or an effect id, a text matches the name of the skill or of the skill it grants
        let query = self.skill_effects_search.trim().to_lowercase();
        let wanted = query.parse::<u32>().ok();
        let shown: Vec<usize> = (0..total)
            .filter(|&i| {
                let e = &self.skill_effects[i];
                match wanted {
                    Some(w) => e.skill_id == w || e.effect_id == w,
                    None => {
                        query.is_empty()
                            || crate::utils::text::contains_ci(active_skill_name(e.skill_id), &query)
                            || (matches!(e.effect_type, 1..=4 | 6) && crate::utils::text::contains_ci(active_skill_name(e.effect_id), &query))
                    }
                }
            })
            .collect();
        let (start, end, total_pages) = crate::app::list_window(&mut self.skill_effects_page, shown.len());
        let rows: Vec<(usize, u32, u32, u32)> = shown[start..end].iter().map(|&i| {
            let e = &self.skill_effects[i];
            (i, e.skill_id, e.effect_type, e.effect_id)
        }).collect();
        MhfdatApp::entity_list(ui, "skill_effects_scroll", "Effect", start, end, shown.len(), |ui| {
            egui::Grid::new("skill_effects_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Skill ID");
                ui.label("Type");
                ui.label("Effect ID");
                ui.end_row();
                for (idx, skill_id, effect_type, effect_id) in &rows {
                    if ui.selectable_label(self.skill_effects_selected == Some(*idx), idx.to_string()).clicked() {
                        self.skill_effects_selected = Some(*idx);
                        self.open_category_details("skill_effects");
                    }
                    ui.label(format!("{skill_id} {}", active_skill_name(*skill_id)));
                    ui.label(effect_type_name(*effect_type));
                    ui.label(effect_target_label(*effect_type, *effect_id));
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.skill_effects_page, total_pages);
    }

    fn show_skill_effect_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("skill_effects");
            return;
        }
        let Some(idx) = self.skill_effects_selected.filter(|&i| i < self.skill_effects.len()) else {
            ui.label("No row selected");
            return;
        };
        ui.heading(format!("Edit Skill effect #{idx}"));
        if ui.button("Delete this row").clicked() {
            self.skill_effects.remove(idx);
            self.skill_effects_modified = true;
            self.skill_effects_selected = None;
            self.back_category_list("skill_effects");
            return;
        }
        ui.separator();
        let row = &mut self.skill_effects[idx];
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label("Skill ID:");
            changed |= ui.add(egui::DragValue::new(&mut row.skill_id)).changed();
            ui.label(active_skill_name(row.skill_id));
        });
        ui.horizontal(|ui| {
            ui.label("Type:");
            changed |= ui.add(egui::DragValue::new(&mut row.effect_type)).changed();
            ui.label(effect_type_name(row.effect_type));
        });
        ui.horizontal(|ui| {
            ui.label("Effect ID:");
            changed |= ui.add(egui::DragValue::new(&mut row.effect_id)).changed();
            if matches!(row.effect_type, 1..=4 | 6) {
                ui.label(active_skill_name(row.effect_id));
            }
        });
        ui.horizontal(|ui| { ui.label("Value:"); changed |= ui.add(egui::DragValue::new(&mut row.value)).changed(); });
        ui.horizontal(|ui| { ui.label("Range min:"); changed |= ui.add(egui::DragValue::new(&mut row.range_min)).changed(); });
        ui.horizontal(|ui| { ui.label("Range max:"); changed |= ui.add(egui::DragValue::new(&mut row.range_max)).changed(); });
        if changed {
            self.skill_effects_modified = true;
        }
    }
}
