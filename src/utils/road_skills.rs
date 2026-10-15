// Hunting Road skill names, from mhfpac.bin (the array behind slot 0xD58 that ui_draw_road_skill_detail indexes by road skill id).
pub const ROAD_SKILL_NAMES: &[(u16, &str)] = &[
    (1, "Attack"),
    (2, "Defense"),
    (3, "希望への活力"),
    (4, "Stamina"),
    (5, "Recovery"),
    (6, "Flame Protection"),
    (7, "Shock Protection"),
    (8, "打撃保護"),
    (9, "斬撃保護"),
    (10, "技巧術"),
    (11, "Negotiation"),
    (12, "砲撃手"),
    (13, "破壊王"),
    (14, "Care"),
    (15, "Pharmacist"),
    (16, "Treasure"),
    (17, "激遭"),
    (18, "Virus Res."),
    (19, "Guardian Offense"),
    (20, "Guardian Defense"),
    (21, "罠回避"),
    (22, "Frost Res."),
    (23, "Fire Res"),
    (24, "Water Res"),
    (25, "Ice Res"),
    (26, "Thunder Res"),
    (27, "Dragon Res"),
    (28, "Starting Gm Up"),
    (29, "Rd P Up"),
    (30, "Revival Sage"),
    (31, "Advancement Guru"),
    (32, "Last Stand"),
    (33, "Bonus Encounter"),
    (34, "All Res"),
    (35, "Rewards Up"),
];

/// Id 0 is an empty prerequisite slot.
pub fn road_skill_name(id: u16) -> &'static str {
    if id == 0 {
        return "none";
    }
    ROAD_SKILL_NAMES.iter().find(|(i, _)| *i == id).map(|(_, n)| *n).unwrap_or("?")
}
