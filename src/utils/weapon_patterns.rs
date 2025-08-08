// Mapping et listes pour les patterns d'armes MHF

// Weapon Length enum
pub fn length_name(n: u32) -> &'static str {
    match n {
        0x00000000 => "Medium",
        0x00000001 => "Short", 
        0x00000002 => "Very Short",
        0x00000003 => "Long",
        0x00000004 => "Very Long",
        _ => "!!Unknown!!",
    }
}

pub const LENGTH_LIST: &[(u32, &str)] = &[
    (0x00000000, "Medium"),
    (0x00000001, "Short"),
    (0x00000002, "Very Short"),
    (0x00000003, "Long"),
    (0x00000004, "Very Long"),
];

pub fn class_name(n: u8) -> &'static str {
    match n {
        0x00 => "Great Sword",
        0x01 => "Heavy Bowgun",
        0x02 => "Hammer",
        0x03 => "Lance",
        0x04 => "Sword and Shield",
        0x05 => "Light Bowgun",
        0x06 => "Dual Blades",
        0x07 => "Long Sword",
        0x08 => "Hunting Horn",
        0x09 => "Gunlance",
        0x0A => "Bow",
        0x0B => "Tonfa",
        0x0C => "Switch Axe F",
        0x0D => "Magnet Spike",
        _ => "!!Unknown!!",
    }
}
pub const CLASS_ID_LIST: &[(u8, &str)] = &[
    (0x00, "Great Sword"),
    (0x01, "Heavy Bowgun"),
    (0x02, "Hammer"),
    (0x03, "Lance"),
    (0x04, "Sword and Shield"),
    (0x05, "Light Bowgun"),
    (0x06, "Dual Blades"),
    (0x07, "Long Sword"),
    (0x08, "Hunting Horn"),
    (0x09, "Gunlance"),
    (0x0A, "Bow"),
    (0x0B, "Tonfa"),
    (0x0C, "Switch Axe F"),
    (0x0D, "Magnet Spike"),
];

pub fn element_name(n: u8) -> &'static str {
    match n {
        0x00 => "None",
        0x01 => "Fire",
        0x02 => "Water",
        0x03 => "Thunder",
        0x04 => "Dragon",
        0x05 => "Ice",
        0x06 => "Flame",
        0x07 => "Light",
        0x08 => "Thunder Pole",
        0x09 => "Tenshou",
        0x0A => "Okiko",
        0x0B => "Black Flame",
        0x0C => "Kanade",
        0x0D => "Darkness",
        0x0E => "Crimson Demon",
        0x0F => "Wind",
        0x10 => "Sound",
        0x11 => "Burning Zero",
        0x12 => "Emperor's Roar",
        _ => "!!Unknown!!",
    }
}
pub const ELEMENT_ID_LIST: &[(u8, &str)] = &[
    (0x00, "None"),
    (0x01, "Fire"),
    (0x02, "Water"),
    (0x03, "Thunder"),
    (0x04, "Dragon"),
    (0x05, "Ice"),
    (0x06, "Flame"),
    (0x07, "Light"),
    (0x08, "Thunder Pole"),
    (0x09, "Tenshou"),
    (0x0A, "Okiko"),
    (0x0B, "Black Flame"),
    (0x0C, "Kanade"),
    (0x0D, "Darkness"),
    (0x0E, "Crimson Demon"),
    (0x0F, "Wind"),
    (0x10, "Sound"),
    (0x11, "Burning Zero"),
    (0x12, "Emperor's Roar"),
];

pub fn ailment_name(n: u8) -> &'static str {
    match n {
        0x00 => "None",
        0x01 => "Poison",
        0x02 => "Paralysis",
        0x03 => "Sleep",
        0x04 => "Blast",
        _ => "!!Unknown!!",
    }
}
pub const AILMENT_ID_LIST: &[(u8, &str)] = &[
    (0x00, "None"),
    (0x01, "Poison"),
    (0x02, "Paralysis"),
    (0x03, "Sleep"),
    (0x04, "Blast"),
];

// Equipment Type Bitfield helpers
pub fn equip_type_flags_to_string(flags: u8) -> String {
    let mut result = Vec::new();
    
    if flags & 0x01 != 0 { result.push("SP"); }
    if flags & 0x02 != 0 { result.push("Gou"); }
    if flags & 0x04 != 0 { result.push("Evolution"); }
    if flags & 0x08 != 0 { result.push("HC"); }
    if flags & 0x10 != 0 { result.push("Random Weapon"); }
    if flags & 0x20 != 0 { result.push("Ravi"); }
    if flags & 0x40 != 0 { result.push("G50"); }
    
    if result.is_empty() {
        "General".to_string()
    } else {
        result.join(" | ")
    }
}

