use eframe::{egui, App};
use native_dialog::FileDialog;
use std::fs;
use std::path::{PathBuf, Path};
use std::io::{Read, Seek, Write, SeekFrom};
use crate::model::mhfdat::{
    MhfdatMeleeWeapon, MhfdatRangedWeapon, ShopEntry,
    DecoShop, SigilTowerTable, G50WUpgrade,
    MWUpgradePath, RWUpgradePath, EvoUpgrade,
    MhfdatEquipment
};
use crate::utils::weapon_patterns::{class_name, CLASS_ID_LIST, element_name, ELEMENT_ID_LIST, ailment_name, AILMENT_ID_LIST, equip_type_name, EQUIP_TYPE_LIST, weapon_type_name, WEAPON_TYPE_LIST, zenith_skill_name, ZENITH_SKILL_LIST, recoil, RECOIL_LIST, reload, RELOAD_LIST};
use crate::core::mhfdat::{
    read_melee_weapons_until_sentinel, read_ranged_weapons_until_sentinel,
    read_shop_entries_until_sentinel, read_deco_shop_until_sentinel,
    read_sigil_tower_until_sentinel, read_g50_weapon_until_sentinel,
    read_mw_upgrade_until_sentinel, read_rw_upgrade_until_sentinel,
    read_evo_upgrade_until_sentinel, read_equipments_until_sentinel,
    extract_melee_weapon_names, extract_melee_weapon_descriptions_v2,
    extract_armor_names, extract_armor_descriptions,
    write_melee_weapon, write_ranged_weapon, write_shop_entry,
    write_deco_shop, write_sigil_tower_table, write_g50_weapon_upgrade,
    write_mw_upgrade_path, write_rw_upgrade_path, write_evo_upgrade,
    write_weapon_names, write_ranged_weapon_names,
    append_bytes_to_file, read_mhfdat_offsets,
};
use crate::core::packing::pack_file;
use crate::model::mhfdat_pointers::{
    MELEE_WEAPONS_PTR, MELEE_WEAPON_NAMES_PTR, MELEE_WEAPON_DESC_PTR,
    RANGED_WEAPON_NAMES_PTR,
    HEAD_ARMOR_PTR, HEAD_ARMOR_NAMES_PTR,
    BODY_ARMOR_PTR, BODY_ARMOR_NAMES_PTR,
    ARM_ARMOR_PTR, ARM_ARMOR_NAMES_PTR,
    WAIST_ARMOR_PTR, WAIST_ARMOR_NAMES_PTR,
    LEG_ARMOR_PTR, LEG_ARMOR_NAMES_PTR,
    EQUIP_DESC_PTR,
};
use std::fs::OpenOptions;
use std::sync::{OnceLock, Mutex};
use egui::{FontData, FontDefinitions, FontFamily};
use std::sync::atomic::{AtomicBool, Ordering};
use std::env;
use image;
use encoding_rs;
use crate::utils::equip_type::{getEquipType};
use crate::utils::equip_flags;

// Use the font from main.rs
use crate::NOTO_FONT;

#[derive(PartialEq)]
pub enum WeaponCategory {
    Melee,
    Ranged,
}

#[derive(PartialEq)]
pub enum MainTab {
    Weapons,
    Workshop,
    Armors, // Ajout de l'onglet principal Armors
}

#[derive(PartialEq)]
pub enum WorkshopTab {
    Transmog,
    ZenithWeapons,
    DecoShop,
    SigilTower,
    G50Weapons,
    MWUpgrades,
    RWUpgrades,
    EvoUpgrades,
}

#[derive(PartialEq)]
pub enum WeaponTab {
    Melee,
    Ranged,
}

#[derive(PartialEq)]
enum ArmorTab {
    Head,
    Chest,
    Arms,
    Waist,
    Legs,
}

pub struct MhfdatApp {
    pub error_message: Option<String>,
    pub on_back: Option<Box<dyn FnMut()>>,
    pub current_file: Option<PathBuf>,
    pub buffer: Vec<u8>,
    pub selected_category: WeaponCategory,
    pub main_tab: MainTab,
    pub melee_weapons: Vec<MhfdatMeleeWeapon>,
    pub ranged_weapons: Vec<MhfdatRangedWeapon>,
    pub workshop_entries: Vec<crate::model::mhfdat::ShopEntry>,
    pub show_weapons_menu: bool,
    pub selected_weapon_view: Option<WeaponCategory>,
    pub selected_melee_index: Option<usize>,
    pub search_query: String,
    pub class_id_filter: Option<u8>,
    pub element_filter: Option<u8>,
    pub ailment_filter: Option<u8>,
    pub equip_type_filter: Option<u8>,
    pub weapon_type_filter: Option<u32>,
    pub zenith_skill_filter: Option<u16>,
    pub melee_weapon_names: Vec<String>,
    pub show_dummy_weapons: bool,
    pub show_dummy_ranged_weapons: bool,
    pub melee_weapon_descriptions: Vec<[String; 4]>,
    pub should_encrypt: bool,
    pub should_pack: bool,
    pub workshop_tab: WorkshopTab,
    pub deco_shop_entries: Vec<DecoShop>,
    pub sigil_tower_entries: Vec<SigilTowerTable>,
    pub g50_weapon_entries: Vec<G50WUpgrade>,
    pub mw_upgrade_entries: Vec<MWUpgradePath>,
    pub rw_upgrade_entries: Vec<RWUpgradePath>,
    pub evo_upgrade_entries: Vec<EvoUpgrade>,
    pub transmog_entries: Vec<ShopEntry>,
    pub zenith_entries: Vec<ShopEntry>,
    pub transmog_open: Vec<bool>,
    pub zenith_open: Vec<bool>,
    pub weapon_tab: WeaponTab,
    pub ranged_weapon_names: Vec<String>,
    pub ranged_weapon_descriptions: Vec<[String; 4]>,
    pub armor_tab: ArmorTab,
    pub head_armors: Vec<MhfdatEquipment>,
    pub chest_armors: Vec<MhfdatEquipment>,
    pub arms_armors: Vec<MhfdatEquipment>,
    pub waist_armors: Vec<MhfdatEquipment>,
    pub legs_armors: Vec<MhfdatEquipment>,
    pub head_armor_names: Vec<String>,
    pub chest_armor_names: Vec<String>,
    pub arms_armor_names: Vec<String>,
    pub waist_armor_names: Vec<String>,
    pub legs_armor_names: Vec<String>,
    pub head_armor_descriptions: Vec<[String; 3]>,
    pub chest_armor_descriptions: Vec<[String; 3]>,
    pub arms_armor_descriptions: Vec<[String; 3]>,
    pub waist_armor_descriptions: Vec<[String; 3]>,
    pub legs_armor_descriptions: Vec<[String; 3]>,
    pub selected_armor_index: Option<usize>,
    pub armor_search_query: String,
    pub armor_loaded: bool,
}

impl Default for MhfdatApp {
    fn default() -> Self {
        Self {
            error_message: None,
            on_back: None,
            current_file: None,
            buffer: Vec::new(),
            selected_category: WeaponCategory::Melee,
            main_tab: MainTab::Weapons,
            melee_weapons: Vec::new(),
            ranged_weapons: Vec::new(),
            workshop_entries: Vec::new(),
            show_weapons_menu: false,
            selected_weapon_view: None,
            selected_melee_index: None,
            search_query: String::new(),
            class_id_filter: None,
            element_filter: None,
            ailment_filter: None,
            equip_type_filter: None,
            weapon_type_filter: None,
            zenith_skill_filter: None,
            melee_weapon_names: Vec::new(),
            show_dummy_weapons: false,
            show_dummy_ranged_weapons: false,
            melee_weapon_descriptions: Vec::new(),
            should_encrypt: false,
            should_pack: false,
            workshop_tab: WorkshopTab::Transmog,
            deco_shop_entries: Vec::new(),
            sigil_tower_entries: Vec::new(),
            g50_weapon_entries: Vec::new(),
            mw_upgrade_entries: Vec::new(),
            rw_upgrade_entries: Vec::new(),
            evo_upgrade_entries: Vec::new(),
            transmog_entries: Vec::new(),
            zenith_entries: Vec::new(),
            transmog_open: vec![true],
            zenith_open: vec![true],
            weapon_tab: WeaponTab::Melee,
            ranged_weapon_names: Vec::new(),
            ranged_weapon_descriptions: Vec::new(),
            armor_tab: ArmorTab::Head,
            head_armors: Vec::new(),
            chest_armors: Vec::new(),
            arms_armors: Vec::new(),
            waist_armors: Vec::new(),
            legs_armors: Vec::new(),
            head_armor_names: Vec::new(),
            chest_armor_names: Vec::new(),
            arms_armor_names: Vec::new(),
            waist_armor_names: Vec::new(),
            legs_armor_names: Vec::new(),
            head_armor_descriptions: Vec::new(),
            chest_armor_descriptions: Vec::new(),
            arms_armor_descriptions: Vec::new(),
            waist_armor_descriptions: Vec::new(),
            legs_armor_descriptions: Vec::new(),
            selected_armor_index: None,
            armor_search_query: String::new(),
            armor_loaded: false,
        }
    }
}

