use super::*;
use crate::app::caravan::{in_columns, page_columns};
use crate::app::weapon_params::WEAPON_CLASS_NAMES;
use crate::model::mhfdat::{ObscurityParams, ObscurityUpParams};

impl MhfdatApp {
    /// Vampirism (header 0x7A0): heal and proc chance of Vampirism +1 / +2 for each weapon class. The table keeps its size.
    pub(in crate::app) fn show_vampirism(&mut self, ui: &mut egui::Ui) {
        let total = self.vampirism.len();
        MhfdatApp::section_header(ui, &format!("Vampirism (found: {total} rows)"), |_| {});
        ui.label("On a hit, Vampirism heals max(1, damage x heal % / 100) with a proc chance in %. Rows are weapon class x 2 + tier (+1 then +2).");
        if total == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No vampirism table found!");
            return;
        }
        let mut changed = false;
        egui::ScrollArea::vertical().id_source("vampirism_scroll").show(ui, |ui| {
            egui::Grid::new("vampirism_grid").striped(true).show(ui, |ui| {
                for h in ["Weapon class", "+1 heal %", "+1 proc %", "+2 heal %", "+2 proc %"] {
                    ui.label(h);
                }
                ui.end_row();
                for class in 0..total.div_ceil(2) {
                    ui.label(format!("{class} {}", WEAPON_CLASS_NAMES.get(class).copied().unwrap_or("?")));
                    for tier in 0..2 {
                        match self.vampirism.get_mut(class * 2 + tier) {
                            Some(row) => {
                                changed |= ui.add(egui::DragValue::new(&mut row.heal_percent)).changed();
                                changed |= ui.add(egui::DragValue::new(&mut row.proc_chance)).changed();
                            }
                            None => {
                                ui.label("-");
                                ui.label("-");
                            }
                        }
                    }
                    ui.end_row();
                }
            });
        });
        if changed {
            self.vampirism_modified = true;
        }
    }

    /// Crit Conversion (header 0xA1C): rate and affinity of the two tiers. Only the first 8 bytes belong to the skill.
    pub(in crate::app) fn show_crit_conversion(&mut self, ui: &mut egui::Ui) {
        MhfdatApp::section_header(ui, "Crit Conversion", |_| {});
        let Some(c) = self.crit_conversion.first_mut() else {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No crit conversion table found!");
            return;
        };
        ui.label("When the affinity is over 100, attack becomes ATK + trunc(sqrt(affinity - 100) x rate x 0.001). The affinity bonus is added on top.");
        let mut changed = false;
        egui::Grid::new("crit_conversion_grid").num_columns(2).show(ui, |ui| {
            changed |= MhfdatApp::num_row(ui, "Tier 1 rate (skill 499)", &mut c.rate[0]);
            changed |= MhfdatApp::num_row(ui, "Tier 2 rate (skill 500)", &mut c.rate[1]);
            changed |= MhfdatApp::num_row(ui, "Tier 1 affinity bonus", &mut c.affinity[0]);
            changed |= MhfdatApp::num_row(ui, "Tier 2 affinity bonus", &mut c.affinity[1]);
        });
        ui.label(format!("Rates as multipliers: {:.3} and {:.3}.", c.rate[0] as f32 * 0.001, c.rate[1] as f32 * 0.001));
        if changed {
            self.crit_conversion_modified = true;
        }
    }
}

impl MhfdatApp {
    /// Rush (skill 516): the gauge thresholds of the stages and, for Rush UP (the Zenith version), the third stage. The attack each
    /// weapon class gains at stage 1 and 2 and the gauge gained per hit and per block are in Weapons > Weapon Params.
    pub(in crate::app) fn show_rush(&mut self, ui: &mut egui::Ui) {
        MhfdatApp::section_header(ui, "Rush", |_| {});
        let Some(r) = self.rush.first_mut() else {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No rush table found!");
            return;
        };
        ui.label("Successful attacks and blocks fill an invisible gauge. Stage 1 starts at the first threshold and stage 2 at the second one, which is also where the gauge stops. The attack each weapon class gains at those stages and the gauge it gains per hit and per block are in Weapons > Weapon Params.");
        let mut changed = false;
        egui::Grid::new("rush_grid").num_columns(2).show(ui, |ui| {
            changed |= MhfdatApp::num_row(ui, "Stage 1 gauge threshold", &mut r.stage1_threshold);
            changed |= MhfdatApp::num_row(ui, "Stage 2 gauge threshold (gauge cap)", &mut r.stage2_threshold);
        });
        ui.separator();
        ui.label(egui::RichText::new("Rush UP (Zenith skill)").strong());
        ui.label("Rush UP adds a third stage to the gauge. Its attack bonus is the same for every weapon class.");
        egui::Grid::new("rush_up_grid").num_columns(2).show(ui, |ui| {
            changed |= MhfdatApp::num_row(ui, "Stage 3 attack bonus", &mut r.up_stage3_bonus);
            changed |= MhfdatApp::num_row(ui, "Stage 3 gauge threshold", &mut r.up_stage3_threshold);
            changed |= MhfdatApp::float_row(ui, "Stage 3 timer start value", &mut r.up_stage3_timer);
        });
        if changed {
            self.rush_modified = true;
        }
    }