pub fn equip_type_name(n: u8) -> &'static str {
    match n {
        0x00 => "General",
        0x01 => "SP",
        0x02 => "Gou",
        0x04 => "Evolution",
        0x08 => "HC",
        0x10 => "Random Weapon",
        0x20 => "Ravi",
        0x40 => "G50",
        _ => "!!Unknown!!",
    }
}
pub const EQUIP_TYPE_LIST: &[(u8, &str)] = &[
    (0x00, "General"),
    (0x01, "SP"),
    (0x02, "Gou"),
    (0x04, "Evolution"),
    (0x08, "HC"),
    (0x10, "Random Weapon"),
    (0x20, "Ravi"),
    (0x40, "G50"),
];

// Weapon Type Bitfield helpers (based on hexpat WeaponType structure)
pub fn weapon_type_flags_to_string(flags: u32) -> String {
    let mut result = Vec::new();
    
    // Special case for Prayer GR (combination of bits 11 + 3)
    if (flags & 0x00000800 != 0) && (flags & 0x00000008 != 0) {
        result.push("Prayer GR");
    } else {
        // Individual handling if not Prayer GR
        if flags & 0x00000800 != 0 { result.push("Prayer LR"); }
        if flags & 0x00000008 != 0 { result.push("GR Gunner"); }
    }
    
    if flags & 0x00000001 != 0 { result.push("Finess Base"); }
    if flags & 0x00000002 != 0 { result.push("Gou HR1"); }
    if flags & 0x00000004 != 0 { result.push("Gou HR2"); }
    if flags & 0x00000010 != 0 { result.push("Gou GR1"); }
    if flags & 0x00000020 != 0 { result.push("Gou GR2"); }
    if flags & 0x00000040 != 0 { result.push("Finess Ext"); }
    if flags & 0x00000080 != 0 { result.push("Tower"); }
    if flags & 0x00000100 != 0 { result.push("Gou GR3"); }
    if flags & 0x00000200 != 0 { result.push("Exotic"); }
    if flags & 0x00000400 != 0 { result.push("Ravi Z"); }
    if flags & 0x00001000 != 0 { result.push("Zenith"); }
    if flags & 0x00004000 != 0 { result.push("Ravi GR+"); }
    if flags & 0x08000000 != 0 { result.push("GR Simple upgrade"); }
    
    if result.is_empty() {
        "General".to_string()
    } else {
        result.join(" | ")
    }
}

pub fn weapon_type_name(n: u32) -> &'static str {
    match n {
        0x00000000 => "General",
        0x00000800 => "Prayer LR",
        0x00000808 => "Prayer GR",
        0x00000200 => "Exotic",
        0x00001000 => "Zenith",
        0x00000002 => "Gou HR1",
        0x00000004 => "Gou HR2",
        0x00000010 => "Gou GR1",
        0x00000020 => "Gou GR2",
        0x00000100 => "Gou GR3",
        0x00000008 => "GR Gunner",
        0x08000000 => "GR Simple upgrade",
        0x00000041 => "Finess weapon",
        0x00004000 => "Ravi GR+",
        0x00000080 => "Tower",
        0x00000400 => "Ravi Z",
        _ => "!!Unknown!!",
    }
}
pub const WEAPON_TYPE_LIST: &[(u32, &str)] = &[
    (0x00000000, "General"),
    (0x00000800, "Prayer LR"),
    (0x00000808, "Prayer GR"),
    (0x00000200, "Exotic"),
    (0x00001000, "Zenith"),
    (0x00000002, "Gou HR1"),
    (0x00000004, "Gou HR2"),
    (0x00000010, "Gou GR1"),
    (0x00000020, "Gou GR2"),
    (0x00000100, "Gou GR3"),
    (0x00000008, "GR Gunner"),
    (0x08000000, "GR Simple upgrade"),
    (0x00000041, "Finess weapon"),
    (0x00004000, "Ravi GR+"),
    (0x00000080, "Tower"),
    (0x00000400, "Ravi Z"),
];

pub fn reload (n: u8) -> &'static str {
    match n {
        0x00 => "Very Slow",
        0x01 => "Slow",
        0x02 => "Normal",
        0x03 => "Fast",
        0x04 => "Very Fast",
        _ => "!!Unknown!!",
    }
}

pub const RELOAD_LIST: &[(u8, &str)] = &[
    (0x00, "Very Slow"),
    (0x01, "Slow"),
    (0x02, "Normal"),
    (0x03, "Fast"),
    (0x04, "Very Fast"),
];

pub fn recoil (n: u8) -> &'static str {
    match n {
        0x00 => "Very High",
        0x01 => "High",
        0x02 => "Normal",
        0x03 => "Low",
        0x04 => "Very Low",
        _ => "!!Unknown!!",
    }
}

pub const RECOIL_LIST: &[(u8, &str)] = &[
    (0x00, "Very High"),
    (0x01, "High"),
    (0x02, "Normal"),
    (0x03, "Low"),
    (0x04, "Very Low"),
];

