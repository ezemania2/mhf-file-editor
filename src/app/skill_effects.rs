use super::*;

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

impl MhfdatApp {
    /// Skill effect / cargo records (header 0x594, 0x1C bytes, count = u16 at +2 of the block behind header 0x988).
    /// Rows are edited where they are: no row can be added or removed.
    pub fn show_skill_effects_tab(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "skills");
        if self.category_details("skill_effects") {
            self.show_skill_effect_details(ui);
        } else {
            self.show_skill_effect_list(ui);
        }
    }

    fn show_skill_effect_list(&mut self, ui: &mut egui::Ui) {
        let total = self.skill_effects.len();
        MhfdatApp::section_header(ui, &format!("Skill effects (found: {total})"), |_| {});
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
        let wanted = self.skill_effects_search.trim().parse::<u32>().ok();
        let shown: Vec<usize> = (0..total)
            .filter(|&i| {
                let e = &self.skill_effects[i];
                wanted.map_or(true, |w| e.skill_id == w || e.effect_id == w)
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
                    ui.label(skill_id.to_string());
                    ui.label(effect_type_name(*effect_type));
                    ui.label(effect_id.to_string());
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
        ui.separator();
        let row = &mut self.skill_effects[idx];
        let mut changed = false;
        ui.horizontal(|ui| { ui.label("Skill ID:"); changed |= ui.add(egui::DragValue::new(&mut row.skill_id)).changed(); });
        ui.horizontal(|ui| {
            ui.label("Type:");
            changed |= ui.add(egui::DragValue::new(&mut row.effect_type)).changed();
            ui.label(effect_type_name(row.effect_type));
        });
        ui.horizontal(|ui| { ui.label("Effect ID:"); changed |= ui.add(egui::DragValue::new(&mut row.effect_id)).changed(); });
        ui.horizontal(|ui| { ui.label("Value:"); changed |= ui.add(egui::DragValue::new(&mut row.value)).changed(); });
        ui.horizontal(|ui| { ui.label("Range min:"); changed |= ui.add(egui::DragValue::new(&mut row.range_min)).changed(); });
        ui.horizontal(|ui| { ui.label("Range max:"); changed |= ui.add(egui::DragValue::new(&mut row.range_max)).changed(); });
        if changed {
            self.skill_effects_modified = true;
        }
    }
}
