use super::*;
use egui;
use crate::model::mhfjmp::{MenuEntry, Area, StringEntry};
use crate::core::mhfjmp::load_mhfjmp_bin_from_buffer;
use std::path::PathBuf;
use serde_json;

pub enum MhfjmpTab {
    MenuEntries,
    Areas,
    Strings,
}

impl Default for MhfjmpTab {
    fn default() -> Self {
        MhfjmpTab::MenuEntries
    }
}

pub struct MhfjmpApp {
    pub on_back: Option<Box<dyn FnMut()>>,
    pub tab: MhfjmpTab,
    pub entries: Vec<MenuEntry>,
    pub selected_index: Option<usize>,
    pub areas: Vec<Area>,
    pub selected_area_index: Option<usize>,
    pub strings: Vec<StringEntry>,
    pub selected_string_index: Option<usize>,
    pub current_file: Option<PathBuf>,
    pub error_message: Option<String>,
}

impl Default for MhfjmpApp {
    fn default() -> Self {
        MhfjmpApp {
            on_back: None,
            tab: MhfjmpTab::MenuEntries,
            entries: Vec::new(),
            selected_index: None,
            areas: Vec::new(),
            selected_area_index: None,
            strings: Vec::new(),
            selected_string_index: None,
            current_file: None,
            error_message: None,
        }
    }
}

impl App for MhfjmpApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Back").clicked() {
                    if let Some(cb) = &mut self.on_back {
                        cb();
                    }
                }
                if ui.button("Exporter JSON").clicked() {
                    if let Some(current_file) = &self.current_file {
                        if let Some(parent) = current_file.parent() {
                            let json_path = parent.join("mhfjmp_export.json");
                            let export = serde_json::json!({
                                "entries": &self.entries,
                                "areas": &self.areas,
                                "strings": &self.strings,
                            });
                            if let Ok(json) = serde_json::to_string_pretty(&export) {
                                std::fs::write(json_path, json).ok();
                            }
                        }
                    }
                }
                if ui.button("Importer JSON").clicked() {
                    if let Some(path) = rfd::FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
                        if let Ok(json) = std::fs::read_to_string(path) {
                            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json) {
                                if let Some(entries) = data.get("entries") {
                                    if let Ok(entries_vec) = serde_json::from_value::<Vec<MenuEntry>>(entries.clone()) {
                                        self.entries = entries_vec;
                                    }
                                }
                                if let Some(areas) = data.get("areas") {
                                    if let Ok(areas_vec) = serde_json::from_value::<Vec<Area>>(areas.clone()) {
                                        self.areas = areas_vec;
                                    }
                                }
                                if let Some(strings) = data.get("strings") {
                                    if let Ok(strings_vec) = serde_json::from_value::<Vec<StringEntry>>(strings.clone()) {
                                        self.strings = strings_vec;
                                    }
                                }
                            }
                        }
                    }
                }
            });
            self.show_mhfjmp_tab(ui);
        });
    }
}