impl App for MhfdatApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply NotoSansCJKjp font everywhere
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "noto".to_owned(),
            FontData::from_owned(NOTO_FONT.to_vec()),
        );
        for family in [
            FontFamily::Proportional,
            FontFamily::Monospace,
            FontFamily::Name("noto".into()),
        ] {
            fonts.families.entry(family).or_default().insert(0, "noto".to_owned());
        }
        ctx.set_fonts(fonts);
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(24.0, FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(18.0, FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(16.0, FontFamily::Monospace)),
            (egui::TextStyle::Button, egui::FontId::new(18.0, FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(14.0, FontFamily::Proportional)),
        ].into();
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Back").clicked() {
                self.selected_melee_index = None;
                self.selected_weapon_view = None;
                if let Some(cb) = &mut self.on_back {
                    cb();
                }
            }

            ui.horizontal(|ui| {
                if ui.button("Open").clicked() {
                    if let Ok(Some(path)) = FileDialog::new().show_open_single_file() {
                        let result = crate::core::packing::open_bin_with_unpack_fallback(&path, |buf| {
                            if let Some((melee_offset, ranged_offset)) = read_mhfdat_offsets(buf) {
                                let mut cursor = std::io::Cursor::new(buf);
                                let melee_weapons = crate::core::mhfdat::read_melee_weapons_until_sentinel(&mut cursor, melee_offset as u64)?;
                                cursor.seek(SeekFrom::Start(0))?;
                                let ranged_weapons = crate::core::mhfdat::read_ranged_weapons_until_sentinel(&mut cursor, ranged_offset as u64)?;
                                Ok((buf.to_vec(), melee_weapons, ranged_weapons))
                            } else {
                                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Could not find weapons offsets."))
                            }
                        });

                        // Variables temporaires pour tous les champs à affecter
                        let mut loaded_path = None;
                        let mut loaded_buffer = None;
                        let mut loaded_melee_weapons = None;
                        let mut loaded_ranged_weapons = None;
                        let mut error_message = None;

                        match result {
                            Ok((buffer, melee_weapons, ranged_weapons)) => {
                                loaded_path = Some(path);
                                loaded_buffer = Some(buffer);
                                loaded_melee_weapons = Some(melee_weapons);
                                loaded_ranged_weapons = Some(ranged_weapons);
                                error_message = Some("File loaded successfully.".to_string());
                            }
                            Err(e) => {
                                error_message = Some(format!("Error loading file: {}", e));
                            }
                        }

                        // Affectation à self et chargement des armures APRÈS le match
                        if let (Some(path), Some(buffer), Some(melee_weapons), Some(ranged_weapons)) =
                            (loaded_path, loaded_buffer, loaded_melee_weapons, loaded_ranged_weapons)
                        {
                            let buffer_ref = buffer.clone();
                            self.current_file = Some(path);
                            self.buffer = buffer;
                            self.melee_weapons = melee_weapons;
                            self.ranged_weapons = ranged_weapons;
                            self.selected_melee_index = None;
                            self.load_transmog_entries();
                            self.load_zenith_entries();
                            // --- Restore melee weapon names/descriptions loading ---
                            {
                                use crate::model::mhfdat_pointers::{MELEE_WEAPON_NAMES_PTR, MELEE_WEAPON_DESC_PTR};
                                use crate::core::mhfdat::{extract_melee_weapon_names, extract_melee_weapon_descriptions_v2};
                                let count = self.melee_weapons.len();
                                let mut cursor = std::io::Cursor::new(&self.buffer);
                                let names = extract_melee_weapon_names(&mut cursor, MELEE_WEAPON_NAMES_PTR, count).unwrap_or_default();
                                self.melee_weapon_names = names;
                                let mut cursor2 = std::io::Cursor::new(&self.buffer);
                                let descs_full = extract_melee_weapon_descriptions_v2(&mut cursor2, MELEE_WEAPON_DESC_PTR, count, 4).unwrap_or_default();
                                // Convert to Vec<[String; 4]> including mhfY field
                                self.melee_weapon_descriptions = descs_full.into_iter()
                                    .map(|descs| {
                                        let mut arr = [String::new(), String::new(), String::new(), String::new()];
                                        for (i, desc) in descs.into_iter().take(4).enumerate() {
                                            arr[i] = desc;
                                        }
                                        arr
                                    })
                                    .collect();
                                // Strict size synchronization
                                let min_count = self.melee_weapons.len().min(self.melee_weapon_names.len()).min(self.melee_weapon_descriptions.len());
                                self.melee_weapons.truncate(min_count);
                                self.melee_weapon_names.truncate(min_count);
                                self.melee_weapon_descriptions.truncate(min_count);
                                if self.melee_weapons.len() != self.melee_weapon_names.len() || self.melee_weapons.len() != self.melee_weapon_descriptions.len() {
                                    self.error_message = Some(format!("[ERROR] Size mismatch: weapons={}, names={}, descriptions={}", 
                                        self.melee_weapons.len(), 
                                        self.melee_weapon_names.len(), 
                                        self.melee_weapon_descriptions.len()));
                                }
                            }
                            // --- End restore ---
                            self.load_ranged_weapon_names();
                            self.load_armor_data(&buffer_ref);
                            self.armor_loaded = true;
                        }
                        if let Some(msg) = error_message {
                            self.error_message = Some(msg);
                        }
                    }
                }

                if ui.button("Save").clicked() {
                    if let Some(path) = &self.current_file {
                        match self.save_modified_data() {
                            Ok(()) => {
                                self.error_message = Some("File saved successfully.".to_string());
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Failed to save file: {e}"));
                            }
                        }
                    } else {
                        self.error_message = Some("No file loaded.".to_string());
                    }
                }

                if ui.button("Save (Pack + Encrypt)").clicked() {
                    if let Some(path) = &self.current_file {
                        match self.save_with_packing() {
                            Ok(()) => {
                                self.error_message = Some("File saved with packing and encryption.".to_string());
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Failed to save file: {e}"));
                            }
                        }
                    } else {
                        self.error_message = Some("No file loaded.".to_string());
                    }
                }
            });

            if let Some(msg) = &self.error_message {
                ui.label(msg);
            }

            ui.separator();

            // Weapons menu button
                ui.horizontal(|ui| {
                if ui.selectable_label(self.main_tab == MainTab::Weapons, "Weapons").clicked() {
                    self.main_tab = MainTab::Weapons;
                    }
                if ui.selectable_label(self.main_tab == MainTab::Workshop, "Workshop").clicked() {
                    self.main_tab = MainTab::Workshop;
                    }
                if ui.selectable_label(self.main_tab == MainTab::Armors, "Armors").clicked() {
                    self.main_tab = MainTab::Armors;
                    }
                });
            ui.separator();

            match self.main_tab {
                MainTab::Weapons => {
                    self.show_weapons_tab(ui);
                }
                MainTab::Workshop => {
                    self.show_workshop_tab(ui);
                }
                MainTab::Armors => {
                    self.show_armor_list(ui);
                }
            }

        });
    }
}

impl MhfdatApp {
    fn save_modified_data(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.current_file {
            // Open file for writing (truncate to avoid buffer overrun)
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(path)?;
            
            // Write the new melee weapons data at the end
            for weapon in &self.melee_weapons {
                write_melee_weapon(&mut file, weapon)?;
            }
            
            // Write sentinel at the end of weapons
            let sentinel = MhfdatMeleeWeapon {
                model_id: 0xFFFF,
                ..Default::default()
            };
            write_melee_weapon(&mut file, &sentinel)?;
            
            // Write 80-byte separator with FF at the end
            let mut separator = vec![0u8; 80];
            separator[79] = 0xFF;  // Set the last byte to FF
            file.write_all(&separator)?;
            
            // Write weapon names after the separator
            let min_count = self.melee_weapons.len().min(self.melee_weapon_names.len()).min(self.melee_weapon_descriptions.len());
            write_weapon_names(&mut file, &self.melee_weapon_names[..min_count])?;

            // Write melee descriptions table (4 pointers per weapon: 3 real + 1 null)
            let table_start = file.seek(SeekFrom::Current(0))? as u32;
            let num_ptrs = min_count * 4;
            let strings_start = table_start + (num_ptrs as u32) * 4;
            
            // Build pointer values and string blob in memory
            let mut ptr_values: Vec<u32> = Vec::with_capacity(num_ptrs);
            let mut strings_blob: Vec<u8> = Vec::new();
            for descs in &self.melee_weapon_descriptions[..min_count] {
                // Write 3 description pointers
                for desc in descs.iter().take(3) {
                    let desc_str: String = desc.chars().take(28).collect();
                    let (sjis_bytes, _, _) = encoding_rs::SHIFT_JIS.encode(&desc_str);
                    let absolute_ptr = strings_start + strings_blob.len() as u32;
                    ptr_values.push(absolute_ptr);
                    strings_blob.extend_from_slice(&sjis_bytes);
                    strings_blob.push(0);
                }
                // 4th pointer is always null
                ptr_values.push(0);
            }
            // Write pointer table
            for p in &ptr_values { file.write_all(&p.to_le_bytes())?; }
            // Write strings
            file.write_all(&strings_blob)?;

            // Write ranged weapons data
            for weapon in &self.ranged_weapons {
                write_ranged_weapon(&mut file, weapon)?;
            }

            // Write sentinel for ranged weapons
            let sentinel = MhfdatRangedWeapon {
                model_id: 0xFFFF,
                ..Default::default()
            };
            write_ranged_weapon(&mut file, &sentinel)?;

            // Write ranged weapon names
            let min_count = self.ranged_weapons.len().min(self.ranged_weapon_names.len()).min(self.ranged_weapon_descriptions.len());
            write_weapon_names(&mut file, &self.ranged_weapon_names[..min_count])?;

            // Write ranged descriptions table (4 pointers per weapon: 3 real + 1 null)
            let ranged_table_start = file.seek(SeekFrom::Current(0))? as u32;
            let ranged_num_ptrs = min_count * 4;
            let ranged_strings_start = ranged_table_start + (ranged_num_ptrs as u32) * 4;
            
            // Build pointer values and string blob in memory
            let mut ranged_ptr_values: Vec<u32> = Vec::with_capacity(ranged_num_ptrs);
            let mut ranged_strings_blob: Vec<u8> = Vec::new();
            for descs in &self.ranged_weapon_descriptions[..min_count] {
                // Write 3 description pointers
                for desc in descs.iter().take(3) {
                    let desc_str: String = desc.chars().take(28).collect();
                    let (sjis_bytes, _, _) = encoding_rs::SHIFT_JIS.encode(&desc_str);
                    let absolute_ptr = ranged_strings_start + ranged_strings_blob.len() as u32;
                    ranged_ptr_values.push(absolute_ptr);
                    ranged_strings_blob.extend_from_slice(&sjis_bytes);
                    ranged_strings_blob.push(0);
                }
                // 4th pointer is always null
                ranged_ptr_values.push(0);
            }
            // Write pointer table
            for p in &ranged_ptr_values { file.write_all(&p.to_le_bytes())?; }
            // Write strings
            file.write_all(&ranged_strings_blob)?;
            
            // Set specific offsets
            file.seek(SeekFrom::Start(0x7C))?;
            file.write_all(&0xD02D0802u32.to_le_bytes())?;
            file.seek(SeekFrom::Start(0x88))?;
            file.write_all(&0x081F1602u32.to_le_bytes())?;

            Ok(())
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No file loaded"))
        }
    }

