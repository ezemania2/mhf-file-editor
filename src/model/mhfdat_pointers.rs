// This file will store all pointers for mhfdat.
// Add pointer definitions here as you reverse or document the format. 

// All known mhfdat pointers (except signatures)
// Names and comments are based on mhf-pattern/mhfdat/header.hexpat

// Header slot of the block that holds the entry counts (u16 each). The *_COUNT_INNER constants below are
// offsets inside that block, so a count is found through this slot, never through a fixed file address.
pub const IMPORTANT_NUMS_PTR: u32 = 0x10;

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
pub const CARVE_PARTS_COUNT_INNER: usize = 0x1C;
pub const PART_BREAK_DROP_PTR: u32 = 0x124;
pub const PART_BREAK_DROP_COUNT_INNER: usize = 0x1E;
pub const MOSNTERS_DESCRIPTION_PTR: u32 = 0x134;
pub const MOSNTERS_DESCRIPTION_COUNT_INNER: usize = 0x22;
pub const SEASONAL_EVENT_PTR: u32 = 0x54C;
pub const SEASONAL_EVENT_COUNT_INNER: usize = 0x1DA;
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
pub const SIGIL_CRAFTING_RECIPES_PTR: u32 = 0x620;
pub const SIGIL_SKILL_PROBABILITIES_PTR: u32 = 0x624;
pub const SIGIL_SKILL_BLACKLISTS_PTR: u32 = 0x628;
pub const TONFA_G50_TOWER_PARAMS_PTR: u32 = 0x888;
pub const TOWER_WEAPON_FORGING_PTR: u32 = 0x940;
pub const TOWER_ARMOR_FORGING_PTR: u32 = 0x998;
pub const SWITCH_AXE_G50_TOWER_PARAMS_PTR: u32 = 0xA90;
pub const MAGNET_SPIKE_G50_TOWER_PARAMS_PTR: u32 = 0xB98;
pub const G50_MELEE_WEAPON_UPGRADE_PTR: u32 = 0x5F8;
pub const G50_MELEE_WEAPON_UPGRADE_COUNT_INNER: usize = 0x22A;
pub const G50_RANGED_WEAPON_UPGRADE_PTR: u32 = 0x5FC;
pub const G50_RANGED_WEAPON_UPGRADE_COUNT_INNER: usize = 0x22C;
pub const GR_QUEST_LIST_PTR: u32 = 0x6E0;
pub const HR_QUEST_LIST_PTR: u32 = 0xA98;


pub const DECO_ID_PTR: u32 = 0xF8;
pub const DECO_ID_COUNT_INNER: usize = 0x0A;

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
pub const AUTOMATIC_SKILLS_COUNT_INNER: usize = 0x2F8;

// Sharpness: header slot 0xAC points to an array of 14 pointers (one per weapon class, in class order:
// GS, HBG, Hammer, Lance, SnS, LBG, DB, LS, HH, GL, Bow, Tonfa, SA, MS), each pointing to 128 entries of
// 8 x u16. These are offsets inside that array; the array itself is found through the header slot.
// Light/Heavy Bowgun and Bow have no sharpness of their own and are not edited.
pub const SHARPNESS_ARRAY_SLOT: u32 = 0xAC;
pub const SHARPNESS_GREAT_SWORD_INNER: usize = 0x00;
pub const SHARPNESS_HAMMER_INNER: usize = 0x08;
pub const SHARPNESS_LANCE_INNER: usize = 0x0C;
pub const SHARPNESS_SWORD_AND_SHIELD_INNER: usize = 0x10;
pub const SHARPNESS_DUAL_BLADES_INNER: usize = 0x18;
pub const SHARPNESS_LONG_SWORD_INNER: usize = 0x1C;
pub const SHARPNESS_HUNTING_HORN_INNER: usize = 0x20;
pub const SHARPNESS_GUNLANCE_INNER: usize = 0x24;
pub const SHARPNESS_TONFA_INNER: usize = 0x2C;
pub const SHARPNESS_SWITCH_AXE_INNER: usize = 0x30;
pub const SHARPNESS_MAGNET_SPIKE_INNER: usize = 0x34;
/// Same order as `MhfdatApp::sharpness_modified` / `original_sharpness_offsets`.
pub const SHARPNESS_INNER_BY_INDEX: [usize; 11] = [
    SHARPNESS_GREAT_SWORD_INNER, SHARPNESS_HAMMER_INNER, SHARPNESS_LANCE_INNER,
    SHARPNESS_SWORD_AND_SHIELD_INNER, SHARPNESS_DUAL_BLADES_INNER, SHARPNESS_LONG_SWORD_INNER,
    SHARPNESS_HUNTING_HORN_INNER, SHARPNESS_GUNLANCE_INNER,
    SHARPNESS_TONFA_INNER, SHARPNESS_SWITCH_AXE_INNER, SHARPNESS_MAGNET_SPIKE_INNER,
];

// ---- Counts in important_nums that were not needed before the table tabs below ----------------------------
pub const SIGIL_RECIPES_COUNT_INNER: usize = 0x23C;
pub const BEGINNER_QUESTS_COUNT_INNER: usize = 0x3C;
pub const TUTORIAL_EQUIP_SETS_COUNT_INNER: usize = 0x48;
pub const HORN_MELODIES_COUNT_INNER: usize = 0x258;
pub const VOYAGE_COUNT_INNER: usize = 0x370; // docs: "counts come from +0x370 / +0x372", which is which is not documented
pub const RANKING_COUNT_INNER: usize = 0x372;
pub const TOWER_JEWEL_RECIPES_COUNT_INNER: usize = 0x38A;

