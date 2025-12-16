pub mod weapons;
pub mod armor;
pub mod items;
pub mod shop;
pub mod save;
pub mod load;
pub mod mhfjmp;
pub mod automatic_skills;

pub use mhfjmp::MhfjmpApp;

use eframe::{egui, App};
use native_dialog::FileDialog;
use std::fs;
use std::path::{PathBuf, Path};
use std::io::{Read, Seek, Write, SeekFrom};
use crate::model::mhfdat::{
    MhfdatMeleeWeapon, MhfdatRangedWeapon, MeleeWeaponExport, RangedWeaponExport, 
    ArmorExport, MhfdatItem, ItemExport, ShopEntry, DecoShop, SigilTowerTable, G50WUpgrade,
    MWUpgradePath, RWUpgradePath, EvoUpgrade, AutomaticSkill,
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
    read_deco_shop, read_automatic_skills,
    write_melee_weapon, write_ranged_weapon, write_shop_entry,
    write_deco_shop, write_sigil_tower_table, write_g50_weapon_upgrade,
    write_mw_upgrade_path, write_rw_upgrade_path, write_evo_upgrade,
    write_weapon_names, write_ranged_weapon_names, write_armor_data, write_armor_names,
    write_armor_descriptions, write_transmog_data, write_zenith_data,
    write_automatic_skills_block,
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
    ITEM_DATA_PTR, ITEM_NAMES_PTR, ITEM_DESC_PTR, TRANSMOG_FORGING_PTR, WEAPON_FORGING_PTR, ARMOR_FORGING_PTR,
    G_RANK_WEAPON_SHOP_PTR, G_RANK_ARMOR_SHOP_PTR, ZENITH_WEAPON_FORGING_PTR, ZENITH_ARMOR_FORGING_PTR,
    MELEE_WEAPON_UPGRADE_PATH_PTR, RANGED_WEAPON_UPGRADE_PATH_PTR, DECO_ID_PTR,
    DECO_SHOP_PTR, DECO_G_SHOP_PTR, CUFF_SHOP_PTR, CUFF_GR_SHOP_PTR,
    AUTOMATIC_SKILLS_TABLE_PTR,
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
    AutomaticSkills,
}

impl Default for MainTab {
    fn default() -> Self {
        Self::Weapons
    }
}

#[derive(PartialEq)]
pub enum WorkshopCategory {
    WeaponShops,
    ArmorShops,
    Deco,
}

impl Default for WorkshopCategory {
    fn default() -> Self {
        Self::ArmorShops
    }
}

#[derive(PartialEq)]
pub enum WeaponShopTab {
    ForgingHR,
    ForgingGR,
    ForgingZenith,
}

impl Default for WeaponShopTab {
    fn default() -> Self {
        Self::ForgingHR
    }
}

#[derive(PartialEq)]
pub enum ArmorShopTab {
    Transmog,
    ForgingHR,
    ForgingGR,
    ForgingZenith,
}

impl Default for ArmorShopTab {
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
    pub zenith_skill_search: String,
    pub armor_skill1_search: String,
    pub armor_skill2_search: String,
    pub armor_skill3_search: String,
    pub armor_skill4_search: String,
    pub armor_skill5_search: String,
    pub armor_zenith_skill_search: String,
    pub melee_weapon_names: Vec<String>,
    pub show_dummy_weapons: bool,
    pub show_dummy_ranged_weapons: bool,
    pub melee_weapon_descriptions: Vec<[String; 4]>,
    pub should_encrypt: bool,
    pub should_pack: bool,
    pub deco_shop_entries: Vec<DecoShop>,
    pub sigil_tower_entries: Vec<SigilTowerTable>,
    pub g50_weapon_entries: Vec<G50WUpgrade>,
    pub mw_upgrade_entries: Vec<MWUpgradePath>,
    pub rw_upgrade_entries: Vec<RWUpgradePath>,
    pub evo_upgrade_entries: Vec<EvoUpgrade>,
    pub transmog_entries: Vec<ShopEntry>,
    pub zenith_entries: Vec<ShopEntry>,
    pub weapon_forging_entries: Vec<ShopEntry>,
    pub armor_forging_entries: Vec<ShopEntry>,
    pub weapon_forging_gr_entries: Vec<ShopEntry>,
    pub armor_forging_gr_entries: Vec<ShopEntry>,
    pub weapon_forging_zenith_entries: Vec<ShopEntry>,
    pub armor_forging_zenith_entries: Vec<ShopEntry>,
    pub transmog_open: Vec<bool>,
    pub zenith_open: Vec<bool>,
    pub selected_transmog_index: Option<usize>,
    pub selected_weapon_forging_index: Option<usize>,
    pub selected_armor_forging_index: Option<usize>,
    pub selected_weapon_forging_gr_index: Option<usize>,
    pub selected_armor_forging_gr_index: Option<usize>,
    pub selected_weapon_forging_zenith_index: Option<usize>,
    pub selected_armor_forging_zenith_index: Option<usize>,
    pub workshop_category: WorkshopCategory,
    pub weapon_shop_tab: WeaponShopTab,
    pub armor_shop_tab: ArmorShopTab,
    pub weapon_tab: WeaponTab,
    pub ranged_weapon_names: Vec<String>,
    pub ranged_weapon_descriptions: Vec<[String; 4]>,
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
    
