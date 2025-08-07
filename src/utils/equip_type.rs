pub fn getEquipType(n: u8) -> &'static str {
    match n {
        0x00 => "Legs",
        0x02 => "Head",
        0x03 => "Chest",
        0x04 => "Arms",
        0x05 => "Waist",
        0x06 => "Melee",
        0x07 => "Ranged",
        _ => "Unknown",
    }
}

pub const EQUIP_TYPE_LIST: &[(u8, &str)] = &[
    (0x00, "Legs"),
    (0x02, "Head"),
    (0x03, "Chest"),
    (0x04, "Arms"),
    (0x05, "Waist"),
    (0x06, "Melee"),
    (0x07, "Ranged"),
];
