use super::*;

impl MhfdatApp {
    /// Monster crown thresholds (header 0x130, 14 bytes per monster, count = important_nums + 0x22).
    /// docs/reverse/monster_size.md 1.3. The count is shared with the monster descriptions (Monsters tab): adding or
    /// deleting a monster here does both.
    pub fn show_crowns_tab(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "crowns");
        if !self.view_mode.contains_key("crowns") {
            self.view_mode.insert("crowns".to_string(), ViewMode::List);
        }
        match self.view_mode.get("crowns").copied().unwrap_or(ViewMode::List) {
            ViewMode::List => self.show_crowns_list(ui),
            ViewMode::Details => self.show_crowns_details(ui),
        }
    }

    fn show_crowns_list(&mut self, ui: &mut egui::Ui) {
        let total = self.crowns.len();
        MhfdatApp::section_header(ui, &format!("Monster crowns (found: {total})"), |ui| {
            if ui.button("Add New").on_hover_text("Adds a monster: a crown row and an empty description").clicked() {
                self.add_monster_entry();
                if self.crowns.len() > total {
                    self.crowns_selected = Some(total);
                    self.view_mode.insert("crowns".to_string(), ViewMode::Details);
                }
            }
            if MhfdatApp::delete_last_button(ui, total, None) {
                self.remove_last_monster_entry();
            }
        });
        ui.label("Thresholds as stored (the community size tables show them divided by 100). Silver is awarded from silver + 2, gold from gold + 4, mini at or under mini + 6. -1 = no crowns.");
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search:");
            if ui.text_edit_singleline(&mut self.search_query).changed() {
                self.crowns_page = 0;
            }
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No crown table found!");
            return;
        }
        let query = self.search_query.clone();
        let filtered: Vec<usize> = (0..total).filter(|&i| {
            let id = self.crowns[i].monster_id;
            query.is_empty() || id.to_string().contains(query.trim())
        }).collect();
        let (start, end, total_pages) = crate::app::list_window(&mut self.crowns_page, filtered.len());
        let rows: Vec<(usize, u8, i16, i16, i16)> = filtered[start..end].iter().map(|&i| {
            let row = &self.crowns[i];
            (i, row.monster_id, row.silver_threshold, row.gold_threshold, row.mini_threshold)
        }).collect();
        MhfdatApp::entity_list(ui, "crowns_scroll", "Crown", start, end, filtered.len(), |ui| {
            egui::Grid::new("crowns_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Monster ID");
                ui.label("Silver");
                ui.label("Gold");
                ui.label("Mini");
                ui.end_row();
                for (i, monster_id, silver, gold, mini) in &rows {
                    if ui.selectable_label(self.crowns_selected == Some(*i), i.to_string()).clicked() {
                        self.crowns_selected = Some(*i);
                        self.view_mode.insert("crowns".to_string(), ViewMode::Details);
                    }
                    ui.label(monster_id.to_string());
                    ui.label(silver.to_string());
                    ui.label(gold.to_string());
                    ui.label(mini.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.crowns_page, total_pages);
    }

    fn show_crowns_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.view_mode.insert("crowns".to_string(), ViewMode::List);
            return;
        }
        let Some(idx) = self.crowns_selected.filter(|&i| i < self.crowns.len()) else {
            ui.label("No crown row selected");
            return;
        };
        ui.heading(format!("Edit Monster crown #{idx}"));
        ui.separator();
        let row = &mut self.crowns[idx];
        let mut changed = false;
        ui.horizontal(|ui| { ui.label("Monster ID:"); changed |= ui.add(egui::DragValue::new(&mut row.monster_id)).changed(); });
        ui.horizontal(|ui| { ui.label("Silver:"); changed |= ui.add(egui::DragValue::new(&mut row.silver_threshold)).changed(); });
        ui.horizontal(|ui| { ui.label("Gold:"); changed |= ui.add(egui::DragValue::new(&mut row.gold_threshold)).changed(); });
        ui.horizontal(|ui| { ui.label("Mini:"); changed |= ui.add(egui::DragValue::new(&mut row.mini_threshold)).changed(); });
        ui.horizontal(|ui| { ui.label("Monster ID (+0x0D):"); changed |= ui.add(egui::DragValue::new(&mut row.monster_id_2)).changed(); });
        if changed {
            self.crowns_modified = true;
        }
    }
}
