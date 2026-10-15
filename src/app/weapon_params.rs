use super::*;
use crate::app::caravan::{in_columns, page_columns};
use crate::model::mhfdat::{DrawSheatheMotion, PoseXform, WeaponParamsRow};

/// Weapon classes, in the order of the weapon params and vampirism tables (the class the game reads at player+3).
pub const WEAPON_CLASS_NAMES: [&str; 14] = [
    "Great Sword", "Heavy Bowgun", "Hammer", "Lance", "Sword and Shield", "Light Bowgun", "Dual Blades",
    "Long Sword", "Hunting Horn", "Gunlance", "Bow", "Tonfa", "Switch Axe F", "Magnet Spike",
];

/// Name of what a weapon upgrade bonus slot value stands for (0-5 defense, 6-11 element, 12-17 attack).
fn bonus_slot_label(v: i16) -> String {
    match v {
        0..=5 => format!("Defense Lv{v}"),
        6..=11 => format!("Element Lv{}", v - 6),
        12..=17 => format!("Attack Lv{}", v - 12),
        _ => "?".to_string(),
    }
}

fn xyz_row(ui: &mut egui::Ui, label: &str, v: &mut [f32; 3]) -> bool {
    ui.label(label);
    let mut changed = false;
    ui.horizontal(|ui| {
        for x in v.iter_mut() {
            changed |= ui.add(egui::DragValue::new(x).speed(0.01)).changed();
        }
    });
    ui.end_row();
    changed
}

fn pose_rows(ui: &mut egui::Ui, id: (&str, usize, usize), title: &str, pose: &mut PoseXform) -> bool {
    let mut changed = false;
    ui.label(egui::RichText::new(title).strong());
    egui::Grid::new(id).num_columns(2).show(ui, |ui| {
        changed |= xyz_row(ui, "Position", &mut pose.pos);
        changed |= xyz_row(ui, "Rotation", &mut pose.rot);
    });
    changed
}

fn motion_rows(ui: &mut egui::Ui, id: (&str, usize, usize), title: &str, m: &mut DrawSheatheMotion) -> bool {
    let mut changed = false;
    ui.label(egui::RichText::new(title).strong());
    egui::Grid::new(id).num_columns(2).show(ui, |ui| {
        changed |= MhfdatApp::num_row(ui, "Motion id", &mut m.motion_id);
        changed |= MhfdatApp::num_row(ui, "Duration", &mut m.duration);
        changed |= MhfdatApp::num_row(ui, "Tick action", &mut m.tick_action);
    });
    changed
}