pub fn zenith_skill_name(n: u16) -> &'static str {
    match n {
        0x0000 => "None",
        0x0001 => "Skill Slot UP +1",
        0x0002 => "Skill Slot UP +2",
        0x0003 => "Skill Slot UP +3",
        0x0004 => "Skill Slot UP +4",
        0x0005 => "Skill Slot UP +5",
        0x0006 => "Skill Slot UP +6",
        0x0007 => "Skill Slot UP +7",
        0x0008 => "Crit Conv UP +1",
        0x0009 => "Crit Conv UP +2",
        0x000A => "Stylish Assault UP +1",
        0x000B => "Stylish Assault UP +2",
        0x000C => "Disolver UP",
        0x000D => "ThunderClad UP +1",
        0x000E => "ThunderClad UP +2",
        0x000F => "Ice Age UP",
        0x0011 => "Earplug UP +1",
        0x0012 => "Earplug UP +2",
        0x0013 => "Earplug UP +3",
        0x0014 => "Wind Res UP +1",
        0x0015 => "Wind Res UP +2",
        0x0016 => "Wind Res UP +3",
        0x0017 => "Wind Res UP +4",
        0x0018 => "Quake Res UP +1",
        0x0019 => "Quake Res UP +2",
        0x001A => "Poison Res UP +1",
        0x001B => "Poison Res UP +2",
        0x001C => "Para Res UP +1",
        0x001D => "Para Res UP +2",
        0x001E => "Sleep Res UP +1",
        0x001F => "Sleep Res UP +2",
        0x0020 => "Vampirism UP +1",
        0x0021 => "Vampirism UP +2",
        0x0022 => "DrugKnowledge UP",
        0x0023 => "Assistance UP",
        0x0024 => "Bullet Shaver UP +1",
        0x0025 => "Bullet Shaver UP +2",
        0x0026 => "Guard UP +1",
        0x0027 => "Guard UP +2",
        0x0028 => "Adaptation UP +1",
        0x0029 => "Adapdation UP +2",
        0x002A => "Encourage UP +1",
        0x002B => "Encourage UP +2",
        0x002C => "Reflect UP +1",
        0x002D => "Reflect UP +2",
        0x002E => "Reflect UP +3",
        0x002F => "Stylish UP",
        0x0030 => "Vigorous UP",
        0x0031 => "Obscurity UP",
        0x0032 => "Soul UP",
        0x0033 => "Ceaseless UP",
        0x0034 => "Rush UP",
        _ => "!!Unknown!!",
    }
}
pub const ZENITH_SKILL_LIST: &[(u16, &str)] = &[
    (0x0000, "None"),
    (0x0001, "Skill Slot UP +1"),
    (0x0002, "Skill Slot UP +2"),
    (0x0003, "Skill Slot UP +3"),
    (0x0004, "Skill Slot UP +4"),
    (0x0005, "Skill Slot UP +5"),
    (0x0006, "Skill Slot UP +6"),
    (0x0007, "Skill Slot UP +7"),
    (0x0008, "Crit Conv UP +1"),
    (0x0009, "Crit Conv UP +2"),
    (0x000A, "Stylish Assault UP +1"),
    (0x000B, "Stylish Assault UP +2"),
    (0x000C, "Disolver UP"),
    (0x000D, "ThunderClad UP +1"),
    (0x000E, "ThunderClad UP +2"),
    (0x000F, "Ice Age UP"),
    (0x0011, "Earplug UP +1"),
    (0x0012, "Earplug UP +2"),
    (0x0013, "Earplug UP +3"),
    (0x0014, "Wind Res UP +1"),
    (0x0015, "Wind Res UP +2"),
    (0x0016, "Wind Res UP +3"),
    (0x0017, "Wind Res UP +4"),
    (0x0018, "Quake Res UP +1"),
    (0x0019, "Quake Res UP +2"),
    (0x001A, "Poison Res UP +1"),
    (0x001B, "Poison Res UP +2"),
    (0x001C, "Para Res UP +1"),
    (0x001D, "Para Res UP +2"),
    (0x001E, "Sleep Res UP +1"),
    (0x001F, "Sleep Res UP +2"),
    (0x0020, "Vampirism UP +1"),
    (0x0021, "Vampirism UP +2"),
    (0x0022, "DrugKnowledge UP"),
    (0x0023, "Assistance UP"),
    (0x0024, "Bullet Shaver UP +1"),
    (0x0025, "Bullet Shaver UP +2"),
    (0x0026, "Guard UP +1"),
    (0x0027, "Guard UP +2"),
    (0x0028, "Adaptation UP +1"),
    (0x0029, "Adapdation UP +2"),
    (0x002A, "Encourage UP +1"),
    (0x002B, "Encourage UP +2"),
    (0x002C, "Reflect UP +1"),
    (0x002D, "Reflect UP +2"),
    (0x002E, "Reflect UP +3"),
    (0x002F, "Stylish UP"),
    (0x0030, "Vigorous UP"),
    (0x0031, "Obscurity UP"),
    (0x0032, "Soul UP"),
    (0x0033, "Ceaseless UP"),
    (0x0034, "Rush UP"),
];