    // Automatic Skills Table
    pub automatic_skills: Vec<AutomaticSkill>,
    pub automatic_skills_search: String,
    pub automatic_skills_page: u32,
    pub selected_automatic_skill_index: Option<usize>,
    pub automatic_skills_eq_type_filter: Option<u8>,
    pub automatic_skills_skill_search: String,
    pub automatic_skills_modified: bool,
    pub original_automatic_skills_offset: Option<u32>,
    
    // Modification tracking for all data types
    pub melee_weapons_modified: bool,
    pub original_melee_weapons_offset: Option<u32>,
    pub ranged_weapons_modified: bool,
    pub original_ranged_weapons_offset: Option<u32>,
    pub head_armors_modified: bool,
    pub original_head_armors_offset: Option<u32>,
    pub body_armors_modified: bool,
    pub original_body_armors_offset: Option<u32>,
    pub arms_armors_modified: bool,
    pub original_arms_armors_offset: Option<u32>,
    pub waist_armors_modified: bool,
    pub original_waist_armors_offset: Option<u32>,
    pub legs_armors_modified: bool,
    pub original_legs_armors_offset: Option<u32>,
    pub items_modified: bool,
    pub original_items_offset: Option<u32>,
    pub transmog_modified: bool,
    pub original_transmog_offset: Option<u32>,
    pub weapon_forging_modified: bool,
    pub original_weapon_forging_offset: Option<u32>,
    pub armor_forging_modified: bool,
    pub original_armor_forging_offset: Option<u32>,
    pub weapon_forging_gr_modified: bool,
    pub original_weapon_forging_gr_offset: Option<u32>,
    pub armor_forging_gr_modified: bool,
    pub original_armor_forging_gr_offset: Option<u32>,
    pub weapon_forging_zenith_modified: bool,
    pub original_weapon_forging_zenith_offset: Option<u32>,
    pub armor_forging_zenith_modified: bool,
    pub original_armor_forging_zenith_offset: Option<u32>,
    pub deco_shop_hr_modified: bool,
    pub original_deco_shop_hr_offset: Option<u32>,
    pub deco_shop_gr_modified: bool,
    pub original_deco_shop_gr_offset: Option<u32>,
    pub cuff_shop_modified: bool,
    pub original_cuff_shop_offset: Option<u32>,
    pub cuff_gr_shop_modified: bool,
    pub original_cuff_gr_shop_offset: Option<u32>,
    pub mw_upgrades_modified: bool,
    pub original_mw_upgrades_offset: Option<u32>,
    pub rw_upgrades_modified: bool,
    pub original_rw_upgrades_offset: Option<u32>,
    pub melee_weapon_names_modified: bool,
    pub original_melee_weapon_names_offset: Option<u32>,
    pub melee_weapon_descriptions_modified: bool,
    pub original_melee_weapon_descriptions_offset: Option<u32>,
    pub ranged_weapon_names_modified: bool,
    pub original_ranged_weapon_names_offset: Option<u32>,
    pub ranged_weapon_descriptions_modified: bool,
    pub original_ranged_weapon_descriptions_offset: Option<u32>,
    pub head_armor_names_modified: bool,
    pub original_head_armor_names_offset: Option<u32>,
    pub body_armor_names_modified: bool,
    pub original_body_armor_names_offset: Option<u32>,
    pub arms_armor_names_modified: bool,
    pub original_arms_armor_names_offset: Option<u32>,
    pub waist_armor_names_modified: bool,
    pub original_waist_armor_names_offset: Option<u32>,
    pub legs_armor_names_modified: bool,
    pub original_legs_armor_names_offset: Option<u32>,
    pub item_names_modified: bool,
    pub original_item_names_offset: Option<u32>,
    pub item_descriptions_modified: bool,
    pub original_item_descriptions_offset: Option<u32>,
}

