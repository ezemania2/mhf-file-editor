use super::*;
use egui;
use crate::model::mhfjmp::{MenuEntry, Area, StringEntry};
use crate::core::mhfjmp::{load_mhfjmp_bin_from_buffer, save_mhfjmp_bin};
use crate::core::packing::{compress_file, encrypt_file};
use crate::utils::maps::{get_map_name, MAP_LIST};
use std::path::PathBuf;
use serde_json;
use std::io::{Read, Write, Seek, SeekFrom};
use std::fs::File;

pub enum MhfjmpTab {
    MenuEntries,
    Areas,
    Strings,
    Handlers,
}

impl Default for MhfjmpTab {
    fn default() -> Self {
        MhfjmpTab::MenuEntries
    }
}

#[derive(Clone)]
pub struct HandlerEntry {
    pub index: u8,
    pub name: String,
    pub address: u32,
    pub description: String,
}

pub struct MhfjmpApp {
    pub tab: MhfjmpTab,
    pub entries: Vec<MenuEntry>,
    pub selected_index: Option<usize>,
    pub areas: Vec<Area>,
    pub strings: Vec<StringEntry>,
    pub handlers: Vec<HandlerEntry>,
    pub current_file: Option<PathBuf>,
    pub error_message: Option<String>,
    pub should_return_to_selector: bool,
    pub dll_file: Option<PathBuf>,
    pub dll_loaded: bool,
    // Map search fields
    pub map_search_area_id: String,
    pub map_search_area_id2: String,
    pub map_search_area_id3: String,
    pub map_search_area_id4: String,
    pub map_search_stage_id: String,
}

impl Default for MhfjmpApp {
    fn default() -> Self {
        let default_handlers = vec![
            HandlerEntry { index: 0, name: "teleport_coords".to_string(), address: 0x10410E50, description: "Direct coordinate teleportation".to_string() },
            HandlerEntry { index: 1, name: "lobby_specific".to_string(), address: 0x10410F50, description: "Specific lobby teleportation".to_string() },
            HandlerEntry { index: 2, name: "change_land".to_string(), address: 0x10410FB0, description: "Land/Salon change handler".to_string() },
            HandlerEntry { index: 3, name: "change_lobby_type".to_string(), address: 0x10410FD0, description: "Lobby type change".to_string() },
            HandlerEntry { index: 4, name: "zone_with_loading".to_string(), address: 0x10410FF0, description: "Zone with loading (to guild)".to_string() },
            HandlerEntry { index: 5, name: "area_classic".to_string(), address: 0x104110E0, description: "Classic area transition".to_string() },
            HandlerEntry { index: 6, name: "area_with_npc_dialogue".to_string(), address: 0x104111A0, description: "Area with NPC dialogue".to_string() },
            HandlerEntry { index: 7, name: "area_change".to_string(), address: 0x10411270, description: "Standard area change".to_string() },
            HandlerEntry { index: 8, name: "hardcoded_area".to_string(), address: 0x10411310, description: "Hardcoded area (house)".to_string() },

        ];
        
        MhfjmpApp {
            tab: MhfjmpTab::MenuEntries,
            entries: Vec::new(),
            selected_index: None,
            areas: Vec::new(),
            strings: Vec::new(),
            handlers: default_handlers,
            current_file: None,
            error_message: None,
            should_return_to_selector: false,
            dll_file: None,
            dll_loaded: false,
            map_search_area_id: String::new(),
            map_search_area_id2: String::new(),
            map_search_area_id3: String::new(),
            map_search_area_id4: String::new(),
            map_search_stage_id: String::new(),
        }
    }
}

