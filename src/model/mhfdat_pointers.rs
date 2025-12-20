// This file will store all pointers for mhfdat.
// Add pointer definitions here as you reverse or document the format. 

// All known mhfdat pointers (except signatures)
// Names and comments are based on mhf-pattern/mhfdat/header.hexpat

pub const ARMOR_FORGING_PTR: u32 = 0x34;
pub const WEAPON_FORGING_PTR: u32 = 0x38;
pub const MELEE_WEAPON_UPGRADE_PATH_PTR: u32 = 0x3C;
pub const RANGED_WEAPON_UPGRADE_PATH_PTR: u32 = 0x40;
pub const DECO_SHOP_PTR: u32 = 0x44;
pub const ARMOR_UPGRADE_MATS_PTR: u32 = 0x4C;

pub const HEAD_ARMOR_PTR: u32 = 0x50;
pub const BODY_ARMOR_PTR: u32 = 0x54;
pub const ARM_ARMOR_PTR: u32 = 0x58;
pub const WAIST_ARMOR_PTR: u32 = 0x5C;
pub const LEG_ARMOR_PTR: u32 = 0x60;

pub const HEAD_ARMOR_NAMES_PTR: u32 = 0x64;
pub const BODY_ARMOR_NAMES_PTR: u32 = 0x68;
pub const ARM_ARMOR_NAMES_PTR: u32 = 0x6C;
pub const WAIST_ARMOR_NAMES_PTR: u32 = 0x70;
pub const LEG_ARMOR_NAMES_PTR: u32 = 0x74;

pub const ARMOR_DESC_PTR: u32 = 0x78;

pub const MELEE_WEAPONS_PTR: u32 = 0x7C;
pub const RANGED_WEAPONS_PTR: u32 = 0x80;
pub const RANGED_WEAPON_NAMES_PTR: u32 = 0x84;
pub const MELEE_WEAPON_NAMES_PTR: u32 = 0x88;
pub const MELEE_WEAPON_DESC_PTR: u32 = 0x8C;
pub const RANGED_WEAPON_DESC_PTR: u32 = 0x90;
pub const ARMOR_STAT_ARRAY_PTR: u32 = 0x94;
pub const ARMOR_WEAPON_NAMES_ARRAY_PTR: u32 = 0x98;
pub const ARMOR_NAME_ARRAY_PTR: u32 = 0x9C;

pub const BULLET_SETS_PTR: u32 = 0xA4;

pub const EQUIPEMENT_COUNT_PTR: u32 = 0xE8;
pub const CARVE_PARTS_PTR: u32 = 0x120;
pub const CARVE_PARTS_COUNT_PTR: u32 = 0x00CD419C;
pub const PART_BREAK_DROP_PTR: u32 = 0x124;
pub const PART_BREAK_DROP_COUNT_PTR: u32 = 0x00CD419E;
pub const MOSNTERS_DESCRIPTION_PTR: u32 = 0x134;
pub const MOSNTERS_DESCRIPTION_COUNT_PTR: u32 = 0x00CD41A2;
pub const SWORD_AND_SHIELD_G50_TOWER_PARAMS_PTR: u32 = 0x5C4;
pub const DUAL_BLADES_G50_TOWER_PARAMS_PTR: u32 = 0x5C8;
pub const GREAT_SWORD_G50_TOWER_PARAMS_PTR: u32 = 0x5CC;
pub const LONG_SWORD_G50_TOWER_PARAMS_PTR: u32 = 0x5D0;
pub const LANCE_G50_TOWER_PARAMS_PTR: u32 = 0x5D4;
pub const GUNLANCE_G50_TOWER_PARAMS_PTR: u32 = 0x5D8;
pub const HAMMER_G50_TOWER_PARAMS_PTR: u32 = 0x5DC;
pub const HUNTING_HORN_G50_TOWER_PARAMS_PTR: u32 = 0x5E0;
pub const HEAVY_BOWGUN_G50_TOWER_PARAMS_PTR: u32 = 0x5E4;
pub const LIGHT_BOWGUN_G50_TOWER_PARAMS_PTR: u32 = 0x5E8;
pub const BOW_G50_TOWER_PARAMS_PTR: u32 = 0x5EC;
pub const TONFA_G50_TOWER_PARAMS_PTR: u32 = 0x888;
pub const SWITCH_AXE_G50_TOWER_PARAMS_PTR: u32 = 0xA90;
pub const MAGNET_SPIKE_G50_TOWER_PARAMS_PTR: u32 = 0xB98;
pub const G50_MELEE_WEAPON_UPGRADE_PTR: u32 = 0x5F8;
pub const G50_MELEE_WEAPON_UPGRADE_COUNT_LIMITER_PTR: u32 = 0x00CD43AA;
pub const G50_RANGED_WEAPON_UPGRADE_PTR: u32 = 0x5FC;
pub const G50_RANGED_WEAPON_UPGRADE_COUNT_LIMITER_PTR: u32 = 0x00CD43AC;
pub const GR_QUEST_LIST_PTR: u32 = 0x6E0;
pub const HR_QUEST_LIST_PTR: u32 = 0xA98;