impl Default for MhfdatApp {
    fn default() -> Self {
        Self {
            error_message: None,
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
            zenith_skill_search: String::new(),
            armor_skill1_search: String::new(),
            armor_skill2_search: String::new(),
            armor_skill3_search: String::new(),
            armor_skill4_search: String::new(),
            armor_skill5_search: String::new(),
            armor_zenith_skill_search: String::new(),
            melee_weapon_names: Vec::new(),
            show_dummy_weapons: false,
            show_dummy_ranged_weapons: false,
            melee_weapon_descriptions: Vec::new(),
            should_encrypt: false,
            should_pack: false,
            workshop_category: WorkshopCategory::default(),
            weapon_shop_tab: WeaponShopTab::default(),
            armor_shop_tab: ArmorShopTab::default(),
            deco_shop_entries: Vec::new(),
            sigil_tower_entries: Vec::new(),
            g50_weapon_entries: Vec::new(),
            mw_upgrade_entries: Vec::new(),
            rw_upgrade_entries: Vec::new(),
            evo_upgrade_entries: Vec::new(),
            transmog_entries: Vec::new(),
            zenith_entries: Vec::new(),
            weapon_forging_entries: Vec::new(),
            armor_forging_entries: Vec::new(),
            weapon_forging_gr_entries: Vec::new(),
            armor_forging_gr_entries: Vec::new(),
            weapon_forging_zenith_entries: Vec::new(),
            armor_forging_zenith_entries: Vec::new(),
            transmog_open: vec![true],
            zenith_open: vec![true],
            selected_transmog_index: None,
            selected_weapon_forging_index: None,
            selected_armor_forging_index: None,
            selected_weapon_forging_gr_index: None,
            selected_armor_forging_gr_index: None,
            selected_weapon_forging_zenith_index: None,
            selected_armor_forging_zenith_index: None,
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
            
            // Automatic Skills Table
            automatic_skills: Vec::new(),
            automatic_skills_search: String::new(),
            automatic_skills_page: 0,
            selected_automatic_skill_index: None,
            automatic_skills_eq_type_filter: None,
            automatic_skills_skill_search: String::new(),
            automatic_skills_modified: false,
            original_automatic_skills_offset: None,
            
            // Modification tracking
            melee_weapons_modified: false,
            original_melee_weapons_offset: None,
            ranged_weapons_modified: false,
            original_ranged_weapons_offset: None,
            head_armors_modified: false,
            original_head_armors_offset: None,
            body_armors_modified: false,
            original_body_armors_offset: None,
            arms_armors_modified: false,
            original_arms_armors_offset: None,
            waist_armors_modified: false,
            original_waist_armors_offset: None,
            legs_armors_modified: false,
            original_legs_armors_offset: None,
            items_modified: false,
            original_items_offset: None,
            transmog_modified: false,
            original_transmog_offset: None,
            weapon_forging_modified: false,
            original_weapon_forging_offset: None,
            armor_forging_modified: false,
            original_armor_forging_offset: None,
            weapon_forging_gr_modified: false,
            original_weapon_forging_gr_offset: None,
            armor_forging_gr_modified: false,
            original_armor_forging_gr_offset: None,
            weapon_forging_zenith_modified: false,
            original_weapon_forging_zenith_offset: None,
            armor_forging_zenith_modified: false,
            original_armor_forging_zenith_offset: None,
            deco_shop_hr_modified: false,
            original_deco_shop_hr_offset: None,
            deco_shop_gr_modified: false,
            original_deco_shop_gr_offset: None,
            cuff_shop_modified: false,
            original_cuff_shop_offset: None,
            cuff_gr_shop_modified: false,
            original_cuff_gr_shop_offset: None,
            mw_upgrades_modified: false,
            original_mw_upgrades_offset: None,
            rw_upgrades_modified: false,
            original_rw_upgrades_offset: None,
            melee_weapon_names_modified: false,
            original_melee_weapon_names_offset: None,
            melee_weapon_descriptions_modified: false,
            original_melee_weapon_descriptions_offset: None,
            ranged_weapon_names_modified: false,
            original_ranged_weapon_names_offset: None,
            ranged_weapon_descriptions_modified: false,
            original_ranged_weapon_descriptions_offset: None,
            head_armor_names_modified: false,
            original_head_armor_names_offset: None,
            body_armor_names_modified: false,
            original_body_armor_names_offset: None,
            arms_armor_names_modified: false,
            original_arms_armor_names_offset: None,
            waist_armor_names_modified: false,
            original_waist_armor_names_offset: None,
            legs_armor_names_modified: false,
            original_legs_armor_names_offset: None,
            item_names_modified: false,
            original_item_names_offset: None,
            item_descriptions_modified: false,
            original_item_descriptions_offset: None,
        }
    }
}