impl MhfdatApp {
    /// Weapon params (header 0x65C): one row per weapon class, read by the skills and by the weapon draw / HUD code. The table
    /// keeps its size; the bytes the pattern does not name are kept as they are.
    pub(in crate::app) fn show_weapon_params(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "weapon");
        if self.category_details("weapon_params") {
            self.show_weapon_params_details(ui);
            return;
        }
        let total = self.weapon_params.len();
        MhfdatApp::section_header(ui, &format!("Weapon params (found: {total})"), |_| {});
        ui.label("One row per weapon class: true raw coefficient, draw and sheathe motions, HUD view and the numbers of several skills (Fencing, Red Soul, Thunder Clad, Rush, Consumption Slayer, Ceaseless).");
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No weapon params table found!");
            return;
        }
        egui::ScrollArea::vertical().id_source("weapon_params_list_scroll").show(ui, |ui| {
            egui::Grid::new("weapon_params_list").striped(true).show(ui, |ui| {
                for h in ["ID", "Weapon class", "Kind", "True raw coefficient"] {
                    ui.label(h);
                }
                ui.end_row();
                for idx in 0..total {
                    let (flags, coefficient) = (self.weapon_params[idx].weapon_class_flags, self.weapon_params[idx].weapon_coefficient);
                    if ui.selectable_label(self.weapon_params_selected == Some(idx), idx.to_string()).clicked() {
                        self.weapon_params_selected = Some(idx);
                        self.open_category_details("weapon_params");
                    }
                    ui.label(WEAPON_CLASS_NAMES.get(idx).copied().unwrap_or("?"));
                    ui.label(match flags {
                        1 => "melee".to_string(),
                        2 => "gunner".to_string(),
                        f => format!("flags {f}"),
                    });
                    ui.label(coefficient.to_string());
                    ui.end_row();
                }
            });
        });
    }

    fn show_weapon_params_details(&mut self, ui: &mut egui::Ui) {
        if ui.button("← Back to List").clicked() {
            self.back_category_list("weapon_params");
            return;
        }
        let Some(sel) = self.weapon_params_selected.filter(|&i| i < self.weapon_params.len()) else {
            ui.label("No weapon class selected");
            return;
        };
        ui.heading(format!("Edit Weapon params #{sel} ({})", WEAPON_CLASS_NAMES.get(sel).copied().unwrap_or("?")));
        ui.separator();
        let w = &mut self.weapon_params[sel];
        let mut changed = false;
        egui::ScrollArea::vertical().id_source("weapon_params_details_scroll").show(ui, |ui| {
            let columns = page_columns(ui, 420.0, 3);
            in_columns(ui, columns, 3, |ui, section| match section {
                0 => changed |= general_and_draw_section(ui, sel, w),
                1 => changed |= skills_section(ui, sel, w),
                _ => changed |= hud_and_poses_section(ui, sel, w),
            });
        });
        if changed {
            self.weapon_params_modified = true;
        }
    }

    /// Weapon upgrade bonus slots (header 0xA8): 11 rows of 4 slots. The table keeps its size.
    pub(in crate::app) fn show_weapon_bonus_slots(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "weapon");
        let total = self.weapon_bonus_slots.len();
        MhfdatApp::section_header(ui, &format!("Weapon upgrade bonus slots (found: {total})"), |_| {});
        ui.label("Each slot is a bonus kind and level: 0-5 Defense, 6-11 Element, 12-17 Attack (level = value modulo 6).");
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No bonus slot table found!");
            return;
        }
        let mut changed = false;
        egui::ScrollArea::vertical().id_source("bonus_slots_scroll").show(ui, |ui| {
            egui::Grid::new("bonus_slots_grid").striped(true).show(ui, |ui| {
                ui.label("Row");
                for k in 1..=4 {
                    ui.label(format!("Slot {k}"));
                    ui.label("");
                }
                ui.end_row();
                for (i, row) in self.weapon_bonus_slots.iter_mut().enumerate() {
                    ui.label(i.to_string());
                    for slot in row.slots.iter_mut() {
                        changed |= ui.add(egui::DragValue::new(slot)).changed();
                        ui.label(bonus_slot_label(*slot));
                    }
                    ui.end_row();
                }
            });
        });
        if changed {
            self.weapon_bonus_slots_modified = true;
        }
    }

    /// Coating modifiers (header 0xB0): 23 rows, one coating delta each. The table keeps its size.
    pub(in crate::app) fn show_coating_modifiers(&mut self, ui: &mut egui::Ui) {
        self.show_table_problems(ui, "weapon");
        let total = self.coating_modifiers.len();
        MhfdatApp::section_header(ui, &format!("Coating modifiers (found: {total})"), |_| {});
        ui.label("Coating delta added for each row; the 22 bytes after it are never read and are kept as they are.");
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No coating modifier table found!");
            return;
        }
        let mut changed = false;
        egui::ScrollArea::vertical().id_source("coating_scroll").show(ui, |ui| {
            egui::Grid::new("coating_grid").striped(true).show(ui, |ui| {
                ui.label("Row");
                ui.label("Delta");
                ui.end_row();
                for (i, row) in self.coating_modifiers.iter_mut().enumerate() {
                    ui.label(i.to_string());
                    changed |= ui.add(egui::DragValue::new(&mut row.delta)).changed();
                    ui.end_row();
                }
            });
        });
        if changed {
            self.coating_modifiers_modified = true;
        }
    }
}

fn general_and_draw_section(ui: &mut egui::Ui, sel: usize, w: &mut WeaponParamsRow) -> bool {
    let mut changed = false;
    ui.strong("General");
    egui::Grid::new(("wp_general", sel)).num_columns(2).show(ui, |ui| {
        changed |= MhfdatApp::num_row(ui, "Class flags (1 melee, 2 gunner)", &mut w.weapon_class_flags);
        changed |= MhfdatApp::num_row(ui, "Melee hold-input gate", &mut w.melee_hold_input_gate);
        changed |= MhfdatApp::num_row(ui, "True raw coefficient", &mut w.weapon_coefficient);
        changed |= MhfdatApp::float_row(ui, "On-hit multiplier", &mut w.onhit_multiplier);
        changed |= MhfdatApp::num_row(ui, "Charge HUD seed", &mut w.charge_hud_seed);
        changed |= MhfdatApp::num_row(ui, "Adaptation gauge seed", &mut w.adaptation_gauge_seed);
        changed |= MhfdatApp::num_row(ui, "Recovery action (motion 0x3EC)", &mut w.recovery_motion_3ec_action);
        changed |= MhfdatApp::num_row(ui, "Turn rate states 0xC8 / 0xF1", &mut w.turn_rate_states_c8_f1);
        changed |= MhfdatApp::num_row(ui, "Category switch penalty", &mut w.category_switch_penalty);
    });
    ui.separator();
    ui.strong("Draw and sheathe");
    changed |= motion_rows(ui, ("wp_draw", sel, 0), "Draw (action 4)", &mut w.draw);
    changed |= motion_rows(ui, ("wp_draw", sel, 1), "Draw alt (action 0x17)", &mut w.draw_alt);
    changed |= motion_rows(ui, ("wp_draw", sel, 2), "Sheathe (action 5 / 0xF / 0x10)", &mut w.sheathe);
    changed |= motion_rows(ui, ("wp_draw", sel, 3), "Sheathe alt (action 0x0A)", &mut w.sheathe_alt);
    changed
}