// Bullet Types Bitfield helpers (for ranged weapons)
pub fn bullet_types_to_string(flags: u32) -> String {
    let mut result = Vec::new();
    
    if flags & 0x00000001 != 0 { result.push("Normal Lv1"); }
    if flags & 0x00000002 != 0 { result.push("Normal Lv2"); }
    if flags & 0x00000004 != 0 { result.push("Normal Lv3"); }
    if flags & 0x00000008 != 0 { result.push("Pierce Lv1"); }
    if flags & 0x00000010 != 0 { result.push("Pierce Lv2"); }
    if flags & 0x00000020 != 0 { result.push("Pierce Lv3"); }
    if flags & 0x00000040 != 0 { result.push("Spread Lv1"); }
    if flags & 0x00000080 != 0 { result.push("Spread Lv2"); }
    if flags & 0x00000100 != 0 { result.push("Spread Lv3"); }
    if flags & 0x00000200 != 0 { result.push("Crag Lv1"); }
    if flags & 0x00000400 != 0 { result.push("Crag Lv2"); }
    if flags & 0x00000800 != 0 { result.push("Crag Lv3"); }
    if flags & 0x00001000 != 0 { result.push("Cluster Lv1"); }
    if flags & 0x00002000 != 0 { result.push("Cluster Lv2"); }
    if flags & 0x00004000 != 0 { result.push("Cluster Lv3"); }
    if flags & 0x00008000 != 0 { result.push("Fire"); }
    if flags & 0x00010000 != 0 { result.push("Water"); }
    if flags & 0x00020000 != 0 { result.push("Thunder"); }
    if flags & 0x00040000 != 0 { result.push("Ice"); }
    if flags & 0x00080000 != 0 { result.push("Dragon"); }
    if flags & 0x00100000 != 0 { result.push("Recovery Lv1"); }
    if flags & 0x00200000 != 0 { result.push("Recovery Lv2"); }
    if flags & 0x00400000 != 0 { result.push("Poison Lv1"); }
    if flags & 0x00800000 != 0 { result.push("Poison Lv2"); }
    if flags & 0x01000000 != 0 { result.push("Paralysis Lv1"); }
    if flags & 0x02000000 != 0 { result.push("Paralysis Lv2"); }
    if flags & 0x04000000 != 0 { result.push("Sleep Lv1"); }
    if flags & 0x08000000 != 0 { result.push("Sleep Lv2"); }
    if flags & 0x10000000 != 0 { result.push("Tranquilizer"); }
    if flags & 0x20000000 != 0 { result.push("Paint"); }
    if flags & 0x40000000 != 0 { result.push("Demon"); }
    if flags & 0x80000000 != 0 { result.push("Armor"); }
    
    if result.is_empty() {
        "None".to_string()
    } else {
        result.join(", ")
    }
}

pub const BULLET_TYPE_LIST: &[(u32, &str)] = &[
    (0x00000001, "Normal Lv1"),
    (0x00000002, "Normal Lv2"),
    (0x00000004, "Normal Lv3"),
    (0x00000008, "Pierce Lv1"),
    (0x00000010, "Pierce Lv2"),
    (0x00000020, "Pierce Lv3"),
    (0x00000040, "Spread Lv1"),
    (0x00000080, "Spread Lv2"),
    (0x00000100, "Spread Lv3"),
    (0x00000200, "Crag Lv1"),
    (0x00000400, "Crag Lv2"),
    (0x00000800, "Crag Lv3"),
    (0x00001000, "Cluster Lv1"),
    (0x00002000, "Cluster Lv2"),
    (0x00004000, "Cluster Lv3"),
    (0x00008000, "Fire"),
    (0x00010000, "Water"),
    (0x00020000, "Thunder"),
    (0x00040000, "Ice"),
    (0x00080000, "Dragon"),
    (0x00100000, "Recovery Lv1"),
    (0x00200000, "Recovery Lv2"),
    (0x00400000, "Poison Lv1"),
    (0x00800000, "Poison Lv2"),
    (0x01000000, "Paralysis Lv1"),
    (0x02000000, "Paralysis Lv2"),
    (0x04000000, "Sleep Lv1"),
    (0x08000000, "Sleep Lv2"),
    (0x10000000, "Tranquilizer"),
    (0x20000000, "Paint"),
    (0x40000000, "Demon"),
    (0x80000000, "Armor"),
]; 