pub const DECO_ID_PTR: u32 = 0xF8;
pub const DECO_ID_COUNT_LIMITER_PTR: u32 = 0x00CD418A;

pub const ITEM_DATA_PTR: u32 = 0xFC;
pub const ITEM_NAMES_PTR: u32 = 0x100;
pub const ITEM_DESC_PTR: u32 = 0x12C;
pub const CUFF_SHOP_PTR: u32 = 0x2C0;
pub const G_RANK_WEAPON_SHOP_PTR: u32 = 0x5F0;
pub const G_RANK_ARMOR_SHOP_PTR: u32 = 0x5F4;
pub const CUFF_GR_SHOP_PTR: u32 = 0x750;
pub const ZENITH_WEAPON_FORGING_PTR: u32 = 0xAC0;
pub const ZENITH_ARMOR_FORGING_PTR: u32 = 0xAC4;
pub const TRANSMOG_FORGING_PTR: u32 = 0xAB8;
pub const DECO_G_SHOP_PTR: u32 = 0xB48;

pub const AUTOMATIC_SKILLS_TABLE_PTR: u32 = 0x804;
pub const AUTOMATIC_SKILLS_COUNT_LIMITER_PTR: u32 = 0x00CD4478;

// Sharpness pointers (base address: 0x018FF4C0)
// Note: Light Bowgun and Heavy Bowgun are ranged weapons and don't have sharpness
pub const SHARPNESS_GREAT_SWORD_PTR: u32 = 0x018FF4C0;      // Offset 0x00
pub const SHARPNESS_HAMMER_PTR: u32 = 0x018FF4C8;           // Offset 0x08
pub const SHARPNESS_LANCE_PTR: u32 = 0x018FF4CC;            // Offset 0x0C
pub const SHARPNESS_SWORD_AND_SHIELD_PTR: u32 = 0x018FF4D0; // Offset 0x10
pub const SHARPNESS_DUAL_BLADES_PTR: u32 = 0x018FF4D8;      // Offset 0x18
pub const SHARPNESS_LONG_SWORD_PTR: u32 = 0x018FF4DC;       // Offset 0x1C
pub const SHARPNESS_HUNTING_HORN_PTR: u32 = 0x018FF4E0;     // Offset 0x20
pub const SHARPNESS_GUNLANCE_PTR: u32 = 0x018FF4E4;         // Offset 0x24
pub const SHARPNESS_BOW_PTR: u32 = 0x018FF4E8;              // Offset 0x28
pub const SHARPNESS_TONFA_PTR: u32 = 0x018FF4EC;            // Offset 0x2C
pub const SHARPNESS_SWITCH_AXE_PTR: u32 = 0x018FF4F0;       // Offset 0x30
pub const SHARPNESS_MAGNET_SPIKE_PTR: u32 = 0x018FF4F4;     // Offset 0x34
