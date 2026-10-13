use super::*;

#[derive(PartialEq, Clone, Copy, Default)]
pub enum WeaponParamsTab {
    #[default]
    Thresholds,
    Coating,
    Melodies,
    Gunlance,
    SwitchAxe,
    MotionPrimary,
    MotionAlt,
    Rasta,
}

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

/// rasta.hexpat: weapon category of a Rasta weapon option
const WEAPON_TYPE_NAMES: [&str; 14] = [
    "GS", "HBG", "Hammer", "Lance", "SnS", "LBG", "DB", "LS", "HH", "GL", "Bow", "Tonfa", "SA", "MS",
];

impl MhfdatApp {
    /// Weapon parameter tables: action thresholds (header 0xA0), coating / tower modifiers (0xB0), horn melodies
    /// (0x670), gunlance random shells (0x4F0 / 0x4F8), switch axe phial strength (0xA88), motion values (0x63C /
    /// 0x74C) and the Rasta tables (0x56C / 0x570 / 0x574). Edited in place; rows cannot be added or removed.
    pub fn show_weapon_params_tab(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "weapon");
        ui.horizontal_wrapped(|ui| {
            for (tab, label) in [
                (WeaponParamsTab::Thresholds, "Action thresholds"),
                (WeaponParamsTab::Coating, "Coating / tower modifiers"),
                (WeaponParamsTab::Melodies, "Horn melodies"),
                (WeaponParamsTab::Gunlance, "Gunlance shells"),
                (WeaponParamsTab::SwitchAxe, "Switch axe phials"),
                (WeaponParamsTab::MotionPrimary, "Motion values"),
                (WeaponParamsTab::MotionAlt, "Motion values (alt)"),
                (WeaponParamsTab::Rasta, "Rasta"),
            ] {
                if ui.selectable_label(self.weapon_params_tab == tab, label).clicked() {
                    self.weapon_params_tab = tab;
                    self.weapon_params_page = 0;
                    self.weapon_params_selected = 0;
                    self.weapon_params_group = 0;
                    self.back_category_list("weapon_params");
                }
            }
        });
        ui.separator();
        let mut changed = false;
        match self.weapon_params_tab {
            WeaponParamsTab::Thresholds => self.show_thresholds(ui, &mut changed),
            WeaponParamsTab::Coating => self.show_coating(ui, &mut changed),
            WeaponParamsTab::Melodies => self.show_melodies(ui, &mut changed),
            WeaponParamsTab::Gunlance => self.show_gunlance(ui, &mut changed),
            WeaponParamsTab::SwitchAxe => self.show_switch_axe(ui, &mut changed),
            WeaponParamsTab::MotionPrimary => self.show_motion(ui, &mut changed, false),
            WeaponParamsTab::MotionAlt => self.show_motion(ui, &mut changed, true),
            WeaponParamsTab::Rasta => self.show_rasta(ui, &mut changed),
        };
        if changed {
            self.weapon_params_modified = true;
        }
    }

    fn show_thresholds(&mut self, ui: &mut egui::Ui, changed: &mut bool) {
        if self.category_details("weapon_params") {
            let sel = self.weapon_params_selected.min(self.weapon_params.thresholds.len().saturating_sub(1));
            if ui.button("← Back to List").clicked() {
                self.back_category_list("weapon_params");
                return;
            }
            ui.heading(format!("Edit Action threshold #{sel:02X}"));
            ui.separator();
            if self.weapon_params.thresholds.is_empty() {
                ui.label("No threshold table loaded.");
                return;
            }
            ui.label("The game only reads threshold 1.");
            for (k, v) in self.weapon_params.thresholds[sel].iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Threshold {k}:"));
                    *changed |= ui.add(egui::DragValue::new(v)).changed();
                });
            }
            return;
        }
        let total = self.weapon_params.thresholds.len();
        MhfdatApp::section_header(ui, &format!("Action thresholds (found: {total})"), |_| {});
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No threshold table found!");
            return;
        }
        let (start, end, total_pages) = crate::app::list_window(&mut self.weapon_params_page, total);
        let rows: Vec<(usize, u8)> = (start..end).map(|i| (i, self.weapon_params.thresholds[i][1])).collect();
        MhfdatApp::entity_list(ui, "thresholds_scroll", "Action", start, end, total, |ui| {
            egui::Grid::new("thresholds_list").striped(true).show(ui, |ui| {
                ui.label("ID");
                ui.label("Threshold 1");
                ui.end_row();
                for (i, t1) in &rows {
                    if ui.selectable_label(false, format!("{i:02X}")).clicked() {
                        self.weapon_params_selected = *i;
                        self.open_category_details("weapon_params");
                    }
                    ui.label(t1.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.weapon_params_page, total_pages);
    }

    fn show_coating(&mut self, ui: &mut egui::Ui, changed: &mut bool) {
        ui.horizontal(|ui| {
            if ui.selectable_label(self.weapon_params_group == 0, "Coating").clicked() {
                self.weapon_params_group = 0;
                self.weapon_params_page = 0;
                self.back_category_list("weapon_params");
            }
            if ui.selectable_label(self.weapon_params_group != 0, "Tower modifiers").clicked() {
                self.weapon_params_group = 1;
                self.weapon_params_page = 0;
                self.back_category_list("weapon_params");
            }
        });
        if self.weapon_params_group == 0 {
            self.show_coating_rows(ui, changed);
        } else {
            self.show_tower_modifier_rows(ui, changed);
        }
    }

    fn show_coating_rows(&mut self, ui: &mut egui::Ui, changed: &mut bool) {
        if self.category_details("weapon_params") {
            if ui.button("← Back to List").clicked() { self.back_category_list("weapon_params"); return; }
            let sel = self.weapon_params_selected.min(self.weapon_params.coating.len().saturating_sub(1));
            ui.heading(format!("Edit Coating row #{sel}"));
            ui.separator();
            if self.weapon_params.coating.is_empty() { ui.label("No coating table loaded."); return; }
            ui.horizontal(|ui| {
                ui.label("Delta:");
                *changed |= ui.add(egui::DragValue::new(&mut self.weapon_params.coating[sel].delta_coating)).changed();
            });
            return;
        }
        let total = self.weapon_params.coating.len();
        MhfdatApp::section_header(ui, &format!("Coating rows (found: {total})"), |_| {});
        if total == 0 { ui.colored_label(egui::Color32::YELLOW, "Warning: No coating table found!"); return; }
        let (start, end, pages) = crate::app::list_window(&mut self.weapon_params_page, total);
        let rows: Vec<(usize, i16)> = (start..end).map(|i| (i, self.weapon_params.coating[i].delta_coating)).collect();
        MhfdatApp::entity_list(ui, "coating_scroll", "Row", start, end, total, |ui| {
            egui::Grid::new("coating_list").striped(true).show(ui, |ui| {
                ui.label("ID"); ui.label("Delta"); ui.end_row();
                for (i, delta) in &rows {
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.weapon_params_selected = *i;
                        self.open_category_details("weapon_params");
                    }
                    ui.label(delta.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.weapon_params_page, pages);
    }

    fn show_tower_modifier_rows(&mut self, ui: &mut egui::Ui, changed: &mut bool) {
        if self.category_details("weapon_params") {
            if ui.button("← Back to List").clicked() { self.back_category_list("weapon_params"); return; }
            let sel = self.weapon_params_selected.min(self.weapon_params.tower_modifiers.len().saturating_sub(1));
            ui.heading(format!("Edit Tower modifier #{sel}"));
            ui.separator();
            if self.weapon_params.tower_modifiers.is_empty() { ui.label("No tower modifier table loaded."); return; }
            ui.horizontal(|ui| {
                ui.label("Delta:");
                *changed |= ui.add(egui::DragValue::new(&mut self.weapon_params.tower_modifiers[sel].delta_tower)).changed();
            });
            return;
        }
        let total = self.weapon_params.tower_modifiers.len();
        MhfdatApp::section_header(ui, &format!("Tower modifiers (found: {total})"), |_| {});
        if total == 0 { ui.colored_label(egui::Color32::YELLOW, "Warning: No tower modifier table found!"); return; }
        let (start, end, pages) = crate::app::list_window(&mut self.weapon_params_page, total);
        let rows: Vec<(usize, i16)> = (start..end).map(|i| (i, self.weapon_params.tower_modifiers[i].delta_tower)).collect();
        MhfdatApp::entity_list(ui, "tower_mod_scroll", "Row", start, end, total, |ui| {
            egui::Grid::new("tower_mod_list").striped(true).show(ui, |ui| {
                ui.label("ID"); ui.label("Delta"); ui.end_row();
                for (i, delta) in &rows {
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.weapon_params_selected = *i;
                        self.open_category_details("weapon_params");
                    }
                    ui.label(delta.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.weapon_params_page, pages);
    }

    fn show_melodies(&mut self, ui: &mut egui::Ui, changed: &mut bool) {
        if self.category_details("weapon_params") {
            if ui.button("← Back to List").clicked() { self.back_category_list("weapon_params"); return; }
            let sel = self.weapon_params_selected.min(self.weapon_params.melodies.len().saturating_sub(1));
            ui.heading(format!("Edit Horn melody #{sel}"));
            ui.separator();
            if self.weapon_params.melodies.is_empty() { ui.label("No melody table loaded."); return; }
            ui.label("Notes are stored in reverse of the play order. 0 = end, 1-8 = colours, 9 = pitch change (~).");
            ui.horizontal(|ui| { ui.label("Song ID:"); *changed |= ui.add(egui::DragValue::new(&mut self.weapon_params.melodies[sel].song_id)).changed(); });
            for (k, n) in self.weapon_params.melodies[sel].notes.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("Note {}:", k + 1));
                    *changed |= ui.add(egui::DragValue::new(n)).changed();
                    ui.label(horn_note_name(*n));
                });
            }
            return;
        }
        let total = self.weapon_params.melodies.len();
        MhfdatApp::section_header(ui, &format!("Horn melodies (found: {total})"), |_| {});
        if total == 0 { ui.colored_label(egui::Color32::YELLOW, "Warning: No melody table found!"); return; }
        let (start, end, pages) = crate::app::list_window(&mut self.weapon_params_page, total);
        let rows: Vec<(usize, u8)> = (start..end).map(|i| (i, self.weapon_params.melodies[i].song_id)).collect();
        MhfdatApp::entity_list(ui, "melodies_scroll", "Melody", start, end, total, |ui| {
            egui::Grid::new("melodies_list").striped(true).show(ui, |ui| {
                ui.label("ID"); ui.label("Song ID"); ui.end_row();
                for (i, song) in &rows {
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.weapon_params_selected = *i;
                        self.open_category_details("weapon_params");
                    }
                    ui.label(song.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.weapon_params_page, pages);
    }

    fn show_gunlance(&mut self, ui: &mut egui::Ui, changed: &mut bool) {
        ui.horizontal(|ui| {
            if ui.selectable_label(self.weapon_params_group == 0, "Shell types").clicked() {
                self.weapon_params_group = 0;
                self.back_category_list("weapon_params");
            }
            if ui.selectable_label(self.weapon_params_group != 0, "Parameter pairs").clicked() {
                self.weapon_params_group = 1;
                self.weapon_params_page = 0;
                self.back_category_list("weapon_params");
            }
        });
        if self.weapon_params_group == 0 {
            if self.category_details("weapon_params") {
                if ui.button("← Back to List").clicked() { self.back_category_list("weapon_params"); return; }
                let sel = self.weapon_params_selected.min(2);
                ui.heading(format!("Edit Shell type #{sel}"));
                ui.separator();
                ui.label("0 = normal, 1 = long, 2 = wide");
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    *changed |= ui.add(egui::DragValue::new(&mut self.weapon_params.gl_shell_types[sel])).changed();
                });
                return;
            }
            MhfdatApp::section_header(ui, "Shell types (found: 3)", |_| {});
            MhfdatApp::entity_list(ui, "gl_shells_scroll", "Type", 0, 3, 3, |ui| {
                egui::Grid::new("gl_shell_list").striped(true).show(ui, |ui| {
                    ui.label("ID"); ui.label("Type"); ui.end_row();
                    for i in 0..3 {
                        let v = self.weapon_params.gl_shell_types[i];
                        if ui.selectable_label(false, i.to_string()).clicked() {
                            self.weapon_params_selected = i;
                            self.open_category_details("weapon_params");
                        }
                        ui.label(v.to_string());
                        ui.end_row();
                    }
                });
            });
            return;
        }
        if self.category_details("weapon_params") {
            if ui.button("← Back to List").clicked() { self.back_category_list("weapon_params"); return; }
            let sel = self.weapon_params_selected.min(self.weapon_params.gl_param_pairs.len().saturating_sub(1));
            ui.heading(format!("Edit Gunlance pair #{sel}"));
            ui.separator();
            if self.weapon_params.gl_param_pairs.is_empty() { ui.label("No parameter pairs loaded."); return; }
            ui.horizontal(|ui| { ui.label("a:"); *changed |= ui.add(egui::DragValue::new(&mut self.weapon_params.gl_param_pairs[sel][0])).changed(); });
            ui.horizontal(|ui| { ui.label("b:"); *changed |= ui.add(egui::DragValue::new(&mut self.weapon_params.gl_param_pairs[sel][1])).changed(); });
            return;
        }
        let total = self.weapon_params.gl_param_pairs.len();
        MhfdatApp::section_header(ui, &format!("Parameter pairs (found: {total})"), |_| {});
        ui.label("0xFF marks the end of a section.");
        if total == 0 { ui.colored_label(egui::Color32::YELLOW, "Warning: No parameter pairs found!"); return; }
        let (start, end, pages) = crate::app::list_window(&mut self.weapon_params_page, total);
        let rows: Vec<(usize, u8, u8)> = (start..end).map(|i| {
            let p = self.weapon_params.gl_param_pairs[i];
            (i, p[0], p[1])
        }).collect();
        MhfdatApp::entity_list(ui, "gl_pairs_scroll", "Pair", start, end, total, |ui| {
            egui::Grid::new("gl_pairs_list").striped(true).show(ui, |ui| {
                ui.label("ID"); ui.label("a"); ui.label("b"); ui.end_row();
                for (i, a, b) in &rows {
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.weapon_params_selected = *i;
                        self.open_category_details("weapon_params");
                    }
                    ui.label(a.to_string());
                    ui.label(b.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.weapon_params_page, pages);
    }

    fn show_switch_axe(&mut self, ui: &mut egui::Ui, changed: &mut bool) {
        if self.category_details("weapon_params") {
            if ui.button("← Back to List").clicked() { self.back_category_list("weapon_params"); return; }
            let sel = self.weapon_params_selected.min(4);
            ui.heading(format!("Edit Switch axe phial #{sel}"));
            ui.separator();
            ui.label("Phial strength in milli.");
            ui.horizontal(|ui| {
                ui.label("Strength:");
                *changed |= ui.add(egui::DragValue::new(&mut self.weapon_params.sa_phials[sel])).changed();
            });
            return;
        }
        MhfdatApp::section_header(ui, "Switch axe phials (found: 5)", |_| {});
        MhfdatApp::entity_list(ui, "sa_phials_scroll", "Phial", 0, 5, 5, |ui| {
            egui::Grid::new("sa_phials_list").striped(true).show(ui, |ui| {
                ui.label("ID"); ui.label("Strength"); ui.end_row();
                for i in 0..5 {
                    let v = self.weapon_params.sa_phials[i];
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.weapon_params_selected = i;
                        self.open_category_details("weapon_params");
                    }
                    ui.label(v.to_string());
                    ui.end_row();
                }
            });
        });
    }

    fn show_motion(&mut self, ui: &mut egui::Ui, changed: &mut bool, alt: bool) {
        let total = if alt { self.weapon_params.motion_alt.len() } else { self.weapon_params.motion_primary.len() };
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No motion value table found!");
            return;
        }
        if self.category_details("weapon_params") {
            if ui.button("← Back to List").clicked() { self.back_category_list("weapon_params"); return; }
            let sel = self.weapon_params_selected.min(total - 1);
            ui.heading(format!("Edit Motion values #{sel:02X}"));
            ui.separator();
            let d = if alt { &mut self.weapon_params.motion_alt[sel] } else { &mut self.weapon_params.motion_primary[sel] };
            egui::Grid::new("motion_flag").show(ui, |ui| {
                *changed |= MhfdatApp::flag_row(ui, "Valid", &mut d.valid);
            });
            egui::Grid::new("motion_grid").striped(true).show(ui, |ui| {
                ui.label("");
                for k in 0..12 { ui.label(format!("#{k}")); }
                ui.end_row();
                macro_rules! array_row {
                    ($label:expr, $arr:expr, $speed:expr) => {{
                        ui.label($label);
                        for v in $arr.iter_mut() {
                            *changed |= ui.add(egui::DragValue::new(v).speed($speed)).changed();
                        }
                        ui.end_row();
                    }};
                }
                array_row!("primary MV", d.primary_mv, 1.0);
                array_row!("element magnitude", d.element_magnitude, 1.0);
                array_row!("attr +0x28", d.attr28, 1.0);
                array_row!("attr +0x34", d.attr34, 1.0);
                array_row!("scale", d.scale, 0.01);
                array_row!("secondary MV", d.secondary_mv, 1.0);
            });
            return;
        }
        let title = if alt { "Motion values (alt)" } else { "Motion values" };
        MhfdatApp::section_header(ui, &format!("{title} (found: {total})"), |_| {});
        let (start, end, pages) = crate::app::list_window(&mut self.weapon_params_page, total);
        let rows: Vec<(usize, u8)> = (start..end).map(|i| {
            let valid = if alt { self.weapon_params.motion_alt[i].valid } else { self.weapon_params.motion_primary[i].valid };
            (i, valid)
        }).collect();
        MhfdatApp::entity_list(ui, if alt { "motion_alt_scroll" } else { "motion_scroll" }, "Action", start, end, total, |ui| {
            egui::Grid::new(if alt { "motion_alt_list" } else { "motion_list" }).striped(true).show(ui, |ui| {
                ui.label("ID"); ui.label("Valid"); ui.end_row();
                for (i, valid) in &rows {
                    if ui.selectable_label(false, format!("{i:02X}")).clicked() {
                        self.weapon_params_selected = *i;
                        self.open_category_details("weapon_params");
                    }
                    ui.label(valid.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.weapon_params_page, pages);
    }

    fn show_rasta(&mut self, ui: &mut egui::Ui, changed: &mut bool) {
        ui.horizontal(|ui| {
            for (id, label) in [(0usize, "NPCs"), (1, "Weapons"), (2, "Armor")] {
                if ui.selectable_label(self.weapon_params_group == id, label).clicked() {
                    self.weapon_params_group = id;
                    self.weapon_params_page = 0;
                    self.back_category_list("weapon_params");
                }
            }
        });
        match self.weapon_params_group {
            1 => self.show_rasta_weapons(ui, changed),
            2 => self.show_rasta_armors(ui, changed),
            _ => self.show_rasta_npcs(ui, changed),
        }
    }

    fn show_rasta_npcs(&mut self, ui: &mut egui::Ui, changed: &mut bool) {
        let total = self.weapon_params.rasta_npcs.len();
        if self.category_details("weapon_params") {
            if ui.button("← Back to List").clicked() { self.back_category_list("weapon_params"); return; }
            let sel = self.weapon_params_selected.min(total.saturating_sub(1));
            ui.heading(format!("Edit Rasta NPC #{sel}"));
            ui.separator();
            if total == 0 { ui.label("No NPC table loaded."); return; }
            let n = &mut self.weapon_params.rasta_npcs[sel];
            ui.horizontal(|ui| { ui.label("Companion type:"); *changed |= ui.add(egui::DragValue::new(&mut n.companion_type)).changed(); });
            for (k, v) in n.visual.iter_mut().enumerate() {
                ui.horizontal(|ui| { ui.label(format!("Visual {}:", k + 1)); *changed |= ui.add(egui::DragValue::new(v)).changed(); });
            }
            return;
        }
        MhfdatApp::section_header(ui, &format!("Rasta NPCs (found: {total})"), |_| {});
        if total == 0 { ui.colored_label(egui::Color32::YELLOW, "Warning: No Rasta NPCs found!"); return; }
        let (start, end, pages) = crate::app::list_window(&mut self.weapon_params_page, total);
        let rows: Vec<(usize, u8)> = (start..end).map(|i| (i, self.weapon_params.rasta_npcs[i].companion_type)).collect();
        MhfdatApp::entity_list(ui, "rasta_npc_scroll", "NPC", start, end, total, |ui| {
            egui::Grid::new("rasta_npc_list").striped(true).show(ui, |ui| {
                ui.label("ID"); ui.label("Companion type"); ui.end_row();
                for (i, kind) in &rows {
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.weapon_params_selected = *i;
                        self.open_category_details("weapon_params");
                    }
                    ui.label(kind.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.weapon_params_page, pages);
    }

    fn show_rasta_weapons(&mut self, ui: &mut egui::Ui, changed: &mut bool) {
        let total = self.weapon_params.rasta_weapons.len();
        if self.category_details("weapon_params") {
            if ui.button("← Back to List").clicked() { self.back_category_list("weapon_params"); return; }
            let sel = self.weapon_params_selected.min(total.saturating_sub(1));
            ui.heading(format!("Edit Rasta weapon #{sel}"));
            ui.separator();
            if total == 0 { ui.label("No weapon options loaded."); return; }
            let w = &mut self.weapon_params.rasta_weapons[sel];
            ui.horizontal(|ui| { ui.label("Companion type:"); *changed |= ui.add(egui::DragValue::new(&mut w.companion_type)).changed(); });
            ui.horizontal(|ui| { ui.label("Weapon type:"); *changed |= ui.add(egui::DragValue::new(&mut w.weapon_type)).changed(); ui.label(WEAPON_TYPE_NAMES.get(w.weapon_type as usize).copied().unwrap_or("?")); });
            ui.horizontal(|ui| { ui.label("Level min:"); *changed |= ui.add(egui::DragValue::new(&mut w.level_min)).changed(); });
            ui.horizontal(|ui| { ui.label("Level max:"); *changed |= ui.add(egui::DragValue::new(&mut w.level_max)).changed(); });
            ui.horizontal(|ui| { ui.label("Weapon ID:"); *changed |= ui.add(egui::DragValue::new(&mut w.weapon_id)).changed(); });
            return;
        }
        MhfdatApp::section_header(ui, &format!("Rasta weapons (found: {total})"), |_| {});
        if total == 0 { ui.colored_label(egui::Color32::YELLOW, "Warning: No Rasta weapons found!"); return; }
        let (start, end, pages) = crate::app::list_window(&mut self.weapon_params_page, total);
        let rows: Vec<(usize, u8, u16)> = (start..end).map(|i| {
            let w = &self.weapon_params.rasta_weapons[i];
            (i, w.weapon_type, w.weapon_id)
        }).collect();
        MhfdatApp::entity_list(ui, "rasta_weapon_scroll", "Weapon", start, end, total, |ui| {
            egui::Grid::new("rasta_weapon_list").striped(true).show(ui, |ui| {
                ui.label("ID"); ui.label("Type"); ui.label("Weapon ID"); ui.end_row();
                for (i, kind, id) in &rows {
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.weapon_params_selected = *i;
                        self.open_category_details("weapon_params");
                    }
                    ui.label(WEAPON_TYPE_NAMES.get(*kind as usize).copied().unwrap_or("?"));
                    ui.label(id.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.weapon_params_page, pages);
    }

    fn show_rasta_armors(&mut self, ui: &mut egui::Ui, changed: &mut bool) {
        let total = self.weapon_params.rasta_armors.len();
        if self.category_details("weapon_params") {
            if ui.button("← Back to List").clicked() { self.back_category_list("weapon_params"); return; }
            let sel = self.weapon_params_selected.min(total.saturating_sub(1));
            ui.heading(format!("Edit Rasta armor #{sel}"));
            ui.separator();
            ui.label("Subtype 6 = special pool.");
            if total == 0 { ui.label("No armor sets loaded."); return; }
            let a = &mut self.weapon_params.rasta_armors[sel];
            ui.horizontal(|ui| { ui.label("Companion type:"); *changed |= ui.add(egui::DragValue::new(&mut a.companion_type)).changed(); });
            ui.horizontal(|ui| { ui.label("Subtype:"); *changed |= ui.add(egui::DragValue::new(&mut a.subtype)).changed(); });
            ui.horizontal(|ui| { ui.label("Level min:"); *changed |= ui.add(egui::DragValue::new(&mut a.level_min)).changed(); });
            ui.horizontal(|ui| { ui.label("Level max:"); *changed |= ui.add(egui::DragValue::new(&mut a.level_max)).changed(); });
            ui.horizontal(|ui| { ui.label("Piece type 2:"); *changed |= ui.add(egui::DragValue::new(&mut a.slot2_id)).changed(); });
            ui.horizontal(|ui| { ui.label("Piece type 3:"); *changed |= ui.add(egui::DragValue::new(&mut a.slot3_id)).changed(); });
            ui.horizontal(|ui| { ui.label("Piece type 4:"); *changed |= ui.add(egui::DragValue::new(&mut a.slot4_id)).changed(); });
            ui.horizontal(|ui| { ui.label("Piece type 5:"); *changed |= ui.add(egui::DragValue::new(&mut a.slot5_id)).changed(); });
            ui.horizontal(|ui| { ui.label("Piece type 0:"); *changed |= ui.add(egui::DragValue::new(&mut a.slot0_id)).changed(); });
            return;
        }
        MhfdatApp::section_header(ui, &format!("Rasta armor (found: {total})"), |_| {});
        if total == 0 { ui.colored_label(egui::Color32::YELLOW, "Warning: No Rasta armor found!"); return; }
        let (start, end, pages) = crate::app::list_window(&mut self.weapon_params_page, total);
        let rows: Vec<(usize, u8, u8)> = (start..end).map(|i| {
            let a = &self.weapon_params.rasta_armors[i];
            (i, a.companion_type, a.subtype)
        }).collect();
        MhfdatApp::entity_list(ui, "rasta_armor_scroll", "Set", start, end, total, |ui| {
            egui::Grid::new("rasta_armor_list").striped(true).show(ui, |ui| {
                ui.label("ID"); ui.label("Companion"); ui.label("Subtype"); ui.end_row();
                for (i, kind, subtype) in &rows {
                    if ui.selectable_label(false, i.to_string()).clicked() {
                        self.weapon_params_selected = *i;
                        self.open_category_details("weapon_params");
                    }
                    ui.label(kind.to_string());
                    ui.label(subtype.to_string());
                    ui.end_row();
                }
            });
        });
        MhfdatApp::pagination_controls(ui, &mut self.weapon_params_page, pages);
    }
}
