pub mod weapons;
pub mod armor;
pub mod items;
pub mod shop;
pub mod save;
pub mod load;
pub mod mhfjmp;

pub use mhfjmp::MhfjmpApp;

use eframe::{egui, App};
use native_dialog::FileDialog;
use std::fs;
use std::path::{PathBuf, Path};
use std::io::{Read, Seek, Write, SeekFrom};
use crate::model::mhfdat::{
    MhfdatMeleeWeapon, MhfdatRangedWeapon, MeleeWeaponExport, RangedWeaponExport, 
    ArmorExport, MhfdatItem, ItemExport, ShopEntry, DecoShop, SigilTowerTable, G50WUpgrade,
    MWUpgradePath, RWUpgradePath, EvoUpgrade,
    MhfdatEquipment, EquipmentCounts
};
use crate::utils::weapon_patterns::{class_name, CLASS_ID_LIST, element_name, ELEMENT_ID_LIST, ailment_name, AILMENT_ID_LIST, equip_type_name, EQUIP_TYPE_LIST, weapon_type_name, WEAPON_TYPE_LIST, zenith_skill_name, ZENITH_SKILL_LIST, recoil, RECOIL_LIST, reload, RELOAD_LIST};
use crate::core::mhfdat::{
    read_melee_weapons_until_sentinel, read_ranged_weapons_until_sentinel,
    read_shop_entries_until_sentinel, read_deco_shop_until_sentinel,
    read_sigil_tower_until_sentinel, read_g50_weapon_until_sentinel,
    read_mw_upgrade_until_sentinel, read_rw_upgrade_until_sentinel,
    read_evo_upgrade_until_sentinel, read_equipments_until_sentinel,
    read_items_until_sentinel, extract_item_names, extract_item_descriptions,
    extract_melee_weapon_names, extract_melee_weapon_descriptions_v2,
    extract_armor_names, extract_armor_descriptions, read_deco_id_table,
    read_deco_shop,
    write_melee_weapon, write_ranged_weapon, write_shop_entry,
    write_deco_shop, write_sigil_tower_table, write_g50_weapon_upgrade,
    write_mw_upgrade_path, write_rw_upgrade_path, write_evo_upgrade,
    write_weapon_names, write_ranged_weapon_names, write_armor_data, write_armor_names,
    write_armor_descriptions, write_transmog_data, write_zenith_data,
    append_bytes_to_file, read_mhfdat_offsets,
};
use crate::core::packing::pack_file;
use crate::model::mhfdat_pointers::{
    MELEE_WEAPONS_PTR, RANGED_WEAPONS_PTR, MELEE_WEAPON_NAMES_PTR, MELEE_WEAPON_DESC_PTR,
    RANGED_WEAPON_NAMES_PTR,
    HEAD_ARMOR_PTR, HEAD_ARMOR_NAMES_PTR,
    BODY_ARMOR_PTR, BODY_ARMOR_NAMES_PTR,
    ARM_ARMOR_PTR, ARM_ARMOR_NAMES_PTR,
    WAIST_ARMOR_PTR, WAIST_ARMOR_NAMES_PTR,
    LEG_ARMOR_PTR, LEG_ARMOR_NAMES_PTR,
    ITEM_DATA_PTR, ITEM_NAMES_PTR, ITEM_DESC_PTR, TRANSMOG_FORGING_PTR,
    MELEE_WEAPON_UPGRADE_PATH_PTR, RANGED_WEAPON_UPGRADE_PATH_PTR, DECO_ID_PTR,
    DECO_SHOP_PTR, DECO_G_SHOP_PTR, CUFF_SHOP_PTR, CUFF_GR_SHOP_PTR,
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
use std::collections::HashMap;

// Use the font from main.rs
use crate::NOTO_FONT;

#[derive(PartialEq)]
pub enum WeaponCategory {
    Melee,
    Ranged,
}

impl Default for WeaponCategory {
    fn default() -> Self {
        Self::Melee
    }
}

#[derive(PartialEq)]
pub enum MainTab {
    Weapons,
    Armor,
    Items,
    Shop,
}

impl Default for MainTab {
    fn default() -> Self {
        Self::Weapons
    }
}

#[derive(PartialEq)]
pub enum WorkshopTab {
    Transmog,
    Deco,
    Weapon,
    Armor,
}

impl Default for WorkshopTab {
    fn default() -> Self {
        Self::Transmog
    }
}

#[derive(PartialEq)]
pub enum WeaponTab {
    Melee,
    Ranged,
}

impl Default for WeaponTab {
    fn default() -> Self {
        Self::Melee
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ArmorTab {
    Head,
    Body,
    Arms,
    Waist,
    Legs,
}

impl Default for ArmorTab {
    fn default() -> Self {
        Self::Head
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ViewMode {
    List,
    Details,
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
    pub workshop_entries: Vec<ShopEntry>,
    pub show_weapons_menu: bool,
    pub selected_weapon_view: Option<WeaponCategory>,
    pub selected_melee_index: Option<usize>,
    pub selected_ranged_index: Option<usize>,
    pub search_query: String,
    pub class_id_filter: Option<u8>,
    pub element_filter: Option<u8>,
    pub ailment_filter: Option<u8>,
    pub equip_type_filter: Option<u8>,
    pub shop_equip_type_filter: Option<u8>,
    pub weapon_type_filter: Option<u32>,
    pub zenith_skill_filter: Option<u16>,
    pub melee_weapon_names: Vec<String>,
    pub show_dummy_weapons: bool,
    pub show_dummy_ranged_weapons: bool,
    pub melee_weapon_descriptions: Vec<[String; 3]>,
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
    pub selected_transmog_index: Option<usize>,
    pub weapon_tab: WeaponTab,
    pub ranged_weapon_names: Vec<String>,
    pub ranged_weapon_descriptions: Vec<[String; 3]>,
    pub armor_tab: ArmorTab,
    pub head_armors: Vec<MhfdatEquipment>,
    pub body_armors: Vec<MhfdatEquipment>,
    pub arms_armors: Vec<MhfdatEquipment>,
    pub waist_armors: Vec<MhfdatEquipment>,
    pub legs_armors: Vec<MhfdatEquipment>,
    pub head_armor_names: Vec<String>,
    pub body_armor_names: Vec<String>,
    pub arms_armor_names: Vec<String>,
    pub waist_armor_names: Vec<String>,
    pub legs_armor_names: Vec<String>,
    pub selected_armor_index: Option<usize>,
    pub armor_search_query: String,
    pub armor_loaded: bool,
    pub debug_logs: Vec<String>,
    pub equip_descs: Vec<MhfdatEquipment>,
    pub equip_desc_names: Vec<String>,
    pub armor_descriptions: Vec<[String; 3]>,
    pub view_mode: HashMap<String, ViewMode>,
    pub shop_page: u32,
    pub armor_page: u32,
    pub melee_weapons_page: u32,
    pub ranged_weapons_page: u32,
    pub equipment_counts: Option<EquipmentCounts>,
    
    // Items/Objects
    pub items: Vec<MhfdatItem>,
    pub item_names: Vec<String>,
    pub item_descriptions: Vec<String>,
    pub selected_item_index: Option<usize>,
    pub item_page: u32,
    pub should_return_to_selector: bool,

    // Decorations (DecoID)
    pub deco_ids: Vec<crate::model::mhfdat::MhfdatDecoId>,
    pub deco_page: u32,
    pub deco_search: String,
    pub deco_skill_filter: Option<u8>,
    pub deco_zenith_filter: Option<u16>,
    pub selected_deco_index: Option<usize>,
    pub deco_skill_search: String,
    pub deco_zenith_search: String,
    pub deco_shop_hr_entries: Vec<crate::model::mhfdat::DecoShop>,
    pub deco_shop_gr_entries: Vec<crate::model::mhfdat::DecoShop>,
    pub cuff_shop_entries: Vec<crate::model::mhfdat::DecoShop>,
    pub cuff_gr_shop_entries: Vec<crate::model::mhfdat::DecoShop>,
    pub selected_deco_shop_index: Option<usize>,
    pub deco_shop_search: String,
    pub deco_shop_page: u32,
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
            selected_ranged_index: None,
            search_query: String::new(),
            class_id_filter: None,
            element_filter: None,
            ailment_filter: None,
            equip_type_filter: None,
            shop_equip_type_filter: None,
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
            selected_transmog_index: None,
            weapon_tab: WeaponTab::Melee,
            ranged_weapon_names: Vec::new(),
            ranged_weapon_descriptions: Vec::new(),
            armor_tab: ArmorTab::Head,
            head_armors: Vec::new(),
            body_armors: Vec::new(),
            arms_armors: Vec::new(),
            waist_armors: Vec::new(),
            legs_armors: Vec::new(),
            head_armor_names: Vec::new(),
            body_armor_names: Vec::new(),
            arms_armor_names: Vec::new(),
            waist_armor_names: Vec::new(),
            legs_armor_names: Vec::new(),
            selected_armor_index: None,
            armor_search_query: String::new(),
            armor_loaded: false,
            debug_logs: Vec::new(),
            equip_descs: Vec::new(),
            equip_desc_names: Vec::new(),
            armor_descriptions: Vec::new(),
            view_mode: HashMap::new(),
            shop_page: 0,
            armor_page: 0,
            melee_weapons_page: 0,
            ranged_weapons_page: 0,
            equipment_counts: None,
            
            // Items/Objects
            items: Vec::new(),
            item_names: Vec::new(),
            item_descriptions: Vec::new(),
            selected_item_index: None,
            item_page: 0,
            should_return_to_selector: false,
            deco_ids: Vec::new(),
            deco_page: 0,
            deco_search: String::new(),
            deco_skill_filter: None,
            deco_zenith_filter: None,
            selected_deco_index: None,
            deco_skill_search: String::new(),
            deco_zenith_search: String::new(),
            deco_shop_hr_entries: Vec::new(),
            deco_shop_gr_entries: Vec::new(),
            cuff_shop_entries: Vec::new(),
            cuff_gr_shop_entries: Vec::new(),
            selected_deco_shop_index: None,
            deco_shop_search: String::new(),
            deco_shop_page: 0,
        }
    }
}

impl MhfdatApp {
    pub fn load_file(&mut self, path: PathBuf, data: Vec<u8>) {
        // Load the file data into the app
        self.current_file = Some(path);
        self.buffer = data.clone();
        // Debug: log header pointers and resolved targets
        let read_ptr = |buf: &Vec<u8>, at: u32| -> Option<u32> {
            let off = at as usize;
            if buf.len() >= off + 4 {
                Some(u32::from_le_bytes([buf[off], buf[off+1], buf[off+2], buf[off+3]]))
            } else { None }
        };
        if let Some(val) = read_ptr(&self.buffer, MELEE_WEAPONS_PTR) {
            self.debug_logs.push(format!("PTR 0x{:03X} (MELEE) -> 0x{:08X}", MELEE_WEAPONS_PTR, val));
        }
        if let Some(val) = read_ptr(&self.buffer, RANGED_WEAPONS_PTR) {
            self.debug_logs.push(format!("PTR 0x{:03X} (RANGED) -> 0x{:08X}", RANGED_WEAPONS_PTR, val));
        }
        if let Some(val) = read_ptr(&self.buffer, ITEM_DATA_PTR) {
            self.debug_logs.push(format!("PTR 0x{:03X} (ITEMS) -> 0x{:08X}", ITEM_DATA_PTR, val));
        }
        if let Some(val) = read_ptr(&self.buffer, TRANSMOG_FORGING_PTR) {
            self.debug_logs.push(format!("PTR 0x{:03X} (TRANSMOG) -> 0x{:08X}", TRANSMOG_FORGING_PTR, val));
        }
        if let Some(val) = read_ptr(&self.buffer, MELEE_WEAPON_UPGRADE_PATH_PTR) {
            self.debug_logs.push(format!("PTR 0x{:03X} (MW UPGRADES) -> 0x{:08X}", MELEE_WEAPON_UPGRADE_PATH_PTR, val));
        }
        if let Some(val) = read_ptr(&self.buffer, RANGED_WEAPON_UPGRADE_PATH_PTR) {
            self.debug_logs.push(format!("PTR 0x{:03X} (RW UPGRADES) -> 0x{:08X}", RANGED_WEAPON_UPGRADE_PATH_PTR, val));
        }
        if let Some(val) = read_ptr(&self.buffer, DECO_ID_PTR) {
            self.debug_logs.push(format!(
                "PTR 0x{:03X} (DECOID) -> 0x{:08X}",
                DECO_ID_PTR, val
            ));
        }
        
        // Load weapons
        if let Some((melee_offset, ranged_offset)) = read_mhfdat_offsets(&self.buffer) {
            let mut cursor = std::io::Cursor::new(&self.buffer);
            if let Ok(melee_weapons) = read_melee_weapons_until_sentinel(&mut cursor, melee_offset as u64) {
                self.melee_weapons = melee_weapons;
            }
            
            let mut cursor2 = std::io::Cursor::new(&self.buffer);
            if let Ok(ranged_weapons) = read_ranged_weapons_until_sentinel(&mut cursor2, ranged_offset as u64) {
                self.ranged_weapons = ranged_weapons;
            }
        }
        
        // Load transmog entries
        self.load_transmog_entries();
        
        
        // Load weapon names and descriptions
        {
            use crate::model::mhfdat_pointers::{MELEE_WEAPON_NAMES_PTR, MELEE_WEAPON_DESC_PTR, RANGED_WEAPON_NAMES_PTR, RANGED_WEAPON_DESC_PTR};
            use crate::model::mhfdat_pointers::{MELEE_WEAPON_UPGRADE_PATH_PTR, RANGED_WEAPON_UPGRADE_PATH_PTR};
            use crate::core::mhfdat::{extract_melee_weapon_names, extract_melee_weapon_descriptions_v2, extract_ranged_weapon_names};
            
            // Melee weapons
            let count = self.melee_weapons.len();
            let mut cursor = std::io::Cursor::new(&self.buffer);
            let names = extract_melee_weapon_names(&mut cursor, MELEE_WEAPON_NAMES_PTR, count).unwrap_or_default();
            self.melee_weapon_names = names;
            
            let mut cursor2 = std::io::Cursor::new(&self.buffer);
            let descs_full = extract_melee_weapon_descriptions_v2(&mut cursor2, MELEE_WEAPON_DESC_PTR, count, 4).unwrap_or_default();
            self.melee_weapon_descriptions = descs_full.into_iter()
                .map(|descs| {
                    let mut arr = [String::new(), String::new(), String::new()];
                    for (i, desc) in descs.into_iter().take(3).enumerate() {
                        arr[i] = desc;
                    }
                    arr
                })
                .collect();
            
            // Ranged weapons
            let ranged_count = self.ranged_weapons.len();
            let mut cursor3 = std::io::Cursor::new(&self.buffer);
            let ranged_names = extract_ranged_weapon_names(&mut cursor3, RANGED_WEAPON_NAMES_PTR, ranged_count).unwrap_or_default();
            self.ranged_weapon_names = ranged_names;

            // Ranged weapon descriptions via header pointer RANGED_WEAPON_DESC_PTR
            let mut cursor4 = std::io::Cursor::new(&self.buffer);
            let ranged_descs_full = extract_melee_weapon_descriptions_v2(&mut cursor4, RANGED_WEAPON_DESC_PTR, ranged_count, 4).unwrap_or_default();
            self.ranged_weapon_descriptions = ranged_descs_full
                .into_iter()
                .map(|descs| {
                    let mut arr = [String::new(), String::new(), String::new()];
                    for (i, desc) in descs.into_iter().take(3).enumerate() {
                        arr[i] = desc;
                    }
                    arr
                })
                .collect();
            // Strict size synchronization
            let min_ranged = self.ranged_weapons.len().min(self.ranged_weapon_names.len()).min(self.ranged_weapon_descriptions.len());
            self.ranged_weapons.truncate(min_ranged);
            self.ranged_weapon_names.truncate(min_ranged);
            self.ranged_weapon_descriptions.truncate(min_ranged);
        }
        
        // Load armor data (this also loads items)
        let buffer_ref = self.buffer.clone();
        self.load_armor_data(&buffer_ref);

        // Load decorations (DecoID table)
        if let Some(deco_off) = read_ptr(&self.buffer, DECO_ID_PTR) {
            // If the table has a fixed count in the pattern, prefer that to avoid premature stop
            self.deco_ids = read_deco_id_table(&self.buffer, deco_off as usize, Some(crate::model::mhfdat_pointers::DECO_ID_COUNT));
            // Debug: log source and small preview
            self.debug_logs.push(format!(
                "DecoID: header@0x{:03X} -> data@0x{:08X}, count={}",
                DECO_ID_PTR,
                deco_off,
                self.deco_ids.len()
            ));
            // Hex preview of the first bytes at target
            if (deco_off as usize) < self.buffer.len() {
                let start = deco_off as usize;
                let end = (start + 64).min(self.buffer.len());
                let mut hex = String::new();
                for b in &self.buffer[start..end] {
                    use std::fmt::Write as _;
                    let _ = write!(&mut hex, "{:02X} ", b);
                }
                self.debug_logs.push(format!("DecoID bytes[0..{}] @0x{:08X}: {}", end-start, deco_off, hex));
            } else {
                self.debug_logs.push(format!("DecoID offset 0x{:08X} is out of buffer range (len={})", deco_off, self.buffer.len()));
            }
            for (i, di) in self.deco_ids.iter().take(3).enumerate() {
                let slots = di.slot_nb as u8;
                let flags = di.flags as u16;
                let price = di.price as u32;
                let s1 = di.skill_id1 as u8; let p1 = di.skill_pts1 as i8;
                let s2 = di.skill_id2 as u8; let p2 = di.skill_pts2 as i8;
                let s3 = di.skill_id3 as u8; let p3 = di.skill_pts3 as i8;
                let s4 = di.skill_id4 as u8; let p4 = di.skill_pts4 as i8;
                let zen = di.zenith_skill as u16;
                self.debug_logs.push(format!(
                    "  [{}] slots={}, flags=0x{:04X}, price={}, skills=({}:{}, {}:{}, {}:{}, {}:{}), zenith={}",
                    i, slots, flags, price, s1, p1, s2, p2, s3, p3, s4, p4, zen
                ));
            }
        }

        // Load upgrade paths for melee and ranged weapons (dereferenced pointers)
        {
            if let Some(mw_off) = read_ptr(&self.buffer, MELEE_WEAPON_UPGRADE_PATH_PTR) {
                let mut cur = std::io::Cursor::new(&self.buffer);
                if let Ok(mw) = read_mw_upgrade_until_sentinel(&mut cur, mw_off as u64) {
                    self.mw_upgrade_entries = mw;
                    // Debug: log source and preview
                    self.debug_logs.push(format!(
                        "MW upgrades: header@0x{:03X} -> data@0x{:08X}, count={}",
                        MELEE_WEAPON_UPGRADE_PATH_PTR,
                        mw_off,
                        self.mw_upgrade_entries.len()
                    ));
                    for (i, up) in self.mw_upgrade_entries.iter().take(3).enumerate() {
                        let mat1 = up.upgrade_material1; let qty1 = up.num_material1;
                        let mat2 = up.upgrade_material2; let qty2 = up.num_material2;
                        let mat3 = up.upgrade_material3; let qty3 = up.num_material3;
                        let to1 = up.upgrades_to1; let to2 = up.upgrades_to2;
                        let to3 = up.upgrades_to3; let to4 = up.upgrades_to4;
                        self.debug_logs.push(format!(
                            "  [{}] mats: ({}x{}, {}x{}, {}x{}), to: [{}, {}, {}, {}]",
                            i, mat1, qty1, mat2, qty2, mat3, qty3, to1, to2, to3, to4
                        ));
                    }
                }
            }
            if let Some(rw_off) = read_ptr(&self.buffer, RANGED_WEAPON_UPGRADE_PATH_PTR) {
                let mut cur2 = std::io::Cursor::new(&self.buffer);
                if let Ok(rw) = read_rw_upgrade_until_sentinel(&mut cur2, rw_off as u64) {
                    self.rw_upgrade_entries = rw;
                    // Debug: log source and preview
                    self.debug_logs.push(format!(
                        "RW upgrades: header@0x{:03X} -> data@0x{:08X}, count={}",
                        RANGED_WEAPON_UPGRADE_PATH_PTR,
                        rw_off,
                        self.rw_upgrade_entries.len()
                    ));
                    for (i, up) in self.rw_upgrade_entries.iter().take(3).enumerate() {
                        let mat1 = up.upgrade_material1; let qty1 = up.num_material1;
                        let mat2 = up.upgrade_material2; let qty2 = up.num_material2;
                        let mat3 = up.upgrade_material3; let qty3 = up.num_material3;
                        let to1 = up.upgrades_to1; let to2 = up.upgrades_to2;
                        let to3 = up.upgrades_to3; let to4 = up.upgrades_to4;
                        self.debug_logs.push(format!(
                            "  [{}] mats: ({}x{}, {}x{}, {}x{}), to: [{}, {}, {}, {}]",
                            i, mat1, qty1, mat2, qty2, mat3, qty3, to1, to2, to3, to4
                        ));
                    }
                }
            }
        }
        
        self.error_message = Some("File loaded successfully.".to_string());
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
                self.should_return_to_selector = true;
            }
            
            ui.horizontal(|ui| {
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

            // Main tabs menu
            ui.horizontal(|ui| {
                if ui.selectable_label(self.main_tab == MainTab::Weapons, "Weapons").clicked() {
                    self.main_tab = MainTab::Weapons;
                }
                if ui.selectable_label(self.main_tab == MainTab::Armor, "Armor").clicked() {
                    self.main_tab = MainTab::Armor;
                }
                if ui.selectable_label(self.main_tab == MainTab::Items, "Items").clicked() {
                    self.main_tab = MainTab::Items;
                }
                if ui.selectable_label(self.main_tab == MainTab::Shop, "Shop").clicked() {
                    self.main_tab = MainTab::Shop;
                }
            });
            ui.separator();

            match self.main_tab {
                MainTab::Weapons => {
                    self.show_weapons_tab(ui);
                }
                MainTab::Armor => {
                    self.show_armor_tab(ui);
                }
                MainTab::Items => {
                    self.show_items_tab(ui);
                }
                MainTab::Shop => {
                    self.show_shop_tab(ui);
                }
            }

            // Place Debug Logs at the end of the CentralPanel UI
            egui::CollapsingHeader::new("Debug Logs")
                .default_open(false)
                .show(ui, |ui| {
                    for log in &self.debug_logs {
                        ui.label(log);
                    }
                });
        });
    }
}

// Re-export the modules
pub use weapons::*;
pub use armor::*;
pub use shop::*; 