impl App for MhfjmpApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Back").clicked() {
                    // Unload file from memory
                    self.entries.clear();
                    self.areas.clear();
                    self.strings.clear();
                    self.current_file = None;
                    self.error_message = None;
                    self.selected_index = None;
                    
                    self.should_return_to_selector = true;
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
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        if let Some(current_file) = &self.current_file {
                            match save_mhfjmp_bin(current_file, &self.entries, &self.areas, &self.strings) {
                                Ok(()) => {
                                    self.error_message = Some("File saved successfully.".to_string());
                                }
                                Err(e) => {
                                    self.error_message = Some(format!("Failed to save file: {}", e));
                                }
                            }
                        } else {
                            self.error_message = Some("No file loaded.".to_string());
                        }
                    }
                    
                    if ui.button("Compress").clicked() {
                        self.compress_file();
                    }
                    
                    if ui.button("Encrypt").clicked() {
                        self.encrypt_file();
                    }
                });
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
                if ui.selectable_label(matches!(self.tab, MhfjmpTab::Handlers), "Handlers (DLL)").clicked() {
                    self.tab = MhfjmpTab::Handlers;
                }
            });
        });
        match self.tab {
            MhfjmpTab::MenuEntries => self.show_menu_entries_tab(ui),
            MhfjmpTab::Areas => self.show_areas_tab(ui),
            MhfjmpTab::Strings => self.show_strings_tab(ui),
            MhfjmpTab::Handlers => self.show_handlers_tab(ui),
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
                ui.label("Area IDs:");
                ui.horizontal(|ui| {
                    ui.label("Area ID 1:");
                    egui::ComboBox::from_id_source("area_id_1")
                        .selected_text(format!("{} - {}", entry.area_id, get_map_name(entry.area_id)))
                        .show_ui(ui, |ui| {
                            ui.text_edit_singleline(&mut self.map_search_area_id);
                            let search = self.map_search_area_id.to_lowercase();
                            egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                                for (id, name) in MAP_LIST {
                                    if search.is_empty() || name.to_lowercase().contains(&search) || id.to_string().contains(&search) {
                                        ui.selectable_value(&mut entry.area_id, *id, format!("{} - {}", id, name));
                                    }
                                }
                            });
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Area ID 2:");
                    egui::ComboBox::from_id_source("area_id_2")
                        .selected_text(format!("{} - {}", entry.area_id2, get_map_name(entry.area_id2)))
                        .show_ui(ui, |ui| {
                            ui.text_edit_singleline(&mut self.map_search_area_id2);
                            let search = self.map_search_area_id2.to_lowercase();
                            egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                                for (id, name) in MAP_LIST {
                                    if search.is_empty() || name.to_lowercase().contains(&search) || id.to_string().contains(&search) {
                                        ui.selectable_value(&mut entry.area_id2, *id, format!("{} - {}", id, name));
                                    }
                                }
                            });
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Area ID 3:");
                    egui::ComboBox::from_id_source("area_id_3")
                        .selected_text(format!("{} - {}", entry.area_id3, get_map_name(entry.area_id3)))
                        .show_ui(ui, |ui| {
                            ui.text_edit_singleline(&mut self.map_search_area_id3);
                            let search = self.map_search_area_id3.to_lowercase();
                            egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                                for (id, name) in MAP_LIST {
                                    if search.is_empty() || name.to_lowercase().contains(&search) || id.to_string().contains(&search) {
                                        ui.selectable_value(&mut entry.area_id3, *id, format!("{} - {}", id, name));
                                    }
                                }
                            });
                        });
                });
                ui.horizontal(|ui| {
                    ui.label("Area ID 4:");
                    egui::ComboBox::from_id_source("area_id_4")
                        .selected_text(format!("{} - {}", entry.area_id4, get_map_name(entry.area_id4)))
                        .show_ui(ui, |ui| {
                            ui.text_edit_singleline(&mut self.map_search_area_id4);
                            let search = self.map_search_area_id4.to_lowercase();
                            egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                                for (id, name) in MAP_LIST {
                                    if search.is_empty() || name.to_lowercase().contains(&search) || id.to_string().contains(&search) {
                                        ui.selectable_value(&mut entry.area_id4, *id, format!("{} - {}", id, name));
                                    }
                                }
                            });
                        });
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
                                if ui.button("❌").clicked() {
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
                                egui::ComboBox::from_id_source(format!("stage_id_{}_{}", i, j))
                                    .selected_text(format!("{} - {}", id, get_map_name(*id)))
                                    .show_ui(ui, |ui| {
                                        ui.text_edit_singleline(&mut self.map_search_stage_id);
                                        let search = self.map_search_stage_id.to_lowercase();
                                        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                                            for (map_id, name) in MAP_LIST {
                                                if search.is_empty() || name.to_lowercase().contains(&search) || map_id.to_string().contains(&search) {
                                                    ui.selectable_value(id, *map_id, format!("{} - {}", map_id, name));
                                                }
                                            }
                                        });
                                    });
                                if ui.button("❌").clicked() {
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

    fn compress_file(&mut self) {
        if let Some(current_file) = &self.current_file {
            let temp_path = current_file.with_extension("tmp");
            if let Err(e) = std::fs::copy(current_file, &temp_path) {
                self.error_message = Some(format!("Error creating temp file for compression: {}", e));
                return;
            }
            if let Err(e) = compress_file(&temp_path, current_file) {
                self.error_message = Some(format!("Error compressing file: {}", e));
                let _ = std::fs::remove_file(&temp_path);
                return;
            }
            let _ = std::fs::remove_file(&temp_path);
            self.error_message = Some("File compressed successfully with JPK Type 4.".to_string());
        } else {
            self.error_message = Some("No file loaded.".to_string());
        }
    }

    fn encrypt_file(&mut self) {
        if let Some(current_file) = &self.current_file {
            let temp_path = current_file.with_extension("tmp");
            if let Err(e) = std::fs::copy(current_file, &temp_path) {
                self.error_message = Some(format!("Error creating temp file for encryption: {}", e));
                return;
            }
            if let Err(e) = encrypt_file(&temp_path, current_file) {
                self.error_message = Some(format!("Error encrypting file: {}", e));
                let _ = std::fs::remove_file(&temp_path);
                return;
            }
            let _ = std::fs::remove_file(&temp_path);
            self.error_message = Some("File encrypted successfully with ECD.".to_string());
        } else {
            self.error_message = Some("No file loaded.".to_string());
        }
    }

    fn load_dll(&mut self) {
        const HANDLER_TABLE_OFFSET: u64 = 0x019227B0;
        const HANDLER_COUNT: usize = 24;
        const HANDLER_SIZE: usize = 4;
        
        if let Some(result) = rfd::FileDialog::new()
            .add_filter("DLL", &["dll"])
            .set_file_name("mhfo-hd.dll")
            .pick_file()
        {
            match File::open(&result) {
                Ok(mut file) => {
                    if let Err(e) = file.seek(SeekFrom::Start(HANDLER_TABLE_OFFSET)) {
                        self.error_message = Some(format!("Error seeking to handler table: {}", e));
                        return;
                    }
                    
                    let mut buffer = vec![0u8; HANDLER_COUNT * HANDLER_SIZE];
                    match file.read_exact(&mut buffer) {
                        Ok(_) => {
                            self.handlers.clear();
                            for i in 0..HANDLER_COUNT {
                                let offset = i * HANDLER_SIZE;
                                let address = u32::from_le_bytes([
                                    buffer[offset],
                                    buffer[offset + 1],
                                    buffer[offset + 2],
                                    buffer[offset + 3],
                                ]);
                                
                                let (name, description) = match i {
                                    0 => ("teleport_coords".to_string(), "Direct coordinate teleportation".to_string()),
                                    1 => ("lobby_specific".to_string(), "Specific lobby teleportation".to_string()),
                                    2 => ("change_land".to_string(), "Land change handler".to_string()),
                                    3 => ("change_lobby_type".to_string(), "Lobby type change".to_string()),
                                    4 => ("zone_with_loading".to_string(), "Zone with loading (to guild)".to_string()),
                                    5 => ("area_classic".to_string(), "Classic area transition".to_string()),
                                    6 => ("area_with_npc_dialogue".to_string(), "Area with NPC dialogue".to_string()),
                                    7 => ("area_change".to_string(), "Standard area change".to_string()),
                                    8 => ("hardcoded_area".to_string(), "Hardcoded area (house)".to_string()),
                                    _ => (format!("handler_{}", i), format!("Handler {} (unknown)", i)),
                                };
                                
                                self.handlers.push(HandlerEntry {
                                    index: i as u8,
                                    name,
                                    address,
                                    description,
                                });
                            }
                            
                            self.dll_file = Some(result);
                            self.dll_loaded = true;
                            self.error_message = Some("DLL loaded successfully! You can now edit the handlers.".to_string());
                        }
                        Err(e) => {
                            self.error_message = Some(format!("Error reading handler table: {}", e));
                        }
                    }
                }
                Err(e) => {
                    self.error_message = Some(format!("Error opening DLL: {}", e));
                }
            }
        }
    }

    fn save_dll(&mut self) {
        const HANDLER_TABLE_OFFSET: u64 = 0x019227B0;
        
        if let Some(dll_path) = &self.dll_file {
            match std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(dll_path)
            {
                Ok(mut file) => {
                    if let Err(e) = file.seek(SeekFrom::Start(HANDLER_TABLE_OFFSET)) {
                        self.error_message = Some(format!("Error seeking to handler table: {}", e));
                        return;
                    }
                    
                    for handler in &self.handlers {
                        let bytes = handler.address.to_le_bytes();
                        if let Err(e) = file.write_all(&bytes) {
                            self.error_message = Some(format!("Error writing handler {}: {}", handler.index, e));
                            return;
                        }
                    }
                    
                    self.error_message = Some("Handler table saved successfully!".to_string());
                }
                Err(e) => {
                    self.error_message = Some(format!("Error opening DLL for writing: {}", e));
                }
            }
        } else {
            self.error_message = Some("No DLL file loaded. Please load mhfo-hd.dll first.".to_string());
        }
    }

    fn show_handlers_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("Teleportation Handlers Editor (mhfo-hd.dll)");
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("mhfo-hd.dll").clicked() {
                self.load_dll();
            }
            
            ui.separator();
            
            if self.dll_loaded {
                if ui.button("Save Changes").clicked() {
                    self.save_dll();
                }
                
                ui.separator();
                
                if let Some(dll_path) = &self.dll_file {
                    ui.label(format!("Loaded: {}", dll_path.file_name().unwrap_or_default().to_string_lossy()));
                }
            } else {
                ui.label("Load mhfo-hd.dll to edit handlers");
            }
        });
        
        ui.add_space(10.0);
        
        if !self.dll_loaded {
            ui.group(|ui| {
                ui.heading("Instructions");
                ui.separator();
                ui.label("1. Click 'Load mhfo-hd.dll' and select your mhfo-hd.dll file");
                ui.label("2. The editor will automatically read the handler table at offset 0x019227B0");
                ui.label("3. Modify handler addresses directly in the table below");
                ui.label("4. Click 'Save Changes' to write modifications to the DLL");
                ui.add_space(10.0);
                ui.label("IMPORTANT:");
                ui.label("• Always backup mhfo-hd.dll before modification");
                ui.label("• Invalid addresses will crash the game");
                ui.label("• Test in a backup game directory first");
            });
            return;
        }
        
        ui.add_space(10.0);
        
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Handler Table - DLL Offset:");
                ui.monospace("0x019227B0");
                ui.separator();
                ui.label("24 × u32 (96 bytes total)");
            });
        });
        
        ui.add_space(10.0);
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            let known_handlers = [
                (0x10410E50, "teleport_coords"),
                (0x10410F50, "lobby_specific"),
                (0x10410FB0, "change_land"),
                (0x10410FD0, "change_lobby_type"),
                (0x10410FF0, "zone_with_loading"),
                (0x104110E0, "area_classic"),
                (0x104111A0, "area_with_npc_dialogue"),
                (0x10411270, "area_change"),
                (0x10411310, "hardcoded_area"),
            ];
            
            for i in 0..self.handlers.len() {
                ui.horizontal(|ui| {
                    ui.monospace(format!("{:2}", i));
                    
                    let current_name = known_handlers.iter()
                        .find(|(addr, _)| *addr == self.handlers[i].address)
                        .map(|(_, name)| *name)
                        .unwrap_or("unknown");
                    
                    egui::ComboBox::from_id_source(format!("handler_{}", i))
                        .selected_text(current_name)
                        .show_ui(ui, |ui| {
                            for (addr, name) in &known_handlers {
                                if ui.selectable_value(&mut self.handlers[i].address, *addr, *name).clicked() {
                                }
                            }
                        });
                });
            }
        });
    }
} 