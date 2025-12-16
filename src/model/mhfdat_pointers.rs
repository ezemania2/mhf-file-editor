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

pub const HEAD_ARMOR_DESC_PTR: u32 = 0x78;

pub const MELEE_WEAPONS_PTR: u32 = 0x7C;
pub const RANGED_WEAPONS_PTR: u32 = 0x80;
pub const RANGED_WEAPON_NAMES_PTR: u32 = 0x84;
pub const MELEE_WEAPON_NAMES_PTR: u32 = 0x88;
pub const MELEE_WEAPON_DESC_PTR: u32 = 0x8C;
pub const RANGED_WEAPON_DESC_PTR: u32 = 0x90;

pub const ARMOR_STAT_ARRAY_PTR: u32 = 0x94;
pub const ARMOR_WEAPON_STAT_ARRAY_PTR: u32 = 0x98;
pub const ARMOR_NAME_ARRAY_PTR: u32 = 0x9C;

pub const BULLET_SETS_PTR: u32 = 0xA8;
pub const SHARPNESS_IDS_PTR: u32 = 0xB0;

pub const EQUIPEMENT_COUNT_PTR: u32 = 0xE8;

pub const CARAVAN_SKILLS_PTR: u32 = 0x0F0;
pub const DECO_ID_PTR: u32 = 0xF8;
pub const DECO_ID_COUNT: usize = 6539;

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