    /// Obscurity (skill 514): attack gained for every blocked attack. The client reads the header 0xAA0 tables for Obscurity and
    /// the header 0xB78 tables for Obscurity UP, the Zenith version of the skill.
    pub(in crate::app) fn show_obscurity(&mut self, ui: &mut egui::Ui) {
        MhfdatApp::section_header(ui, "Obscurity (skill 514)", |_| {});
        let (Some(points), Some(up)) = (self.obscurity.first_mut(), self.obscurity_up.first_mut()) else {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No obscurity table found!");
            return;
        };
        self.obscurity_class = self.obscurity_class.min(WEAPON_CLASS_NAMES.len() - 1);
        let class = &mut self.obscurity_class;
        let mut changed = false;
        let mut up_changed = false;
        egui::ScrollArea::vertical().id_source("obscurity_scroll").show(ui, |ui| {
            ui.label("Every blocked attack adds a stack; a hit from a monster resets them. The first half of the stacks counts double. \"At max\" is the attack gained at the most stacks.");
            ui.horizontal_wrapped(|ui| {
                ui.label("Delay between two stacks (ticks):");
                changed |= obscurity_drag(ui, &mut points.stack_delay, 0..=i16::MAX);
                ui.weak("the same for Obscurity and Obscurity UP");
            });
            ui.add_space(6.0);
            let columns = page_columns(ui, 580.0, 2);
            in_columns(ui, columns, 2, |ui, section| match section {
                0 => changed |= obscurity_section(ui, points),
                _ => up_changed |= obscurity_up_section(ui, up, class),
            });
        });
        if changed {
            self.obscurity_modified = true;
        }
        if up_changed {
            self.obscurity_up_modified = true;
        }
    }
}

const OBSCURITY_CELL_W: f32 = 84.0;
const OBSCURITY_ROW_H: f32 = 22.0;

/// A number cell of the Obscurity tables: every cell has the same size so the columns line up under their headers.
fn obscurity_drag(ui: &mut egui::Ui, value: &mut i16, range: std::ops::RangeInclusive<i16>) -> bool {
    ui.add_sized([OBSCURITY_CELL_W, OBSCURITY_ROW_H], egui::DragValue::new(value).clamp_range(range)).changed()
}

/// Column header: as wide as the cells under it, long titles go on two lines.
fn obscurity_header(ui: &mut egui::Ui, text: &str, hint: &str) {
    ui.allocate_ui_with_layout(egui::vec2(OBSCURITY_CELL_W, 38.0), egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.set_min_size(egui::vec2(OBSCURITY_CELL_W, 38.0));
        ui.set_max_width(OBSCURITY_CELL_W);
        ui.add(egui::Label::new(egui::RichText::new(text).strong().size(13.0)).wrap(true)).on_hover_text(hint);
    });
}

/// Attack of a computed cell, drawn like a value but not editable.
fn obscurity_total(ui: &mut egui::Ui, total: i32) {
    ui.add_sized([OBSCURITY_CELL_W, OBSCURITY_ROW_H], egui::Label::new(egui::RichText::new(format!("+{total}")).strong()));
}

/// Obscurity: every stack gives the same attack; the first half of the stacks counts double (skill_obscurity_apply_attack_bonus).
fn obscurity_points_total(attack_per_stack: i16, max_stacks: i16) -> i32 {
    let max = i32::from(max_stacks.max(0));
    let half = max / 2;
    (max - half) * i32::from(attack_per_stack) + half * i32::from(attack_per_stack) * 2
}

/// Obscurity UP: every stack has its own attack, the first half counts double (skill_obscurity_apply_stacked_attack_bonus).
fn obscurity_up_total(values: &[i16], first: usize, max_stacks: i16) -> i32 {
    let max = max_stacks.max(0) as usize;
    let half = max / 2;
    (0..max).map(|k| values.get(first + k).map_or(0, |&v| i32::from(v) * if k < half { 2 } else { 1 })).sum()
}