    fn save_modified_data_to_writer<W: std::io::Write + std::io::Seek>(&self, mut writer: W) -> std::io::Result<()> {
        // Même logique que save_modified_data, mais sur writer générique
        for weapon in &self.melee_weapons {
            write_melee_weapon(&mut writer, weapon)?;
        }
        let sentinel = MhfdatMeleeWeapon {
            model_id: 0xFFFF,
            ..Default::default()
        };
        write_melee_weapon(&mut writer, &sentinel)?;
        let mut separator = vec![0u8; 80];
        separator[79] = 0xFF;
        writer.write_all(&separator)?;
        let min_count = self.melee_weapons.len().min(self.melee_weapon_names.len()).min(self.melee_weapon_descriptions.len());
        write_weapon_names(&mut writer, &self.melee_weapon_names[..min_count])?;
        
        // Write melee descriptions table (4 pointers per weapon: 3 real + 1 null)
        let table_start = writer.seek(std::io::SeekFrom::Current(0))? as u32;
        let num_ptrs = min_count * 4;
        let strings_start = table_start + (num_ptrs as u32) * 4;
        
        let mut ptr_values: Vec<u32> = Vec::with_capacity(num_ptrs);
        let mut strings_blob: Vec<u8> = Vec::new();
        for descs in &self.melee_weapon_descriptions[..min_count] {
            for desc in descs.iter().take(3) {
                let desc_str: String = desc.chars().take(28).collect();
                let (sjis_bytes, _, _) = encoding_rs::SHIFT_JIS.encode(&desc_str);
                let absolute_ptr = strings_start + strings_blob.len() as u32;
                ptr_values.push(absolute_ptr);
                strings_blob.extend_from_slice(&sjis_bytes);
                strings_blob.push(0);
            }
            ptr_values.push(0); // 4th pointer is always null
        }
        for p in &ptr_values { writer.write_all(&p.to_le_bytes())?; }
        writer.write_all(&strings_blob)?;
        writer.seek(std::io::SeekFrom::Start(0x7C))?;
        writer.write_all(&0xD02D0802u32.to_le_bytes())?;
        writer.seek(SeekFrom::Start(0x88))?;
        writer.write_all(&0x081F1602u32.to_le_bytes())?;
        Ok(())
    }

    fn save_with_packing(&mut self) -> std::io::Result<()> {
        use std::io::Write;
        if let Some(path) = &self.current_file {
            let temp_path = path.with_extension("temp");
            // 1. Générer le buffer à partir des données courantes
            {
                let mut temp_file = std::fs::File::create(&temp_path)?;
                self.save_modified_data_to_writer(&mut temp_file)?;
                temp_file.flush()?;
            }
            // 2. Utiliser RsFrontier pour packer et chiffrer
            let pack_result = pack_file(&temp_path, path, true);
            // 3. Nettoyer le fichier temporaire
            let _ = std::fs::remove_file(&temp_path);
            // 4. Gérer l'erreur RsFrontier
            pack_result.map_err(|e| {
                self.error_message = Some(format!("Erreur RsFrontier lors du packing: {e}"));
                e
            })
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No file loaded"))
        }
    }

    fn show_weapons_tab(&mut self, ui: &mut egui::Ui) {
        // Add weapon category tabs
        ui.horizontal(|ui| {
            if ui.selectable_label(self.weapon_tab == WeaponTab::Melee, "Melee Weapons").clicked() {
                self.weapon_tab = WeaponTab::Melee;
            }
            if ui.selectable_label(self.weapon_tab == WeaponTab::Ranged, "Ranged Weapons").clicked() {
                self.weapon_tab = WeaponTab::Ranged;
            }
        });
        ui.separator();

        match self.weapon_tab {
            WeaponTab::Melee => {
                self.show_melee_weapons_list(ui);
            }
            WeaponTab::Ranged => {
                self.show_ranged_weapons_list(ui);
            }
        }
    }

