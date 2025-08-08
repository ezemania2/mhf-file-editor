// This file will store all pointers for mhfdat.
// Add pointer definitions here as you reverse or document the format. 

// All known mhfdat pointers (except signatures)
// Names and comments are based on mhf-pattern/mhfdat/header.hexpat

pub const ARMOR_FORGING_PTR: u32 = 0x034;
pub const OTHER_WEAPON_FORGING_PTR: u32 = 0x038;
pub const MELEE_WEAPON_UPGRADE_PATH_PTR: u32 = 0x03C;
pub const RANGED_WEAPON_UPGRADE_PATH_PTR: u32 = 0x040;
pub const DECO_SHOP_PTR: u32 = 0x044;
pub const ARMOR_UPGRADE_MATS_PTR: u32 = 0x04C;

pub const HEAD_ARMOR_PTR: u32 = 0x050;
pub const BODY_ARMOR_PTR: u32 = 0x054;
pub const ARM_ARMOR_PTR: u32 = 0x058;
pub const WAIST_ARMOR_PTR: u32 = 0x05C;
pub const LEG_ARMOR_PTR: u32 = 0x060;

pub const HEAD_ARMOR_NAMES_PTR: u32 = 0x064;
pub const BODY_ARMOR_NAMES_PTR: u32 = 0x068;
pub const ARM_ARMOR_NAMES_PTR: u32 = 0x06C;
pub const WAIST_ARMOR_NAMES_PTR: u32 = 0x070;
pub const LEG_ARMOR_NAMES_PTR: u32 = 0x074;

pub const HEAD_ARMOR_DESC_PTR: u32 = 0x078;

pub const MELEE_WEAPONS_PTR: u32 = 0x07C;
pub const RANGED_WEAPONS_PTR: u32 = 0x080;
pub const RANGED_WEAPON_NAMES_PTR: u32 = 0x084;
pub const MELEE_WEAPON_NAMES_PTR: u32 = 0x088;
pub const MELEE_WEAPON_DESC_PTR: u32 = 0x08C;
pub const RANGED_WEAPON_DESC_PTR: u32 = 0x090;

pub const ARMOR_STAT_ARRAY_PTR: u32 = 0x094;
pub const ARMOR_WEAPON_STAT_ARRAY_PTR: u32 = 0x098;
pub const ARMOR_NAME_ARRAY_PTR: u32 = 0x09C;

pub const BULLET_SETS_PTR: u32 = 0x0A8;
pub const SHARPNESS_IDS_PTR: u32 = 0x0B0;

pub const EQUIPEMENT_COUNT_PTR: u32 = 0x0E8;

pub const HUNTER_PEARL_SKILLS_PTR: u32 = 0x0F0;
pub const DECO_ID_PTR: u32 = 0x0F8;

pub const ITEM_DATA_PTR: u32 = 0xFC;
pub const ITEM_NAMES_PTR: u32 = 0x100;
pub const ITEM_DESC_PTR: u32 = 0x12C;
pub const TRANSMOG_FORGING_PTR: u32 = 0xAB8;