impl MhfdatApp {
    pub fn load_file(&mut self, path: PathBuf, data: Vec<u8>) {
        // Load the file data into the app
        self.current_file = Some(path);
        self.buffer = data.clone();
        // Clear previous logs; only armor-upgrade debug is kept elsewhere on demand
        self.debug_logs.clear();
        // Armor Upgrade logs (RegAUpgradeRow) – pointer, offset, count, preview
        // Armor Upgrade logs: if the pointer region is a table of pointers, each points to 16 raw rows
        // removed armor-upgrade debug block
        
        // Load weapons
        if let Some((melee_offset, ranged_offset)) = read_mhfdat_offsets(&self.buffer) {
            self.original_melee_weapons_offset = Some(melee_offset);
            self.melee_weapons_modified = false;
            self.original_ranged_weapons_offset = Some(ranged_offset);
            self.ranged_weapons_modified = false;
            
            let mut cursor = std::io::Cursor::new(&self.buffer);
            if let Ok(melee_weapons) = read_melee_weapons_until_sentinel(&mut cursor, melee_offset as u64) {
                self.melee_weapons = melee_weapons;
            }
            
            let mut cursor2 = std::io::Cursor::new(&self.buffer);
            if let Ok(ranged_weapons) = read_ranged_weapons_until_sentinel(&mut cursor2, ranged_offset as u64) {
                self.ranged_weapons = ranged_weapons;
            }
        }

        // After weapons are loaded, refresh counts from entries
        #[allow(unused)]
        {
            // Access refresh fn from weapons module impl
            self.refresh_weapon_counts_from_entries();
        }
        
        // Load transmog entries
        self.load_transmog_entries();
        
        // Load weapon forging entries
        self.load_weapon_forging_entries();
        
        // Load armor forging entries
        self.load_armor_forging_entries();
        
        // Load G-Rank forging entries
        self.load_weapon_forging_gr_entries();
        self.load_armor_forging_gr_entries();
        
        // Load Zenith forging entries
        self.load_weapon_forging_zenith_entries();
        self.load_armor_forging_zenith_entries();
        
        // Load weapon names and descriptions
        {
            use crate::model::mhfdat_pointers::{
                MELEE_WEAPON_NAMES_PTR, MELEE_WEAPON_DESC_PTR,
                RANGED_WEAPON_NAMES_PTR, RANGED_WEAPON_DESC_PTR
            };
            use crate::model::mhfdat_pointers::{MELEE_WEAPON_UPGRADE_PATH_PTR, RANGED_WEAPON_UPGRADE_PATH_PTR};
            use crate::core::mhfdat::{extract_melee_weapon_names, extract_melee_weapon_descriptions_v2, extract_ranged_weapon_names};
            
            // Melee weapons
            let count = self.melee_weapons.len();
            
            // Save original offsets
            if self.buffer.len() >= MELEE_WEAPON_NAMES_PTR as usize + 4 {
                self.original_melee_weapon_names_offset = Some(u32::from_le_bytes(self.buffer[MELEE_WEAPON_NAMES_PTR as usize..MELEE_WEAPON_NAMES_PTR as usize+4].try_into().unwrap()));
                self.melee_weapon_names_modified = false;
            }
            if self.buffer.len() >= MELEE_WEAPON_DESC_PTR as usize + 4 {
                self.original_melee_weapon_descriptions_offset = Some(u32::from_le_bytes(self.buffer[MELEE_WEAPON_DESC_PTR as usize..MELEE_WEAPON_DESC_PTR as usize+4].try_into().unwrap()));
                self.melee_weapon_descriptions_modified = false;
            }
            
            let mut cursor = std::io::Cursor::new(&self.buffer);
            let names = extract_melee_weapon_names(&mut cursor, MELEE_WEAPON_NAMES_PTR, count).unwrap_or_default();
            self.melee_weapon_names = names;
            
            let mut cursor2 = std::io::Cursor::new(&self.buffer);
            let descs_full = extract_melee_weapon_descriptions_v2(&mut cursor2, MELEE_WEAPON_DESC_PTR, count, 4).unwrap_or_default();
            self.melee_weapon_descriptions = descs_full.into_iter()
                .map(|descs| {
                    let mut arr = [String::new(), String::new(), String::new(), String::new()];
                    for (i, desc) in descs.into_iter().take(4).enumerate() {
                        arr[i] = desc;
                    }
                    arr
                })
                .collect();
            
            // Ranged weapons
            let ranged_count = self.ranged_weapons.len();
            
            // Save original offsets
            if self.buffer.len() >= RANGED_WEAPON_NAMES_PTR as usize + 4 {
                self.original_ranged_weapon_names_offset = Some(u32::from_le_bytes(self.buffer[RANGED_WEAPON_NAMES_PTR as usize..RANGED_WEAPON_NAMES_PTR as usize+4].try_into().unwrap()));
                self.ranged_weapon_names_modified = false;
            }
            if self.buffer.len() >= RANGED_WEAPON_DESC_PTR as usize + 4 {
                self.original_ranged_weapon_descriptions_offset = Some(u32::from_le_bytes(self.buffer[RANGED_WEAPON_DESC_PTR as usize..RANGED_WEAPON_DESC_PTR as usize+4].try_into().unwrap()));
                self.ranged_weapon_descriptions_modified = false;
            }
            
            let mut cursor3 = std::io::Cursor::new(&self.buffer);
            let ranged_names = extract_ranged_weapon_names(&mut cursor3, RANGED_WEAPON_NAMES_PTR, ranged_count).unwrap_or_default();
            self.ranged_weapon_names = ranged_names;

            // Ranged weapon descriptions via header pointer RANGED_WEAPON_DESC_PTR
            let mut cursor4 = std::io::Cursor::new(&self.buffer);
            let ranged_descs_full = extract_melee_weapon_descriptions_v2(&mut cursor4, RANGED_WEAPON_DESC_PTR, ranged_count, 4).unwrap_or_default();
            self.ranged_weapon_descriptions = ranged_descs_full
                .into_iter()
                .map(|descs| {
                    let mut arr = [String::new(), String::new(), String::new(), String::new()];
                    for (i, desc) in descs.into_iter().take(4).enumerate() {
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

        // After armors are loaded, refresh equipment counts from entries
        #[allow(unused)]
        {
            self.refresh_equipment_counts_from_entries();
        }

        // Load decorations (DecoID table)
        if let Some(deco_off) = {
            let off = DECO_ID_PTR as usize;
            if self.buffer.len() >= off + 4 { Some(u32::from_le_bytes(self.buffer[off..off+4].try_into().unwrap())) } else { None }
        } {
            self.deco_ids = read_deco_id_table(&self.buffer, deco_off as usize, Some(crate::model::mhfdat_pointers::DECO_ID_COUNT));
        }

        // Load automatic skills table
        if let Some(auto_skills_off) = {
            let off = AUTOMATIC_SKILLS_TABLE_PTR as usize;
            if self.buffer.len() >= off + 4 { Some(u32::from_le_bytes(self.buffer[off..off+4].try_into().unwrap())) } else { None }
        } {
            self.automatic_skills = read_automatic_skills(&self.buffer, auto_skills_off as usize);
            self.original_automatic_skills_offset = Some(auto_skills_off);
            self.automatic_skills_modified = false;
        }

        // Load upgrade paths for melee and ranged weapons (dereferenced pointers)
        if let Some(mw_off) = {
            let off = MELEE_WEAPON_UPGRADE_PATH_PTR as usize;
            if self.buffer.len() >= off + 4 { Some(u32::from_le_bytes(self.buffer[off..off+4].try_into().unwrap())) } else { None }
        } {
            self.original_mw_upgrades_offset = Some(mw_off);
            self.mw_upgrades_modified = false;
            let mut cur = std::io::Cursor::new(&self.buffer);
            if let Ok(mw) = read_mw_upgrade_until_sentinel(&mut cur, mw_off as u64) {
                self.mw_upgrade_entries = mw;
            }
        }
        if let Some(rw_off) = {
            let off = RANGED_WEAPON_UPGRADE_PATH_PTR as usize;
            if self.buffer.len() >= off + 4 { Some(u32::from_le_bytes(self.buffer[off..off+4].try_into().unwrap())) } else { None }
        } {
            self.original_rw_upgrades_offset = Some(rw_off);
            self.rw_upgrades_modified = false;
            let mut cur2 = std::io::Cursor::new(&self.buffer);
            if let Ok(rw) = read_rw_upgrade_until_sentinel(&mut cur2, rw_off as u64) {
                self.rw_upgrade_entries = rw;
            }
        }
        
        self.error_message = Some("File loaded successfully.".to_string());
    }

    pub fn unload_file(&mut self) {
        // Clear all file data from memory
        self.buffer.clear();
        self.current_file = None;
        self.error_message = None;
        
        // Clear weapons
        self.melee_weapons.clear();
        self.ranged_weapons.clear();
        self.melee_weapon_names.clear();
        self.ranged_weapon_names.clear();
        self.melee_weapon_descriptions.clear();
        self.ranged_weapon_descriptions.clear();
        self.selected_melee_index = None;
        self.selected_ranged_index = None;
        self.selected_weapon_view = None;
        
        // Clear armors
        self.head_armors.clear();
        self.body_armors.clear();
        self.arms_armors.clear();
        self.waist_armors.clear();
        self.legs_armors.clear();
        self.head_armor_names.clear();
        self.body_armor_names.clear();
        self.arms_armor_names.clear();
        self.waist_armor_names.clear();
        self.legs_armor_names.clear();
        self.armor_descriptions.clear();
        self.selected_armor_index = None;
        
        // Clear items
        self.items.clear();
        self.item_names.clear();
        self.item_descriptions.clear();
        
        // Clear shop/upgrade entries
        self.workshop_entries.clear();
        self.transmog_entries.clear();
        self.zenith_entries.clear();
        self.deco_shop_entries.clear();
        self.sigil_tower_entries.clear();
        self.g50_weapon_entries.clear();
        self.mw_upgrade_entries.clear();
        self.rw_upgrade_entries.clear();
        self.evo_upgrade_entries.clear();
        
        // Clear decorations and automatic skills
        self.deco_ids.clear();
        self.automatic_skills.clear();
        self.automatic_skills_modified = false;
        self.original_automatic_skills_offset = None;
        
        // Clear equipment descriptions
        self.equip_descs.clear();
        self.equip_desc_names.clear();
        
        // Clear search/filter states
        self.search_query.clear();
        self.armor_search_query.clear();
        self.zenith_skill_search.clear();
        self.armor_skill1_search.clear();
        self.armor_skill2_search.clear();
        self.armor_skill3_search.clear();
        self.armor_skill4_search.clear();
        self.armor_skill5_search.clear();
        self.armor_zenith_skill_search.clear();
        self.class_id_filter = None;
        self.element_filter = None;
        self.ailment_filter = None;
        self.equip_type_filter = None;
        self.shop_equip_type_filter = None;
        self.weapon_type_filter = None;
        self.zenith_skill_filter = None;
        
        // Reset pages
        self.shop_page = 0;
        self.armor_page = 0;
        
        // Clear debug logs
        self.debug_logs.clear();
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
                // Unload file from memory
                self.unload_file();
                self.should_return_to_selector = true;
            }
            
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    if let Some(path) = &self.current_file {
                        // 1. Sauvegarder les données
                        match self.save_modified_data() {
                            Ok(()) => {
                                self.error_message = Some("File saved successfully.".to_string());
                            },
                            Err(e) => self.error_message = Some(format!("Failed to save file: {e}")),
                        }
                    } else {
                        self.error_message = Some("No file loaded.".to_string());
                    }
                }
                
                // Bouton de compression JPK Type 4
                if ui.button("Compress").clicked() {
                    if let Some(_path) = &self.current_file {
                        match self.compress_file() {
                            Ok(()) => {
                                if let Some(path) = &self.current_file {
                                    let file_name = path.file_name()
                                        .unwrap_or_default().to_string_lossy();
                                    self.error_message = Some(format!("File {} compressed successfully.", file_name));
                                } else {
                                    self.error_message = Some("File compressed successfully.".to_string());
                                }
                            },
                            Err(e) => self.error_message = Some(format!("Compression failed: {e}")),
                        }
                    } else {
                        self.error_message = Some("No file loaded.".to_string());
                    }
                }
                
                // Bouton d'encryption ECD
                if ui.button("Encrypt").clicked() {
                    if let Some(_path) = &self.current_file {
                        match self.encrypt_file() {
                            Ok(()) => {
                                if let Some(path) = &self.current_file {
                                    let file_name = path.file_name()
                                        .unwrap_or_default().to_string_lossy();
                                    self.error_message = Some(format!("File {} encrypted successfully.", file_name));
                                } else {
                                    self.error_message = Some("File encrypted successfully.".to_string());
                                }
                            },
                            Err(e) => self.error_message = Some(format!("Encryption failed: {e}")),
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

            // Main tabs menu (wrapped for responsiveness)
            ui.horizontal_wrapped(|ui| {
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
                if ui.selectable_label(self.main_tab == MainTab::AutomaticSkills, "Automatic Skills").clicked() {
                    self.main_tab = MainTab::AutomaticSkills;
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
                MainTab::AutomaticSkills => {
                    self.show_automatic_skills_tab(ui);
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

impl MhfdatApp {
    /// Render a horizontally wrapped row for responsive layouts
    pub fn responsive_row<F: FnOnce(&mut egui::Ui)>(ui: &mut egui::Ui, add_contents: F) {
        ui.horizontal_wrapped(|ui| {
            add_contents(ui);
        });
    }

    /// Compute a reasonable max height for list areas based on available space
    pub fn compute_list_max_height(ui: &egui::Ui) -> f32 {
        let available = ui.available_height();
        (available - 150.0).max(240.0)
    }

    /// Standard vertical scroll with responsive max height
    pub fn list_scroll<F: FnOnce(&mut egui::Ui)>(ui: &mut egui::Ui, id: &str, add_contents: F) {
        egui::ScrollArea::vertical()
            .id_source(id)
            .max_height(Self::compute_list_max_height(ui))
            .auto_shrink([false, false])
            .show(ui, |ui| {
                add_contents(ui);
            });
    }

    /// Unified pagination controls
    pub fn pagination_controls(ui: &mut egui::Ui, current_page: &mut u32, total_pages: usize) {
        let total_pages = total_pages.max(1);
        ui.horizontal(|ui| {
            let cp = (*current_page as usize).min(total_pages - 1);
            
            // First page button
            if ui.add_enabled(cp > 0, egui::Button::new("⏮ First")).clicked() {
                *current_page = 0;
            }
            
            // Previous page button
            if ui.add_enabled(cp > 0, egui::Button::new("← Previous")).clicked() {
                *current_page = (cp - 1) as u32;
            }
            
            // Page indicator
            ui.label(format!("Page {} of {}", cp + 1, total_pages));
            
            // Next page button
            if ui.add_enabled(cp + 1 < total_pages, egui::Button::new("Next →")).clicked() {
                *current_page = (cp + 1) as u32;
            }
            
            // Last page button
            if ui.add_enabled(cp + 1 < total_pages, egui::Button::new("Last ⏭")).clicked() {
                *current_page = (total_pages - 1) as u32;
            }
        });
    }

    /// Unified section header with top-left aligned actions, then title
    pub fn section_header<F: FnOnce(&mut egui::Ui)>(ui: &mut egui::Ui, title: &str, add_left_actions: F) {
        ui.horizontal(|ui| {
            add_left_actions(ui);
            ui.heading(title);
        });
        ui.separator();
    }
}

// Re-export the modules
pub use weapons::*;
pub use armor::*;
pub use shop::*; 