    fn show_melee_weapons_list(&mut self, ui: &mut egui::Ui) {
                    let count = self.melee_weapons.len();
                    use crate::model::mhfdat_pointers::MELEE_WEAPONS_PTR;
                    let melee_offset = if let Some((melee_offset, _)) = read_mhfdat_offsets(&self.buffer) {
                        melee_offset as usize
                    } else {
                        MELEE_WEAPONS_PTR as usize
                    };

                    ui.heading(format!("Melee Weapons (found: {})", count));

                    // Search and filters
                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        egui::ComboBox::from_id_source("class_id_filter_combo")
                            .selected_text(self.class_id_filter.map(class_name).unwrap_or("All"))
                            .show_ui(ui, |ui| {
                                if ui.selectable_value(&mut self.class_id_filter, None, "All").clicked() {}
                                for (id, name) in CLASS_ID_LIST {
                                    if ui.selectable_value(&mut self.class_id_filter, Some(*id), *name).clicked() {}
                                }
                            });

                        ui.label("Element:");
                        egui::ComboBox::from_id_source("element_filter_combo")
                            .selected_text(self.element_filter.map(element_name).unwrap_or("All"))
                            .show_ui(ui, |ui| {
                                if ui.selectable_value(&mut self.element_filter, None, "All").clicked() {}
                                for (id, name) in ELEMENT_ID_LIST {
                                    if ui.selectable_value(&mut self.element_filter, Some(*id), *name).clicked() {}
                                }
                            });

                        ui.label("Ailment:");
                        egui::ComboBox::from_id_source("ailment_filter_combo")
                            .selected_text(self.ailment_filter.map(ailment_name).unwrap_or("All"))
                            .show_ui(ui, |ui| {
                                if ui.selectable_value(&mut self.ailment_filter, None, "All").clicked() {}
                                for (id, name) in AILMENT_ID_LIST {
                                    if ui.selectable_value(&mut self.ailment_filter, Some(*id), *name).clicked() {}
                                }
                            });

                        ui.label("Equip:");
                        egui::ComboBox::from_id_source("equip_type_filter_combo")
                            .selected_text(self.equip_type_filter.map(equip_type_name).unwrap_or("All"))
                            .show_ui(ui, |ui| {
                                if ui.selectable_value(&mut self.equip_type_filter, None, "All").clicked() {}
                                for (id, name) in EQUIP_TYPE_LIST {
                                    if ui.selectable_value(&mut self.equip_type_filter, Some(*id), *name).clicked() {}
                                }
                            });

                        ui.label("WeaponType:");
                        egui::ComboBox::from_id_source("weapon_type_filter_combo")
                            .selected_text(self.weapon_type_filter.map(weapon_type_name).unwrap_or("All"))
                            .show_ui(ui, |ui| {
                                if ui.selectable_value(&mut self.weapon_type_filter, None, "All").clicked() {}
                                for (id, name) in WEAPON_TYPE_LIST {
                                    if ui.selectable_value(&mut self.weapon_type_filter, Some(*id), *name).clicked() {}
                                }
                            });

                        ui.label("Zenith:");
                        egui::ComboBox::from_id_source("zenith_skill_filter_combo")
                            .selected_text(self.zenith_skill_filter.map(zenith_skill_name).unwrap_or("All"))
                            .show_ui(ui, |ui| {
                                if ui.selectable_value(&mut self.zenith_skill_filter, None, "All").clicked() {}
                                for (id, name) in ZENITH_SKILL_LIST {
                                    if ui.selectable_value(&mut self.zenith_skill_filter, Some(*id), *name).clicked() {}
                                }
                            });

                        ui.label("Search:");
                        ui.text_edit_singleline(&mut self.search_query);
                        ui.checkbox(&mut self.show_dummy_weapons, "Show Dummy Weapons");
                    });

                    if count == 0 {
                        ui.colored_label(egui::Color32::YELLOW, "Warning: No melee weapons found at the expected offset!");
                        } else {
            // Show selected weapon details if any
            if let Some(index) = self.selected_melee_index {
                if let Some(weapon) = self.melee_weapons.get_mut(index) {
                    let name = self.melee_weapon_names.get(index).cloned().unwrap_or_default();
                    let descriptions = self.melee_weapon_descriptions.get(index).cloned().unwrap_or_default();
                            egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.heading("Selected Weapon Details");
                        
                        // Editable name
                        ui.horizontal(|ui| {
                            ui.label("Name:");
                            let mut name_edit = name.clone();
                            if ui.text_edit_singleline(&mut name_edit).changed() {
                                if let Some(name_ref) = self.melee_weapon_names.get_mut(index) {
                                    *name_ref = name_edit;
                                }
                            }
                        });
                        
                        // Editable descriptions
                        ui.horizontal(|ui| {
                            ui.label("Description 1:");
                            let mut desc1 = descriptions[0].clone();
                            if ui.text_edit_singleline(&mut desc1).changed() {
                                if let Some(descs) = self.melee_weapon_descriptions.get_mut(index) {
                                    descs[0] = desc1;
                                }
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Description 2:");
                            let mut desc2 = descriptions[1].clone();
                            if ui.text_edit_singleline(&mut desc2).changed() {
                                if let Some(descs) = self.melee_weapon_descriptions.get_mut(index) {
                                    descs[1] = desc2;
                                }
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Description 3:");
                            let mut desc3 = descriptions[2].clone();
                            if ui.text_edit_singleline(&mut desc3).changed() {
                                if let Some(descs) = self.melee_weapon_descriptions.get_mut(index) {
                                    descs[2] = desc3;
                                }
                            }
                        });
                        
                        ui.separator();
                        Self::render_melee_weapon_details(ui, weapon);
                    });
                }
            }

            // Weapon list
            egui::CollapsingHeader::new("Weapon List")
                .default_open(true)
                .show(ui, |ui| {
                                egui::ScrollArea::vertical()
                                    .id_source("weapon_list_scroll")
                                    .max_height(600.0)
                                    .show(ui, |ui| {
                            egui::Grid::new("weapon_list_grid")
                                        .striped(true)
                                        .show(ui, |ui| {
                                            ui.label("ID");
                                            ui.label("Model ID");
                                            ui.label("Name");
                                            ui.label("Rarity");
                                            ui.label("Damage");
                                            ui.label("Affinity");
                                            ui.label("Element");
                                            ui.label("Slots");
                                    ui.label("Type");
                                            ui.end_row();

                                            let query = self.search_query.to_lowercase();
                                            for (i, weapon) in self.melee_weapons.iter().enumerate() {
                                        // Copy fields to local variables to avoid unaligned references
                                        let model_id = weapon.model_id;
                                        let rarity = weapon.rarity;
                                        let raw_damage = weapon.raw_damage;
                                        let affinity = weapon.affinity;
                                        let element_id = weapon.element_id;
                                        let slots = weapon.slots;
                                        let weapon_type = weapon.weapon_type;
                                        let weapon_name = self.melee_weapon_names.get(i).cloned().unwrap_or_default();

                                        // Dummy weapon detection
                                        let is_dummy = model_id == 0x05AD
                                            && rarity == 0x00
                                                        && weapon.class_id == 0x00
                                                        && weapon.zenny_cost == 0x00000528
                                                        && weapon.sharpness_id == 0x00
                                                        && weapon.sharpness_max == 0x00
                                            && raw_damage == 0x003C
                                                        && weapon.defense == 0x0000
                                            && affinity == 0x00
                                            && element_id == 0x00
                                                        && weapon.ele_damage == 0x00
                                                        && weapon.ailment_id == 0x00
                                                        && weapon.ail_damage == 0x00
                                            && slots == 0x00
                                                        && weapon.weapon_attribute == 0x00
                                                        && weapon.unk15 == 0x00
                                                        && weapon.upgrade_path == 0x00FF
                                                        && weapon.other_model == 0x0000
                                                        && weapon.equip_type == 0x00
                                                        && weapon.unk1b == 0x00
                                                        && weapon.length == 0x00000000
                                            && weapon_type == 0x00000000
                                                        && weapon.visual_effects == 0x0000
                                                        && weapon.tower_g50_param_id == 0x0000
                                                        && weapon.g_rank == 0x00
                                                        && weapon.unk29 == 0x00
                                                        && weapon.unk2a == 0x00
                                                        && weapon.zero_f == 0x0F
                                                        && weapon.unk2c == 0x00000000
                                                        && weapon.zenith_skill == 0x0000;

                                                    if self.show_dummy_weapons {
                                                        if !is_dummy { continue; }
                                                    } else {
                                                        if is_dummy { continue; }
                                                    }

                                                // Apply filters
                                                if let Some(class_id) = self.class_id_filter {
                                                    if weapon.class_id != class_id { continue; }
                                                }
                                        if let Some(element_id_filter) = self.element_filter {
                                            if element_id != element_id_filter { continue; }
                                                }
                                                if let Some(ailment_id) = self.ailment_filter {
                                                    if weapon.ailment_id != ailment_id { continue; }
                                                }
                                                if let Some(equip_type_id) = self.equip_type_filter {
                                                    if weapon.equip_type != equip_type_id { continue; }
                                                }
                                                if let Some(weapon_type_id) = self.weapon_type_filter {
                                            if weapon_type != weapon_type_id { continue; }
                                                }
                                                if let Some(zenith_skill_id) = self.zenith_skill_filter {
                                                    if weapon.zenith_skill != zenith_skill_id { continue; }
                                                }

                                        if !query.is_empty() && !weapon_name.to_lowercase().contains(&query) {
                                            continue;
                                        }

                                                let selected = self.selected_melee_index == Some(i);
                                                if ui.selectable_label(selected, format!("{}", i + 1)).clicked() {
                                                    self.selected_melee_index = Some(i);
                                                }
                                                ui.label(format!("{}", model_id));
                                                ui.label(&weapon_name);
                                        ui.label(format!("{}", rarity + 1));
                                                ui.label(format!("{}", raw_damage));
                                                ui.label(format!("{}", affinity));
                                        ui.label(format!("{}", element_name(element_id)));
                                                ui.label(format!("{}", slots));
                                                ui.label(weapon_type_name(weapon_type));
                                                ui.end_row();
                                            }
                                        });
                                });
                            });
        }
    }

    fn show_ranged_weapons_list(&mut self, ui: &mut egui::Ui) {
        let count = self.ranged_weapons.len();
        let ranged_offset = if let Some((_, ranged_offset)) = read_mhfdat_offsets(&self.buffer) {
            ranged_offset as usize
                                } else {
            0 // Default offset if not found
        };

        ui.heading(format!("Ranged Weapons (found: {})", count));

        // Search and filters
                                                ui.horizontal(|ui| {
            ui.label("Type:");
            egui::ComboBox::from_id_source("class_id_filter_combo")
                .selected_text(self.class_id_filter.map(class_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.class_id_filter, None, "All").clicked() {}
                    for (id, name) in CLASS_ID_LIST {
                        if ui.selectable_value(&mut self.class_id_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Element:");
            egui::ComboBox::from_id_source("element_filter_combo")
                .selected_text(self.element_filter.map(element_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.element_filter, None, "All").clicked() {}
                    for (id, name) in ELEMENT_ID_LIST {
                        if ui.selectable_value(&mut self.element_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Equip:");
            egui::ComboBox::from_id_source("equip_type_filter_combo")
                .selected_text(self.equip_type_filter.map(equip_type_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.equip_type_filter, None, "All").clicked() {}
                    for (id, name) in EQUIP_TYPE_LIST {
                        if ui.selectable_value(&mut self.equip_type_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("WeaponType:");
            egui::ComboBox::from_id_source("weapon_type_filter_combo")
                .selected_text(self.weapon_type_filter.map(weapon_type_name).unwrap_or("All"))
                                                        .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.weapon_type_filter, None, "All").clicked() {}
                    for (id, name) in WEAPON_TYPE_LIST {
                        if ui.selectable_value(&mut self.weapon_type_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Zenith:");
            egui::ComboBox::from_id_source("zenith_skill_filter_combo")
                .selected_text(self.zenith_skill_filter.map(zenith_skill_name).unwrap_or("All"))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.zenith_skill_filter, None, "All").clicked() {}
                    for (id, name) in ZENITH_SKILL_LIST {
                        if ui.selectable_value(&mut self.zenith_skill_filter, Some(*id), *name).clicked() {}
                    }
                });

            ui.label("Search:");
            ui.text_edit_singleline(&mut self.search_query);
            ui.checkbox(&mut self.show_dummy_ranged_weapons, "Show Dummy Weapons");
        });

        if count == 0 {
            ui.colored_label(egui::Color32::YELLOW, "Warning: No ranged weapons found at the expected offset!");
        } else {
            // Show selected weapon details if any
            if let Some(index) = self.selected_melee_index {
                if let Some(weapon) = self.ranged_weapons.get_mut(index) {
                    let name = self.ranged_weapon_names.get(index).cloned().unwrap_or_default();
                    let descriptions = self.ranged_weapon_descriptions.get(index).cloned().unwrap_or_default();
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.heading("Selected Weapon Details");
                        
                        // Editable name
                                            ui.horizontal(|ui| {
                                                ui.label("Name:");
                                                let mut name_edit = name.clone();
                                                if ui.text_edit_singleline(&mut name_edit).changed() {
                                if let Some(name_ref) = self.ranged_weapon_names.get_mut(index) {
                                    *name_ref = name_edit;
                                }
                            }
                        });
                        
                        // Editable descriptions
                        ui.horizontal(|ui| {
                            ui.label("Description 1:");
                            let mut desc1 = descriptions[0].clone();
                            if ui.text_edit_singleline(&mut desc1).changed() {
                                if let Some(descs) = self.ranged_weapon_descriptions.get_mut(index) {
                                    descs[0] = desc1;
                                }
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Description 2:");
                            let mut desc2 = descriptions[1].clone();
                            if ui.text_edit_singleline(&mut desc2).changed() {
                                if let Some(descs) = self.ranged_weapon_descriptions.get_mut(index) {
                                    descs[1] = desc2;
                                }
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Description 3:");
                            let mut desc3 = descriptions[2].clone();
                            if ui.text_edit_singleline(&mut desc3).changed() {
                                if let Some(descs) = self.ranged_weapon_descriptions.get_mut(index) {
                                    descs[2] = desc3;
                                }
                            }
                        });
                        
                        ui.separator();
                        Self::render_ranged_weapon_details(ui, weapon);
                    });
                }
            }

            // Weapon list
            egui::CollapsingHeader::new("Weapon List")
                .default_open(true)
                .show(ui, |ui| {
                    egui::ScrollArea::vertical()
                        .id_source("ranged_weapon_list_scroll")
                        .max_height(1000.0)
                        .show(ui, |ui| {
                            egui::Grid::new("ranged_weapon_list_grid")
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label("ID");
                                    ui.label("Model ID");
                                    ui.label("Name");
                                    ui.label("Rarity");
                                    ui.label("Damage");
                                    ui.label("Affinity");
                                    ui.label("Element");
                                    ui.label("Slots");
                                    ui.label("Type");
                                    ui.end_row();

                                    let query = self.search_query.to_lowercase();
                                    for (i, weapon) in self.ranged_weapons.iter().enumerate() {
                                        // Copy fields to local variables to avoid unaligned references
                                        let model_id = weapon.model_id;
                                        let rarity = weapon.rarity;
                                        let raw_damage = weapon.raw_damage;
                                        let affinity = weapon.affinity;
                                        let element_id = weapon.element_id;
                                        let slots = weapon.slots;
                                        let weapon_type = weapon.weapon_type;
                                        let weapon_name = self.ranged_weapon_names.get(i).cloned().unwrap_or_default();

                                        // Dummy weapon detection using the pattern
                                        let is_dummy = model_id == 0x00E0
                                            && rarity == 0x05
                                            && weapon.class_id == 0x0A
                                            && weapon.zenny_cost == 0x00035B60
                                            && raw_damage == 0x00DC
                                            && weapon.defense == 0x0000
                                            && affinity == 0x0F
                                            && element_id == 0x00
                                            && weapon.ele_damage == 0x00
                                            && slots == 0x01
                                            && weapon.weapon_attribute == 0x19
                                            && weapon.equip_type == 0x00
                                            && weapon_type == 0x00000000
                                            && weapon.bullet == 0x0000001A
                                            && weapon.recoil == 0x00
                                            && weapon.reload == 0x00
                                            && weapon.tower_g50_param_id == 0x0000
                                            && weapon.g_rank == 0x00
                                            && weapon.zenith_skill == 0x0000;

                                        if self.show_dummy_ranged_weapons {
                                            if !is_dummy { continue; }
                                            } else {
                                            if is_dummy { continue; }
                                        }

                                        // Apply filters
                                        if let Some(class_id) = self.class_id_filter {
                                            if weapon.class_id != class_id { continue; }
                                        }
                                        if let Some(element_id_filter) = self.element_filter {
                                            if element_id != element_id_filter { continue; }
                                        }
                                        if let Some(equip_type_id) = self.equip_type_filter {
                                            if weapon.equip_type != equip_type_id { continue; }
                                        }
                                        if let Some(weapon_type_id) = self.weapon_type_filter {
                                            if weapon_type != weapon_type_id { continue; }
                                        }
                                        if let Some(zenith_skill_id) = self.zenith_skill_filter {
                                            if weapon.zenith_skill != zenith_skill_id { continue; }
                                        }

                                        if !query.is_empty() && !weapon_name.to_lowercase().contains(&query) {
                                            continue;
                                        }

                                        let selected = self.selected_melee_index == Some(i);
                                        if ui.selectable_label(selected, format!("{}", i + 1)).clicked() {
                                            self.selected_melee_index = Some(i);
                                        }
                                        ui.label(format!("{}", model_id));
                                        ui.label(&weapon_name);
                                        ui.label(format!("{}", rarity + 1));
                                        ui.label(format!("{}", raw_damage));
                                        ui.label(format!("{}", affinity));
                                        ui.label(format!("{}", element_name(element_id)));
                                        ui.label(format!("{}", slots));
                                        ui.label(weapon_type_name(weapon_type));
                                        ui.end_row();
                                    }
                                });
                        });
                });
        }
    }

    fn show_ranged_weapons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Back to List").clicked() {
                self.selected_weapon_view = None;
            }
            ui.label(format!("Ranged Weapon Details"));
        });

        if let Some(index) = self.selected_melee_index {
            if index < self.ranged_weapons.len() {
                let weapon = &mut self.ranged_weapons[index];
                let name = self.ranged_weapon_names.get(index).cloned().unwrap_or_default();
                
                ui.horizontal(|ui| {
                    ui.label(format!("Name: {}", name));
                });

                Self::render_ranged_weapon_details(ui, weapon);
            }
        }
    }

    fn render_ranged_weapon_details(ui: &mut egui::Ui, weapon: &mut MhfdatRangedWeapon) {
        egui::ScrollArea::vertical().show(ui, |ui| {
                                            // Basic Stats
                                            ui.collapsing("Basic Stats", |ui| {
                                                let mut model_id = weapon.model_id;
                                                let mut rarity = weapon.rarity + 1;
                                                let mut class_id = weapon.class_id;
                                                let mut zenny_cost = weapon.zenny_cost;

                Self::render_editable_field(ui, "Model ID", &mut model_id);
                Self::render_editable_field(ui, "Rarity", &mut rarity);
                Self::render_combo_field(ui, "Class", &mut class_id, CLASS_ID_LIST);
                Self::render_editable_field(ui, "Zenny Cost", &mut zenny_cost);

                                                weapon.model_id = model_id;
                                                weapon.rarity = rarity.saturating_sub(1);
                                                weapon.class_id = class_id;
                                                weapon.zenny_cost = zenny_cost;
                                            });

                                            // Combat Stats
                                            ui.collapsing("Combat Stats", |ui| {
                                                let mut raw_damage = weapon.raw_damage;
                                                let mut defense = weapon.defense;
                                                let mut affinity = weapon.affinity;
                let mut recoil = weapon.recoil;
                let mut reload = weapon.reload;

                Self::render_editable_field(ui, "Raw Damage", &mut raw_damage);
                Self::render_editable_field(ui, "Defense", &mut defense);
                Self::render_editable_field(ui, "Affinity", &mut affinity);
                Self::render_combo_field(ui, "Recoil", &mut recoil, RECOIL_LIST);
                Self::render_combo_field(ui, "Reload", &mut reload, RELOAD_LIST);

                                                weapon.raw_damage = raw_damage;
                                                weapon.defense = defense;
                                                weapon.affinity = affinity;
                weapon.recoil = recoil;
                weapon.reload = reload;
                                            });

                                            // Element & Status
                                            ui.collapsing("Element & Status", |ui| {
                                                let mut element_id = weapon.element_id;
                                                let mut ele_damage = weapon.ele_damage;

                Self::render_combo_field(ui, "Element", &mut element_id, ELEMENT_ID_LIST);
                Self::render_editable_field(ui, "Element Damage", &mut ele_damage);

                                                weapon.element_id = element_id;
                                                weapon.ele_damage = ele_damage;
                                            });

                                            // Weapon Properties
                                            ui.collapsing("Weapon Properties", |ui| {
                                                let mut slots = weapon.slots;
                                                let mut weapon_attribute = weapon.weapon_attribute;
                                                let mut equip_type = weapon.equip_type;
                                                let mut weapon_type = weapon.weapon_type;
                let mut bullet = weapon.bullet;

                Self::render_editable_field(ui, "Slots", &mut slots);
                Self::render_editable_field(ui, "Weapon Attribute", &mut weapon_attribute);
                Self::render_combo_field(ui, "Equip Type", &mut equip_type, EQUIP_TYPE_LIST);
                Self::render_combo_field(ui, "Weapon Type", &mut weapon_type, WEAPON_TYPE_LIST);
                Self::render_editable_field(ui, "Bullet", &mut bullet);

                                                weapon.slots = slots;
                                                weapon.weapon_attribute = weapon_attribute;
                                                weapon.equip_type = equip_type;
                                                weapon.weapon_type = weapon_type;
                weapon.bullet = bullet;
                                            });

                                            // Advanced Properties
                                            ui.collapsing("Advanced Properties", |ui| {
                                                let mut tower_g50_param_id = weapon.tower_g50_param_id;
                                                let mut g_rank = weapon.g_rank;
                                                let mut zenith_skill = weapon.zenith_skill;
                let mut sort_order = weapon.sort_order_maybe;
                let mut max_slots = weapon.max_slots_maybe;

                Self::render_editable_field(ui, "Tower G50 Param ID", &mut tower_g50_param_id);
                Self::render_combo_field(ui, "G Rank", &mut g_rank, &[(0, "Non-G"), (1, "G-Rank")]);
                Self::render_combo_field(ui, "Zenith Skill", &mut zenith_skill, ZENITH_SKILL_LIST);
                Self::render_editable_field(ui, "Sort Order", &mut sort_order);
                Self::render_editable_field(ui, "Max Slots", &mut max_slots);

                                                weapon.tower_g50_param_id = tower_g50_param_id;
                                                weapon.g_rank = g_rank;
                                                weapon.zenith_skill = zenith_skill;
                weapon.sort_order_maybe = sort_order;
                weapon.max_slots_maybe = max_slots;
                                        });
                                });
                            }

    fn show_melee_weapons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Back to List").clicked() {
                self.selected_weapon_view = None;
            }
            ui.label(format!("Melee Weapon Details"));
        });

        if let Some(index) = self.selected_melee_index {
            if index < self.melee_weapons.len() {
                let weapon = &mut self.melee_weapons[index];
                let name = self.melee_weapon_names.get(index).cloned().unwrap_or_default();
                
                ui.horizontal(|ui| {
                    ui.label(format!("Name: {}", name));
                });

                Self::render_melee_weapon_details(ui, weapon);
            }
        }
    }

    fn render_melee_weapon_details(ui: &mut egui::Ui, weapon: &mut MhfdatMeleeWeapon) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Basic Stats
            ui.collapsing("Basic Stats", |ui| {
                let mut model_id = weapon.model_id;
                let mut rarity = weapon.rarity + 1;
                let mut class_id = weapon.class_id;
                let mut zenny_cost = weapon.zenny_cost;

                Self::render_editable_field(ui, "Model ID", &mut model_id);
                Self::render_editable_field(ui, "Rarity", &mut rarity);
                Self::render_combo_field(ui, "Class", &mut class_id, CLASS_ID_LIST);
                Self::render_editable_field(ui, "Zenny Cost", &mut zenny_cost);

                weapon.model_id = model_id;
                weapon.rarity = rarity.saturating_sub(1);
                weapon.class_id = class_id;
                weapon.zenny_cost = zenny_cost;
            });

            // Combat Stats
            ui.collapsing("Combat Stats", |ui| {
                let mut raw_damage = weapon.raw_damage;
                let mut defense = weapon.defense;
                let mut affinity = weapon.affinity;
                let mut sharpness_id = weapon.sharpness_id;
                let mut sharpness_max = weapon.sharpness_max;

                Self::render_editable_field(ui, "Raw Damage", &mut raw_damage);
                Self::render_editable_field(ui, "Defense", &mut defense);
                Self::render_editable_field(ui, "Affinity", &mut affinity);
                Self::render_editable_field(ui, "Sharpness ID", &mut sharpness_id);
                Self::render_editable_field(ui, "Sharpness Max", &mut sharpness_max);

                weapon.raw_damage = raw_damage;
                weapon.defense = defense;
                weapon.affinity = affinity;
                weapon.sharpness_id = sharpness_id;
                weapon.sharpness_max = sharpness_max;
            });

            // Element & Status
            ui.collapsing("Element & Status", |ui| {
                let mut element_id = weapon.element_id;
                let mut ele_damage = weapon.ele_damage;
                let mut ailment_id = weapon.ailment_id;
                let mut ail_damage = weapon.ail_damage;

                Self::render_combo_field(ui, "Element", &mut element_id, ELEMENT_ID_LIST);
                Self::render_editable_field(ui, "Element Damage", &mut ele_damage);
                Self::render_combo_field(ui, "Ailment", &mut ailment_id, AILMENT_ID_LIST);
                Self::render_editable_field(ui, "Ailment Damage", &mut ail_damage);

                weapon.element_id = element_id;
                weapon.ele_damage = ele_damage;
                weapon.ailment_id = ailment_id;
                weapon.ail_damage = ail_damage;
            });

            // Weapon Properties
            ui.collapsing("Weapon Properties", |ui| {
                let mut slots = weapon.slots;
                let mut weapon_attribute = weapon.weapon_attribute;
                let mut equip_type = weapon.equip_type;
                let mut weapon_type = weapon.weapon_type;
                let mut upgrade_path = weapon.upgrade_path;
                let mut other_model = weapon.other_model;

                Self::render_editable_field(ui, "Slots", &mut slots);
                Self::render_editable_field(ui, "Weapon Attribute", &mut weapon_attribute);
                Self::render_combo_field(ui, "Equip Type", &mut equip_type, EQUIP_TYPE_LIST);
                Self::render_combo_field(ui, "Weapon Type", &mut weapon_type, WEAPON_TYPE_LIST);
                Self::render_editable_field(ui, "Upgrade Path", &mut upgrade_path);
                Self::render_editable_field(ui, "Other Model", &mut other_model);

                weapon.slots = slots;
                weapon.weapon_attribute = weapon_attribute;
                weapon.equip_type = equip_type;
                weapon.weapon_type = weapon_type;
                weapon.upgrade_path = upgrade_path;
                weapon.other_model = other_model;
            });

            // Advanced Properties
            ui.collapsing("Advanced Properties", |ui| {
                let mut length = weapon.length;
                let mut visual_effects = weapon.visual_effects;
                let mut tower_g50_param_id = weapon.tower_g50_param_id;
                let mut g_rank = weapon.g_rank;
                let mut zenith_skill = weapon.zenith_skill;

                Self::render_editable_field(ui, "Length", &mut length);
                Self::render_editable_field(ui, "Visual Effects", &mut visual_effects);
                Self::render_editable_field(ui, "Tower G50 Param ID", &mut tower_g50_param_id);
                Self::render_combo_field(ui, "G Rank", &mut g_rank, &[(0, "Non-G"), (1, "G-Rank")]);
                Self::render_combo_field(ui, "Zenith Skill", &mut zenith_skill, ZENITH_SKILL_LIST);

                weapon.length = length;
                weapon.visual_effects = visual_effects;
                weapon.tower_g50_param_id = tower_g50_param_id;
                weapon.g_rank = g_rank;
                weapon.zenith_skill = zenith_skill;
            });
        });
    }

    fn load_transmog_entries(&mut self) {
        use crate::model::mhfdat::ShopEntry;
        use std::mem::size_of;
        let valid_types = [0x00, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let ptr_offset = crate::model::mhfdat_pointers::TRANSMOG_FORGING_PTR as usize;
        if self.buffer.len() < ptr_offset + 4 { return; }
        let data_offset = u32::from_le_bytes(self.buffer[ptr_offset..ptr_offset+4].try_into().unwrap()) as usize;
        let entry_size = size_of::<ShopEntry>();
        let mut entries = Vec::new();
        let mut cursor = data_offset;
        while cursor + entry_size <= self.buffer.len() {
            let equip_type = self.buffer[cursor];
            if !valid_types.contains(&equip_type) { break; }
            let entry = unsafe { std::ptr::read_unaligned(self.buffer.as_ptr().add(cursor) as *const ShopEntry) };
            entries.push(entry);
            cursor += entry_size;
        }
        self.transmog_entries = entries;
    }

    fn load_zenith_entries(&mut self) {
        use crate::model::mhfdat::ShopEntry;
        use std::mem::size_of;
        let valid_types = [0x00, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let ptr_offset = crate::model::mhfdat_pointers::ZENITH_WEAPON_FORGING_PTR as usize;
        if self.buffer.len() < ptr_offset + 4 { return; }
        let data_offset = u32::from_le_bytes(self.buffer[ptr_offset..ptr_offset+4].try_into().unwrap()) as usize;
        let entry_size = size_of::<ShopEntry>();
        let mut entries = Vec::new();
        let mut cursor = data_offset;
        while cursor + entry_size <= self.buffer.len() {
            let equip_type = self.buffer[cursor];
            if !valid_types.contains(&equip_type) { break; }
            let entry = unsafe { std::ptr::read_unaligned(self.buffer.as_ptr().add(cursor) as *const ShopEntry) };
            entries.push(entry);
            cursor += entry_size;
        }
        self.zenith_entries = entries;
    }

    fn load_ranged_weapon_names(&mut self) {
        use crate::model::mhfdat_pointers::{RANGED_WEAPON_NAMES_PTR, RANGED_WEAPON_DESC_PTR};
        let count = self.ranged_weapons.len();
        let mut offsets = Vec::with_capacity(count);
        for i in 0..count {
            let ptr = RANGED_WEAPON_NAMES_PTR as u64 + (i as u64) * 4;
            let mut cursor = std::io::Cursor::new(&self.buffer);
            cursor.seek(std::io::SeekFrom::Start(ptr)).ok();
            let mut buf = [0u8; 4];
            if cursor.read_exact(&mut buf).is_ok() {
                let str_offset = u32::from_le_bytes(buf);
                offsets.push(str_offset);
            }
        }
        let mut cursor2 = std::io::Cursor::new(&self.buffer);
        let names = extract_melee_weapon_names(
            &mut cursor2,
            RANGED_WEAPON_NAMES_PTR,
            count
        ).unwrap_or_default();
        self.ranged_weapon_names = names;

        // Load descriptions
        let mut cursor3 = std::io::Cursor::new(&self.buffer);
        let descs_full = extract_melee_weapon_descriptions_v2(
            &mut cursor3,
            RANGED_WEAPON_DESC_PTR,
            count,
            4
        ).unwrap_or_default();
        // Convert to Vec<[String; 4]> including mhfY field
        self.ranged_weapon_descriptions = descs_full.into_iter()
            .map(|descs| {
                let mut arr = [String::new(), String::new(), String::new(), String::new()];
                for (i, desc) in descs.into_iter().take(4).enumerate() {
                    arr[i] = desc;
                }
                arr
            })
            .collect();
        // Strict size synchronization
        let min_count = self.ranged_weapons.len().min(self.ranged_weapon_names.len()).min(self.ranged_weapon_descriptions.len());
        self.ranged_weapons.truncate(min_count);
        self.ranged_weapon_names.truncate(min_count);
        self.ranged_weapon_descriptions.truncate(min_count);
        if self.ranged_weapons.len() != self.ranged_weapon_names.len() || self.ranged_weapons.len() != self.ranged_weapon_descriptions.len() {
            self.error_message = Some(format!("[ERROR] Size mismatch: weapons={}, names={}, descriptions={}", 
                self.ranged_weapons.len(), 
                self.ranged_weapon_names.len(), 
                self.ranged_weapon_descriptions.len()));
        }
    }

    fn render_editable_field<T: eframe::emath::Numeric + Copy + PartialEq>(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut T
    ) {
        ui.horizontal(|ui| {
            ui.label(label);
            let mut temp = *value;
            if ui.add(egui::DragValue::new(&mut temp)).changed() {
                *value = temp;
            }
        });
    }

    fn render_combo_field<T: Copy + PartialEq>(
        ui: &mut egui::Ui,
        label: &str,
        value: &mut T,
        options: &[(T, &str)]
    ) {
        ui.horizontal(|ui| {
            ui.label(label);
            let current_text = options.iter()
                .find(|(v, _)| *v == *value)
                .map(|(_, name)| *name)
                .unwrap_or("Unknown");
            
            egui::ComboBox::from_id_source(format!("{}_{}", label, std::any::type_name::<T>()))
                .selected_text(current_text)
                .show_ui(ui, |ui| {
                    for (id, name) in options {
                        ui.selectable_value(value, *id, *name);
                    }
                });
        });
}

    fn show_workshop_tab(&mut self, ui: &mut egui::Ui) {
        // Ajout des sous-tabs
        ui.horizontal(|ui| {
            if ui.selectable_label(self.workshop_tab == WorkshopTab::Transmog, "Transmog").clicked() {
                self.workshop_tab = WorkshopTab::Transmog;
            }
            if ui.selectable_label(self.workshop_tab == WorkshopTab::ZenithWeapons, "Zenith Weapons").clicked() {
                self.workshop_tab = WorkshopTab::ZenithWeapons;
            }
            // Ajoutez ici d'autres sous-tabs si besoin
        });
        ui.separator();

        // Destructure fields into locals
        let (transmog_entries, transmog_open, zenith_entries, zenith_open, selected_melee_index, search_query) = (
            &mut self.transmog_entries,
            &mut self.transmog_open,
            &mut self.zenith_entries,
            &mut self.zenith_open,
            &mut self.selected_melee_index,
            &mut self.search_query,
        );
        match self.workshop_tab {
            WorkshopTab::Transmog => {
                Self::show_shop_entries(ui, transmog_entries, "Transmog", transmog_open, selected_melee_index, search_query, &self.melee_weapons, &self.melee_weapon_names, &self.ranged_weapon_names, &self.ranged_weapon_descriptions, &self.head_armor_names, &self.chest_armor_names, &self.arms_armor_names, &self.waist_armor_names, &self.legs_armor_names);
            }
            WorkshopTab::ZenithWeapons => {
                Self::show_shop_entries(ui, zenith_entries, "Zenith Weapons", zenith_open, selected_melee_index, search_query, &self.melee_weapons, &self.melee_weapon_names, &self.ranged_weapon_names, &self.ranged_weapon_descriptions, &self.head_armor_names, &self.chest_armor_names, &self.arms_armor_names, &self.waist_armor_names, &self.legs_armor_names);
            }
            _ => {}
    }
}

fn show_shop_entries(
    ui: &mut egui::Ui,
    entries: &mut Vec<ShopEntry>,
    title: &str,
    open: &mut Vec<bool>,
    selected_melee_index: &mut Option<usize>,
    search_query: &mut String,
    melee_weapons: &[MhfdatMeleeWeapon],
    melee_weapon_names: &[String],
    ranged_weapon_names: &[String],
    ranged_weapon_descriptions: &[[String; 4]],
    head_armor_names: &[String],
    chest_armor_names: &[String],
    arms_armor_names: &[String],
    waist_armor_names: &[String],
    legs_armor_names: &[String],
) {
        ui.horizontal(|ui| {
            ui.heading(title);
            if ui.button("Add New Entry").clicked() {
                entries.push(ShopEntry {
                    equip_type: 0,
                    purchaseable: 0,
                    equip_id: 0,
                    material_id1: 0,
                    material_id2: 0,
                    material_id3: 0,
                    material_id4: 0,
                    material_amnt1: 0,
                    material_amnt2: 0,
                    material_amnt3: 0,
                    material_amnt4: 0,
                    hr_req: 0,
                    preview_able: false,
                    ..Default::default()
                });
                *selected_melee_index = Some(entries.len() - 1);
            }
        });
        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(search_query);
        });

        // Show selected entry details if any
        if let Some(index) = *selected_melee_index {
            if let Some(entry) = entries.get_mut(index) {
                // Copy fields to local variables to avoid unaligned references
            let mut equip_type = entry.equip_type;
                let mut purchaseable = entry.purchaseable;
            let mut equip_id = entry.equip_id;
            let mut material_id1 = entry.material_id1;
            let mut material_id2 = entry.material_id2;
            let mut material_id3 = entry.material_id3;
            let mut material_id4 = entry.material_id4;
                let mut material_amnt1 = entry.material_amnt1;
                let mut material_amnt2 = entry.material_amnt2;
                let mut material_amnt3 = entry.material_amnt3;
            let mut material_amnt4 = entry.material_amnt4;
            let mut hr_req = entry.hr_req;
                let mut preview_able = entry.preview_able;

            egui::Frame::group(ui.style()).show(ui, |ui| {
                    ui.heading("Selected Entry Details");
                    egui::Grid::new("selected_entry_details")
                        .striped(true)
                        .show(ui, |ui| {
                        ui.label("Equip Type:");
                            if ui.add(egui::DragValue::new(&mut equip_type)).changed() {
                                entry.equip_type = equip_type;
                            }
                            ui.end_row();

                            ui.label("Purchaseable:");
                            if ui.add(egui::DragValue::new(&mut purchaseable)).changed() {
                                entry.purchaseable = purchaseable;
                            }
                            ui.end_row();

                        ui.label("Equip ID:");
                            if ui.add(egui::DragValue::new(&mut equip_id)).changed() {
                                entry.equip_id = equip_id;
                            }
                            ui.end_row();

                            ui.label("Name:");
                            let weapon_name = match equip_type {
                                0x06 => melee_weapon_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                0x07 => ranged_weapon_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                // Ajoute ici les cas pour les armures si besoin, ex:
                                0x00 => head_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                0x02 => chest_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                0x03 => arms_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                0x04 => waist_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                0x05 => legs_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                _ => format!("Unknown ({})", equip_id)
                            };
                            ui.label(&weapon_name);
                            ui.end_row();

                        ui.label("Material 1:");
                    ui.horizontal(|ui| {
                                if ui.add(egui::DragValue::new(&mut material_id1)).changed() {
                                    entry.material_id1 = material_id1;
                                }
                        ui.label("x");
                                if ui.add(egui::DragValue::new(&mut material_amnt1)).changed() {
                                    entry.material_amnt1 = material_amnt1;
                                }
                    });
                            ui.end_row();

                            ui.label("Material 2:");
                    ui.horizontal(|ui| {
                                if ui.add(egui::DragValue::new(&mut material_id2)).changed() {
                                    entry.material_id2 = material_id2;
                                }
                        ui.label("x");
                                if ui.add(egui::DragValue::new(&mut material_amnt2)).changed() {
                                    entry.material_amnt2 = material_amnt2;
                                }
                    });
                            ui.end_row();

                            ui.label("Material 3:");
                    ui.horizontal(|ui| {
                                if ui.add(egui::DragValue::new(&mut material_id3)).changed() {
                                    entry.material_id3 = material_id3;
                                }
                        ui.label("x");
                                if ui.add(egui::DragValue::new(&mut material_amnt3)).changed() {
                                    entry.material_amnt3 = material_amnt3;
                                }
                    });
                            ui.end_row();

                            ui.label("Material 4:");
                    ui.horizontal(|ui| {
                                if ui.add(egui::DragValue::new(&mut material_id4)).changed() {
                    entry.material_id4 = material_id4;
                                }
                                ui.label("x");
                                if ui.add(egui::DragValue::new(&mut material_amnt4)).changed() {
                    entry.material_amnt4 = material_amnt4;
                                }
                            });
                            ui.end_row();

                            ui.label("HR Required:");
                            if ui.add(egui::DragValue::new(&mut hr_req)).changed() {
                    entry.hr_req = hr_req;
                            }
                            ui.end_row();

                            ui.label("Previewable:");
                            if ui.checkbox(&mut preview_able, "").changed() {
                                entry.preview_able = preview_able;
                            }
                            ui.end_row();
                });
            });
        }
    }

        // List of entries
        egui::CollapsingHeader::new("Shop Entries")
            .default_open(true)
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("shop_entries_grid")
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("ID");
                ui.label("Equip Type");
                            ui.label("Purchaseable");
                ui.label("Equip ID");
                ui.label("Name");
                            ui.label("Materials");
                ui.label("HR Req");
                            ui.label("Preview");
                ui.end_row();

                let query = search_query.to_lowercase();
                            for (i, entry) in entries.iter_mut().enumerate() {
                                // Copy fields to local variables to avoid unaligned references
                    let equip_type = entry.equip_type;
                                let purchaseable = entry.purchaseable;
                    let equip_id = entry.equip_id;
                    let material_id1 = entry.material_id1;
                    let material_id2 = entry.material_id2;
                    let material_id3 = entry.material_id3;
                    let material_id4 = entry.material_id4;
                                let material_amnt1 = entry.material_amnt1;
                                let material_amnt2 = entry.material_amnt2;
                                let material_amnt3 = entry.material_amnt3;
                    let material_amnt4 = entry.material_amnt4;
                    let hr_req = entry.hr_req;
                                let preview_able = entry.preview_able;

                                let weapon_name = match equip_type {
                                    0x06 => melee_weapon_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                    0x07 => ranged_weapon_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                    // Ajoute ici les cas pour les armures si besoin, ex:
                                    0x00 => head_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                    0x02 => chest_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                    0x03 => arms_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                    0x04 => waist_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                    0x05 => legs_armor_names.get(equip_id as usize).cloned().unwrap_or_default(),
                                    _ => format!("Unknown ({})", equip_id)
                                };

                                if !query.is_empty() && !weapon_name.to_lowercase().contains(&query) {
                                    continue;
                                }

                                let selected = *selected_melee_index == Some(i);
                    if ui.selectable_label(selected, format!("{}", i + 1)).clicked() {
                                    *selected_melee_index = Some(i);
                    }

                                ui.label(format!("{}", equip_type));
                                ui.label(format!("{}", purchaseable));
                    ui.label(format!("{}", equip_id));
                                ui.label(&weapon_name);
                                
                                // Combine materials into a single cell
                                let materials = format!(
                                    "{}x{} {}x{} {}x{} {}x{}",
                                    material_id1, material_amnt1,
                                    material_id2, material_amnt2,
                                    material_id3, material_amnt3,
                                    material_id4, material_amnt4
                                );
                                ui.label(materials);
                                
                    ui.label(format!("{}", hr_req));
                                ui.label(format!("{}", preview_able));
                    ui.end_row();
                }
                        });
            });
        });
    }

    fn load_armor_data(&mut self, buffer: &[u8]) {
        use std::io::Cursor;
        let mut cursor = Cursor::new(buffer);

        // Head armors
        if let Ok(armors) = read_equipments_until_sentinel(&mut cursor, HEAD_ARMOR_PTR as u64) {
            self.head_armors = armors;
            if let Ok(names) = extract_armor_names(&mut cursor, HEAD_ARMOR_NAMES_PTR, self.head_armors.len()) {
                self.head_armor_names = names;
            }
            if let Ok(descs) = extract_armor_descriptions(&mut cursor, EQUIP_DESC_PTR, self.head_armors.len()) {
                self.head_armor_descriptions = descs;
            }
        }

        // Chest armors
        if let Ok(armors) = read_equipments_until_sentinel(&mut cursor, BODY_ARMOR_PTR as u64) {
            self.chest_armors = armors;
            if let Ok(names) = extract_armor_names(&mut cursor, BODY_ARMOR_NAMES_PTR, self.chest_armors.len()) {
                self.chest_armor_names = names;
            }
            if let Ok(descs) = extract_armor_descriptions(&mut cursor, EQUIP_DESC_PTR, self.chest_armors.len()) {
                self.chest_armor_descriptions = descs;
            }
        }
        // Arms armors
        if let Ok(armors) = read_equipments_until_sentinel(&mut cursor, ARM_ARMOR_PTR as u64) {
            self.arms_armors = armors;
            if let Ok(names) = extract_armor_names(&mut cursor, ARM_ARMOR_NAMES_PTR, self.arms_armors.len()) {
                self.arms_armor_names = names;
            }
            if let Ok(descs) = extract_armor_descriptions(&mut cursor, EQUIP_DESC_PTR, self.arms_armors.len()) {
                self.arms_armor_descriptions = descs;
            }
        }
        // Waist armors
        if let Ok(armors) = read_equipments_until_sentinel(&mut cursor, WAIST_ARMOR_PTR as u64) {
            self.waist_armors = armors;
            if let Ok(names) = extract_armor_names(&mut cursor, WAIST_ARMOR_NAMES_PTR, self.waist_armors.len()) {
                self.waist_armor_names = names;
            }
            if let Ok(descs) = extract_armor_descriptions(&mut cursor, EQUIP_DESC_PTR, self.waist_armors.len()) {
                self.waist_armor_descriptions = descs;
            }
        }
        // Legs armors
        if let Ok(armors) = read_equipments_until_sentinel(&mut cursor, LEG_ARMOR_PTR as u64) {
            self.legs_armors = armors;
            if let Ok(names) = extract_armor_names(&mut cursor, LEG_ARMOR_NAMES_PTR, self.legs_armors.len()) {
                self.legs_armor_names = names;
            }
            if let Ok(descs) = extract_armor_descriptions(&mut cursor, EQUIP_DESC_PTR, self.legs_armors.len()) {
                self.legs_armor_descriptions = descs;
            }
        }
    }
    
    fn show_armor_list(&mut self, ui: &mut egui::Ui) {
                    ui.horizontal(|ui| {
            ui.selectable_value(&mut self.armor_tab, ArmorTab::Head, "Head");
            ui.selectable_value(&mut self.armor_tab, ArmorTab::Chest, "Chest");
            ui.selectable_value(&mut self.armor_tab, ArmorTab::Arms, "Arms");
            ui.selectable_value(&mut self.armor_tab, ArmorTab::Waist, "Waist");
            ui.selectable_value(&mut self.armor_tab, ArmorTab::Legs, "Legs");
        });
        ui.add_space(8.0);
                    ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut self.armor_search_query);
        });
        ui.add_space(8.0);

        // Affichage des détails si sélectionné
        if let Some(index) = self.selected_armor_index {
            let (armor, name, descs) = match self.armor_tab {
                ArmorTab::Head => (&mut self.head_armors[index], &self.head_armor_names[index], &self.head_armor_descriptions[index]),
                ArmorTab::Chest => (&mut self.chest_armors[index], &self.chest_armor_names[index], &self.chest_armor_descriptions[index]),
                ArmorTab::Arms => (&mut self.arms_armors[index], &self.arms_armor_names[index], &self.arms_armor_descriptions[index]),
                ArmorTab::Waist => (&mut self.waist_armors[index], &self.waist_armor_names[index], &self.waist_armor_descriptions[index]),
                ArmorTab::Legs => (&mut self.legs_armors[index], &self.legs_armor_names[index], &self.legs_armor_descriptions[index]),
            };
            // --- Clone name and descriptions before closure ---
            let mut name_edit = name.clone();
            let mut descs_edit = [descs[0].clone(), descs[1].clone(), descs[2].clone()];
            egui::Frame::group(ui.style()).show(ui, |ui| {
                ui.heading(format!("Armor Details: {}", name_edit));
                // Editable name
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut name_edit);
                });
                // Editable descriptions
                for i in 0..3 {
                    ui.horizontal(|ui| {
                        ui.label(format!("Description {}:", i + 1));
                        ui.text_edit_singleline(&mut descs_edit[i]);
                    });
                }
                ui.separator();
                // ... rest of the armor details UI ...
                // (unchanged)
            });
            // --- Write back if changed ---
            match self.armor_tab {
                ArmorTab::Head => {
                    if self.head_armor_names[index] != name_edit {
                        self.head_armor_names[index] = name_edit.clone();
                    }
                    for i in 0..3 {
                        if self.head_armor_descriptions[index][i] != descs_edit[i] {
                            self.head_armor_descriptions[index][i] = descs_edit[i].clone();
                        }
                    }
                }
                ArmorTab::Chest => {
                    if self.chest_armor_names[index] != name_edit {
                        self.chest_armor_names[index] = name_edit.clone();
                    }
                    for i in 0..3 {
                        if self.chest_armor_descriptions[index][i] != descs_edit[i] {
                            self.chest_armor_descriptions[index][i] = descs_edit[i].clone();
                        }
                    }
                }
                ArmorTab::Arms => {
                    if self.arms_armor_names[index] != name_edit {
                        self.arms_armor_names[index] = name_edit.clone();
                    }
                    for i in 0..3 {
                        if self.arms_armor_descriptions[index][i] != descs_edit[i] {
                            self.arms_armor_descriptions[index][i] = descs_edit[i].clone();
                        }
                    }
                }
                ArmorTab::Waist => {
                    if self.waist_armor_names[index] != name_edit {
                        self.waist_armor_names[index] = name_edit.clone();
                    }
                    for i in 0..3 {
                        if self.waist_armor_descriptions[index][i] != descs_edit[i] {
                            self.waist_armor_descriptions[index][i] = descs_edit[i].clone();
                        }
                    }
                }
                ArmorTab::Legs => {
                    if self.legs_armor_names[index] != name_edit {
                        self.legs_armor_names[index] = name_edit.clone();
                    }
                    for i in 0..3 {
                        if self.legs_armor_descriptions[index][i] != descs_edit[i] {
                            self.legs_armor_descriptions[index][i] = descs_edit[i].clone();
                        }s
                    }
                }
            }
        }

        egui::CollapsingHeader::new("Armor List")
            .default_open(true)
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("armor_list_grid").striped(true).show(ui, |ui| {
                        ui.label("ID");
                        ui.label("Name");
                        ui.label("Rarity");
                        ui.label("Defense");
                        ui.label("Slots");
                        ui.label("Type");
                        ui.end_row();
                        let (armors, names) = match self.armor_tab {
                            ArmorTab::Head => (&self.head_armors, &self.head_armor_names),
                            ArmorTab::Chest => (&self.chest_armors, &self.chest_armor_names),
                            ArmorTab::Arms => (&self.arms_armors, &self.arms_armor_names),
                            ArmorTab::Waist => (&self.waist_armors, &self.waist_armor_names),
                            ArmorTab::Legs => (&self.legs_armors, &self.legs_armor_names),
                        };
                        for (i, (armor, name)) in armors.iter().zip(names.iter()).enumerate() {
                            if self.armor_search_query.is_empty() || name.to_lowercase().contains(&self.armor_search_query.to_lowercase()) {
                                let selected = self.selected_armor_index == Some(i);
                                let equip_id = armor.equip_id;
                                let rarity = armor.rarity;
                                let base_defense = armor.base_defense;
                                let slots = format!("{}/{}", armor.base_slots, armor.max_slots);
                                let armor_type = match self.armor_tab {
                                    ArmorTab::Head => "Head", ArmorTab::Chest => "Chest", ArmorTab::Arms => "Arms", ArmorTab::Waist => "Waist", ArmorTab::Legs => "Legs"
                                };
                                if ui.selectable_label(selected, format!("{}", equip_id)).clicked() {
                                    self.selected_armor_index = Some(i);
                                }
                                ui.label(name);
                                ui.label(format!("{}", rarity));
                                ui.label(format!("{}", base_defense));
                                ui.label(slots);
                                ui.label(armor_type);
                                ui.end_row();
                            }
                        }
                    });
                });
            });
    }
}