impl MhfjmpApp {
    fn show_mhfjmp_tab(&mut self, ui: &mut egui::Ui) {
        egui::TopBottomPanel::top("mhfjmp_tabs").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(matches!(self.tab, MhfjmpTab::MenuEntries), "Menu Entry").clicked() {
                    self.tab = MhfjmpTab::MenuEntries;
                }
                if ui.selectable_label(matches!(self.tab, MhfjmpTab::Areas), "Area Entry").clicked() {
                    self.tab = MhfjmpTab::Areas;
                }
                if ui.selectable_label(matches!(self.tab, MhfjmpTab::Strings), "Strings").clicked() {
                    self.tab = MhfjmpTab::Strings;
                }
            });
        });
        match self.tab {
            MhfjmpTab::MenuEntries => self.show_menu_entries_tab(ui),
            MhfjmpTab::Areas => self.show_areas_tab(ui),
            MhfjmpTab::Strings => self.show_strings_tab(ui),
        }
    }

    fn show_menu_entries_tab(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("Menu Entries")
            .default_open(true)
            .show(ui, |ui| {
                egui::Grid::new("menuentry_grid")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("#");
                        ui.label("Title");
                        ui.label("Jump ID");
                        ui.label("Area IDs");
                        ui.label(""); // Edit
                        ui.end_row();
                        for (i, entry) in self.entries.iter().enumerate() {
                            ui.label(format!("{}", i + 1));
                            ui.label(&entry.title);
                            ui.label(format!("{}", entry.jump_id));
                            ui.label(format!("{}, {}, {}, {}", entry.area_id, entry.area_id2, entry.area_id3, entry.area_id4));
                            if ui.button("Edit").clicked() {
                                self.selected_index = Some(i);
                            }
                            ui.end_row();
                        }
                    });
            });
        let len = self.entries.len();
        if let Some(index) = self.selected_index {
            if let Some(entry) = self.entries.get_mut(index) {
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.add_enabled(index > 0, egui::Button::new("Previous")).clicked() {
                        self.selected_index = Some(index - 1);
                    }
                    if ui.add_enabled(index + 1 < len, egui::Button::new("Next")).clicked() {
                        self.selected_index = Some(index + 1);
                    }
                    ui.label(format!("Entry {}/{}", index + 1, len));
                });
                ui.add_space(8.0);
                ui.label("Title:");
                ui.text_edit_singleline(&mut entry.title);
                ui.label("Description:");
                ui.text_edit_multiline(&mut entry.description);
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label("Jump ID:");
                    ui.add(egui::DragValue::new(&mut entry.jump_id));
                });
                ui.horizontal(|ui| {
                    ui.label("Area IDs:");
                    ui.add(egui::DragValue::new(&mut entry.area_id));
                    ui.add(egui::DragValue::new(&mut entry.area_id2));
                    ui.add(egui::DragValue::new(&mut entry.area_id3));
                    ui.add(egui::DragValue::new(&mut entry.area_id4));
                });
                ui.add_space(8.0);
                ui.label("Player Position:");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut entry.player_pos_x));
                    ui.add(egui::DragValue::new(&mut entry.player_pos_y));
                    ui.add(egui::DragValue::new(&mut entry.player_pos_z));
                });
                ui.horizontal(|ui| {
                    ui.label("Player Rotation:");
                    ui.add(egui::DragValue::new(&mut entry.rotation));
                });
                ui.label("Camera Position:");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut entry.camera_pos_x));
                    ui.add(egui::DragValue::new(&mut entry.camera_pos_y));
                    ui.add(egui::DragValue::new(&mut entry.camera_pos_z));
                });
                ui.horizontal(|ui| {
                    ui.label("Camera Rotation:");
                    ui.add(egui::DragValue::new(&mut entry.rotation1));
                });
            }
        }
    }

    fn show_areas_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Area Entries");
        if ui.button("Add new Area").clicked() {
            self.areas.push(Area {
                p_entry_data: 0,
                len_entry_data: 0,
                p_stage_ids: 0,
                entries: Vec::new(),
                stage_ids: Vec::new(),
            });
        }
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, area) in self.areas.iter_mut().enumerate() {
                egui::CollapsingHeader::new(format!("Area {}", i + 1))
                    .default_open(i == 0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("p_entry_data: 0x{:08X}", area.p_entry_data));
                            ui.add(egui::DragValue::new(&mut area.len_entry_data));
                            ui.label(format!("p_stage_ids: 0x{:08X}", area.p_stage_ids));
                        });
                        ui.label(format!("Entries ({})", area.entries.len()));
                        if ui.button("Add Entry").clicked() {
                            area.entries.push(crate::model::mhfjmp::AreaEntry { index: 0, flags: 0 });
                        }
                        egui::Grid::new(format!("area_entries_{}", i)).show(ui, |ui| {
                            ui.label("Index");
                            ui.label("Flags (dec)");
                            ui.label(""); // up
                            ui.label(""); // down
                            ui.label(""); // delete
                            ui.end_row();
                            let mut move_up = None;
                            let mut move_down = None;
                            let mut to_remove = None;
                            let len = area.entries.len();
                            for (j, entry) in area.entries.iter_mut().enumerate() {
                                ui.add(egui::DragValue::new(&mut entry.index));
                                ui.add(egui::DragValue::new(&mut entry.flags));
                                if ui.add_enabled(j > 0, egui::Button::new("↑")).clicked() {
                                    move_up = Some(j);
                                }
                                if ui.add_enabled(j + 1 < len, egui::Button::new("↓")).clicked() {
                                    move_down = Some(j);
                                }
                                if ui.button("✕").clicked() {
                                    to_remove = Some(j);
                                }
                                ui.end_row();
                            }
                            if let Some(j) = move_up {
                                area.entries.swap(j, j - 1);
                            }
                            if let Some(j) = move_down {
                                area.entries.swap(j, j + 1);
                            }
                            if let Some(j) = to_remove {
                                area.entries.remove(j);
                            }
                        });
                        ui.label(format!("Stage IDs ({})", area.stage_ids.len()));
                        if ui.button("Add Stage ID").clicked() {
                            area.stage_ids.push(0);
                        }
                        egui::Grid::new(format!("area_stage_ids_{}", i)).show(ui, |ui| {
                            ui.label("ID");
                            ui.end_row();
                            let mut to_remove = None;
                            for (j, id) in area.stage_ids.iter_mut().enumerate() {
                                ui.add(egui::DragValue::new(id));
                                if ui.button("✕").clicked() {
                                    to_remove = Some(j);
                                }
                                ui.end_row();
                            }
                            if let Some(j) = to_remove {
                                area.stage_ids.remove(j);
                            }
                        });
                    });
            }
        });
    }

    fn show_strings_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Strings");
        egui::ScrollArea::vertical().show(ui, |ui| {
            for s in &mut self.strings {
                egui::Frame::group(ui.style())
                    .margin(egui::vec2(8.0, 8.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("ID: {}", s.id));
                        });
                        ui.add(
                            egui::TextEdit::multiline(&mut s.text)
                                .desired_width(f32::INFINITY)
                                .lock_focus(true)
                        );
                    });
                ui.add_space(8.0);
            }
        });
    }

    pub fn load_file(&mut self, path: PathBuf, data: Vec<u8>) {
        match load_mhfjmp_bin_from_buffer(&data) {
            Ok((entries, areas, strings)) => {
                self.entries = entries;
                self.areas = areas;
                self.strings = strings;
                self.current_file = Some(path);
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(format!("Error loading file: {}", e));
            }
        }
    }
} 