// ---- Quests ------------------------------------------------------------------------------------------------
// Menu roots of the GR lists: slot -> root blob -> first dword -> QuestItem stream (G7 is GR_QUEST_LIST_PTR)
pub const GR_MENU_G1_PTR: u32 = 0x6C8;
pub const GR_MENU_G2_PTR: u32 = 0x6CC;
pub const GR_MENU_G3_PTR: u32 = 0x6D0;
pub const GR_MENU_G4_PTR: u32 = 0x6D4;
pub const GR_MENU_G5_PTR: u32 = 0x6D8;
pub const GR_MENU_G6_PTR: u32 = 0x6DC;
pub const GR_MENU_G8_PTR: u32 = 0x740;
pub const GR_MENU_G9_PTR: u32 = 0x744;
pub const GR_MENU_G10_PTR: u32 = 0x748;
/// G1 to G10 in order; G7 is `GR_QUEST_LIST_PTR`.
pub const GR_MENU_PTRS: [u32; 10] = [
    GR_MENU_G1_PTR, GR_MENU_G2_PTR, GR_MENU_G3_PTR, GR_MENU_G4_PTR, GR_MENU_G5_PTR,
    GR_MENU_G6_PTR, GR_QUEST_LIST_PTR, GR_MENU_G8_PTR, GR_MENU_G9_PTR, GR_MENU_G10_PTR,
];
pub const BEGINNER_QUEST_IDS_PTR: u32 = 0x1B8;

// ---- Monster crowns / skills ---------------------------------------------------------------------------------
pub const CROWNS_PTR: u32 = 0x130; // 0x0E bytes x important_nums+0x22 (monster count, same as MOSNTERS_DESCRIPTION_COUNT_INNER)
pub const SKILL_EFFECTS_PTR: u32 = 0x594; // 0x1C bytes x u16 at *(SKILL_COUNT_HOST_PTR)+2
pub const SKILL_COUNT_HOST_PTR: u32 = 0x988;

// ---- Caravan ---------------------------------------------------------------------------------------------------
pub const VOYAGE_ROUTES_PTR: u32 = 0x22C; // 0xA1C bytes per route, ends on an empty title
pub const VOYAGE_LEVEL_INDEX_PTR: u32 = 0x230; // 15 levels x 12 s32
pub const RANKING_ROUTES_PTR: u32 = 0x91C; // 16 group offsets, records of 0x10C

// Caravan gems (hunter pearls). mhfo-hd.dll: hunterPearl_lookup_gem_level 0x10494EA0, hunterPearl_check_caravan_unlock
// 0x104954D0, hunterPearl_build_available_skill_list 0x10494CA0, caravanTier_get_pts_cap 0x104957A0,
// caravanTier_check_unlock_req 0x104958B0.
pub const HUNTER_PEARL_SKILLS_PTR: u32 = 0xF0; // 0x30-byte entries, the list ends on a skill id of -1
pub const HUNTER_PEARL_SKILL_LISTS_PTR: u32 = 0xF4; // pointers, indexed by the caravan tier, to s16 skill id lists ending on -1
pub const CARAVAN_TIERS_PTR: u32 = 0x234; // 8-byte rows indexed by the same tier
/// The tier byte only takes the values 0..=8: FUN_10496200 (the only code that raises it) stops at 8. So the pointer
/// array behind 0xF4 and the table behind 0x234 are used for 9 tiers; nothing in the header gives their length.
pub const CARAVAN_TIER_COUNT: usize = 9;

// ---- Hunting Road -------------------------------------------------------------------------------------------------
pub const ROAD_SKILL_META_PTR: u32 = 0x910; // u16 skill count at +8
pub const ROAD_SKILLS_PTR: u32 = 0x930;
pub const ROAD_COUNTERS_PTR: u32 = 0xB04; // +4 felyne shop count, +8 monster point count
pub const ROAD_CAT_SHOP_PTR: u32 = 0xB10;
pub const ROAD_MONSTER_POINTS_PTR: u32 = 0xB20;

// ---- Crafting tables shown on the Shop tab ---------------------------------------------------------------------
pub const SPECIAL_WEAPON_FORGING_PTR: u32 = 0x7AC;
pub const ARMOR_PREMIUM_FORGING_PTR: u32 = 0x7B0;
pub const COMBINE_SHOP_PTR: u32 = 0xBAC;
pub const TOWER_JEWEL_RECIPES_PTR: u32 = 0x938;
pub const TOWER_UPGRADE_COST_TABLES_PTR: u32 = 0x93C; // pointers, ending on a 0 pointer, to 24-byte steps; a block ends on an all-zero step
pub const TOWER_SEAL_FORGING_PTR: u32 = 0x944; // 0x14 bytes per recipe, ends on item id 0

// ---- Hunting horn melodies ---------------------------------------------------------------------------------------
pub const HUNTING_HORN_MELODIES_PTR: u32 = 0x670; // 5 bytes x important_nums+0x258

// ---- Arena / tutorial sets -----------------------------------------------------------------------------------------
pub const ARENA_ITEMS_PTR: u32 = 0x144; // 68 pointers to {u16 quantity, u16 id} lists ending on quantity -1
pub const ARENA_EQUIPMENT_PTR: u32 = 0x148; // 17 pointers to 4 x 10 u16
pub const ARENA_STAGE_IDS_1_PTR: u32 = 0x14C; // u16[34]
pub const ARENA_STAGE_IDS_0_PTR: u32 = 0x154; // u16[28]
pub const ARENA_AMMO_PTR: u32 = 0x158; // 23 pointers to item lists
pub const TUTORIAL_EQUIP_SETS_PTR: u32 = 0x15C; // important_nums+0x48 pointers (null = none) to 20-byte sets
