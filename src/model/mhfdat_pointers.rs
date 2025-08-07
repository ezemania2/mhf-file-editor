// This file will store all pointers for mhfdat.
// Add pointer definitions here as you reverse or document the format. 

// All known mhfdat pointers (except signatures)
// Names and comments are based on mhf-pattern/mhfdat/header.hexpat

pub const IMPORTANT_NUMS_PTR: u32 = 0x010;
pub const ARMOR_FORGING_PTR: u32 = 0x028;
pub const OTHER_WEAPON_FORGING_PTR: u32 = 0x02C;
pub const MELEE_WEAPON_UPGRADE_PATH_PTR: u32 = 0x030;
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

pub const EQUIP_DESC_PTR: u32 = 0x078;

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

pub const CARVE_ITEM_DROPS_PTR: u32 = 0x120;
pub const PART_BREAK_ITEM_DROPS_PTR: u32 = 0x124;

pub const MONSTER_DESC_PTR: u32 = 0x134;

pub const ARENA_ITEMS_PTR: u32 = 0x144;
pub const ARENA_EQUIPMENT_PTR: u32 = 0x148;
pub const ARENA_STAGE_ID_C1_PTR: u32 = 0x14C;
pub const ARENA_STAGE_ID_C0_PTR: u32 = 0x150;
pub const ARENA_AMMO_PTR: u32 = 0x154;

pub const MACHA_POT_REWARDS_PTR: u32 = 0x160;

pub const WEAPON_GR_FORGING_PTR: u32 = 0x5F0;
pub const ARMOR_GR_FORGING_PTR: u32 = 0x5F4;
pub const WEAPON_G50_FORGING_PTR: u32 = 0x5FC;

pub const SEASONAL_EVENT_TIMING_PTR: u32 = 0x54C;

pub const CUFF_CRAFTING_PTR: u32 = 0x2C0;
pub const CARVE_HC_ITEM_DROPS_PTR: u32 = 0x328;
pub const CARVE_GRHC_ITEM_DROPS_PTR: u32 = 0x738;
pub const GRANK_CUFF_CRAFTING_PTR: u32 = 0x750;

pub const WEAPON_SPECIAL_FORGING_PTR: u32 = 0x7A8;
pub const ARMOR_PREMIUM_FORGING_PTR: u32 = 0x7B0;

pub const WEAPON_TOWER_FORGING_PTR: u32 = 0x940;
pub const SIGIL_TOWER_FORGING_PTR: u32 = 0x944;
pub const ARMOR_TOWER_FORGING_PTR: u32 = 0x994;

pub const EVO_UPGRADES_PTR: u32 = 0xA18;
pub const ITEM_SOURCE_STRINGS_PTR: u32 = 0xA40;
pub const QUEST_LIST_PTR: u32 = 0xA98;
pub const TRANSMOG_FORGING_PTR: u32 = 0xAB8;
pub const TRANSMOG_FORGING2_PTR: u32 = 0xABC;
pub const ZENITH_WEAPON_FORGING_PTR: u32 = 0xAC0;
pub const ARMOR_ZENITH_FORGING_PTR: u32 = 0xAC4;
pub const SPECIAL_SHOPS_PTR: u32 = 0xB28;
pub const GRANK_DECO_SHOP_PTR: u32 = 0xB48;

// Add more known pointers as you reverse or document the format. 

// Sigil Tower
pub const SIGIL_TOWER_PTR: u32 = 0x944;

// G50 Weapons
pub const G50_WEAPON_PTR: u32 = 0x5FC;

// MW Upgrades
pub const MW_UPGRADE_PTR: u32 = 0x030;

// RW Upgrades
pub const RW_UPGRADE_PTR: u32 = 0x040;

// Evo Upgrades
pub const EVO_UPGRADE_PTR: u32 = 0xA18;

// Transmog
pub const TRANSMOG_PTR: u32 = 0xAB8;

// Zenith
pub const ZENITH_PTR: u32 = 0xAC0; 