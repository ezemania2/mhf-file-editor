use super::*;

/// hunting_horn_melodies.hexpat HornNote
fn horn_note_name(n: u8) -> &'static str {
    match n {
        0 => "end",
        1 => "white",
        2 => "purple",
        3 => "red",
        4 => "blue",
        5 => "green",
        6 => "yellow",
        7 => "cyan",
        8 => "pink",
        9 => "~",
        _ => "?",
    }
}

impl MhfdatApp {
    /// Hunting horn melodies (header 0x670, 5 bytes each, count = important_nums + 0x258): a song id and four notes.
    /// Melodies can be added at the end (the save moves the table and updates the count). Shown by the Weapons tab.
    pub fn show_horn_melodies(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "weapon");
        if self.category_details("horn_melodies") {
            self.show_horn_melody_details(ui);
            return;
        }
        let total = self.horn_melodies.len();
        MhfdatApp::section_header(ui, &format!("Horn Melodies (found: {total})"), |ui| {
            if ui.add_enabled(total > 0, egui::Button::new("Add New")).on_disabled_hover_text("The list is empty: nothing to copy the new melody from").clicked() {
                // a copy of the last melody with the next free song id
                let mut melody = self.horn_melodies[total - 1].clone();
                melody.song_id = self.horn_melodies.iter().map(|m| m.song_id).max().unwrap_or(0).saturating_add(1);
                self.horn_melodies.push(melody);
                self.horn_melodies_modified = true;
                self.horn_melodies_selected = Some(total);
                self.open_category_details("horn_melodies");
            }
            if MhfdatApp::delete_last_button(ui, total, None) {
                self.horn_melodies.pop();
                self.horn_melodies_modified = true;
            }
        });
        ui.label("Notes are stored in the reverse of the play order. 0 = end, 1-8 = colours, 9 = pitch change (~).");
        MhfdatApp::responsive_row(ui, |ui| {
            ui.label("Search:");
            if ui.text_edit_singleline(&mut self.horn_melodies_search).changed() {
                self.horn_melodies_page = 0;
            }
        });
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No horn melody table found!");
            return;
        }
        let query = self.horn_melodies_search.trim().to_string();
        let filtered: Vec<usize> = (0..total)
            .filter(|&i| query.is_empty() || self.horn_melodies[i].song_id.to_string().contains(&query))
            .collect();
        let (start, end, total_pages) = crate::app::list_window(&mut self.horn_melodies_page, filtered.len());
        let rows: Vec<(usize, u8, String)> = filtered[start..end].iter().map(|&i| {
            let m = &self.horn_melodies[i];
            let notes = m.notes.iter().map(|n| horn_note_name(*n)).collect::<Vec<_>>().join(" ");
            (i, m.song_id, notes)
        }).collect();
        MhfdatApp::entity_list(ui, "horn_melodies_scroll", "Melody", start, end, filtered.len(), |ui| {
            egui::Grid::new("horn_melodies_list").striped(true).show(ui, |ui| {
                for h in ["ID", "Song ID", "Notes"] {
                    ui.label(h);
                }
                ui.end_row();
                for (idx, song, notes) in &rows {
                    if ui.selectable_label(self.horn_melodies_selected == Some(*idx), idx.to_string()).clicked() {
                        self.horn_melodies_selected = Some(*idx);
                        self.open_category_details("horn_melodies");
                    }
                    ui.label(song.to_string());
                    ui.label(notes);
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.horn_melodies_page, total_pages);
    }

    fn show_horn_melody_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("horn_melodies");
            return;
        }
        let Some(sel) = self.horn_melodies_selected.filter(|&i| i < self.horn_melodies.len()) else {
            ui.label("No melody selected");
            return;
        };
        ui.heading(format!("Edit Horn melody #{sel}"));
        ui.separator();
        let m = &mut self.horn_melodies[sel];
        let mut changed = false;
        ui.horizontal(|ui| {
            ui.label("Song ID:");
            changed |= ui.add(egui::DragValue::new(&mut m.song_id)).changed();
        });
        for (k, note) in m.notes.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("Note {}:", k + 1));
                changed |= ui.add(egui::DragValue::new(note)).changed();
                ui.label(horn_note_name(*note));
            });
        }
        if changed {
            self.horn_melodies_modified = true;
        }
    }
}