fn skills_section(ui: &mut egui::Ui, sel: usize, w: &mut WeaponParamsRow) -> bool {
    let mut changed = false;
    ui.strong("Skills");
    egui::Grid::new(("wp_skills", sel)).num_columns(2).show(ui, |ui| {
        ui.label(egui::RichText::new("Fencing+2 (367 / 368)").strong());
        ui.end_row();
        changed |= MhfdatApp::num_row(ui, "Hitstop frames", &mut w.melee_hitstop_frames);
        changed |= MhfdatApp::num_row(ui, "Hitstop reduction", &mut w.melee_hitstop_reduction);
        changed |= MhfdatApp::num_row(ui, "Bonus-hit sound", &mut w.fencing_bonus_hit_sound);
        ui.label(egui::RichText::new("Red Soul (458)").strong());
        ui.end_row();
        changed |= MhfdatApp::num_row(ui, "Gauge threshold", &mut w.red_soul_gauge_threshold);
        changed |= MhfdatApp::num_row(ui, "Tier bonus", &mut w.red_soul_tier_bonus);
        ui.label(egui::RichText::new("Thunder Clad (494)").strong());
        ui.end_row();
        changed |= MhfdatApp::num_row(ui, "Gauge decay", &mut w.thunder_clad_gauge_decay);
        ui.label(egui::RichText::new("Rush (527 / 528)").strong());
        ui.end_row();
        changed |= MhfdatApp::num_row(ui, "On-hit gauge multiplier", &mut w.rush_onhit_gauge_mult);
        changed |= MhfdatApp::num_row(ui, "Gauge increment", &mut w.rush_gauge_increment);
        changed |= MhfdatApp::num_row(ui, "Tier 1 bonus", &mut w.rush_tier1_bonus);
        changed |= MhfdatApp::num_row(ui, "Tier 2 bonus", &mut w.rush_tier2_bonus);
        // the same two bytes are Rush tier 3 (s16) and the Consumption Slayer thresholds (two u8)
        let mut tier3 = i16::from_le_bytes(w.rush_t3_or_slayer);
        if MhfdatApp::num_row(ui, "Tier 3 bonus", &mut tier3) {
            w.rush_t3_or_slayer = tier3.to_le_bytes();
            changed = true;
        }
        ui.label(egui::RichText::new("Consumption Slayer (505), same two bytes as Rush tier 3").strong());
        ui.end_row();
        changed |= MhfdatApp::num_row(ui, "Threshold, skill off", &mut w.rush_t3_or_slayer[0]);
        changed |= MhfdatApp::num_row(ui, "Threshold, skill on", &mut w.rush_t3_or_slayer[1]);
        ui.label(egui::RichText::new("Ceaseless (520)").strong());
        ui.end_row();
        changed |= MhfdatApp::num_row(ui, "Timer", &mut w.ceaseless_timer);
        for k in 0..3 {
            changed |= MhfdatApp::num_row(ui, &format!("Threshold {k}"), &mut w.ceaseless_thresh[k]);
        }
    });
    changed
}

fn hud_and_poses_section(ui: &mut egui::Ui, sel: usize, w: &mut WeaponParamsRow) -> bool {
    let mut changed = false;
    ui.strong("HUD weapon view");
    egui::Grid::new(("wp_hud", sel)).num_columns(2).show(ui, |ui| {
        changed |= MhfdatApp::float_row(ui, "Default Z", &mut w.hud_view_default_z);
        changed |= xyz_row(ui, "Position", &mut w.hud_view_pos);
        changed |= xyz_row(ui, "Position addend", &mut w.hud_view_pos_add);
        changed |= xyz_row(ui, "Rotation", &mut w.hud_view_rot);
        changed |= xyz_row(ui, "Rotation addend", &mut w.hud_view_rot_add);
    });
    ui.separator();
    ui.strong("Poses");
    changed |= pose_rows(ui, ("wp_pose", sel, 0), "Variant 0", &mut w.pose_variant0);
    changed |= pose_rows(ui, ("wp_pose", sel, 1), "Variant 1", &mut w.pose_variant1);
    changed |= pose_rows(ui, ("wp_pose", sel, 2), "Variant 2", &mut w.pose_variant2);
    changed |= pose_rows(ui, ("wp_pose", sel, 3), "Sheathe / draw override", &mut w.pose_sheathe_draw);
    changed
}