fn obscurity_section(ui: &mut egui::Ui, points: &mut ObscurityParams) -> bool {
    let mut changed = false;
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Obscurity").heading());
            ui.weak("header 0xAA0");
        });
        egui::Grid::new("obscurity_points_grid").striped(true).spacing([6.0, 4.0]).show(ui, |ui| {
            ui.label("");
            obscurity_header(ui, "Attack per stack", "Attack gained by each stack");
            obscurity_header(ui, "Most stacks", "A class with 0 does not use the skill");
            obscurity_header(ui, "At max", "Attack at the most stacks: the first half of the stacks counts double");
            ui.end_row();
            for class in 0..points.attack_per_stack.len() {
                ui.label(WEAPON_CLASS_NAMES.get(class).copied().unwrap_or("?"));
                changed |= obscurity_drag(ui, &mut points.attack_per_stack[class], i16::MIN..=i16::MAX);
                changed |= obscurity_drag(ui, &mut points.max_stacks[class], 0..=255);
                obscurity_total(ui, obscurity_points_total(points.attack_per_stack[class], points.max_stacks[class]));
                ui.end_row();
            }
        });
    });
    changed
}

fn obscurity_up_section(ui: &mut egui::Ui, up: &mut ObscurityUpParams, class: &mut usize) -> bool {
    let mut changed = false;
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Obscurity UP").heading());
            ui.weak("the Zenith version, header 0xB78");
        });
        egui::Grid::new("obscurity_up_grid").striped(true).spacing([6.0, 4.0]).show(ui, |ui| {
            ui.label("");
            obscurity_header(ui, "Most stacks", "A class with 0 has no stack in Obscurity UP");
            obscurity_header(ui, "At max", "Attack at the most stacks: the first half of the stacks counts double");
            obscurity_header(ui, "Sharpness", "Sharpness amount used when the stacks are already at their maximum");
            obscurity_header(ui, "Sharpness (alt.)", "Used instead when another condition of the client is met (the client also adds 2 extra stacks per block then)");
            ui.end_row();
            let mut first = 0usize;
            for c in 0..up.max_stacks.len() {
                let name = WEAPON_CLASS_NAMES.get(c).copied().unwrap_or("?").to_string();
                let name = if up.max_stacks[c] == 0 { egui::RichText::new(name).weak() } else { egui::RichText::new(name) };
                if ui.selectable_label(*class == c, name).on_hover_text("Edit the attack of each stack of this class below").clicked() {
                    *class = c;
                }
                changed |= obscurity_drag(ui, &mut up.max_stacks[c], 0..=255);
                if up.max_stacks[c] == 0 {
                    ui.add_sized([OBSCURITY_CELL_W, OBSCURITY_ROW_H], egui::Label::new(egui::RichText::new("-").weak()));
                } else {
                    obscurity_total(ui, obscurity_up_total(&up.stack_bonus, first, up.max_stacks[c]));
                }
                changed |= obscurity_drag(ui, &mut up.sharpness[c], i16::MIN..=i16::MAX);
                changed |= obscurity_drag(ui, &mut up.sharpness_alt[c], i16::MIN..=i16::MAX);
                ui.end_row();
                first += up.max_stacks[c].max(0) as usize;
            }
        });
        ui.separator();
        changed |= obscurity_stacks(ui, up, *class);
    });
    changed
}

/// The attack of each stack of one weapon class of Obscurity UP. The stacks of all the classes are one list in the file: a
/// class takes as many values as its most stacks, after the classes before it.
fn obscurity_stacks(ui: &mut egui::Ui, up: &mut ObscurityUpParams, class: usize) -> bool {
    let mut changed = false;
    let first: usize = up.max_stacks[..class].iter().map(|&m| m.max(0) as usize).sum();
    let n = up.max_stacks[class].max(0) as usize;
    let room = up.stack_bonus.len();
    let used: usize = up.max_stacks.iter().map(|&m| m.max(0) as usize).sum();
    ui.label(egui::RichText::new(format!("Attack of each stack: {}", WEAPON_CLASS_NAMES[class])).strong());
    if n == 0 {
        ui.weak("This weapon class has no stack in Obscurity UP (most stacks 0). Pick another class above.");
    } else {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for k in 0..n {
                ui.vertical(|ui| {
                    let text = if k < n / 2 { format!("Stack {} x2", k + 1) } else { format!("Stack {}", k + 1) };
                    ui.add_sized([OBSCURITY_CELL_W, 16.0], egui::Label::new(egui::RichText::new(text).small()));
                    match up.stack_bonus.get_mut(first + k) {
                        Some(v) => changed |= obscurity_drag(ui, v, i16::MIN..=i16::MAX),
                        None => {
                            ui.colored_label(egui::Color32::from_rgb(220, 70, 70), "no room");
                        }
                    }
                });
            }
        });
    }
    ui.add_space(4.0);
    if used > room {
        ui.colored_label(egui::Color32::from_rgb(220, 70, 70), format!("The most stacks add up to {used}, the file holds {room} stack values: this cannot be saved."));
    } else {
        ui.weak(format!("Stack values of all the classes: {used} of {room} used."));
    }
    changed
}
