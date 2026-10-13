use std::fs::OpenOptions;
use std::io::{Write, Seek, SeekFrom, Result, Read, Cursor};
use crate::model::mhfdat::{MhfdatMeleeWeapon, MhfdatRangedWeapon, ShopEntry, DecoShop, SigilTowerTable, G50WUpgrade, MWUpgradePath, RWUpgradePath, EvoUpgrade, EvoUpgradeSub, MhfdatEquipment, EquipmentCounts, MhfdatItem, MhfdatDecoId, AutomaticSkill, SharpnessItem, SharpnessData, BulletSet, TowerG50WeaponParams, ArmorUpgradeRow, ArmorUpgradeTable, ArmorUpgradeMats, CarveDrop, CarveDropTable, CarveParts, PartBreakDrop, PartBreakDropTable, PartBreakParts};
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use encoding_rs::SHIFT_JIS;
use std::env;
use std::path::PathBuf;
use std::mem::size_of;
use crate::model::mhfdat_pointers::{EQUIPEMENT_COUNT_PTR, IMPORTANT_NUMS_PTR};

fn get_exe_dir() -> PathBuf {
    env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn append_bytes_to_file<P: AsRef<std::path::Path>>(path: P, data: &[u8]) -> Result<()> {
    let exe_dir = get_exe_dir();
    let full_path = exe_dir.join(path);
    let mut file = OpenOptions::new()
        .append(true)
        .open(full_path)?;
    file.seek(SeekFrom::End(0))?;
    file.write_all(data)?;
    Ok(())
}

pub fn read_melee_weapons<R: Read + Seek>(reader: &mut R, offset: u64, count: usize) -> Result<Vec<MhfdatMeleeWeapon>> {
    let mut weapons = Vec::with_capacity(count);
    reader.seek(SeekFrom::Start(offset))?;
    for _ in 0..count {
        weapons.push(read_melee_weapon(reader)?);
    }
    Ok(weapons)
}

pub fn read_ranged_weapons<R: Read + Seek>(reader: &mut R, offset: u64, count: usize) -> Result<Vec<MhfdatRangedWeapon>> {
    let mut weapons = Vec::with_capacity(count);
    reader.seek(SeekFrom::Start(offset))?;
    for _ in 0..count {
        weapons.push(read_ranged_weapon(reader)?);
    }
    Ok(weapons)
}

pub fn read_melee_weapon<R: Read>(r: &mut R) -> Result<MhfdatMeleeWeapon> {
    let model_id = r.read_u16::<LittleEndian>()?;
    let rarity = r.read_u8()?;
    let class_id = r.read_u8()?;
    let zenny_cost = r.read_u32::<LittleEndian>()?;
    let sharpness_id = r.read_u8()?;
    let sharpness_max = r.read_u8()?;
    let raw_damage = r.read_u16::<LittleEndian>()?;
    let defense = r.read_u16::<LittleEndian>()?;
    let affinity = r.read_i8()?;
    let element_id = r.read_u8()?;
    let ele_damage = r.read_u8()?;
    let ailment_id = r.read_u8()?;
    let ail_damage = r.read_u8()?;
    let slots = r.read_u8()?;
    let weapon_attribute = r.read_u8()?;
    let unk15 = r.read_u8()?;
    let upgrade_path = r.read_u16::<LittleEndian>()?;
    let other_model = r.read_u16::<LittleEndian>()?;
    let equip_type = r.read_u8()?;
    let unk1b = r.read_u8()?;
    let length = r.read_u32::<LittleEndian>()?;
    let weapon_type = r.read_u32::<LittleEndian>()?;
    let visual_effects = r.read_u16::<LittleEndian>()?;
    let tower_g50_param_id = r.read_u16::<LittleEndian>()?;
    let g_rank = r.read_u8()?;
    let unk29 = r.read_u8()?;
    let unk2a = r.read_u8()?;
    let zero_f = r.read_u8()?;
    let unk2c = r.read_u32::<LittleEndian>()?;
    let zenith_skill = r.read_u16::<LittleEndian>()?;
    let mut _padding = [0u8; 2];
    r.read_exact(&mut _padding)?;
    Ok(MhfdatMeleeWeapon {
        model_id,
        rarity,
        class_id,
        zenny_cost,
        sharpness_id,
        sharpness_max,
        raw_damage,
        defense,
        affinity,
        element_id,
        ele_damage,
        ailment_id,
        ail_damage,
        slots,
        weapon_attribute,
        unk15,
        upgrade_path,
        other_model,
        equip_type,
        unk1b,
        length,
        weapon_type,
        visual_effects,
        tower_g50_param_id,
        g_rank,
        unk29,
        unk2a,
        zero_f,
        unk2c,
        zenith_skill,
        _padding,
    })
}

pub fn read_ranged_weapon<R: Read>(r: &mut R) -> Result<MhfdatRangedWeapon> {
    Ok(MhfdatRangedWeapon {
        model_id: r.read_u16::<LittleEndian>()?,
        rarity: r.read_u8()?,
        max_slots_maybe: r.read_u8()?,
        class_id: r.read_u8()?,
        unk05: r.read_u8()?,
        equip_type: r.read_u8()?,
        unk07: r.read_u8()?,
        unk08: r.read_u8()?,
        unk09: r.read_u8()?,
        unk11: r.read_u8()?,
        unk12: r.read_u8()?,
        weapon_type: r.read_u32::<LittleEndian>()?,
        unk10: r.read_u32::<LittleEndian>()?,
        zenny_cost: r.read_u32::<LittleEndian>()?,
        raw_damage: r.read_u16::<LittleEndian>()?,
        defense: r.read_u16::<LittleEndian>()?,
        recoil: r.read_u8()?,
        slots: r.read_u8()?,
        affinity: r.read_i8()?,
        sort_order_maybe: r.read_u8()?,
        weapon_attribute: r.read_u8()?,
        element_id: r.read_u8()?,
        ele_damage: r.read_u8()?,
        reload: r.read_u8()?,
        unk24: r.read_u16::<LittleEndian>()?,
        unk26: r.read_u16::<LittleEndian>()?,
        bullet: r.read_u32::<LittleEndian>()?,
        tower_g50_param_id: r.read_u16::<LittleEndian>()?,
        unk2e: r.read_u16::<LittleEndian>()?,
        g_rank: r.read_u8()?,
        unk32: r.read_u8()?,
        unk34: r.read_u8()?,
        zero_f: r.read_u8()?,
        unk38: r.read_u16::<LittleEndian>()?,
        zenith_skill: r.read_u16::<LittleEndian>()?,
        unk42: r.read_u32::<LittleEndian>()?,
    })
}

// Fonction pour vérifier si un byte est valide en Shift-JIS
fn is_valid_shift_jis_byte(b: u8) -> bool {
    // TAB / LF / CR are real characters of the game text (the monster descriptions break lines with 0x0A);
    // dropping them on save glued sentences together.
    b == 0x09 || b == 0x0A || b == 0x0D ||
    // ASCII
    (b >= 0x20 && b <= 0x7E) ||
    // Katakana
    (b >= 0xA1 && b <= 0xDF) ||
    // Premier byte des kanji
    (b >= 0x81 && b <= 0x9F) ||
    (b >= 0xE0 && b <= 0xEF)
}

// Fonction pour nettoyer les bytes Shift-JIS
fn clean_shift_jis_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if is_valid_shift_jis_byte(b) {
            if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xEF) {
                if i + 1 < bytes.len() {
                    let b2 = bytes[i + 1];
                    if (b2 >= 0x40 && b2 <= 0xFC) && b2 != 0x7F {
                        result.push(b);
                        result.push(b2);
                        i += 2;
                        continue;
                    }
                }
            } else {
                result.push(b);
            }
        }
        i += 1;
    }
    result
}

pub fn extract_melee_weapon_names<R: Read + Seek>(
    reader: &mut R,
    names_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<String>> {
    reader.seek(SeekFrom::Start(names_ptr as u64))?;
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    reader.seek(SeekFrom::Start(table_offset as u64))?;
    let mut names = Vec::with_capacity(count);
    for _ in 0..count {
        let mut ptr_buf = [0u8; 4];
        reader.read_exact(&mut ptr_buf)?;
        let str_offset = u32::from_le_bytes(ptr_buf);
        
        if str_offset == 0 {
            names.push(String::new());
            continue;
        }
        
        // Sauvegarder la position actuelle
        let cur = reader.seek(SeekFrom::Current(0))?;
        
        // Aller à l'offset de la chaîne
        reader.seek(SeekFrom::Start(str_offset as u64))?;
        
        // Lire les bytes jusqu'au null terminator
        let mut bytes = Vec::new();
        let mut b = [0u8; 1];
        while reader.read_exact(&mut b).is_ok() && b[0] != 0 {
            bytes.push(b[0]);
        }
        
        // Nettoyer et décoder les bytes
        let cleaned_bytes = clean_shift_jis_bytes(&bytes);
        let (cow, _, _) = SHIFT_JIS.decode(&cleaned_bytes);
        names.push(cow.to_string());
        
        // Retourner à la position précédente
        reader.seek(SeekFrom::Start(cur))?;
    }
    Ok(names)
}

pub fn read_mhfdat_offsets(buffer: &[u8]) -> Option<(u32, u32)> {
    if buffer.len() < 0x084 {
        return None;
    }
    let melee_offset = u32::from_le_bytes(buffer.get(0x07C..0x080)?.try_into().ok()?);
    let ranged_offset = u32::from_le_bytes(buffer.get(0x080..0x084)?.try_into().ok()?);
    Some((melee_offset, ranged_offset))
}

pub fn read_melee_weapons_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<MhfdatMeleeWeapon>> {
    let mut weapons = Vec::new();
    reader.seek(SeekFrom::Start(offset))?;
    loop {
        let weapon = read_melee_weapon(reader)?;
        if weapon.model_id == 0xFFFF {
            break;
        }
        weapons.push(weapon);
    }
    Ok(weapons)
}

pub fn parse_melee_weapons(buffer: &[u8]) -> Vec<MhfdatMeleeWeapon> {
    use std::io::Cursor;
    if let Some((melee_offset, _)) = read_mhfdat_offsets(buffer) {
        let mut cursor = Cursor::new(buffer);
        match read_melee_weapons_until_sentinel(&mut cursor, melee_offset as u64) {
            Ok(weapons) => weapons,
            Err(_) => vec![]
        }
    } else {
        vec![]
    }
}

pub fn write_melee_weapon(writer: &mut impl Write, weapon: &MhfdatMeleeWeapon) -> Result<()> {
    writer.write_all(&weapon.model_id.to_le_bytes())?;
    writer.write_all(&[weapon.rarity])?;
    writer.write_all(&[weapon.class_id])?;
    writer.write_all(&weapon.zenny_cost.to_le_bytes())?;
    writer.write_all(&[weapon.sharpness_id])?;
    writer.write_all(&[weapon.sharpness_max])?;
    writer.write_all(&weapon.raw_damage.to_le_bytes())?;
    writer.write_all(&weapon.defense.to_le_bytes())?;
    writer.write_all(&[weapon.affinity as u8])?;
    writer.write_all(&[weapon.element_id])?;
    writer.write_all(&[weapon.ele_damage])?;
    writer.write_all(&[weapon.ailment_id])?;
    writer.write_all(&[weapon.ail_damage])?;
    writer.write_all(&[weapon.slots])?;
    writer.write_all(&[weapon.weapon_attribute])?;
    writer.write_all(&[weapon.unk15])?;
    writer.write_all(&weapon.upgrade_path.to_le_bytes())?;
    writer.write_all(&weapon.other_model.to_le_bytes())?;
    writer.write_all(&[weapon.equip_type])?;
    writer.write_all(&[weapon.unk1b])?;
    writer.write_all(&weapon.length.to_le_bytes())?;
    writer.write_all(&weapon.weapon_type.to_le_bytes())?;
    writer.write_all(&weapon.visual_effects.to_le_bytes())?;
    writer.write_all(&weapon.tower_g50_param_id.to_le_bytes())?;
    writer.write_all(&[weapon.g_rank])?;
    writer.write_all(&[weapon.unk29])?;
    writer.write_all(&[weapon.unk2a])?;
    writer.write_all(&[weapon.zero_f])?;
    writer.write_all(&weapon.unk2c.to_le_bytes())?;
    writer.write_all(&weapon.zenith_skill.to_le_bytes())?;
    writer.write_all(&weapon._padding)?;
    Ok(())
}

pub fn write_weapon_names<W: Write + Seek>(writer: &mut W, names: &[String]) -> Result<u32> {
    let mut name_offsets = Vec::new();
    let mut name_data = Vec::new();
    for name in names {
        let offset = name_data.len() as u32;
        name_offsets.push(offset);
        let (sjis_bytes, _, had_errors) = SHIFT_JIS.encode(name);
        if had_errors {
            println!("Warning: Shift-JIS encoding had errors for name: {}", name);
        }
        let mut valid_bytes = Vec::new();
        let mut i = 0;
        while i < sjis_bytes.len() {
            let b = sjis_bytes[i];
            if is_valid_shift_jis_byte(b) {
                if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xEF) {
                    if i + 1 < sjis_bytes.len() {
                        let b2 = sjis_bytes[i + 1];
                        if (b2 >= 0x40 && b2 <= 0xFC) && b2 != 0x7F {
                            valid_bytes.push(b);
                            valid_bytes.push(b2);
                            i += 2;
                            continue;
                        }
                    }
                } else {
                    valid_bytes.push(b);
                }
            }
            i += 1;
        }
        
        name_data.extend_from_slice(&valid_bytes);
        name_data.push(0);
    }
    let table_offset = writer.seek(SeekFrom::End(0))? as u32;
    for offset in &name_offsets {
        writer.write_all(&(offset + table_offset + (names.len() as u32 * 4)).to_le_bytes())?;
    }
    writer.write_all(&name_data)?;
    
    Ok(table_offset)
}

pub fn extract_melee_weapon_descriptions<R: Read + Seek>(
    reader: &mut R,
    desc_ptr_offset: u64,
    count: usize,
    buffer_len: usize,
) -> std::io::Result<Vec<[String; 4]>> {
    reader.seek(SeekFrom::Start(desc_ptr_offset))?;
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let desc_table_offset = u32::from_le_bytes(buf);
    reader.seek(SeekFrom::Start(desc_table_offset as u64))?;

    let mut descriptions = Vec::with_capacity(count);
    for _ in 0..count {
        let mut desc_offsets = [0u32; 4];
        for i in 0..4 {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            desc_offsets[i] = u32::from_le_bytes(buf);
        }
        let mut descs = [String::new(), String::new(), String::new(), String::new()];
        for (i, &offset) in desc_offsets.iter().enumerate() {
            if offset != 0 && (offset as usize) < buffer_len {
                let cur = reader.seek(SeekFrom::Current(0)).unwrap_or(0);
                if reader.seek(SeekFrom::Start(offset as u64)).is_ok() {
                    let mut bytes = Vec::new();
                    let mut b = [0u8; 1];
                    let mut char_count = 0;
                    while char_count < 256 {
                        match reader.read_exact(&mut b) {
                            Ok(_) => {
                                if b[0] == 0 { break; }
                                bytes.push(b[0]);
                                char_count += 1;
                            }
                            Err(_) => break, // Stop on any read error
                        }
                    }
                    let (cow, _, _) = SHIFT_JIS.decode(&bytes);
                    descs[i] = cow.to_string();
                }
                let _ = reader.seek(SeekFrom::Start(cur));
            }
        }
        descriptions.push(descs);
    }
    Ok(descriptions)
}

/// Extraction générique de descriptions (ou autres champs string) via table de pointeurs, comme extract_melee_weapon_names
pub fn extract_melee_weapon_descriptions_v2<R: Read + Seek>(
    reader: &mut R,
    desc_ptr: u32,
    count: usize,
    fields_per_entry: usize,
) -> std::io::Result<Vec<Vec<String>>> {
    // Aller à la table de pointeurs principale
    reader.seek(SeekFrom::Start(desc_ptr as u64))?;
    // Lire le pointeur vers la vraie table de pointeurs
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    reader.seek(SeekFrom::Start(table_offset as u64))?;

    // Pour chaque entrée (arme)
    let mut all_descs = Vec::with_capacity(count);
    for _ in 0..count {
        // Lire fields_per_entry pointeurs
        let mut field_ptrs = Vec::with_capacity(fields_per_entry);
        for _ in 0..fields_per_entry {
            let mut ptr_buf = [0u8; 4];
            reader.read_exact(&mut ptr_buf)?;
            let str_offset = u32::from_le_bytes(ptr_buf);
            field_ptrs.push(str_offset);
        }
        // Pour chaque champ, lire la chaîne
        let mut descs = Vec::with_capacity(fields_per_entry);
        for &str_offset in &field_ptrs {
            if str_offset == 0 {
                descs.push(String::new());
                continue;
            }
            let cur = reader.seek(SeekFrom::Current(0))?;
            reader.seek(SeekFrom::Start(str_offset as u64))?;
            let mut bytes = Vec::new();
            let mut b = [0u8; 1];
            while reader.read_exact(&mut b).is_ok() && b[0] != 0 {
                bytes.push(b[0]);
            }
            let cleaned_bytes = clean_shift_jis_bytes(&bytes);
            let (cow, _, _) = SHIFT_JIS.decode(&cleaned_bytes);
            descs.push(cow.to_string());
            reader.seek(SeekFrom::Start(cur))?;
        }
        all_descs.push(descs);
    }
    Ok(all_descs)
}

/// Automatically computes the number of ShopEntry entries in a buffer, given a start offset and an optional maximum count.
/// This is a generic utility for any shop table: pass the offset where the shop data starts in the buffer.
/// If max_count is None, it will read as many entries as possible until the end of the buffer.
pub fn count_shop_entries(buffer: &[u8], offset: usize, max_count: Option<usize>) -> usize {
    let entry_size = size_of::<ShopEntry>();
    let available = buffer.len().saturating_sub(offset);
    let max_possible = available / entry_size;
    match max_count {
        Some(n) => n.min(max_possible),
        None => max_possible,
    }
}

/// Reads all ShopEntry entries from a buffer, starting at the given offset.
/// This is a generic utility for any shop table: pass the offset where the shop data starts in the buffer.
/// If max_count is None, it will read as many entries as possible until the end of the buffer.
pub fn read_shop_entries(buffer: &[u8], offset: usize, max_count: Option<usize>) -> Vec<ShopEntry> {
    let entry_size = size_of::<ShopEntry>();
    let count = count_shop_entries(buffer, offset, max_count);
    let mut entries = Vec::with_capacity(count);
    for i in 0..count {
        let start = offset + i * entry_size;
        let end = start + entry_size;
        if end > buffer.len() { break; }
        let entry = unsafe { std::ptr::read_unaligned(buffer[start..end].as_ptr() as *const ShopEntry) };
        entries.push(entry);
    }
    entries
}

pub fn read_craft_mat_table(buffer: &[u8], offset: usize) -> Vec<ShopEntry> {
    let mut entries = Vec::new();
    let mut cursor = offset;
    while cursor + size_of::<ShopEntry>() <= buffer.len() {
        let entry = unsafe { std::ptr::read_unaligned(buffer.as_ptr().add(cursor) as *const ShopEntry) };
        if entry.footer == 0xFF || entry.equip_id == 0xFFFF {
            break;
        }
        entries.push(entry);
        cursor += size_of::<ShopEntry>();
    }
    entries
}

pub fn read_deco_shop(buffer: &[u8], offset: usize) -> Vec<DecoShop> {
    let mut entries = Vec::new();
    let mut cursor = offset;
    while cursor + size_of::<DecoShop>() <= buffer.len() {
        let entry = unsafe { std::ptr::read_unaligned(buffer.as_ptr().add(cursor) as *const DecoShop) };
        if entry.deco_item_id == 0 {
            break;
        }
        entries.push(entry);
        cursor += size_of::<DecoShop>();
    }
    entries
}

pub fn write_deco_shop(writer: &mut impl Write, entry: &DecoShop) -> Result<()> {
    writer.write_all(&entry.deco_item_id.to_le_bytes())?;
    writer.write_all(&entry.receipt_category.to_le_bytes())?;
    writer.write_all(&entry.item_id1.to_le_bytes())?;
    writer.write_all(&[entry.item_qty1])?;
    writer.write_all(&[entry.item_unlock_flag1])?;
    writer.write_all(&entry.item_id2.to_le_bytes())?;
    writer.write_all(&[entry.item_qty2])?;
    writer.write_all(&[entry.item_unlock_flag2])?;
    writer.write_all(&entry.item_id3.to_le_bytes())?;
    writer.write_all(&[entry.item_qty3])?;
    writer.write_all(&[entry.item_unlock_flag3])?;
    writer.write_all(&entry.item_id4.to_le_bytes())?;
    writer.write_all(&[entry.item_qty4])?;
    writer.write_all(&[entry.item_unlock_flag4])?;
    Ok(())
}

pub fn write_deco_shop_block(entries: &[DecoShop]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    for e in entries {
        write_deco_shop(&mut data, e)?;
    }
    // Sentinel: deco_item_id == 0
    let mut sentinel = DecoShop::default();
    sentinel.deco_item_id = 0;
    write_deco_shop(&mut data, &sentinel)?;
    Ok(data)
}

/// Read automatic skills table from buffer at offset
pub fn read_automatic_skills(buffer: &[u8], offset: usize) -> Vec<AutomaticSkill> {
    let mut entries = Vec::new();
    let mut cursor = offset;
    let entry_size = size_of::<AutomaticSkill>();
    
    // Valid eq_type values: 0x00, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07
    let is_valid_eq_type = |eq_type: u8| -> bool {
        matches!(eq_type, 0x00 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07)
    };
    
    const MAX_SKILL_ID: u16 = 534;
    
    while cursor + entry_size <= buffer.len() {
        let entry = unsafe { std::ptr::read_unaligned(buffer.as_ptr().add(cursor) as *const AutomaticSkill) };
        
        // Stop conditions:
        // - All zeros
        if !entry.is_armor && entry.eq_type == 0 && entry.equip_id == 0 && entry.skill_id == 0 {
            break;
        }
        // - equipID == 0xFFFF
        if entry.equip_id == 0xFFFF {
            break;
        }
        // - Invalid eq_type (not in valid list)
        if !is_valid_eq_type(entry.eq_type) {
            break;
        }
        // - Invalid skill_id (> 534)
        if entry.skill_id > MAX_SKILL_ID {
            break;
        }
        
        entries.push(entry);
        cursor += entry_size;
    }
    entries
}

/// Write a single automatic skill entry
pub fn write_automatic_skill(writer: &mut impl Write, entry: &AutomaticSkill) -> Result<()> {
    writer.write_all(&[entry.is_armor as u8])?;
    writer.write_all(&[entry.eq_type])?;
    writer.write_all(&entry.equip_id.to_le_bytes())?;
    writer.write_all(&entry.skill_id.to_le_bytes())?;
    writer.write_all(&entry.padding)?;
    Ok(())
}

/// Write a block of automatic skills with sentinel
pub fn write_automatic_skills_block(entries: &[AutomaticSkill]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    for e in entries {
        write_automatic_skill(&mut data, e)?;
    }
    // Sentinel: all zeros
    let sentinel = AutomaticSkill::default();
    write_automatic_skill(&mut data, &sentinel)?;
    Ok(data)
}

/// Write a block of deco IDs with sentinel
pub fn write_deco_ids_block(entries: &[crate::model::mhfdat::MhfdatDecoId]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    for e in entries {
        // Write each field in order (packed struct)
        data.push(e.slot_nb);
        data.extend_from_slice(&e.flags.to_le_bytes());
        data.extend_from_slice(&e.price.to_le_bytes());
        data.push(e._pad0);
        data.push(e.skill_id1);
        data.push(e.skill_pts1 as u8);
        data.push(e.skill_id2);
        data.push(e.skill_pts2 as u8);
        data.push(e.skill_id3);
        data.push(e.skill_pts3 as u8);
        data.push(e.skill_id4);
        data.push(e.skill_pts4 as u8);
        data.extend_from_slice(&e.special_flags.to_le_bytes());
        data.extend_from_slice(&e.zenith_skill.to_le_bytes());
    }
    // Sentinel: all 0xFF (or all zeros depending on format)
    data.extend_from_slice(&[0xFF; 18]); // Size of MhfdatDecoId is 18 bytes
    Ok(data)
}

/// Read armor upgrade materials from buffer
/// Structure: pointer at ARMOR_UPGRADE_MATS_PTR points to array of u32 pointers
/// Each pointer points to a table of ArmorUpgradeRow entries until item_id == 0
/// The pointer array ends when pointer == 0x00000000
pub fn read_armor_upgrade_mats(buffer: &[u8], ptr_offset: u32) -> ArmorUpgradeMats {
    let mut mats = ArmorUpgradeMats { tables: Vec::new() };
    
    // Read the main offset from the pointer (points to a table of pointers)
    if ptr_offset as usize + 4 > buffer.len() {
        return mats;
    }
    let ptr_table_offset = u32::from_le_bytes([
        buffer[ptr_offset as usize],
        buffer[ptr_offset as usize + 1],
        buffer[ptr_offset as usize + 2],
        buffer[ptr_offset as usize + 3],
    ]) as usize;
    
    if ptr_table_offset == 0 || ptr_table_offset >= buffer.len() {
        return mats;
    }
    
    // Read successive pointers until we hit 0x00000000
    let row_size = size_of::<ArmorUpgradeRow>(); // 16 bytes
    let mut ptr_cursor = ptr_table_offset;
    
    while ptr_cursor + 4 <= buffer.len() {
        let table_offset = u32::from_le_bytes([
            buffer[ptr_cursor],
            buffer[ptr_cursor + 1],
            buffer[ptr_cursor + 2],
            buffer[ptr_cursor + 3],
        ]) as usize;
        
        // End of pointer list
        if table_offset == 0 {
            break;
        }
        
        ptr_cursor += 4;
        
        if table_offset >= buffer.len() {
            continue;
        }
        
        // Read rows for this table until we hit 16 zero bytes
        let mut table = ArmorUpgradeTable { rows: Vec::new() };
        let mut row_cursor = table_offset;
        
        while row_cursor + row_size <= buffer.len() {
            // Check if next 16 bytes are all zero (terminator)
            let chunk = &buffer[row_cursor..row_cursor + row_size];
            if chunk.iter().all(|&b| b == 0) {
                break;
            }
            
            let row = unsafe {
                std::ptr::read_unaligned(buffer.as_ptr().add(row_cursor) as *const ArmorUpgradeRow)
            };
            
            table.rows.push(row);
            row_cursor += row_size;
        }
        
        mats.tables.push(table);
    }
    
    mats
}

/// Write armor upgrade materials to a byte buffer
/// Structure: pointer table (u32 per table) ending with 0x00000000, then data blocks
/// `base` = absolute file offset where the returned block will be written; the pointer table holds
/// absolute offsets (the file stores file offsets, never block-relative ones).
pub fn write_armor_upgrade_mats_block(mats: &ArmorUpgradeMats, base: u32) -> Result<Vec<u8>> {
    const ROW_SIZE: usize = size_of::<ArmorUpgradeRow>(); // 16 bytes
    let table_count = mats.tables.len();
    
    // Calculate pointer table size: (table_count + 1) * 4 bytes (extra for 0x00000000 terminator)
    let ptr_table_size = (table_count + 1) * 4;
    
    // Calculate data offsets for each table
    let mut data_offsets = Vec::new();
    let mut current_offset = base + ptr_table_size as u32;

    for table in &mats.tables {
        data_offsets.push(current_offset);
        current_offset += (table.rows.len() * ROW_SIZE) as u32;
        current_offset += ROW_SIZE as u32; // Add 16 bytes for terminator
    }
    
    let mut data = Vec::new();
    
    // Write pointer table
    for offset in &data_offsets {
        data.extend_from_slice(&offset.to_le_bytes());
    }
    // Write terminator for pointer table (0x00000000)
    data.extend_from_slice(&[0u8; 4]);
    
    // Write data blocks for each table
    for table in &mats.tables {
        for row in &table.rows {
            data.extend_from_slice(&row.item_id.to_le_bytes());
            data.extend_from_slice(&row.lv1_upgrade.to_le_bytes());
            data.extend_from_slice(&row.lv2_upgrade.to_le_bytes());
            data.extend_from_slice(&row.lv3_upgrade.to_le_bytes());
            data.extend_from_slice(&row.lv4_upgrade.to_le_bytes());
            data.extend_from_slice(&row.lv5_upgrade.to_le_bytes());
            data.extend_from_slice(&row.lv6_upgrade.to_le_bytes());
            data.extend_from_slice(&row.lv7_upgrade.to_le_bytes());
        }
        // Write terminator for this table (16 zero bytes)
        data.extend_from_slice(&[0u8; ROW_SIZE]);
    }
    
    Ok(data)
}

/// Read carve parts from buffer
/// Structure: pointer at CARVE_PARTS_PTR points to array of u32 pointers
/// The number of pointers is given by count parameter (from CARVE_PARTS_COUNT_PTR)
/// Each pointer points to a table of CarveDrop entries until a signed u16 == -1
pub fn read_carve_parts(buffer: &[u8], ptr_offset: u32, count: usize) -> CarveParts {
    let mut parts = CarveParts { tables: Vec::new() };
    
    // Read the main offset from the pointer (points to a table of pointers)
    if ptr_offset as usize + 4 > buffer.len() {
        return parts;
    }
    let ptr_table_offset = u32::from_le_bytes([
        buffer[ptr_offset as usize],
        buffer[ptr_offset as usize + 1],
        buffer[ptr_offset as usize + 2],
        buffer[ptr_offset as usize + 3],
    ]) as usize;
    
    if ptr_table_offset == 0 || ptr_table_offset >= buffer.len() {
        return parts;
    }
    
    // Read exactly 'count' pointers
    let carve_size = size_of::<CarveDrop>(); // 4 bytes (2 u16)
    let mut ptr_cursor = ptr_table_offset;
    
    for _ in 0..count {
        if ptr_cursor + 4 > buffer.len() {
            break;
        }
        
        let table_offset = u32::from_le_bytes([
            buffer[ptr_cursor],
            buffer[ptr_cursor + 1],
            buffer[ptr_cursor + 2],
            buffer[ptr_cursor + 3],
        ]) as usize;
        
        ptr_cursor += 4;
        
        if table_offset == 0 || table_offset >= buffer.len() {
            parts.tables.push(CarveDropTable { carves: Vec::new() });
            continue;
        }
        
        // Read carves for this table until we hit a signed u16 == -1
        let mut table = CarveDropTable { carves: Vec::new() };
        let mut carve_cursor = table_offset;
        
        while carve_cursor + carve_size <= buffer.len() {
            // Read the next u16 as signed to check for -1 terminator
            let terminator_check = i16::from_le_bytes([
                buffer[carve_cursor],
                buffer[carve_cursor + 1],
            ]);
            
            if terminator_check == -1 {
                break;
            }
            
            let carve = unsafe {
                std::ptr::read_unaligned(buffer.as_ptr().add(carve_cursor) as *const CarveDrop)
            };
            
            table.carves.push(carve);
            carve_cursor += carve_size;
        }
        
        parts.tables.push(table);
    }
    
    parts
}

/// Write carve parts to a byte buffer
/// Structure: pointer table (u32 per table), then data blocks
/// `base` = absolute file offset where the returned block will be written (pointers are absolute).
pub fn write_carve_parts_block(parts: &CarveParts, base: u32) -> Result<Vec<u8>> {
    const CARVE_SIZE: usize = size_of::<CarveDrop>(); // 4 bytes
    let table_count = parts.tables.len();
    
    // Calculate pointer table size: table_count * 4 bytes
    let ptr_table_size = table_count * 4;
    
    // Calculate data offsets for each table
    let mut data_offsets = Vec::new();
    let mut current_offset = base + ptr_table_size as u32;

    for table in &parts.tables {
        data_offsets.push(current_offset);
        // Each carve is 4 bytes, plus 2 bytes for -1 terminator
        current_offset += (table.carves.len() * CARVE_SIZE) as u32;
        current_offset += 2; // Add 2 bytes for -1 terminator (signed u16)
    }
    
    let mut data = Vec::new();
    
    // Write pointer table
    for offset in &data_offsets {
        data.extend_from_slice(&offset.to_le_bytes());
    }
    
    // Write data blocks for each table
    for table in &parts.tables {
        for carve in &table.carves {
            data.extend_from_slice(&carve.percentage.to_le_bytes());
            data.extend_from_slice(&carve.item_id.to_le_bytes());
        }
        // Write terminator for this table (-1 as signed u16)
        data.extend_from_slice(&(-1i16).to_le_bytes());
    }
    
    Ok(data)
}

/// Read part break parts from buffer
/// Structure: pointer at PART_BREAK_DROP_PTR points to array of u32 pointers
/// The number of pointers is given by count parameter (from PART_BREAK_DROP_COUNT_PTR)
/// Each pointer points to a table of PartBreakDrop entries until a signed u16 == -1
pub fn read_part_break_parts(buffer: &[u8], ptr_offset: u32, count: usize) -> PartBreakParts {
    
    let mut parts = PartBreakParts { tables: Vec::new() };
    
    // Read the main offset from the pointer (points to a table of pointers)
    if ptr_offset as usize + 4 > buffer.len() {
        return parts;
    }
    let ptr_table_offset = u32::from_le_bytes([
        buffer[ptr_offset as usize],
        buffer[ptr_offset as usize + 1],
        buffer[ptr_offset as usize + 2],
        buffer[ptr_offset as usize + 3],
    ]) as usize;
    
    if ptr_table_offset == 0 || ptr_table_offset >= buffer.len() {
        return parts;
    }
    
    // Read exactly 'count' pointers
    let drop_size = size_of::<PartBreakDrop>(); // 6 bytes (3 u16)
    let mut ptr_cursor = ptr_table_offset;
    
    for _ in 0..count {
        if ptr_cursor + 4 > buffer.len() {
            break;
        }
        
        let table_offset = u32::from_le_bytes([
            buffer[ptr_cursor],
            buffer[ptr_cursor + 1],
            buffer[ptr_cursor + 2],
            buffer[ptr_cursor + 3],
        ]) as usize;
        
        ptr_cursor += 4;
        
        if table_offset == 0 || table_offset >= buffer.len() {
            parts.tables.push(PartBreakDropTable { break_drops: Vec::new() });
            continue;
        }
        
        // Read drops for this table until we hit a signed u16 == -1
        let mut table = PartBreakDropTable { break_drops: Vec::new() };
        let mut drop_cursor = table_offset;
        
        while drop_cursor + drop_size <= buffer.len() {
            // Read the next u16 as signed to check for -1 terminator
            let terminator_check = i16::from_le_bytes([
                buffer[drop_cursor],
                buffer[drop_cursor + 1],
            ]);
            
            if terminator_check == -1 {
                break;
            }
            
            let drop = unsafe {
                std::ptr::read_unaligned(buffer.as_ptr().add(drop_cursor) as *const PartBreakDrop)
            };
            
            table.break_drops.push(drop);
            drop_cursor += drop_size;
        }
        
        parts.tables.push(table);
    }
    
    parts
}

/// Write part break parts to a byte buffer
/// Structure: pointer table (u32 per table), then data blocks
/// `base` = absolute file offset where the returned block will be written (pointers are absolute).
pub fn write_part_break_parts_block(parts: &PartBreakParts, base: u32) -> Result<Vec<u8>> {
    
    const DROP_SIZE: usize = size_of::<PartBreakDrop>(); // 6 bytes
    let table_count = parts.tables.len();
    
    // Calculate pointer table size: table_count * 4 bytes
    let ptr_table_size = table_count * 4;
    
    // Calculate data offsets for each table
    let mut data_offsets = Vec::new();
    let mut current_offset = base + ptr_table_size as u32;

    for table in &parts.tables {
        data_offsets.push(current_offset);
        // Each drop is 6 bytes, plus 2 bytes for -1 terminator
        current_offset += (table.break_drops.len() * DROP_SIZE) as u32;
        current_offset += 2; // Add 2 bytes for -1 terminator (signed u16)
    }
    
    let mut data = Vec::new();
    
    // Write pointer table
    for offset in &data_offsets {
        data.extend_from_slice(&offset.to_le_bytes());
    }
    
    // Write data blocks for each table
    for table in &parts.tables {
        for drop in &table.break_drops {
            data.extend_from_slice(&drop.percentage.to_le_bytes());
            data.extend_from_slice(&drop.item_id.to_le_bytes());
            data.extend_from_slice(&drop.number.to_le_bytes());
        }
        // Write terminator for this table (-1 as signed u16)
        data.extend_from_slice(&(-1i16).to_le_bytes());
    }
    
    Ok(data)
}

pub fn read_sigil_tower_table(buffer: &[u8], offset: usize) -> Vec<SigilTowerTable> {
    let mut entries = Vec::new();
    let mut cursor = offset;
    while cursor + size_of::<SigilTowerTable>() <= buffer.len() {
        let entry = unsafe { std::ptr::read_unaligned(buffer.as_ptr().add(cursor) as *const SigilTowerTable) };
        if entry.item_id == 0 {
            break;
        }
        entries.push(entry);
        cursor += size_of::<SigilTowerTable>();
    }
    entries
}

pub fn read_deco_id_table(buffer: &[u8], offset: usize, max_count: Option<usize>) -> Vec<MhfdatDecoId> {
    let mut entries = Vec::new();
    let mut cursor = offset;
    let entry_size = std::mem::size_of::<MhfdatDecoId>();
    let limit = max_count.unwrap_or(usize::MAX);
    while cursor + entry_size <= buffer.len() && entries.len() < limit {
        let entry = unsafe { std::ptr::read_unaligned(buffer.as_ptr().add(cursor) as *const MhfdatDecoId) };
        // If no fixed count specified, stop on an all-zero style entry; otherwise keep reading
        if max_count.is_none() && entry.slot_nb == 0 && entry.flags == 0 && entry.price == 0 { break; }
        entries.push(entry);
        cursor += entry_size;
    }
    entries
}

pub fn read_g50_weapon_upgrades(buffer: &[u8], offset: usize) -> Vec<G50WUpgrade> {
    let mut entries = Vec::new();
    let mut cursor = offset;
    while cursor + size_of::<G50WUpgrade>() <= buffer.len() {
        let entry = unsafe { std::ptr::read_unaligned(buffer.as_ptr().add(cursor) as *const G50WUpgrade) };
        if entry.weapon_id == 0 {
            break;
        }
        entries.push(entry);
        cursor += size_of::<G50WUpgrade>();
    }
    entries
}

pub fn read_mw_upgrade_paths(buffer: &[u8], offset: usize, count: usize) -> Vec<MWUpgradePath> {
    let mut entries = Vec::with_capacity(count);
    let mut cursor = offset;
    for _ in 0..count {
        if cursor + size_of::<MWUpgradePath>() <= buffer.len() {
            let entry = unsafe { std::ptr::read_unaligned(buffer.as_ptr().add(cursor) as *const MWUpgradePath) };
            entries.push(entry);
            cursor += size_of::<MWUpgradePath>();
        }
    }
    entries
}

pub fn read_rw_upgrade_paths(buffer: &[u8], offset: usize, count: usize) -> Vec<RWUpgradePath> {
    let mut entries = Vec::with_capacity(count);
    let mut cursor = offset;
    for _ in 0..count {
        if cursor + size_of::<RWUpgradePath>() <= buffer.len() {
            let entry = unsafe { std::ptr::read_unaligned(buffer.as_ptr().add(cursor) as *const RWUpgradePath) };
            entries.push(entry);
            cursor += size_of::<RWUpgradePath>();
        }
    }
    entries
}

pub fn read_evo_upgrades(buffer: &[u8], offset: usize, count: usize) -> Vec<EvoUpgrade> {
    let mut entries = Vec::with_capacity(count);
    let mut cursor = offset;
    for _ in 0..count {
        if cursor + size_of::<EvoUpgrade>() <= buffer.len() {
            let entry = unsafe { std::ptr::read_unaligned(buffer.as_ptr().add(cursor) as *const EvoUpgrade) };
            entries.push(entry);
            cursor += size_of::<EvoUpgrade>();
        }
    }
    entries
}

/// DEBUG: Always read and print the first 5 ShopEntry at the transmog offset
#[allow(dead_code)]
pub fn debug_print_first_transmog_entries(buffer: &[u8], offset: usize) {
    use std::mem::size_of;
    let entry_size = size_of::<ShopEntry>();
    for i in 0..5 {
        let start = offset + i * entry_size;
        let end = start + entry_size;
        if end > buffer.len() { break; }
        let entry = unsafe { std::ptr::read_unaligned(buffer[start..end].as_ptr() as *const ShopEntry) };
        let equip_id = entry.equip_id;
        let material_id1 = entry.material_id1;
        let material_amnt1 = entry.material_amnt1;
        let material_id2 = entry.material_id2;
        let material_amnt2 = entry.material_amnt2;
        let material_id3 = entry.material_id3;
        let material_amnt3 = entry.material_amnt3;
        let material_id4 = entry.material_id4;
        let material_amnt4 = entry.material_amnt4;
        let hr_req = entry.hr_req;
        let purchaseable = entry.purchaseable;
        let footer = entry.footer;

        println!(
            "[DEBUG] TransmogEntry #{}: equip_id={:08X} mat1={:04X}x{} mat2={:04X}x{} mat3={:04X}x{} mat4={:04X}x{} hr_req={} purchaseable={} footer={:02X}",
            i+1,
            equip_id,
            material_id1, material_amnt1,
            material_id2, material_amnt2,
            material_id3, material_amnt3,
            material_id4, material_amnt4,
            hr_req,
            purchaseable,
            footer
        );
    }
}

pub fn read_deco_shop_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<DecoShop>> {
    let mut entries = Vec::new();
    reader.seek(SeekFrom::Start(offset))?;
    loop {
        let entry = read_deco_shop_entry(reader)?;
        if entry.deco_item_id == 0xFFFF {
            break;
        }
        entries.push(entry);
    }
    Ok(entries)
}

pub fn read_sigil_tower_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<SigilTowerTable>> {
    let mut entries = Vec::new();
    reader.seek(SeekFrom::Start(offset))?;
    loop {
        let entry = read_sigil_tower_entry(reader)?;
        if entry.item_id == 0xFFFF {
            break;
        }
        entries.push(entry);
    }
    Ok(entries)
}

pub fn read_g50_weapon_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<G50WUpgrade>> {
    let mut entries = Vec::new();
    reader.seek(SeekFrom::Start(offset))?;
    loop {
        let entry = read_g50_weapon_entry(reader)?;
        if entry.weapon_id == 0xFFFF {
            break;
        }
        entries.push(entry);
    }
    Ok(entries)
}

pub fn read_g50_weapon_by_count<R: Read + Seek>(reader: &mut R, offset: u64, count: usize) -> Result<Vec<G50WUpgrade>> {
    let mut entries = Vec::with_capacity(count);
    reader.seek(SeekFrom::Start(offset))?;
    for _ in 0..count {
        let entry = read_g50_weapon_entry(reader)?;
        entries.push(entry);
    }
    Ok(entries)
}

use crate::model::mhfdat::{G50WeaponLevels, G50WeaponTypeData};

// G50 Tower Weapon Params - read 130 weapon pointers, each pointing to 50 level entries
pub fn read_tower_g50_weapon_type(buffer: &[u8], ptr_table_offset: usize) -> G50WeaponTypeData {
    const TABLE_COUNT: usize = 130;
    const LEVEL_COUNT: usize = 50;
    let entry_size = std::mem::size_of::<TowerG50WeaponParams>(); // 16 bytes
    
    let mut weapons = Vec::with_capacity(TABLE_COUNT);
    
    for w in 0..TABLE_COUNT {
        let ptr_offset = ptr_table_offset + w * 4;
        if ptr_offset + 4 > buffer.len() {
            break;
        }
        let data_offset = u32::from_le_bytes(buffer[ptr_offset..ptr_offset + 4].try_into().unwrap()) as usize;
        
        let mut levels = Vec::with_capacity(LEVEL_COUNT);
        for l in 0..LEVEL_COUNT {
            let start = data_offset + l * entry_size;
            if start + entry_size > buffer.len() {
                break;
            }
            let entry = unsafe {
                std::ptr::read_unaligned(buffer.as_ptr().add(start) as *const TowerG50WeaponParams)
            };
            levels.push(entry);
        }
        weapons.push(G50WeaponLevels { levels });
    }
    
    G50WeaponTypeData { weapons }
}

pub fn write_tower_g50_weapon_type(data: &G50WeaponTypeData, base_offset: u32) -> Result<(Vec<u8>, Vec<u8>)> {
    let entry_size = std::mem::size_of::<TowerG50WeaponParams>();
    let level_block_size = 50 * entry_size;
    
    // Calculate total data size: 130 weapons * 50 levels * 16 bytes
    let mut data_block = Vec::new();
    let mut ptr_table = Vec::new();
    
    let data_start = base_offset + (130 * 4) as u32; // After pointer table
    
    for (w, weapon) in data.weapons.iter().enumerate() {
        let weapon_data_offset = data_start + (w * level_block_size) as u32;
        ptr_table.extend_from_slice(&weapon_data_offset.to_le_bytes());
        
        for level in &weapon.levels {
            let bytes = unsafe {
                std::slice::from_raw_parts(level as *const TowerG50WeaponParams as *const u8, entry_size)
            };
            data_block.extend_from_slice(bytes);
        }
        // Pad if less than 50 levels
        for _ in weapon.levels.len()..50 {
            data_block.extend_from_slice(&[0u8; 16]);
        }
    }
    
    Ok((ptr_table, data_block))
}

pub fn read_mw_upgrade_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<MWUpgradePath>> {
    let mut entries = Vec::new();
    reader.seek(SeekFrom::Start(offset))?;
    loop {
        let entry = read_mw_upgrade_entry(reader)?;
        if entry.upgrade_material1 == 0xFFFF {
            break;
        }
        entries.push(entry);
    }
    Ok(entries)
}

pub fn read_rw_upgrade_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<RWUpgradePath>> {
    let mut entries = Vec::new();
    reader.seek(SeekFrom::Start(offset))?;
    loop {
        let entry = read_rw_upgrade_entry(reader)?;
        if entry.upgrade_material1 == 0xFFFF {
            break;
        }
        entries.push(entry);
    }
    Ok(entries)
}

pub fn read_evo_upgrade_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<EvoUpgrade>> {
    let mut entries = Vec::new();
    reader.seek(SeekFrom::Start(offset))?;
    loop {
        let entry = read_evo_upgrade_entry(reader)?;
        if entry.g_cost == 0xFFFFFFFF {
            break;
        }
        entries.push(entry);
    }
    Ok(entries)
}

pub fn read_shop_entries_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<ShopEntry>> {
    let mut entries = Vec::new();
    reader.seek(SeekFrom::Start(offset))?;
    loop {
        let entry = read_shop_entry(reader)?;
        if entry.equip_id == 0xFFFF {
            break;
        }
        entries.push(entry);
    }
    Ok(entries)
}

pub fn read_armor_upgrades_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<crate::model::mhfdat::ArmorUpgradeRow>> {
    use crate::model::mhfdat::ArmorUpgradeRow;
    reader.seek(SeekFrom::Start(offset))?;
    let mut rows: Vec<ArmorUpgradeRow> = Vec::new();
    loop {
        // Read one row (16 bytes)
        let item_id = reader.read_u16::<LittleEndian>()?;
        if item_id == 0xFFFF { break; }
        let lv1 = reader.read_u16::<LittleEndian>()?;
        let lv2 = reader.read_u16::<LittleEndian>()?;
        let lv3 = reader.read_u16::<LittleEndian>()?;
        let lv4 = reader.read_u16::<LittleEndian>()?;
        let lv5 = reader.read_u16::<LittleEndian>()?;
        let lv6 = reader.read_u16::<LittleEndian>()?;
        let lv7 = reader.read_u16::<LittleEndian>()?;
        rows.push(ArmorUpgradeRow {
            item_id,
            lv1_upgrade: lv1,
            lv2_upgrade: lv2,
            lv3_upgrade: lv3,
            lv4_upgrade: lv4,
            lv5_upgrade: lv5,
            lv6_upgrade: lv6,
            lv7_upgrade: lv7,
        });
    }
    Ok(rows)
}

pub fn read_armor_upgrades_bounded<R: Read + Seek>(
    reader: &mut R,
    offset: u64,
    end: u64,
) -> Result<Vec<crate::model::mhfdat::ArmorUpgradeRow>> {
    use crate::model::mhfdat::ArmorUpgradeRow;
    reader.seek(SeekFrom::Start(offset))?;
    let mut rows: Vec<ArmorUpgradeRow> = Vec::new();
    loop {
        // Ensure we have room for a full row (16 bytes)
        if reader.stream_position()? + 16 > end { break; }
        let item_id = reader.read_u16::<LittleEndian>()?;
        if item_id == 0xFFFF { break; }
        let lv1 = reader.read_u16::<LittleEndian>()?;
        let lv2 = reader.read_u16::<LittleEndian>()?;
        let lv3 = reader.read_u16::<LittleEndian>()?;
        let lv4 = reader.read_u16::<LittleEndian>()?;
        let lv5 = reader.read_u16::<LittleEndian>()?;
        let lv6 = reader.read_u16::<LittleEndian>()?;
        let lv7 = reader.read_u16::<LittleEndian>()?;
        rows.push(ArmorUpgradeRow {
            item_id,
            lv1_upgrade: lv1,
            lv2_upgrade: lv2,
            lv3_upgrade: lv3,
            lv4_upgrade: lv4,
            lv5_upgrade: lv5,
            lv6_upgrade: lv6,
            lv7_upgrade: lv7,
        });
    }
    Ok(rows)
}

fn read_armor_upgrade_row_at<R: Read + Seek>(reader: &mut R, at: u64) -> Result<crate::model::mhfdat::ArmorUpgradeRow> {
    use crate::model::mhfdat::ArmorUpgradeRow;
    reader.seek(SeekFrom::Start(at))?;
    let item_id = reader.read_u16::<LittleEndian>()?;
    let lv1 = reader.read_u16::<LittleEndian>()?;
    let lv2 = reader.read_u16::<LittleEndian>()?;
    let lv3 = reader.read_u16::<LittleEndian>()?;
    let lv4 = reader.read_u16::<LittleEndian>()?;
    let lv5 = reader.read_u16::<LittleEndian>()?;
    let lv6 = reader.read_u16::<LittleEndian>()?;
    let lv7 = reader.read_u16::<LittleEndian>()?;
    Ok(ArmorUpgradeRow {
        item_id,
        lv1_upgrade: lv1,
        lv2_upgrade: lv2,
        lv3_upgrade: lv3,
        lv4_upgrade: lv4,
        lv5_upgrade: lv5,
        lv6_upgrade: lv6,
        lv7_upgrade: lv7,
    })
}

pub fn read_armor_upgrades_from_pointer_table<R: Read + Seek>(
    reader: &mut R,
    table_off: u64,
    table_end: u64,
    file_len: u64,
    rows_per_ptr: usize,
) -> Result<Vec<crate::model::mhfdat::ArmorUpgradeRow>> {
    use crate::model::mhfdat::ArmorUpgradeRow;
    let mut rows: Vec<ArmorUpgradeRow> = Vec::new();
    reader.seek(SeekFrom::Start(table_off))?;
    while reader.stream_position()? + 4 <= table_end {
        let ptr = reader.read_u32::<LittleEndian>()?;
        if ptr == 0 { break; }
        // Pointers in a clear file are file offsets; one past the end is corrupt, not a memory address.
        let start = ptr as u64;
        // Ensure we have enough space for fixed rows
        let need = (rows_per_ptr as u64) * 16;
        if start + need > file_len { break; }
        for k in 0..rows_per_ptr {
            let at = start + (k as u64) * 16;
            let row = read_armor_upgrade_row_at(reader, at)?;
            if row.item_id == 0xFFFF { break; }
            rows.push(row);
        }
    }
    Ok(rows)
}

// Fonctions de lecture individuelles pour chaque type d'entrée
fn read_deco_shop_entry<R: Read>(reader: &mut R) -> Result<DecoShop> {
    let deco_item_id = reader.read_u16::<LittleEndian>()?;
    let receipt_category = reader.read_u16::<LittleEndian>()?;
    let item_id1 = reader.read_u16::<LittleEndian>()?;
    let item_qty1 = reader.read_u8()?;
    let item_unlock_flag1 = reader.read_u8()?;
    let item_id2 = reader.read_u16::<LittleEndian>()?;
    let item_qty2 = reader.read_u8()?;
    let item_unlock_flag2 = reader.read_u8()?;
    let item_id3 = reader.read_u16::<LittleEndian>()?;
    let item_qty3 = reader.read_u8()?;
    let item_unlock_flag3 = reader.read_u8()?;
    let item_id4 = reader.read_u16::<LittleEndian>()?;
    let item_qty4 = reader.read_u8()?;
    let item_unlock_flag4 = reader.read_u8()?;
    Ok(DecoShop {
        deco_item_id,
        receipt_category,
        item_id1,
        item_qty1,
        item_unlock_flag1,
        item_id2,
        item_qty2,
        item_unlock_flag2,
        item_id3,
        item_qty3,
        item_unlock_flag3,
        item_id4,
        item_qty4,
        item_unlock_flag4,
    })
}

fn read_sigil_tower_entry<R: Read>(reader: &mut R) -> Result<SigilTowerTable> {
    let item_id = reader.read_u16::<LittleEndian>()?;
    let receipt_category = reader.read_u16::<LittleEndian>()?;
    let item_craft1 = reader.read_u16::<LittleEndian>()?;
    let item_qty1 = reader.read_u8()?;
    let item_unlock_flag1 = reader.read_u8()?;
    let item_craft2 = reader.read_u16::<LittleEndian>()?;
    let item_qty2 = reader.read_u8()?;
    let item_unlock_flag2 = reader.read_u8()?;
    let item_craft3 = reader.read_u16::<LittleEndian>()?;
    let item_qty3 = reader.read_u8()?;
    let item_unlock_flag3 = reader.read_u8()?;
    let item_craft4 = reader.read_u16::<LittleEndian>()?;
    let item_qty4 = reader.read_u8()?;
    let item_unlock_flag4 = reader.read_u8()?;
    Ok(SigilTowerTable {
        item_id,
        receipt_category,
        item_craft1,
        item_qty1,
        item_unlock_flag1,
        item_craft2,
        item_qty2,
        item_unlock_flag2,
        item_craft3,
        item_qty3,
        item_unlock_flag3,
        item_craft4,
        item_qty4,
        item_unlock_flag4,
    })
}

fn read_g50_weapon_entry<R: Read>(reader: &mut R) -> Result<G50WUpgrade> {
    let weapon_id = reader.read_u16::<LittleEndian>()?;
    let level1 = reader.read_u16::<LittleEndian>()?;
    let level2 = reader.read_u16::<LittleEndian>()?;
    let full_succ_rate = reader.read_u16::<LittleEndian>()?;
    let zenny_cost = reader.read_u32::<LittleEndian>()?;
    let upgrade_material1 = reader.read_u16::<LittleEndian>()?;
    let num_material1 = reader.read_u8()?;
    let mut padding1 = [0u8; 5];
    reader.read_exact(&mut padding1)?;
    let upgrade_material2 = reader.read_u16::<LittleEndian>()?;
    let num_material2 = reader.read_u8()?;
    let mut padding2 = [0u8; 5];
    reader.read_exact(&mut padding2)?;
    let upgrade_material3 = reader.read_u16::<LittleEndian>()?;
    let num_material3 = reader.read_u8()?;
    let mut padding3 = [0u8; 9];
    reader.read_exact(&mut padding3)?;
    Ok(G50WUpgrade {
        weapon_id,
        level1,
        level2,
        full_succ_rate,
        zenny_cost,
        upgrade_material1,
        num_material1,
        padding1,
        upgrade_material2,
        num_material2,
        padding2,
        upgrade_material3,
        num_material3,
        padding3,
    })
}

fn read_mw_upgrade_entry<R: Read>(reader: &mut R) -> Result<MWUpgradePath> {
    let upgrade_material1 = reader.read_u16::<LittleEndian>()?;
    let num_material1 = reader.read_u16::<LittleEndian>()?;
    let mut padding1 = [0u8; 4];
    reader.read_exact(&mut padding1)?;
    let upgrade_material2 = reader.read_u16::<LittleEndian>()?;
    let num_material2 = reader.read_u16::<LittleEndian>()?;
    let mut padding2 = [0u8; 4];
    reader.read_exact(&mut padding2)?;
    let upgrade_material3 = reader.read_u16::<LittleEndian>()?;
    let num_material3 = reader.read_u16::<LittleEndian>()?;
    let mut padding3 = [0u8; 4];
    reader.read_exact(&mut padding3)?;
    let upgrades_to1 = reader.read_u16::<LittleEndian>()?;
    let upgrades_to2 = reader.read_u16::<LittleEndian>()?;
    let upgrades_to3 = reader.read_u16::<LittleEndian>()?;
    let upgrades_to4 = reader.read_u16::<LittleEndian>()?;
    let mut padding4 = [0u8; 4];
    reader.read_exact(&mut padding4)?;
    Ok(MWUpgradePath {
        upgrade_material1,
        num_material1,
        padding1,
        upgrade_material2,
        num_material2,
        padding2,
        upgrade_material3,
        num_material3,
        padding3,
        upgrades_to1,
        upgrades_to2,
        upgrades_to3,
        upgrades_to4,
        padding4,
    })
}

fn read_rw_upgrade_entry<R: Read>(reader: &mut R) -> Result<RWUpgradePath> {
    let upgrade_material1 = reader.read_u16::<LittleEndian>()?;
    let num_material1 = reader.read_u16::<LittleEndian>()?;
    let mut padding1 = [0u8; 4];
    reader.read_exact(&mut padding1)?;
    let upgrade_material2 = reader.read_u16::<LittleEndian>()?;
    let num_material2 = reader.read_u16::<LittleEndian>()?;
    let mut padding2 = [0u8; 4];
    reader.read_exact(&mut padding2)?;
    let upgrade_material3 = reader.read_u16::<LittleEndian>()?;
    let num_material3 = reader.read_u16::<LittleEndian>()?;
    let mut padding3 = [0u8; 4];
    reader.read_exact(&mut padding3)?;
    let upgrades_to1 = reader.read_u16::<LittleEndian>()?;
    let upgrades_to2 = reader.read_u16::<LittleEndian>()?;
    let upgrades_to3 = reader.read_u16::<LittleEndian>()?;
    let upgrades_to4 = reader.read_u16::<LittleEndian>()?;
    let mut padding4 = [0u8; 4];
    reader.read_exact(&mut padding4)?;
    Ok(RWUpgradePath {
        upgrade_material1,
        num_material1,
        padding1,
        upgrade_material2,
        num_material2,
        padding2,
        upgrade_material3,
        num_material3,
        padding3,
        upgrades_to1,
        upgrades_to2,
        upgrades_to3,
        upgrades_to4,
        padding4,
    })
}

fn read_evo_upgrade_entry<R: Read>(reader: &mut R) -> Result<EvoUpgrade> {
    let g_cost = reader.read_u32::<LittleEndian>()?;
    let unk04 = reader.read_u16::<LittleEndian>()?;
    let level = reader.read_u16::<LittleEndian>()?;
    let mut sub = [EvoUpgradeSub::default(); 11];
    for i in 0..11 {
        let dmg = reader.read_u16::<LittleEndian>()?;
        let unk_sub2 = reader.read_u16::<LittleEndian>()?;
        let ele_damage = reader.read_u16::<LittleEndian>()?;
        let mut padding1 = [0u8; 2];
        reader.read_exact(&mut padding1)?;
        let unk_sub8 = reader.read_u16::<LittleEndian>()?;
        let mut padding2 = [0u8; 2];
        reader.read_exact(&mut padding2)?;
        sub[i] = EvoUpgradeSub {
            dmg,
            unk_sub2,
            ele_damage,
            padding1,
            unk_sub8,
            padding2,
        };
    }
    let unk8c = reader.read_u16::<LittleEndian>()?;
    let unk8e = reader.read_u8()?;
    let mut padding1 = [0u8; 1];
    reader.read_exact(&mut padding1)?;
    let unk90 = reader.read_u16::<LittleEndian>()?;
    let mut padding2 = [0u8; 2];
    reader.read_exact(&mut padding2)?;
    let unk94 = reader.read_f32::<LittleEndian>()?;
    let unk98 = reader.read_f32::<LittleEndian>()?;
    let unk9c = reader.read_f32::<LittleEndian>()?;
    let unka0 = reader.read_f32::<LittleEndian>()?;
    let unka4 = reader.read_f32::<LittleEndian>()?;
    let unka8 = reader.read_u16::<LittleEndian>()?;
    let unkaa = reader.read_u16::<LittleEndian>()?;
    let unkac = reader.read_u8()?;
    let unkad = reader.read_u8()?;
    let mut padding3 = [0u8; 2];
    reader.read_exact(&mut padding3)?;
    Ok(EvoUpgrade {
        g_cost,
        unk04,
        level,
        sub,
        unk8c,
        unk8e,
        padding1,
        unk90,
        padding2,
        unk94,
        unk98,
        unk9c,
        unka0,
        unka4,
        unka8,
        unkaa,
        unkac,
        unkad,
        padding3,
    })
}

fn read_shop_entry<R: Read>(reader: &mut R) -> Result<ShopEntry> {
    let equip_type = reader.read_u8()?;
    let purchaseable = reader.read_u8()?;
    let equip_id = reader.read_u16::<LittleEndian>()?;
    let material_id1 = reader.read_u16::<LittleEndian>()?;
    let material_amnt1 = reader.read_u16::<LittleEndian>()?;
    let mut padding1 = [0u8; 4];
    reader.read_exact(&mut padding1)?;
    let material_id2 = reader.read_u16::<LittleEndian>()?;
    let material_amnt2 = reader.read_u16::<LittleEndian>()?;
    let mut padding2 = [0u8; 4];
    reader.read_exact(&mut padding2)?;
    let material_id3 = reader.read_u16::<LittleEndian>()?;
    let material_amnt3 = reader.read_u16::<LittleEndian>()?;
    let mut padding3 = [0u8; 4];
    reader.read_exact(&mut padding3)?;
    let material_id4 = reader.read_u16::<LittleEndian>()?;
    let material_amnt4 = reader.read_u16::<LittleEndian>()?;
    let mut padding4 = [0u8; 4];
    reader.read_exact(&mut padding4)?;
    let unk24 = reader.read_u16::<LittleEndian>()?;
    let mut padding5 = [0u8; 2];
    reader.read_exact(&mut padding5)?;
    let hr_req = reader.read_u16::<LittleEndian>()?;
    let unk2a = reader.read_u16::<LittleEndian>()?;
    let preview_able = reader.read_u8()? != 0;
    let mut padding6 = [0u8; 3];
    reader.read_exact(&mut padding6)?;
    let footer = reader.read_u8()?;
    let mut padding7 = [0u8; 3];
    reader.read_exact(&mut padding7)?;
    let unk34 = reader.read_u8()?;
    let mut padding8 = [0u8; 3];
    reader.read_exact(&mut padding8)?;
    Ok(ShopEntry {
        equip_type,
        purchaseable,
        equip_id,
        material_id1,
        material_amnt1,
        padding1,
        material_id2,
        material_amnt2,
        padding2,
        material_id3,
        material_amnt3,
        padding3,
        material_id4,
        material_amnt4,
        padding4,
        unk24,
        padding5,
        hr_req,
        unk2a,
        preview_able,
        padding6,
        footer,
        padding7,
        unk34,
        padding8,
    })
}

pub fn write_shop_entry(writer: &mut impl Write, entry: &ShopEntry) -> Result<()> {
    writer.write_all(&entry.equip_type.to_le_bytes())?;
    writer.write_all(&entry.purchaseable.to_le_bytes())?;
    writer.write_all(&entry.equip_id.to_le_bytes())?;
    writer.write_all(&entry.material_id1.to_le_bytes())?;
    writer.write_all(&entry.material_amnt1.to_le_bytes())?;
    writer.write_all(&entry.padding1)?;
    writer.write_all(&entry.material_id2.to_le_bytes())?;
    writer.write_all(&entry.material_amnt2.to_le_bytes())?;
    writer.write_all(&entry.padding2)?;
    writer.write_all(&entry.material_id3.to_le_bytes())?;
    writer.write_all(&entry.material_amnt3.to_le_bytes())?;
    writer.write_all(&entry.padding3)?;
    writer.write_all(&entry.material_id4.to_le_bytes())?;
    writer.write_all(&entry.material_amnt4.to_le_bytes())?;
    writer.write_all(&entry.padding4)?;
    writer.write_all(&entry.unk24.to_le_bytes())?;
    writer.write_all(&entry.padding5)?;
    writer.write_all(&entry.hr_req.to_le_bytes())?;
    writer.write_all(&entry.unk2a.to_le_bytes())?;
    writer.write_all(&[entry.preview_able as u8])?;
    writer.write_all(&entry.padding6)?;
    writer.write_all(&[entry.footer])?;
    writer.write_all(&entry.padding7)?;
    writer.write_all(&[entry.unk34])?;
    writer.write_all(&entry.padding8)?;
    Ok(())
}

pub fn write_sigil_tower_table(writer: &mut impl Write, entry: &SigilTowerTable) -> Result<()> {
    writer.write_all(&entry.item_id.to_le_bytes())?;
    writer.write_all(&entry.receipt_category.to_le_bytes())?;
    writer.write_all(&entry.item_craft1.to_le_bytes())?;
    writer.write_all(&[entry.item_qty1])?;
    writer.write_all(&[entry.item_unlock_flag1])?;
    writer.write_all(&entry.item_craft2.to_le_bytes())?;
    writer.write_all(&[entry.item_qty2])?;
    writer.write_all(&[entry.item_unlock_flag2])?;
    writer.write_all(&entry.item_craft3.to_le_bytes())?;
    writer.write_all(&[entry.item_qty3])?;
    writer.write_all(&[entry.item_unlock_flag3])?;
    writer.write_all(&entry.item_craft4.to_le_bytes())?;
    writer.write_all(&[entry.item_qty4])?;
    writer.write_all(&[entry.item_unlock_flag4])?;
    Ok(())
}

pub fn write_g50_weapon_upgrade(writer: &mut impl Write, entry: &G50WUpgrade) -> Result<()> {
    writer.write_all(&entry.weapon_id.to_le_bytes())?;
    writer.write_all(&entry.level1.to_le_bytes())?;
    writer.write_all(&entry.level2.to_le_bytes())?;
    writer.write_all(&entry.full_succ_rate.to_le_bytes())?;
    writer.write_all(&entry.zenny_cost.to_le_bytes())?;
    writer.write_all(&entry.upgrade_material1.to_le_bytes())?;
    writer.write_all(&[entry.num_material1])?;
    writer.write_all(&entry.padding1)?;
    writer.write_all(&entry.upgrade_material2.to_le_bytes())?;
    writer.write_all(&[entry.num_material2])?;
    writer.write_all(&entry.padding2)?;
    writer.write_all(&entry.upgrade_material3.to_le_bytes())?;
    writer.write_all(&[entry.num_material3])?;
    writer.write_all(&entry.padding3)?;
    Ok(())
}

pub fn write_g50_weapon_upgrades_block(entries: &[G50WUpgrade]) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    for entry in entries {
        write_g50_weapon_upgrade(&mut buffer, entry)?;
    }
    // Add sentinel (all zeros - same size as G50WUpgrade: 44 bytes)
    buffer.write_all(&[0u8; 44])?;
    Ok(buffer)
}

pub fn write_mw_upgrade_path(writer: &mut impl Write, entry: &MWUpgradePath) -> Result<()> {
    writer.write_all(&entry.upgrade_material1.to_le_bytes())?;
    writer.write_all(&entry.num_material1.to_le_bytes())?;
    writer.write_all(&entry.padding1)?;
    writer.write_all(&entry.upgrade_material2.to_le_bytes())?;
    writer.write_all(&entry.num_material2.to_le_bytes())?;
    writer.write_all(&entry.padding2)?;
    writer.write_all(&entry.upgrade_material3.to_le_bytes())?;
    writer.write_all(&entry.num_material3.to_le_bytes())?;
    writer.write_all(&entry.padding3)?;
    writer.write_all(&entry.upgrades_to1.to_le_bytes())?;
    writer.write_all(&entry.upgrades_to2.to_le_bytes())?;
    writer.write_all(&entry.upgrades_to3.to_le_bytes())?;
    writer.write_all(&entry.upgrades_to4.to_le_bytes())?;
    writer.write_all(&entry.padding4)?;
    Ok(())
}

pub fn write_rw_upgrade_path(writer: &mut impl Write, entry: &RWUpgradePath) -> Result<()> {
    writer.write_all(&entry.upgrade_material1.to_le_bytes())?;
    writer.write_all(&entry.num_material1.to_le_bytes())?;
    writer.write_all(&entry.padding1)?;
    writer.write_all(&entry.upgrade_material2.to_le_bytes())?;
    writer.write_all(&entry.num_material2.to_le_bytes())?;
    writer.write_all(&entry.padding2)?;
    writer.write_all(&entry.upgrade_material3.to_le_bytes())?;
    writer.write_all(&entry.num_material3.to_le_bytes())?;
    writer.write_all(&entry.padding3)?;
    writer.write_all(&entry.upgrades_to1.to_le_bytes())?;
    writer.write_all(&entry.upgrades_to2.to_le_bytes())?;
    writer.write_all(&entry.upgrades_to3.to_le_bytes())?;
    writer.write_all(&entry.upgrades_to4.to_le_bytes())?;
    writer.write_all(&entry.padding4)?;
    Ok(())
}

pub fn write_evo_upgrade(writer: &mut impl Write, entry: &EvoUpgrade) -> Result<()> {
    writer.write_all(&entry.g_cost.to_le_bytes())?;
    writer.write_all(&entry.unk04.to_le_bytes())?;
    writer.write_all(&entry.level.to_le_bytes())?;
    
    for sub in &entry.sub {
        writer.write_all(&sub.dmg.to_le_bytes())?;
        writer.write_all(&sub.unk_sub2.to_le_bytes())?;
        writer.write_all(&sub.ele_damage.to_le_bytes())?;
        writer.write_all(&sub.padding1)?;
        writer.write_all(&sub.unk_sub8.to_le_bytes())?;
        writer.write_all(&sub.padding2)?;
    }
    
    writer.write_all(&entry.unk8c.to_le_bytes())?;
    writer.write_all(&[entry.unk8e])?;
    writer.write_all(&entry.padding1)?;
    writer.write_all(&entry.unk90.to_le_bytes())?;
    writer.write_all(&entry.padding2)?;
    writer.write_all(&entry.unk94.to_le_bytes())?;
    writer.write_all(&entry.unk98.to_le_bytes())?;
    writer.write_all(&entry.unk9c.to_le_bytes())?;
    writer.write_all(&entry.unka0.to_le_bytes())?;
    writer.write_all(&entry.unka4.to_le_bytes())?;
    writer.write_all(&entry.unka8.to_le_bytes())?;
    writer.write_all(&entry.unkaa.to_le_bytes())?;
    writer.write_all(&[entry.unkac])?;
    writer.write_all(&[entry.unkad])?;
    writer.write_all(&entry.padding3)?;
    Ok(())
}

/// Serialize MW upgrade paths followed by sentinel (upgrade_material1 = 0xFFFF)
pub fn write_mw_upgrades_block(entries: &[MWUpgradePath]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    for e in entries {
        write_mw_upgrade_path(&mut data, e)?;
    }
    let mut sentinel = MWUpgradePath::default();
    sentinel.upgrade_material1 = 0xFFFF;
    write_mw_upgrade_path(&mut data, &sentinel)?;
    Ok(data)
}

/// Serialize RW upgrade paths followed by sentinel (upgrade_material1 = 0xFFFF)
pub fn write_rw_upgrades_block(entries: &[RWUpgradePath]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    for e in entries {
        write_rw_upgrade_path(&mut data, e)?;
    }
    let mut sentinel = RWUpgradePath::default();
    sentinel.upgrade_material1 = 0xFFFF;
    write_rw_upgrade_path(&mut data, &sentinel)?;
    Ok(data)
}

pub fn extract_ranged_weapon_names<R: Read + Seek>(
    reader: &mut R,
    names_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<String>> {
    reader.seek(SeekFrom::Start(names_ptr as u64))?;
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    reader.seek(SeekFrom::Start(table_offset as u64))?;
    let mut names = Vec::with_capacity(count);
    for i in 0..count {
        let mut ptr_buf = [0u8; 4];
        reader.read_exact(&mut ptr_buf)?;
        let str_offset = u32::from_le_bytes(ptr_buf);
        
        if str_offset == 0 {
            names.push(String::new());
            continue;
        }
        
        // Sauvegarder la position actuelle
        let cur = reader.seek(SeekFrom::Current(0))?;
        
        // Aller à l'offset de la chaîne
        reader.seek(SeekFrom::Start(str_offset as u64))?;
        
        // Lire les bytes jusqu'au null terminator
        let mut bytes = Vec::new();
        let mut b = [0u8; 1];
        while reader.read_exact(&mut b).is_ok() && b[0] != 0 {
            bytes.push(b[0]);
        }
        
        // Nettoyer et décoder les bytes
        let cleaned_bytes = clean_shift_jis_bytes(&bytes);
        let (cow, _, _) = SHIFT_JIS.decode(&cleaned_bytes);
        names.push(cow.to_string());
        
        // Retourner à la position précédente
        reader.seek(SeekFrom::Start(cur))?;
    }
    Ok(names)
}

pub fn write_ranged_weapon_names<W: Write + Seek>(writer: &mut W, names: &[String]) -> Result<u32> {
    let mut name_offsets = Vec::new();
    let mut name_data = Vec::new();
    for name in names {
        let offset = name_data.len() as u32;
        name_offsets.push(offset);
        let (sjis_bytes, _, had_errors) = SHIFT_JIS.encode(name);
        if had_errors {
            println!("Warning: Shift-JIS encoding had errors for name: {}", name);
        }
        let mut valid_bytes = Vec::new();
        let mut i = 0;
        while i < sjis_bytes.len() {
            let b = sjis_bytes[i];
            if is_valid_shift_jis_byte(b) {
                if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xEF) {
                    if i + 1 < sjis_bytes.len() {
                        let b2 = sjis_bytes[i + 1];
                        if (b2 >= 0x40 && b2 <= 0xFC) && b2 != 0x7F {
                            valid_bytes.push(b);
                            valid_bytes.push(b2);
                            i += 2;
                            continue;
                        }
                    }
                } else {
                    valid_bytes.push(b);
                }
            }
            i += 1;
        }
        
        name_data.extend_from_slice(&valid_bytes);
        name_data.push(0);
    }
    let table_offset = writer.seek(SeekFrom::End(0))? as u32;
    for offset in &name_offsets {
        writer.write_all(&(offset + table_offset + (names.len() as u32 * 4)).to_le_bytes())?;
    }
    writer.write_all(&name_data)?;
    
    Ok(table_offset)
}

pub fn read_ranged_weapons_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<MhfdatRangedWeapon>> {
    let mut weapons = Vec::new();
    reader.seek(SeekFrom::Start(offset))?;
    loop {
        let weapon = read_ranged_weapon(reader)?;
        if weapon.model_id == 0xFFFF {
            break;
        }
        weapons.push(weapon);
    }
    Ok(weapons)
}

pub fn parse_ranged_weapons(buffer: &[u8]) -> Vec<MhfdatRangedWeapon> {
    use std::io::Cursor;
    if let Some((_, ranged_offset)) = read_mhfdat_offsets(buffer) {
        let mut cursor = Cursor::new(buffer);
        match read_ranged_weapons_until_sentinel(&mut cursor, ranged_offset as u64) {
            Ok(weapons) => weapons,
            Err(_) => vec![]
        }
    } else {
        vec![]
    }
}

pub fn write_ranged_weapon(writer: &mut impl Write, weapon: &MhfdatRangedWeapon) -> Result<()> {
    writer.write_all(&weapon.model_id.to_le_bytes())?;
    writer.write_all(&[weapon.rarity])?;
    writer.write_all(&[weapon.max_slots_maybe])?;
    writer.write_all(&[weapon.class_id])?;
    writer.write_all(&[weapon.unk05])?;
    writer.write_all(&[weapon.equip_type])?;
    writer.write_all(&[weapon.unk07])?;
    writer.write_all(&[weapon.unk08])?;
    writer.write_all(&[weapon.unk09])?;
    writer.write_all(&[weapon.unk11])?;
    writer.write_all(&[weapon.unk12])?;
    writer.write_all(&weapon.weapon_type.to_le_bytes())?;
    writer.write_all(&weapon.unk10.to_le_bytes())?;
    writer.write_all(&weapon.zenny_cost.to_le_bytes())?;
    writer.write_all(&weapon.raw_damage.to_le_bytes())?;
    writer.write_all(&weapon.defense.to_le_bytes())?;
    writer.write_all(&[weapon.recoil])?;
    writer.write_all(&[weapon.slots])?;
    writer.write_all(&[weapon.affinity as u8])?;
    writer.write_all(&[weapon.sort_order_maybe])?;
    writer.write_all(&[weapon.weapon_attribute])?;
    writer.write_all(&[weapon.element_id])?;
    writer.write_all(&[weapon.ele_damage])?;
    writer.write_all(&[weapon.reload])?;
    writer.write_all(&weapon.unk24.to_le_bytes())?;
    writer.write_all(&weapon.unk26.to_le_bytes())?;
    writer.write_all(&weapon.bullet.to_le_bytes())?;
    writer.write_all(&weapon.tower_g50_param_id.to_le_bytes())?;
    writer.write_all(&weapon.unk2e.to_le_bytes())?;
    writer.write_all(&[weapon.g_rank])?;
    writer.write_all(&[weapon.unk32])?;
    writer.write_all(&[weapon.unk34])?;
    writer.write_all(&[weapon.zero_f])?;
    writer.write_all(&weapon.unk38.to_le_bytes())?;
    writer.write_all(&weapon.zenith_skill.to_le_bytes())?;
    writer.write_all(&weapon.unk42.to_le_bytes())?;
    Ok(())
}

pub fn read_equipment<R: Read>(r: &mut R) -> Result<MhfdatEquipment> {
    Ok(MhfdatEquipment {
        model_id_male: r.read_u16::<LittleEndian>()?,
        model_id_female: r.read_u16::<LittleEndian>()?,
        equipable_by: r.read_u8()?,
        rarity: r.read_u8()?,
        max_level: r.read_u8()?,
        unk07: r.read_u8()?,
        unk08: r.read_u16::<LittleEndian>()?,
        unk0A: r.read_u16::<LittleEndian>()?,
        zenny_cost: r.read_u32::<LittleEndian>()?,
        unk10: r.read_u16::<LittleEndian>()?,
        base_defense: r.read_u16::<LittleEndian>()?,
        fire_res: r.read_i8()?,
        water_res: r.read_i8()?,
        thunder_res: r.read_i8()?,
        dragon_res: r.read_i8()?,
        ice_res: r.read_i8()?,
        coef_upgrade: r.read_u8()?,
        unk1A: r.read_u8()?,
        base_slots: r.read_u8()?,
        max_slots: r.read_u8()?,
        post_festi: r.read_u8()?,
        show_next_level: r.read_u16::<LittleEndian>()?,
        equip_id: r.read_u16::<LittleEndian>()?,
        unk22: r.read_u16::<LittleEndian>()?,
        unk24: r.read_u32::<LittleEndian>()?,
        unk28: r.read_u16::<LittleEndian>()?,
        skill_id1: r.read_u8()?,
        skill_pts1: r.read_i8()?,
        skill_id2: r.read_u8()?,
        skill_pts2: r.read_i8()?,
        skill_id3: r.read_u8()?,
        skill_pts3: r.read_i8()?,
        skill_id4: r.read_u8()?,
        skill_pts4: r.read_i8()?,
        skill_id5: r.read_u8()?,
        skill_pts5: r.read_i8()?,
        armor_type: r.read_u32::<LittleEndian>()?,
        weap_hiden: r.read_u16::<LittleEndian>()?,
        deco_item_id: r.read_u16::<LittleEndian>()?,
        towerslots: r.read_u16::<LittleEndian>()?,
        g_rank: r.read_u8()?,
        zero_f: r.read_u8()?,
        app_price: r.read_u32::<LittleEndian>()?,
        unk44: r.read_u16::<LittleEndian>()?,
        zenith_skill: r.read_u16::<LittleEndian>()?,
    })
}

pub fn write_equipment<W: Write>(w: &mut W, eq: &MhfdatEquipment) -> Result<()> {
    use byteorder::WriteBytesExt;
    
    w.write_u16::<LittleEndian>(eq.model_id_male)?;
    w.write_u16::<LittleEndian>(eq.model_id_female)?;
    w.write_u8(eq.equipable_by)?;
    w.write_u8(eq.rarity)?;
    w.write_u8(eq.max_level)?;
    w.write_u8(eq.unk07)?;
    w.write_u16::<LittleEndian>(eq.unk08)?;
    w.write_u16::<LittleEndian>(eq.unk0A)?;
    w.write_u32::<LittleEndian>(eq.zenny_cost)?;
    w.write_u16::<LittleEndian>(eq.unk10)?;
    w.write_u16::<LittleEndian>(eq.base_defense)?;
    w.write_i8(eq.fire_res)?;
    w.write_i8(eq.water_res)?;
    w.write_i8(eq.thunder_res)?;
    w.write_i8(eq.dragon_res)?;
    w.write_i8(eq.ice_res)?;
    w.write_u8(eq.coef_upgrade)?;
    w.write_u8(eq.unk1A)?;
    w.write_u8(eq.base_slots)?;
    w.write_u8(eq.max_slots)?;
    w.write_u8(eq.post_festi)?;
    w.write_u16::<LittleEndian>(eq.show_next_level)?;
    w.write_u16::<LittleEndian>(eq.equip_id)?;
    w.write_u16::<LittleEndian>(eq.unk22)?;
    w.write_u32::<LittleEndian>(eq.unk24)?;
    w.write_u16::<LittleEndian>(eq.unk28)?;
    w.write_u8(eq.skill_id1)?;
    w.write_i8(eq.skill_pts1)?;
    w.write_u8(eq.skill_id2)?;
    w.write_i8(eq.skill_pts2)?;
    w.write_u8(eq.skill_id3)?;
    w.write_i8(eq.skill_pts3)?;
    w.write_u8(eq.skill_id4)?;
    w.write_i8(eq.skill_pts4)?;
    w.write_u8(eq.skill_id5)?;
    w.write_i8(eq.skill_pts5)?;
    w.write_u32::<LittleEndian>(eq.armor_type)?;
    w.write_u16::<LittleEndian>(eq.weap_hiden)?;
    w.write_u16::<LittleEndian>(eq.deco_item_id)?;
    w.write_u16::<LittleEndian>(eq.towerslots)?;
    w.write_u8(eq.g_rank)?;
    w.write_u8(eq.zero_f)?;
    w.write_u32::<LittleEndian>(eq.app_price)?;
    w.write_u16::<LittleEndian>(eq.unk44)?;
    w.write_u16::<LittleEndian>(eq.zenith_skill)?;
    Ok(())
}

pub fn read_equipments_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<MhfdatEquipment>> {
    let mut armors = Vec::new();
    reader.seek(SeekFrom::Start(offset))?;
    loop {
        let armor = read_equipment(reader)?;
        if armor.model_id_male == 0xFFFF {
            break;
        }
        armors.push(armor);
    }
    Ok(armors)
}

/// Bytes of the NUL-terminated string at `offset` (the NUL excluded; a missing terminator ends at EOF).
/// Reads in 128-byte chunks: the byte-at-a-time `read_exact` loop it replaces made reading the 68 730
/// armor descriptions take ~180 ms.
fn read_c_string_bytes<R: Read + Seek>(reader: &mut R, offset: u64) -> std::io::Result<Vec<u8>> {
    reader.seek(SeekFrom::Start(offset))?;
    let mut out = Vec::new();
    let mut chunk = [0u8; 128];
    loop {
        let n = reader.read(&mut chunk)?;
        if n == 0 {
            break;
        }
        if let Some(end) = chunk[..n].iter().position(|&b| b == 0) {
            out.extend_from_slice(&chunk[..end]);
            break;
        }
        out.extend_from_slice(&chunk[..n]);
    }
    Ok(out)
}

/// Extraction des noms d'armures (head, chest, arms, waist, legs) via table de pointeurs, même format que pour les armes
pub fn extract_armor_names<R: Read + Seek>(
    reader: &mut R,
    names_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<String>> {
    reader.seek(SeekFrom::Start(names_ptr as u64))?;
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    reader.seek(SeekFrom::Start(table_offset as u64))?;
    let mut names = Vec::with_capacity(count);
    for _ in 0..count {
        let mut ptr_buf = [0u8; 4];
        reader.read_exact(&mut ptr_buf)?;
        let str_offset = u32::from_le_bytes(ptr_buf);
        
        if str_offset == 0 {
            names.push(String::new());
            continue;
        }
        let cur = reader.seek(SeekFrom::Current(0))?;
        let bytes = read_c_string_bytes(reader, str_offset as u64)?;
        let cleaned_bytes = clean_shift_jis_bytes(&bytes);
        let (cow, _, _) = SHIFT_JIS.decode(&cleaned_bytes);
        names.push(cow.to_string());
        reader.seek(SeekFrom::Start(cur))?;
    }
    Ok(names)
}

/// Extraction des descriptions d'armures (4 pointeurs par entrée, 3 utilisés pour le texte, 1 toujours 0x00000000) via table de pointeurs, même format que pour les armes
pub fn extract_armor_descriptions<R: Read + Seek>(
    reader: &mut R,
    desc_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<[String; 4]>> {
    // Aller à la table de pointeurs principale
    reader.seek(SeekFrom::Start(desc_ptr as u64))?;
    // Lire le pointeur vers la vraie table de pointeurs
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    reader.seek(SeekFrom::Start(table_offset as u64))?;
    // Pour chaque entrée (armure)
    let mut all_descs = Vec::with_capacity(count);
    for _ in 0..count {
        // Lire 4 pointeurs
        let mut field_ptrs = [0u32; 4];
        for i in 0..4 {
            let mut ptr_buf = [0u8; 4];
            reader.read_exact(&mut ptr_buf)?;
            field_ptrs[i] = u32::from_le_bytes(ptr_buf);
        }
        // Pour chaque champ, lire la chaîne (le 4ème pointeur est toujours 0x00000000)
        let mut descs = [String::new(), String::new(), String::new(), String::new()];
        for (i, &str_offset) in field_ptrs.iter().enumerate() {
            if str_offset == 0 {
                descs[i] = String::new();
                continue;
            }
            let cur = reader.seek(SeekFrom::Current(0))?;
            let bytes = read_c_string_bytes(reader, str_offset as u64)?;
            let cleaned_bytes = clean_shift_jis_bytes(&bytes);
            let (cow, _, _) = SHIFT_JIS.decode(&cleaned_bytes);
            descs[i] = cow.to_string();
            reader.seek(SeekFrom::Start(cur))?;
        }
        all_descs.push(descs);
    }
    Ok(all_descs)
}

/// Value stored in header slot `slot` (a file offset), or None when the slot is outside the buffer.
pub fn read_header_pointer(buffer: &[u8], slot: u32) -> Option<u32> {
    let at = slot as usize;
    Some(u32::from_le_bytes(buffer.get(at..at + 4)?.try_into().ok()?))
}

/// File offset of the u16 count at `important_nums + inner`; important_nums is found through its header slot.
pub fn important_count_offset(buffer: &[u8], inner: usize) -> Option<usize> {
    let base = read_header_pointer(buffer, IMPORTANT_NUMS_PTR)? as usize;
    let at = base.checked_add(inner)?;
    buffer.get(at..at + 2)?;
    Some(at)
}

pub fn read_important_count(buffer: &[u8], inner: usize) -> Option<u16> {
    let at = important_count_offset(buffer, inner)?;
    Some(u16::from_le_bytes([buffer[at], buffer[at + 1]]))
}

/// Ok when `count` rows of `stride` bytes starting at `offset` lie inside a file of `file_len` bytes.
/// The message quotes the offset and both sizes so a refusal can be matched to a file.
pub fn check_table_fits(name: &str, offset: u64, stride: usize, count: usize, file_len: usize) -> std::result::Result<(), String> {
    let end = (stride as u64).checked_mul(count as u64).and_then(|n| n.checked_add(offset));
    match end {
        Some(end) if end <= file_len as u64 => Ok(()),
        _ => Err(format!(
            "{name}: {count} rows of {stride} bytes at offset 0x{offset:X} do not fit in the {file_len}-byte file"
        )),
    }
}

/// mhfdat starts with u32 0x1A66686D, 0x59, padding, 0x0BC8 (little endian in the file). A wrong first word means
/// this is not an mhfdat (Err); the other two words differ between builds (the Vita file has 0x0BC4), so they only
/// give warnings.
pub fn check_mhfdat_signature(buffer: &[u8]) -> std::result::Result<Vec<String>, String> {
    let word = |at: usize| buffer.get(at..at + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
    let show = |w: Option<u32>| w.map(|v| format!("0x{v:08X}")).unwrap_or_else(|| "<missing>".to_string());
    if word(0) != Some(0x1A66_686D) {
        return Err(format!(
            "not a clear mhfdat.bin: the first word is {}, expected 0x1A66686D (bytes 6D 68 66 1A). The file may still be \
             compressed/encrypted, or be a different kind of file",
            show(word(0))
        ));
    }
    let mut warnings = Vec::new();
    if word(4) != Some(0x59) {
        warnings.push(format!("signature word at 0x4 is {} (expected 0x00000059)", show(word(4))));
    }
    if word(12) != Some(0x0BC8) {
        warnings.push(format!("signature word at 0xC is {} (expected 0x00000BC8): a different build?", show(word(12))));
    }
    Ok(warnings)
}

/// Equipment counts: header slot 0xE8 holds a *pointer* to 8 x u16 (legs, type-1 hole, head, chest, arms, waist,
/// melee, ranged). The slot itself is not the block.
pub fn equipment_counts_offset(buffer: &[u8]) -> Option<usize> {
    let at = read_header_pointer(buffer, EQUIPEMENT_COUNT_PTR)? as usize;
    if at == 0 {
        return None;
    }
    buffer.get(at..at.checked_add(16)?)?;
    Some(at)
}

pub fn read_equipment_counts(buffer: &[u8]) -> Option<EquipmentCounts> {
    let at = equipment_counts_offset(buffer)?;
    let b = buffer.get(at..at + 16)?;
    let w = |i: usize| u16::from_le_bytes([b[i * 2], b[i * 2 + 1]]);
    Some(EquipmentCounts {
        numLegA: w(0),
        numUnk: w(1),
        numHeadA: w(2),
        numBodyA: w(3),
        numArmA: w(4),
        numWaistA: w(5),
        numMeleeW: w(6),
        numRangedW: w(7),
    })
}

/// Writes the counts into the block behind header slot 0xE8. Returns false (and writes nothing) when
/// the pointer is null or the block does not fit in the buffer.
pub fn write_equipment_counts(buffer: &mut [u8], counts: &EquipmentCounts) -> bool {
    let Some(at) = equipment_counts_offset(buffer) else { return false };
    let values = [
        counts.numLegA, counts.numUnk, counts.numHeadA, counts.numBodyA,
        counts.numArmA, counts.numWaistA, counts.numMeleeW, counts.numRangedW,
    ];
    for (i, v) in values.iter().enumerate() {
        buffer[at + i * 2..at + i * 2 + 2].copy_from_slice(&v.to_le_bytes());
    }
    true
}

pub fn write_data_with_padding<W: Seek + Write>(writer: &mut W, data: &[u8]) -> Result<()> {
    // Write 20 bytes of padding
    let padding = [0u8; 20];
    writer.write_all(&padding)?;
    
    // Write the actual data
    writer.write_all(data)?;
    
    Ok(())
}

pub fn write_equipment_data(melee_weapons: &[MhfdatMeleeWeapon], ranged_weapons: &[MhfdatRangedWeapon]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    
    // Write melee weapons
    for weapon in melee_weapons {
        write_melee_weapon(&mut data, weapon)?;
    }
    // Add sentinel
    let sentinel = unsafe { std::ptr::read_unaligned(&MhfdatMeleeWeapon {
        model_id: 0xFFFF,
        ..Default::default()
    }) };
    write_melee_weapon(&mut data, &sentinel)?;

    // Write ranged weapons
    for weapon in ranged_weapons {
        write_ranged_weapon(&mut data, weapon)?;
    }
    // Add sentinel
    let sentinel = unsafe { std::ptr::read_unaligned(&MhfdatRangedWeapon {
        model_id: 0xFFFF,
        ..Default::default()
    }) };
    write_ranged_weapon(&mut data, &sentinel)?;

    Ok(data)
}

/// Serialize melee weapons followed by a 0xFFFF sentinel
pub fn write_melee_weapons_block(weapons: &[MhfdatMeleeWeapon]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    for weapon in weapons {
        write_melee_weapon(&mut data, weapon)?;
    }
    // Sentinel entry with model_id = 0xFFFF
    let mut sentinel = MhfdatMeleeWeapon::default();
    sentinel.model_id = 0xFFFF;
    write_melee_weapon(&mut data, &sentinel)?;
    Ok(data)
}

/// Serialize ranged weapons followed by a 0xFFFF sentinel
pub fn write_ranged_weapons_block(weapons: &[MhfdatRangedWeapon]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    for weapon in weapons {
        write_ranged_weapon(&mut data, weapon)?;
    }
    // Sentinel entry with model_id = 0xFFFF
    let mut sentinel = MhfdatRangedWeapon::default();
    sentinel.model_id = 0xFFFF;
    write_ranged_weapon(&mut data, &sentinel)?;
    Ok(data)
}

/// Write a single RegAUpgradeRow to a writer
// Armor upgrade writing helpers removed per feature removal

pub fn save<W: Write + Seek>(
    writer: &mut W,
    melee_weapons: &[MhfdatMeleeWeapon],
    ranged_weapons: &[MhfdatRangedWeapon],
) -> Result<()> {
    // Write equipment data with padding
    write_data_with_padding(writer, &write_equipment_data(melee_weapons, ranged_weapons)?)?;

    // Write melee weapon names
    let mut melee_names_data = Vec::new();
    let mut melee_names_cursor = Cursor::new(&mut melee_names_data);
    let melee_names: Vec<String> = melee_weapons.iter()
        .map(|w| {
            let model_id = unsafe { std::ptr::read_unaligned(w as *const _ as *const u16) };
            format!("Weapon {}", model_id)
        })
        .collect();
    write_weapon_names(&mut melee_names_cursor, &melee_names)?;
    write_data_with_padding(writer, &melee_names_data)?;

    // Write ranged weapon names
    let mut ranged_names_data = Vec::new();
    let mut ranged_names_cursor = Cursor::new(&mut ranged_names_data);
    let ranged_names: Vec<String> = ranged_weapons.iter()
        .map(|w| {
            let model_id = unsafe { std::ptr::read_unaligned(w as *const _ as *const u16) };
            format!("Weapon {}", model_id)
        })
        .collect();
    write_ranged_weapon_names(&mut ranged_names_cursor, &ranged_names)?;
    write_data_with_padding(writer, &ranged_names_data)?;

    Ok(())
}

pub fn write_armor_data(head_armors: &[MhfdatEquipment], body_armors: &[MhfdatEquipment], 
                       arms_armors: &[MhfdatEquipment], waist_armors: &[MhfdatEquipment], 
                       legs_armors: &[MhfdatEquipment]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    
    // Write head armors
    for armor in head_armors {
        write_equipment(&mut data, armor)?;
    }
    // Add sentinel for head armors
    let sentinel = MhfdatEquipment {
        model_id_male: 0xFFFF,
        model_id_female: 0xFFFF,
        ..Default::default()
    };
    write_equipment(&mut data, &sentinel)?;

    // Write body armors
    for armor in body_armors {
        write_equipment(&mut data, armor)?;
    }
    write_equipment(&mut data, &sentinel)?;

    // Write arms armors
    for armor in arms_armors {
        write_equipment(&mut data, armor)?;
    }
    write_equipment(&mut data, &sentinel)?;

    // Write waist armors
    for armor in waist_armors {
        write_equipment(&mut data, armor)?;
    }
    write_equipment(&mut data, &sentinel)?;

    // Write legs armors
    for armor in legs_armors {
        write_equipment(&mut data, armor)?;
    }
    write_equipment(&mut data, &sentinel)?;

    Ok(data)
}

/// Serialize a single armor section followed by a sentinel (model_id_male and model_id_female = 0xFFFF)
pub fn write_armors_block(armors: &[MhfdatEquipment]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    // Only write armors that are not sentinels or dummy entries
    for armor in armors {
        // Skip sentinel entries (0xFFFF)
        if armor.model_id_male == 0xFFFF && armor.model_id_female == 0xFFFF {
            continue;
        }
        
        // Skip dummy entries (all zeros) - these cause ID 0 in game
        let is_dummy = armor.model_id_male == 0x0000
            && armor.model_id_female == 0x0000
            && armor.rarity == 0x00
            && armor.equipable_by == 0x00
            && armor.zenny_cost == 0x00000000
            && armor.base_defense == 0x0000
            && armor.zenith_skill == 0x0000;
        
        if is_dummy {
            continue;
        }
        
        write_equipment(&mut data, armor)?;
    }
    // Add the sentinel at the end
    let sentinel = MhfdatEquipment {
        model_id_male: 0xFFFF,
        model_id_female: 0xFFFF,
        ..Default::default()
    };
    write_equipment(&mut data, &sentinel)?;
    Ok(data)
}

pub fn write_armor_names<W: Write + Seek>(writer: &mut W, names: &[String]) -> Result<u32> {
    let mut name_offsets = Vec::new();
    let mut name_data = Vec::new();
    for name in names {
        let offset = name_data.len() as u32;
        name_offsets.push(offset);
        let (sjis_bytes, _, had_errors) = SHIFT_JIS.encode(name);
        if had_errors {
            println!("Warning: Shift-JIS encoding had errors for armor name: {}", name);
        }
        let mut valid_bytes = Vec::new();
        let mut i = 0;
        while i < sjis_bytes.len() {
            let b = sjis_bytes[i];
            if is_valid_shift_jis_byte(b) {
                if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xEF) {
                    if i + 1 < sjis_bytes.len() {
                        let b2 = sjis_bytes[i + 1];
                        if (b2 >= 0x40 && b2 <= 0xFC) && b2 != 0x7F {
                            valid_bytes.push(b);
                            valid_bytes.push(b2);
                            i += 2;
                            continue;
                        }
                    }
                } else {
                    valid_bytes.push(b);
                }
            }
            i += 1;
        }
        
        name_data.extend_from_slice(&valid_bytes);
        name_data.push(0);
    }
    let table_offset = writer.seek(SeekFrom::End(0))? as u32;
    for offset in &name_offsets {
        writer.write_all(&(offset + table_offset + (names.len() as u32 * 4)).to_le_bytes())?;
    }
    writer.write_all(&name_data)?;
    
    Ok(table_offset)
}

pub fn write_transmog_data(transmog_entries: &[ShopEntry]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    
    for entry in transmog_entries {
        write_shop_entry(&mut data, entry)?;
    }
    
    // Add sentinel entry
    let sentinel = ShopEntry {
        equip_type: 0xFF, // Invalid type to mark end
        ..Default::default()
    };
    write_shop_entry(&mut data, &sentinel)?;
    
    Ok(data)
}

pub fn write_zenith_data(zenith_entries: &[ShopEntry]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    
    for entry in zenith_entries {
        write_shop_entry(&mut data, entry)?;
    }
    
    // Add sentinel entry
    let sentinel = ShopEntry {
        equip_type: 0xFF, // Invalid type to mark end
        ..Default::default()
    };
    write_shop_entry(&mut data, &sentinel)?;
    
    Ok(data)
}

pub fn write_armor_descriptions<W: Write + Seek>(writer: &mut W, descriptions: &[[String; 4]]) -> Result<u32> {
    // Absolute offset where the table starts. Header slot 0x78 points straight at it (the original file
    // has no extra pointer in front of the rows, and the reader treats the slot value as the table).
    let base_offset = writer.seek(SeekFrom::End(0))? as u32;

    let mut field_offsets = Vec::new();
    let mut strings_data = Vec::new();

    // Table size: descriptions.len() * 16 (4 pointers * 4 bytes per entry); strings follow it
    let table_size = (descriptions.len() * 16) as u32;
    let strings_start = base_offset + table_size;
    let mut current_string_offset: u32 = 0;
    
    for desc in descriptions {
        let mut entry_offsets = Vec::new();
        // First 3 fields contain text (same as extract: 4 ptrs per entry, 4th is always 0)
        for j in 0..3 {
            let absolute_ptr = if desc[j].is_empty() {
                0u32
            } else {
                strings_start + current_string_offset
            };
            entry_offsets.push(absolute_ptr);
            let encoded = SHIFT_JIS.encode(&desc[j]).0;
            strings_data.extend_from_slice(&encoded);
            strings_data.push(0);
            current_string_offset += encoded.len() as u32 + 1;
        }
        // 4th pointer is always 0x00000000
        entry_offsets.push(0u32);
        field_offsets.extend(entry_offsets);
    }
    
    // Pointer table: 4 absolute pointers per entry
    for i in 0..descriptions.len() {
        for j in 0..4 {
            writer.write_all(&field_offsets[i * 4 + j].to_le_bytes())?;
        }
    }
    
    // Actual strings
    writer.write_all(&strings_data)?;
    
    Ok(base_offset)
}

// Item reading functions
pub fn read_item<R: Read>(r: &mut R) -> Result<MhfdatItem> {
    Ok(MhfdatItem {
        unk00: r.read_u8()?,
        unk01: r.read_u8()?,
        rarity: r.read_u8()?,
        max_stack: r.read_u8()?,
        unk04: r.read_u8()?,
        icon: r.read_u8()?,
        icon_color: r.read_u8()?,
        unk07: r.read_u8()?,
        bottle: r.read_u16::<LittleEndian>()?,
        unk0A: r.read_u16::<LittleEndian>()?,
        buy_price: r.read_u32::<LittleEndian>()?,
        sell_price: r.read_u32::<LittleEndian>()?,
        item_type: r.read_u16::<LittleEndian>()?,
        deco_id: r.read_u16::<LittleEndian>()?,
        unk18: r.read_u16::<LittleEndian>()?,
        unk1A: r.read_u8()?,
        unk1B: r.read_u8()?,
        equip_type: r.read_u16::<LittleEndian>()?,
        is_gz: r.read_u8()?,
        unk1F: r.read_u8()?,
        unk20: r.read_u16::<LittleEndian>()?,
        unk22: r.read_u16::<LittleEndian>()?,
    })
}

pub fn read_item_offsets(buffer: &[u8]) -> Option<(u32, u32, u32)> {
    if buffer.len() < 0x108 {
        return None;
    }
    let data_offset = u32::from_le_bytes(buffer.get(0xFC..0x100)?.try_into().ok()?);
    let names_offset = u32::from_le_bytes(buffer.get(0x100..0x104)?.try_into().ok()?);
    let desc_offset = u32::from_le_bytes(buffer.get(0x104..0x108)?.try_into().ok()?);
    Some((data_offset, names_offset, desc_offset))
}

const MAX_ITEMS: usize = 16700;

pub fn read_items_until_sentinel<R: Read + Seek>(reader: &mut R, offset: u64) -> Result<Vec<MhfdatItem>> {
    let mut items = Vec::new();
    reader.seek(SeekFrom::Start(offset))?;
    loop {
        let item = read_item(reader)?;
        // Check for sentinel: if unk00 and unk01 are both 0xFF
        if item.unk00 == 0xFF && item.unk01 == 0xFF {
            break;
        }
        items.push(item);
        
        // Limit maximum number of items to prevent memory issues
        if items.len() >= MAX_ITEMS {
            break;
        }
    }
    Ok(items)
}

pub fn parse_items(buffer: &[u8]) -> Vec<MhfdatItem> {
    use std::io::Cursor;
    if let Some((data_offset, _, _)) = read_item_offsets(buffer) {
        let mut cursor = Cursor::new(buffer);
        match read_items_until_sentinel(&mut cursor, data_offset as u64) {
            Ok(items) => items,
            Err(_) => vec![]
        }
    } else {
        vec![]
    }
}

/// Write a single item entry in the same order as read_item
pub fn write_item(writer: &mut impl Write, item: &MhfdatItem) -> Result<()> {
    writer.write_all(&[item.unk00])?;
    writer.write_all(&[item.unk01])?;
    writer.write_all(&[item.rarity])?;
    writer.write_all(&[item.max_stack])?;
    writer.write_all(&[item.unk04])?;
    writer.write_all(&[item.icon])?;
    writer.write_all(&[item.icon_color])?;
    writer.write_all(&[item.unk07])?;
    writer.write_all(&item.bottle.to_le_bytes())?;
    writer.write_all(&item.unk0A.to_le_bytes())?;
    writer.write_all(&item.buy_price.to_le_bytes())?;
    writer.write_all(&item.sell_price.to_le_bytes())?;
    writer.write_all(&item.item_type.to_le_bytes())?;
    writer.write_all(&item.deco_id.to_le_bytes())?;
    writer.write_all(&item.unk18.to_le_bytes())?;
    writer.write_all(&[item.unk1A])?;
    writer.write_all(&[item.unk1B])?;
    writer.write_all(&item.equip_type.to_le_bytes())?;
    writer.write_all(&[item.is_gz])?;
    writer.write_all(&[item.unk1F])?;
    writer.write_all(&item.unk20.to_le_bytes())?;
    writer.write_all(&item.unk22.to_le_bytes())?;
    Ok(())
}

/// Serialize items followed by a 0xFFFF sentinel (first two bytes of an item set to 0xFF)
pub fn write_items_block(items: &[MhfdatItem]) -> Result<Vec<u8>> {
    // Enforce maximum item limit
    if items.len() > MAX_ITEMS {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Too many items: {} (maximum is {})", items.len(), MAX_ITEMS)
        ));
    }
    
    let mut data = Vec::new();
    for item in items {
        write_item(&mut data, item)?;
    }
    // Append sentinel: first two bytes 0xFF 0xFF, rest zero to match struct size (0x24)
    data.push(0xFF);
    data.push(0xFF);
    // Item struct size is 0x24; we already wrote 2 bytes, add 0x22 zero bytes
    data.extend_from_slice(&[0u8; 0x22]);
    Ok(data)
}

pub fn extract_item_names<R: Read + Seek>(
    reader: &mut R,
    names_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<String>> {
    reader.seek(SeekFrom::Start(names_ptr as u64))?;
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    reader.seek(SeekFrom::Start(table_offset as u64))?;
    let mut names = Vec::with_capacity(count);
    for _ in 0..count {
        let mut ptr_buf = [0u8; 4];
        reader.read_exact(&mut ptr_buf)?;
        let string_ptr = u32::from_le_bytes(ptr_buf);
        let current_pos = reader.seek(SeekFrom::Current(0))?;
        reader.seek(SeekFrom::Start(string_ptr as u64))?;
        let mut string_bytes = Vec::new();
        let mut byte = [0u8; 1];
        loop {
            reader.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            string_bytes.push(byte[0]);
        }
        
        // Convert from Shift-JIS to UTF-8
        let (cow, _, _) = SHIFT_JIS.decode(&string_bytes);
        let name = cow.into_owned();
        names.push(name);
        reader.seek(SeekFrom::Start(current_pos))?;
    }
    
    Ok(names)
}

pub fn extract_item_descriptions<R: Read + Seek>(
    reader: &mut R,
    desc_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<String>> {
    reader.seek(SeekFrom::Start(desc_ptr as u64))?;
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    reader.seek(SeekFrom::Start((table_offset + 0x60) as u64))?;
    let mut descriptions = Vec::with_capacity(count);
    for _ in 0..count {
        let mut ptr_buf = [0u8; 4];
        reader.read_exact(&mut ptr_buf)?;
        let string_ptr = u32::from_le_bytes(ptr_buf);
        let current_pos = reader.seek(SeekFrom::Current(0))?;
        reader.seek(SeekFrom::Start(string_ptr as u64))?;
        let mut string_bytes = Vec::new();
        let mut byte = [0u8; 1];
        loop {
            reader.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            string_bytes.push(byte[0]);
        }
        
        // Convert from Shift-JIS to UTF-8
        let (cow, _, _) = SHIFT_JIS.decode(&string_bytes);
        let description = cow.into_owned();
        descriptions.push(description);
        reader.seek(SeekFrom::Start(current_pos))?;
    }
    
    Ok(descriptions)
}

pub fn write_item_names<W: Write + Seek>(writer: &mut W, names: &[String]) -> Result<u32> {
    let mut name_offsets = Vec::new();
    let mut name_data = Vec::new();
    for name in names {
        let offset = name_data.len() as u32;
        name_offsets.push(offset);
        let (sjis_bytes, _, had_errors) = SHIFT_JIS.encode(name);
        if had_errors {
            println!("Warning: Shift-JIS encoding had errors for item name: {}", name);
        }
        let mut valid_bytes = Vec::new();
        let mut i = 0;
        while i < sjis_bytes.len() {
            let b = sjis_bytes[i];
            if is_valid_shift_jis_byte(b) {
                if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xEF) {
                    if i + 1 < sjis_bytes.len() {
                        let b2 = sjis_bytes[i + 1];
                        if (b2 >= 0x40 && b2 <= 0xFC) && b2 != 0x7F {
                            valid_bytes.push(b);
                            valid_bytes.push(b2);
                            i += 2;
                            continue;
                        }
                    }
                } else {
                    valid_bytes.push(b);
                }
            }
            i += 1;
        }
        
        name_data.extend_from_slice(&valid_bytes);
        name_data.push(0);
    }
    let table_offset = writer.seek(SeekFrom::End(0))? as u32;
    for offset in &name_offsets {
        writer.write_all(&(offset + table_offset + (names.len() as u32 * 4)).to_le_bytes())?;
    }
    writer.write_all(&name_data)?;
    
    Ok(table_offset)
}

/// `preface` = the 0x60 bytes (24 UI-message pointers) that sit in front of the description table in the
/// original file; the reader starts the table at `slot target + 0x60`.
pub fn write_item_descriptions<W: Write + Seek>(writer: &mut W, descriptions: &[String], preface: &[u8]) -> Result<u32> {
    let mut desc_offsets = Vec::new();
    let mut desc_data = Vec::new();
    for desc in descriptions {
        let offset = desc_data.len() as u32;
        desc_offsets.push(offset);
        let (sjis_bytes, _, had_errors) = SHIFT_JIS.encode(desc);
        if had_errors {
            println!("Warning: Shift-JIS encoding had errors for item description: {}", desc);
        }
        let mut valid_bytes = Vec::new();
        let mut i = 0;
        while i < sjis_bytes.len() {
            let b = sjis_bytes[i];
            if is_valid_shift_jis_byte(b) {
                if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xEF) {
                    if i + 1 < sjis_bytes.len() {
                        let b2 = sjis_bytes[i + 1];
                        if (b2 >= 0x40 && b2 <= 0xFC) && b2 != 0x7F {
                            valid_bytes.push(b);
                            valid_bytes.push(b2);
                            i += 2;
                            continue;
                        }
                    }
                } else {
                    valid_bytes.push(b);
                }
            }
            i += 1;
        }
        
        desc_data.extend_from_slice(&valid_bytes);
        desc_data.push(0);
    }
    
    // Item descriptions have a special structure: 0x60 bytes of existing pointers, then the table
    let start_offset = writer.seek(SeekFrom::End(0))? as u32;

    const PREFACE_SIZE: usize = 0x60;
    let mut head = preface.to_vec();
    head.resize(PREFACE_SIZE, 0);
    writer.write_all(&head)?;
        
    // Write the description offsets table (after padding)
    let table_offset = writer.seek(SeekFrom::End(0))? as u32;
    for offset in &desc_offsets {
        writer.write_all(&(offset + table_offset + (descriptions.len() as u32 * 4)).to_le_bytes())?;
    }
    writer.write_all(&desc_data)?;
    
    Ok(start_offset)
}

pub fn parse_item_names(buffer: &[u8], count: usize) -> Vec<String> {
    use std::io::Cursor;
    use crate::model::mhfdat_pointers::ITEM_NAMES_PTR;
    let mut cursor = Cursor::new(buffer);
    match extract_item_names(&mut cursor, ITEM_NAMES_PTR, count) {
        Ok(names) => names,
        Err(_) => vec![]
    }
}

pub fn parse_item_descriptions(buffer: &[u8], count: usize) -> Vec<String> {
    use std::io::Cursor;
    use crate::model::mhfdat_pointers::ITEM_DESC_PTR;
    let mut cursor = Cursor::new(buffer);
    match extract_item_descriptions(&mut cursor, ITEM_DESC_PTR, count) {
        Ok(descriptions) => descriptions,
        Err(_) => vec![]
    }
}

/// Extract monster descriptions via pointer table, same format as item names
pub fn extract_monster_descriptions<R: Read + Seek>(
    reader: &mut R,
    desc_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<String>> {
    reader.seek(SeekFrom::Start(desc_ptr as u64))?;
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    reader.seek(SeekFrom::Start(table_offset as u64))?;
    let mut descriptions = Vec::with_capacity(count);
    for _ in 0..count {
        let mut ptr_buf = [0u8; 4];
        reader.read_exact(&mut ptr_buf)?;
        let string_ptr = u32::from_le_bytes(ptr_buf);
        let current_pos = reader.seek(SeekFrom::Current(0))?;
        reader.seek(SeekFrom::Start(string_ptr as u64))?;
        let mut string_bytes = Vec::new();
        let mut byte = [0u8; 1];
        loop {
            reader.read_exact(&mut byte)?;
            if byte[0] == 0 {
                break;
            }
            string_bytes.push(byte[0]);
        }
        
        // Convert from Shift-JIS to UTF-8
        let (cow, _, _) = SHIFT_JIS.decode(&string_bytes);
        let description = cow.into_owned();
        descriptions.push(description);
        reader.seek(SeekFrom::Start(current_pos))?;
    }
    
    Ok(descriptions)
}

/// Write monster descriptions via pointer table, same format as item names
pub fn write_monster_descriptions<W: Write + Seek>(writer: &mut W, descriptions: &[String]) -> Result<u32> {
    let mut desc_offsets = Vec::new();
    let mut desc_data = Vec::new();
    for desc in descriptions {
        let offset = desc_data.len() as u32;
        desc_offsets.push(offset);
        let (sjis_bytes, _, had_errors) = SHIFT_JIS.encode(desc);
        if had_errors {
            println!("Warning: Shift-JIS encoding had errors for monster description: {}", desc);
        }
        let mut valid_bytes = Vec::new();
        let mut i = 0;
        while i < sjis_bytes.len() {
            let b = sjis_bytes[i];
            if is_valid_shift_jis_byte(b) {
                if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xEF) {
                    if i + 1 < sjis_bytes.len() {
                        let b2 = sjis_bytes[i + 1];
                        if (b2 >= 0x40 && b2 <= 0xFC) && b2 != 0x7F {
                            valid_bytes.push(b);
                            valid_bytes.push(b2);
                            i += 2;
                            continue;
                        }
                    }
                } else {
                    valid_bytes.push(b);
                }
            }
            i += 1;
        }
        
        desc_data.extend_from_slice(&valid_bytes);
        desc_data.push(0);
    }
    let table_offset = writer.seek(SeekFrom::End(0))? as u32;
    for offset in &desc_offsets {
        writer.write_all(&(offset + table_offset + (descriptions.len() as u32 * 4)).to_le_bytes())?;
    }
    writer.write_all(&desc_data)?;
    
    Ok(table_offset)
}

/// Write monster descriptions as a block (returns Vec<u8> instead of writing to writer)
/// `base` = absolute file offset where the returned block will be written (pointers are absolute).
pub fn write_monster_descriptions_block(descriptions: &[String], base: u32) -> Result<Vec<u8>> {
    let mut desc_offsets = Vec::new();
    let mut desc_data = Vec::new();
    for desc in descriptions {
        let offset = desc_data.len() as u32;
        desc_offsets.push(offset);
        let (sjis_bytes, _, had_errors) = SHIFT_JIS.encode(desc);
        if had_errors {
            println!("Warning: Shift-JIS encoding had errors for monster description: {}", desc);
        }
        let mut valid_bytes = Vec::new();
        let mut i = 0;
        while i < sjis_bytes.len() {
            let b = sjis_bytes[i];
            if is_valid_shift_jis_byte(b) {
                if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xEF) {
                    if i + 1 < sjis_bytes.len() {
                        let b2 = sjis_bytes[i + 1];
                        if (b2 >= 0x40 && b2 <= 0xFC) && b2 != 0x7F {
                            valid_bytes.push(b);
                            valid_bytes.push(b2);
                            i += 2;
                            continue;
                        }
                    }
                } else {
                    valid_bytes.push(b);
                }
            }
            i += 1;
        }
        
        desc_data.extend_from_slice(&valid_bytes);
        desc_data.push(0);
    }
    
    let ptr_table_size = descriptions.len() * 4;
    let mut data = Vec::new();
    
    // Write pointer table (absolute file offsets)
    for offset in &desc_offsets {
        let absolute_offset = base + (ptr_table_size as u32) + offset;
        data.extend_from_slice(&absolute_offset.to_le_bytes());
    }
    
    // Write description data
    data.extend_from_slice(&desc_data);
    
    Ok(data)
}

pub fn parse_monster_descriptions(buffer: &[u8], count: usize) -> Vec<String> {
    use std::io::Cursor;
    use crate::model::mhfdat_pointers::MOSNTERS_DESCRIPTION_PTR;
    let mut cursor = Cursor::new(buffer);
    match extract_monster_descriptions(&mut cursor, MOSNTERS_DESCRIPTION_PTR, count) {
        Ok(descriptions) => descriptions,
        Err(_) => vec![]
    }
}

// Sharpness functions
pub fn read_sharpness_item<R: Read>(r: &mut R) -> Result<SharpnessItem> {
    Ok(SharpnessItem {
        red: r.read_u16::<LittleEndian>()?,
        orange: r.read_u16::<LittleEndian>()?,
        yellow: r.read_u16::<LittleEndian>()?,
        green: r.read_u16::<LittleEndian>()?,
        blue: r.read_u16::<LittleEndian>()?,
        white: r.read_u16::<LittleEndian>()?,
        purple: r.read_u16::<LittleEndian>()?,
        sky_blue: r.read_u16::<LittleEndian>()?,
    })
}

pub fn write_sharpness_item<W: Write>(w: &mut W, item: &SharpnessItem) -> Result<()> {
    w.write_u16::<LittleEndian>(item.red)?;
    w.write_u16::<LittleEndian>(item.orange)?;
    w.write_u16::<LittleEndian>(item.yellow)?;
    w.write_u16::<LittleEndian>(item.green)?;
    w.write_u16::<LittleEndian>(item.blue)?;
    w.write_u16::<LittleEndian>(item.white)?;
    w.write_u16::<LittleEndian>(item.purple)?;
    w.write_u16::<LittleEndian>(item.sky_blue)?;
    Ok(())
}

pub fn read_sharpness_data(buffer: &[u8], offset: usize) -> SharpnessData {
    let mut cursor = Cursor::new(buffer);
    cursor.set_position(offset as u64);
    
    let mut data = Vec::with_capacity(128);
    for _ in 0..128 {
        if let Ok(item) = read_sharpness_item(&mut cursor) {
            data.push(item);
        } else {
            break;
        }
    }
    data
}

pub fn write_sharpness_data_block(data: &SharpnessData) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    for item in data {
        write_sharpness_item(&mut buffer, item)?;
    }
    Ok(buffer)
}

// Bullet Set functions
pub fn read_bullet_set<R: Read>(r: &mut R) -> Result<crate::model::mhfdat::BulletSet> {
    Ok(BulletSet {
        normal_lv1_capacity: r.read_u8()?,
        normal_lv2_capacity: r.read_u8()?,
        normal_lv3_capacity: r.read_u8()?,
        pierce_lv1_capacity: r.read_u8()?,
        pierce_lv2_capacity: r.read_u8()?,
        pierce_lv3_capacity: r.read_u8()?,
        spread_lv1_capacity: r.read_u8()?,
        spread_lv2_capacity: r.read_u8()?,
        spread_lv3_capacity: r.read_u8()?,
        crag_lv1_capacity: r.read_u8()?,
        crag_lv2_capacity: r.read_u8()?,
        crag_lv3_capacity: r.read_u8()?,
        cluster_lv1_capacity: r.read_u8()?,
        cluster_lv2_capacity: r.read_u8()?,
        cluster_lv3_capacity: r.read_u8()?,
        fire_capacity: r.read_u8()?,
        water_capacity: r.read_u8()?,
        thunder_capacity: r.read_u8()?,
        ice_capacity: r.read_u8()?,
        dragon_capacity: r.read_u8()?,
        recovery_lv1_capacity: r.read_u8()?,
        recovery_lv2_capacity: r.read_u8()?,
        poison_lv1_capacity: r.read_u8()?,
        poison_lv2_capacity: r.read_u8()?,
        paralysis_lv1_capacity: r.read_u8()?,
        paralysis_lv2_capacity: r.read_u8()?,
        sleep_lv1_capacity: r.read_u8()?,
        sleep_lv2_capacity: r.read_u8()?,
        tranquilizer_capacity: r.read_u8()?,
        paint_capacity: r.read_u8()?,
        demon_capacity: r.read_u8()?,
        armor_capacity: r.read_u8()?,
        _padding: {
            let mut padding = [0u8; 68];
            r.read_exact(&mut padding)?;
            padding
        },
    })
}

pub fn write_bullet_set<W: Write>(w: &mut W, item: &crate::model::mhfdat::BulletSet) -> Result<()> {
    w.write_all(&[item.normal_lv1_capacity])?;
    w.write_all(&[item.normal_lv2_capacity])?;
    w.write_all(&[item.normal_lv3_capacity])?;
    w.write_all(&[item.pierce_lv1_capacity])?;
    w.write_all(&[item.pierce_lv2_capacity])?;
    w.write_all(&[item.pierce_lv3_capacity])?;
    w.write_all(&[item.spread_lv1_capacity])?;
    w.write_all(&[item.spread_lv2_capacity])?;
    w.write_all(&[item.spread_lv3_capacity])?;
    w.write_all(&[item.crag_lv1_capacity])?;
    w.write_all(&[item.crag_lv2_capacity])?;
    w.write_all(&[item.crag_lv3_capacity])?;
    w.write_all(&[item.cluster_lv1_capacity])?;
    w.write_all(&[item.cluster_lv2_capacity])?;
    w.write_all(&[item.cluster_lv3_capacity])?;
    w.write_all(&[item.fire_capacity])?;
    w.write_all(&[item.water_capacity])?;
    w.write_all(&[item.thunder_capacity])?;
    w.write_all(&[item.ice_capacity])?;
    w.write_all(&[item.dragon_capacity])?;
    w.write_all(&[item.recovery_lv1_capacity])?;
    w.write_all(&[item.recovery_lv2_capacity])?;
    w.write_all(&[item.poison_lv1_capacity])?;
    w.write_all(&[item.poison_lv2_capacity])?;
    w.write_all(&[item.paralysis_lv1_capacity])?;
    w.write_all(&[item.paralysis_lv2_capacity])?;
    w.write_all(&[item.sleep_lv1_capacity])?;
    w.write_all(&[item.sleep_lv2_capacity])?;
    w.write_all(&[item.tranquilizer_capacity])?;
    w.write_all(&[item.paint_capacity])?;
    w.write_all(&[item.demon_capacity])?;
    w.write_all(&[item.armor_capacity])?;
    w.write_all(&item._padding)?;
    Ok(())
}

pub fn read_bullet_sets(buffer: &[u8], offset: usize, count: usize) -> Vec<crate::model::mhfdat::BulletSet> {
    let mut cursor = Cursor::new(buffer);
    cursor.set_position(offset as u64);
    
    let mut data = Vec::with_capacity(count);
    for _ in 0..count {
        if let Ok(item) = read_bullet_set(&mut cursor) {
            data.push(item);
        } else {
            break;
        }
    }
    data
}

pub fn write_bullet_sets_block(data: &[crate::model::mhfdat::BulletSet]) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    for item in data {
        write_bullet_set(&mut buffer, item)?;
    }
    Ok(buffer)
}

// Quest reading/writing functions
use crate::model::mhfdat::{QuestItem, HRQuests, GRQuests};

pub fn read_quest_item<R: Read>(reader: &mut R) -> Result<QuestItem> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(QuestItem {
        quest_id: u16::from_le_bytes([buf[0], buf[1]]),
        quest_number: u16::from_le_bytes([buf[2], buf[3]]),
        key_quest: buf[4],
        urgent_quest: buf[5],
        unknown: u16::from_le_bytes([buf[6], buf[7]]),
    })
}

pub fn write_quest_item<W: Write>(w: &mut W, item: &QuestItem) -> Result<()> {
    w.write_all(&item.quest_id.to_le_bytes())?;
    w.write_all(&item.quest_number.to_le_bytes())?;
    w.write_all(&[item.key_quest])?;
    w.write_all(&[item.urgent_quest])?;
    w.write_all(&item.unknown.to_le_bytes())?;
    Ok(())
}

fn is_quest_terminator(buffer: &[u8], offset: usize) -> bool {
    if offset + 8 > buffer.len() {
        return true;
    }
    let quest_id = u16::from_le_bytes([buffer[offset], buffer[offset + 1]]);
    let quest_number = u16::from_le_bytes([buffer[offset + 2], buffer[offset + 3]]);
    
    // Check for padding: questId == 0 && questNumber == 0 AND 6+ zeroes in 8 bytes
    if quest_id == 0x0000 && quest_number == 0x0000 {
        let mut zeroes = 0;
        for i in 0..8 {
            if offset + i < buffer.len() && buffer[offset + i] == 0x00 {
                zeroes += 1;
            }
        }
        if zeroes >= 6 {
            return true;
        }
    }
    
    if quest_id > 0x8000 || quest_id == 0xFFFF {
        return true;
    }
    false
}

fn read_quest_list_dynamic(buffer: &[u8], start: usize) -> Vec<QuestItem> {
    let mut items = Vec::new();
    let mut offset = start;
    
    while offset + 8 <= buffer.len() && items.len() < 300 {
        if is_quest_terminator(buffer, offset) {
            break;
        }
        let mut cursor = Cursor::new(&buffer[offset..offset + 8]);
        if let Ok(item) = read_quest_item(&mut cursor) {
            items.push(item);
            offset += 8;
        } else {
            break;
        }
    }
    items
}

fn read_quest_list_by_range(buffer: &[u8], start: u32, end: u32) -> Vec<QuestItem> {
    if end <= start {
        return read_quest_list_dynamic(buffer, start as usize);
    }
    let count = ((end - start) / 8) as usize;
    if count == 0 || count > 500 {
        return read_quest_list_dynamic(buffer, start as usize);
    }
    
    let mut items = Vec::with_capacity(count);
    let mut offset = start as usize;
    for _ in 0..count {
        if offset + 8 > buffer.len() {
            break;
        }
        let mut cursor = Cursor::new(&buffer[offset..offset + 8]);
        if let Ok(item) = read_quest_item(&mut cursor) {
            items.push(item);
            offset += 8;
        } else {
            break;
        }
    }
    items
}

pub fn read_hr_quests(buffer: &[u8], pointers_offset: u32) -> HRQuests {
    let off = pointers_offset as usize;
    if off + 24 > buffer.len() {
        return HRQuests::default();
    }
    
    // Read 6 pointers
    let one_star_ptr = u32::from_le_bytes([buffer[off], buffer[off + 1], buffer[off + 2], buffer[off + 3]]);
    let two_stars_ptr = u32::from_le_bytes([buffer[off + 4], buffer[off + 5], buffer[off + 6], buffer[off + 7]]);
    let three_stars_ptr = u32::from_le_bytes([buffer[off + 8], buffer[off + 9], buffer[off + 10], buffer[off + 11]]);
    let four_stars_ptr = u32::from_le_bytes([buffer[off + 12], buffer[off + 13], buffer[off + 14], buffer[off + 15]]);
    let five_stars_ptr = u32::from_le_bytes([buffer[off + 16], buffer[off + 17], buffer[off + 18], buffer[off + 19]]);
    let six_stars_ptr = u32::from_le_bytes([buffer[off + 20], buffer[off + 21], buffer[off + 22], buffer[off + 23]]);
    
    HRQuests {
        one_star: read_quest_list_by_range(buffer, one_star_ptr, two_stars_ptr),
        two_stars: read_quest_list_by_range(buffer, two_stars_ptr, three_stars_ptr),
        three_stars: read_quest_list_by_range(buffer, three_stars_ptr, four_stars_ptr),
        four_stars: read_quest_list_by_range(buffer, four_stars_ptr, five_stars_ptr),
        five_stars: read_quest_list_by_range(buffer, five_stars_ptr, six_stars_ptr),
        six_stars: read_quest_list_dynamic(buffer, six_stars_ptr as usize),
    }
}

/// Returns the number of quest items in a GR quest list
fn get_gr_count(buffer: &[u8], start: usize) -> usize {
    let mut offset = start;
    let mut count = 0;
    
    while offset + 8 <= buffer.len() && count < 300 {
        let quest_id = u16::from_le_bytes([buffer[offset], buffer[offset + 1]]);
        let quest_number = u16::from_le_bytes([buffer[offset + 2], buffer[offset + 3]]);
        
        if quest_id == 0x0000 && quest_number == 0x0000 {
            // Check for padding (6+ zeroes)
            let mut zeroes = 0;
            for i in 0..8 {
                if offset + i < buffer.len() && buffer[offset + i] == 0x00 {
                    zeroes += 1;
                }
            }
            if zeroes >= 6 {
                break;
            }
        }
        if quest_id == 0xFFFF || quest_id > 0x8000 {
            break;
        }
        count += 1;
        offset += 8;
    }
    count
}

pub fn read_gr_quests(buffer: &[u8], struct_ptr: u32) -> GRQuests {
    // Layout: G7_ptr -> G7_data[n] -> padding[8] -> G6_ptr -> G6_data[n] -> padding[8] -> ...
    
    let struct_offset = struct_ptr as usize;
    if struct_offset + 4 > buffer.len() {
        return GRQuests::default();
    }
    
    let read_ptr = |off: usize| -> usize {
        if off + 4 > buffer.len() { return 0; }
        u32::from_le_bytes([buffer[off], buffer[off+1], buffer[off+2], buffer[off+3]]) as usize
    };
    
    // G7
    let g7_ptr = read_ptr(struct_offset);
    if g7_ptr == 0 || g7_ptr >= buffer.len() { return GRQuests::default(); }
    let g7 = read_quest_list_dynamic(buffer, g7_ptr);
    
    // G6
    let g6_ptr_offset = g7_ptr + g7.len() * 8 + 8;
    let g6_ptr = read_ptr(g6_ptr_offset);
    let g6 = if g6_ptr > 0 && g6_ptr < buffer.len() { read_quest_list_dynamic(buffer, g6_ptr) } else { Vec::new() };
    
    // G5
    let g5_ptr_offset = g6_ptr + g6.len() * 8 + 8;
    let g5_ptr = read_ptr(g5_ptr_offset);
    let g5 = if g5_ptr > 0 && g5_ptr < buffer.len() { read_quest_list_dynamic(buffer, g5_ptr) } else { Vec::new() };
    
    // G4
    let g4_ptr_offset = g5_ptr + g5.len() * 8 + 8;
    let g4_ptr = read_ptr(g4_ptr_offset);
    let g4 = if g4_ptr > 0 && g4_ptr < buffer.len() { read_quest_list_dynamic(buffer, g4_ptr) } else { Vec::new() };
    
    // G3
    let g3_ptr_offset = g4_ptr + g4.len() * 8 + 8;
    let g3_ptr = read_ptr(g3_ptr_offset);
    let g3 = if g3_ptr > 0 && g3_ptr < buffer.len() { read_quest_list_dynamic(buffer, g3_ptr) } else { Vec::new() };
    
    // G2
    let g2_ptr_offset = g3_ptr + g3.len() * 8 + 8;
    let g2_ptr = read_ptr(g2_ptr_offset);
    let g2 = if g2_ptr > 0 && g2_ptr < buffer.len() { read_quest_list_dynamic(buffer, g2_ptr) } else { Vec::new() };
    
    // G1
    let g1_ptr_offset = g2_ptr + g2.len() * 8 + 8;
    let g1_ptr = read_ptr(g1_ptr_offset);
    let g1 = if g1_ptr > 0 && g1_ptr < buffer.len() { read_quest_list_dynamic(buffer, g1_ptr) } else { Vec::new() };
    
    GRQuests { g1, g2, g3, g4, g5, g6, g7 }
}

pub fn write_quest_list_block(quests: &[QuestItem]) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    for quest in quests {
        write_quest_item(&mut buffer, quest)?;
    }
    // Add terminator (8 zero bytes)
    buffer.write_all(&[0u8; 8])?;
    Ok(buffer)
}

pub fn write_hr_quests_block(hr: &HRQuests) -> Result<(Vec<u8>, [u32; 6])> {
    let mut buffer = Vec::new();
    let mut offsets = [0u32; 6];
    
    offsets[0] = buffer.len() as u32;
    for q in &hr.one_star { write_quest_item(&mut buffer, q)?; }
    
    offsets[1] = buffer.len() as u32;
    for q in &hr.two_stars { write_quest_item(&mut buffer, q)?; }
    
    offsets[2] = buffer.len() as u32;
    for q in &hr.three_stars { write_quest_item(&mut buffer, q)?; }
    
    offsets[3] = buffer.len() as u32;
    for q in &hr.four_stars { write_quest_item(&mut buffer, q)?; }
    
    offsets[4] = buffer.len() as u32;
    for q in &hr.five_stars { write_quest_item(&mut buffer, q)?; }
    
    offsets[5] = buffer.len() as u32;
    for q in &hr.six_stars { write_quest_item(&mut buffer, q)?; }
    // Terminator for last list
    buffer.write_all(&[0u8; 8])?;
    
    Ok((buffer, offsets))
}

/// Layout: [G7_ptr][G7_data][padding8][G6_ptr][G6_data][padding8]...[G1_ptr][G1_data][padding8]
pub fn write_gr_quests_block(gr: &GRQuests, base_addr: u32) -> Result<Vec<u8>> {
    // Calculate offsets sequentially
    // Each section: ptr(4) + data(n*8) + padding(8)
    let mut off = 0u32;
    
    let g7_ptr_off = off; off += 4;
    let g7_data_off = off; off += gr.g7.len() as u32 * 8 + 8;
    
    let g6_ptr_off = off; off += 4;
    let g6_data_off = off; off += gr.g6.len() as u32 * 8 + 8;
    
    let g5_ptr_off = off; off += 4;
    let g5_data_off = off; off += gr.g5.len() as u32 * 8 + 8;
    
    let g4_ptr_off = off; off += 4;
    let g4_data_off = off; off += gr.g4.len() as u32 * 8 + 8;
    
    let g3_ptr_off = off; off += 4;
    let g3_data_off = off; off += gr.g3.len() as u32 * 8 + 8;
    
    let g2_ptr_off = off; off += 4;
    let g2_data_off = off; off += gr.g2.len() as u32 * 8 + 8;
    
    let g1_ptr_off = off; off += 4;
    let g1_data_off = off;
    let _ = (g7_ptr_off, g6_ptr_off, g5_ptr_off, g4_ptr_off, g3_ptr_off, g2_ptr_off, g1_ptr_off); // silence warnings
    
    let mut buffer = Vec::new();
    
    // G7
    buffer.write_all(&(base_addr + g7_data_off).to_le_bytes())?;
    for q in &gr.g7 { write_quest_item(&mut buffer, q)?; }
    buffer.write_all(&[0u8; 8])?;
    
    // G6
    buffer.write_all(&(base_addr + g6_data_off).to_le_bytes())?;
    for q in &gr.g6 { write_quest_item(&mut buffer, q)?; }
    buffer.write_all(&[0u8; 8])?;
    
    // G5
    buffer.write_all(&(base_addr + g5_data_off).to_le_bytes())?;
    for q in &gr.g5 { write_quest_item(&mut buffer, q)?; }
    buffer.write_all(&[0u8; 8])?;
    
    // G4
    buffer.write_all(&(base_addr + g4_data_off).to_le_bytes())?;
    for q in &gr.g4 { write_quest_item(&mut buffer, q)?; }
    buffer.write_all(&[0u8; 8])?;
    
    // G3
    buffer.write_all(&(base_addr + g3_data_off).to_le_bytes())?;
    for q in &gr.g3 { write_quest_item(&mut buffer, q)?; }
    buffer.write_all(&[0u8; 8])?;
    
    // G2
    buffer.write_all(&(base_addr + g2_data_off).to_le_bytes())?;
    for q in &gr.g2 { write_quest_item(&mut buffer, q)?; }
    buffer.write_all(&[0u8; 8])?;
    
    // G1
    buffer.write_all(&(base_addr + g1_data_off).to_le_bytes())?;
    for q in &gr.g1 { write_quest_item(&mut buffer, q)?; }
    buffer.write_all(&[0u8; 8])?;
    
    Ok(buffer)
}

// ── Sigil Crafting ──────────────────────────────────────────────────────

use crate::model::mhfdat::{SigilRecipe, SigilMaterial, SigilSkillProbabilities, SigilSkillProbability};

pub fn read_sigil_recipes(buffer: &[u8], offset: usize) -> Vec<SigilRecipe> {
    const RECIPE_SIZE: usize = 0x28; // 40 bytes
    let mut entries = Vec::new();
    let mut pos = offset;

    while pos + RECIPE_SIZE <= buffer.len() {
        if buffer.len() >= pos + 4 {
            let first4 = u32::from_le_bytes(buffer[pos..pos + 4].try_into().unwrap());
            if first4 == 0 {
                break;
            }
        }
        let b = &buffer[pos..pos + RECIPE_SIZE];
        let mut mats: [SigilMaterial; 5] = Default::default();
        for i in 0..5 {
            let m_off = 8 + i * 4;
            mats[i] = SigilMaterial {
                item: u16::from_le_bytes(b[m_off..m_off + 2].try_into().unwrap()),
                percentage_filled: b[m_off + 2],
                unk: b[m_off + 3],
            };
        }
        let mut pad2 = [0u8; 12];
        pad2.copy_from_slice(&b[28..40]);

        entries.push(SigilRecipe {
            unk0: b[0],
            extra_skills_low: b[1],
            extra_skills_high: b[2],
            unk1: b[3],
            cost: u16::from_le_bytes(b[4..6].try_into().unwrap()),
            padding: [b[6], b[7]],
            key_materials: mats,
            padding2: pad2,
        });
        pos += RECIPE_SIZE;
    }
    entries
}

pub fn read_sigil_skill_probabilities(buffer: &[u8], offset: usize, count: usize) -> Vec<SigilSkillProbabilities> {
    const PROB_SIZE: usize = 8;
    const GROUP_SIZE: usize = PROB_SIZE * 8; // 64 bytes per recipe
    let mut entries = Vec::new();

    for i in 0..count {
        let group_off = offset + i * GROUP_SIZE;
        if group_off + GROUP_SIZE > buffer.len() {
            break;
        }
        let mut probs: [SigilSkillProbability; 8] = Default::default();
        for j in 0..8 {
            let p_off = group_off + j * PROB_SIZE;
            let b = &buffer[p_off..p_off + PROB_SIZE];
            probs[j] = SigilSkillProbability {
                skill: u16::from_le_bytes(b[0..2].try_into().unwrap()),
                percentage_chance: u16::from_le_bytes(b[2..4].try_into().unwrap()),
                low_points: i16::from_le_bytes(b[4..6].try_into().unwrap()),
                high_points: i16::from_le_bytes(b[6..8].try_into().unwrap()),
            };
        }
        entries.push(SigilSkillProbabilities { probabilities: probs });
    }
    entries
}

pub fn read_sigil_blacklists(buffer: &[u8], offset: usize, count: usize) -> Vec<Vec<u16>> {
    let mut result = Vec::new();
    for i in 0..count {
        let ptr_off = offset + i * 4;
        if ptr_off + 4 > buffer.len() {
            result.push(Vec::new());
            continue;
        }
        let data_ptr = u32::from_le_bytes(buffer[ptr_off..ptr_off + 4].try_into().unwrap()) as usize;
        let mut skills = Vec::new();
        if data_ptr > 0 && data_ptr + 2 <= buffer.len() {
            let mut pos = data_ptr;
            while pos + 2 <= buffer.len() {
                let val = i16::from_le_bytes(buffer[pos..pos + 2].try_into().unwrap());
                if val == -1 {
                    break;
                }
                skills.push(val as u16);
                pos += 2;
            }
        }
        result.push(skills);
    }
    result
}

fn write_sigil_recipe(w: &mut impl Write, r: &SigilRecipe) -> Result<()> {
    w.write_all(&[r.unk0, r.extra_skills_low, r.extra_skills_high, r.unk1])?;
    w.write_all(&r.cost.to_le_bytes())?;
    w.write_all(&r.padding)?;
    for mat in &r.key_materials {
        w.write_all(&mat.item.to_le_bytes())?;
        w.write_all(&[mat.percentage_filled, mat.unk])?;
    }
    w.write_all(&r.padding2)?;
    Ok(())
}

pub fn write_sigil_recipes_block(recipes: &[SigilRecipe]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    for r in recipes {
        write_sigil_recipe(&mut data, r)?;
    }
    data.write_all(&[0u8; 4])?; // sentinel
    Ok(data)
}

fn write_sigil_prob(w: &mut impl Write, p: &SigilSkillProbability) -> Result<()> {
    w.write_all(&p.skill.to_le_bytes())?;
    w.write_all(&p.percentage_chance.to_le_bytes())?;
    w.write_all(&p.low_points.to_le_bytes())?;
    w.write_all(&p.high_points.to_le_bytes())?;
    Ok(())
}

pub fn write_sigil_probabilities_block(probs: &[SigilSkillProbabilities]) -> Result<Vec<u8>> {
    let mut data = Vec::new();
    for group in probs {
        for p in &group.probabilities {
            write_sigil_prob(&mut data, p)?;
        }
    }
    Ok(data)
}

/// Writes the blacklist data + pointer table.
/// Returns (ptr_table_bytes, base_offset_for_ptrs) where ptr_table is written first,
/// then data follows. The absolute file offset must be known at call time.
pub fn write_sigil_blacklists_block(blacklists: &[Vec<u16>], base_addr: u32) -> Result<Vec<u8>> {
    let count = blacklists.len();
    let ptr_table_size = (count * 4) as u32;

    // First pass: compute data offsets for each blacklist
    let mut data_buf = Vec::new();
    let mut offsets = Vec::new();
    for bl in blacklists {
        let data_offset = base_addr + ptr_table_size + data_buf.len() as u32;
        offsets.push(data_offset);
        for &skill in bl {
            data_buf.write_all(&skill.to_le_bytes())?;
        }
        data_buf.write_all(&0xFFFFu16.to_le_bytes())?; // -1 sentinel
    }

    // Build final buffer: ptr table + data
    let mut out = Vec::new();
    for off in &offsets {
        out.write_all(&off.to_le_bytes())?;
    }
    out.write_all(&data_buf)?;
    Ok(out)
}

// Seasonal event timing functions

use crate::model::mhfdat::EventTime;

const EVENT_TIME_SIZE: usize = 12;

pub fn read_seasonal_events(buffer: &[u8], offset: usize, count: usize) -> Vec<EventTime> {
    let mut entries = Vec::with_capacity(count);
    let mut pos = offset;
    for _ in 0..count {
        if pos + EVENT_TIME_SIZE > buffer.len() {
            break;
        }
        entries.push(EventTime {
            event_id: u32::from_le_bytes(buffer[pos..pos+4].try_into().unwrap()),
            start_time: u32::from_le_bytes(buffer[pos+4..pos+8].try_into().unwrap()),
            end_time: u32::from_le_bytes(buffer[pos+8..pos+12].try_into().unwrap()),
        });
        pos += EVENT_TIME_SIZE;
    }
    entries
}

pub fn write_seasonal_events_block(events: &[EventTime]) -> Result<Vec<u8>> {
    let mut data = Vec::with_capacity(events.len() * EVENT_TIME_SIZE);
    for ev in events {
        data.write_all(&ev.event_id.to_le_bytes())?;
        data.write_all(&ev.start_time.to_le_bytes())?;
        data.write_all(&ev.end_time.to_le_bytes())?;
    }
    Ok(data)
}

// ---------------------------------------------------------------------------------------------------------
// Tables edited in place: crowns, skill effects, GR menus, Caravan, Hunting Road, extra crafting, weapon
// parameters, arena. They are read into typed rows (model::mhfdat) and written back over the same bytes:
// no pointer moves and the table cannot grow (a longer table would run into the block that follows it).
// ---------------------------------------------------------------------------------------------------------

use crate::model::mhfdat::{
    PointerBlock, CrownRow, SkillEffect, VoyageLeg, VoyageRoute, VoyageLevel, RankingLeg, RankingRoute, Caravan,
    RoadMonsterPoint, RoadCatItem, RoadShopRow, RoadSkillPrereq, RoadSkillDef, RoadData, ComboRecipe, ComboShopRow,
    TowerJewelMat, TowerJewelRecipe, TowerUpgradeStep, ActionDescriptor, HornMelody, CoatingRow, TowerModifierRow, RastaNpc,
    RastaWeaponOption, RastaArmorSet, WeaponParams, ItemBlock, DefBlock, TutorialSet, Arena,
};
use crate::model::mhfdat_pointers::*;

type Res<T> = std::result::Result<T, String>;

/// Bytes to write over an existing table: `offset` is where the table already is.
#[derive(Clone, Debug)]
pub struct InPlacePatch {
    pub table: String,
    pub offset: u64,
    pub bytes: Vec<u8>,
}

/// Rows of `stride` bytes from `start` until `is_end(row)`, `max` rows or the end of the buffer.
fn count_rows(buffer: &[u8], start: usize, stride: usize, max: usize, is_end: impl Fn(&[u8]) -> bool) -> usize {
    let mut n = 0;
    while n < max {
        let at = start + n * stride;
        match buffer.get(at..at + stride) {
            Some(row) if !is_end(row) => n += 1,
            _ => break,
        }
    }
    n
}

/// Pointer stored in header slot `slot`; Err (with the slot) when null or outside the buffer.
fn header_target(buffer: &[u8], name: &str, slot: u32) -> Res<usize> {
    match read_header_pointer(buffer, slot) {
        Some(0) => Err(format!("{name}: header slot 0x{slot:X} is null")),
        Some(p) if (p as usize) <= buffer.len() => Ok(p as usize),
        Some(p) => Err(format!("{name}: header slot 0x{slot:X} points to 0x{p:X}, outside the {}-byte file", buffer.len())),
        None => Err(format!("{name}: header slot 0x{slot:X} is outside the {}-byte file", buffer.len())),
    }
}

fn important_count(buffer: &[u8], name: &str, inner: usize) -> Res<usize> {
    read_important_count(buffer, inner)
        .map(|c| c as usize)
        .ok_or_else(|| format!("{name}: important_nums (header 0x10) + 0x{inner:X} is outside the {}-byte file", buffer.len()))
}

/// Parses `count` rows of `stride` bytes at `offset`; the table must fit in the file.
fn read_table<'b, T>(
    buffer: &'b [u8],
    name: &str,
    offset: usize,
    stride: usize,
    count: usize,
    read_row: impl Fn(&mut Cursor<&'b [u8]>) -> Result<T>,
) -> Res<Vec<T>> {
    check_table_fits(name, offset as u64, stride, count, buffer.len())?;
    let mut rows = Vec::with_capacity(count);
    for i in 0..count {
        let at = offset + i * stride;
        let mut cur = Cursor::new(&buffer[at..at + stride]);
        rows.push(read_row(&mut cur).map_err(|e| format!("{name}: row {i} at 0x{at:X}: {e}"))?);
    }
    Ok(rows)
}

fn write_table<T>(rows: &[T], write_row: impl Fn(&mut Vec<u8>, &T) -> Result<()>) -> Vec<u8> {
    let mut out = Vec::new();
    for r in rows {
        // writing into a Vec cannot fail
        let _ = write_row(&mut out, r);
    }
    out
}

/// Patch for a flat table. Refused when the table has more rows than it had when it was read.
fn flat_patch<T>(
    table: &str,
    offset: Option<u32>,
    original_rows: usize,
    rows: &[T],
    write_row: impl Fn(&mut Vec<u8>, &T) -> Result<()>,
) -> Res<Option<InPlacePatch>> {
    if rows.len() > original_rows {
        return Err(format!(
            "{table}: {} rows now, {original_rows} when read. A longer table would overwrite the block that follows it, so it is not saved",
            rows.len()
        ));
    }
    Ok(offset.map(|o| InPlacePatch { table: table.to_string(), offset: o as u64, bytes: write_table(rows, write_row) }))
}

/// Reads the blocks behind `n` pointers (the pointer array is at `array_offset`). A null pointer is an empty
/// block; a pointer that was already used by an earlier block makes this one an alias.
fn read_pointer_blocks<'b, T>(
    buffer: &'b [u8],
    name: &str,
    array_offset: usize,
    n: usize,
    stride: usize,
    title: impl Fn(usize) -> String,
    rows_of: impl Fn(&[u8], usize) -> usize,
    read_row: impl Fn(&mut Cursor<&'b [u8]>) -> Result<T>,
) -> Res<Vec<PointerBlock<T>>> {
    check_table_fits(&format!("{name} pointer array"), array_offset as u64, 4, n, buffer.len())?;
    let mut blocks: Vec<PointerBlock<T>> = Vec::with_capacity(n);
    let mut seen: Vec<(u32, usize)> = Vec::new();
    for i in 0..n {
        let at = array_offset + i * 4;
        let ptr = u32::from_le_bytes(buffer[at..at + 4].try_into().unwrap());
        let mut block = PointerBlock { name: title(i), offset: ptr, alias_of: None, rows: Vec::new() };
        if ptr != 0 {
            if let Some(&(_, first)) = seen.iter().find(|(o, _)| *o == ptr) {
                block.alias_of = Some(first);
            } else {
                seen.push((ptr, i));
                let count = rows_of(buffer, ptr as usize);
                block.rows = read_table(buffer, &format!("{name} {}", block.name), ptr as usize, stride, count, &read_row)?;
            }
        }
        blocks.push(block);
    }
    Ok(blocks)
}

fn block_patches<T>(
    table: &str,
    blocks: &[PointerBlock<T>],
    original_rows: &[usize],
    write_row: impl Fn(&mut Vec<u8>, &T) -> Result<()>,
) -> Res<Vec<InPlacePatch>> {
    let mut out = Vec::new();
    for (i, b) in blocks.iter().enumerate() {
        if b.offset == 0 || b.alias_of.is_some() {
            continue;
        }
        let was = original_rows.get(i).copied().unwrap_or(0);
        if b.rows.len() > was {
            return Err(format!(
                "{table} {}: {} rows now, {was} when read. A longer list would overwrite what follows it, so it is not saved",
                b.name, b.rows.len()
            ));
        }
        out.push(InPlacePatch { table: format!("{table} {}", b.name), offset: b.offset as u64, bytes: write_table(&b.rows, &write_row) });
    }
    Ok(out)
}

// ---- small field readers / writers ---------------------------------------------------------------------------

fn read_array_u8<const N: usize, R: Read>(r: &mut R) -> Result<[u8; N]> {
    let mut a = [0u8; N];
    r.read_exact(&mut a)?;
    Ok(a)
}

fn read_array_u16<const N: usize, R: Read>(r: &mut R) -> Result<[u16; N]> {
    let mut a = [0u16; N];
    for v in a.iter_mut() {
        *v = r.read_u16::<LittleEndian>()?;
    }
    Ok(a)
}

fn read_vec_u8<R: Read>(r: &mut R, n: usize) -> Result<Vec<u8>> {
    let mut v = vec![0u8; n];
    r.read_exact(&mut v)?;
    Ok(v)
}

/// A fixed-width text field written back at exactly `width` bytes (shorter data is zero padded, longer is cut).
fn write_fixed(w: &mut Vec<u8>, bytes: &[u8], width: usize) {
    let mut b = bytes.to_vec();
    b.resize(width, 0);
    w.extend_from_slice(&b);
}

/// Shift-JIS bytes of `text` for a `width`-byte field (NUL padded). Err when it does not fit (the terminator
/// needs a byte) or has a character Shift-JIS cannot store; never truncates silently.
pub fn encode_fixed_text(text: &str, width: usize) -> std::result::Result<Vec<u8>, String> {
    let (bytes, _, had_errors) = SHIFT_JIS.encode(text);
    if had_errors {
        return Err("contains a character that Shift-JIS cannot store".to_string());
    }
    if bytes.len() + 1 > width {
        return Err(format!("{} bytes, but only {} fit (the last byte is the terminator)", bytes.len(), width - 1));
    }
    let mut out = bytes.into_owned();
    out.resize(width, 0);
    Ok(out)
}

/// Text of a NUL terminated Shift-JIS field.
pub fn decode_fixed_text(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&c| c == 0).unwrap_or(bytes.len());
    SHIFT_JIS.decode(&bytes[..end]).0.into_owned()
}

/// NUL terminated Shift-JIS string at `offset`, at most `max` bytes (read-only previews of text pointers).
pub fn preview_text_at(buffer: &[u8], offset: u32, max: usize) -> String {
    let start = offset as usize;
    if offset == 0 || start >= buffer.len() {
        return String::new();
    }
    decode_fixed_text(&buffer[start..(start + max).min(buffer.len())])
}

// ---- crowns -----------------------------------------------------------------------------------------------------

const CROWN_SIZE: usize = 14;

fn read_crown<R: Read>(r: &mut R) -> Result<CrownRow> {
    Ok(CrownRow {
        unk00: r.read_u8()?,
        monster_id: r.read_u8()?,
        silver_threshold: r.read_i16::<LittleEndian>()?,
        gold_threshold: r.read_i16::<LittleEndian>()?,
        mini_threshold: r.read_i16::<LittleEndian>()?,
        unk08: r.read_i16::<LittleEndian>()?,
        unk0a: r.read_u16::<LittleEndian>()?,
        unk0c: r.read_u8()?,
        monster_id_2: r.read_u8()?,
    })
}

fn write_crown(w: &mut Vec<u8>, c: &CrownRow) -> Result<()> {
    w.write_u8(c.unk00)?;
    w.write_u8(c.monster_id)?;
    w.write_i16::<LittleEndian>(c.silver_threshold)?;
    w.write_i16::<LittleEndian>(c.gold_threshold)?;
    w.write_i16::<LittleEndian>(c.mini_threshold)?;
    w.write_i16::<LittleEndian>(c.unk08)?;
    w.write_u16::<LittleEndian>(c.unk0a)?;
    w.write_u8(c.unk0c)?;
    w.write_u8(c.monster_id_2)
}

pub fn read_crowns(buffer: &[u8]) -> Res<(u32, Vec<CrownRow>)> {
    let name = "monster crowns (header 0x130)";
    let count = important_count(buffer, name, MOSNTERS_DESCRIPTION_COUNT_INNER)?;
    let offset = header_target(buffer, name, CROWNS_PTR)?;
    Ok((offset as u32, read_table(buffer, name, offset, CROWN_SIZE, count, read_crown)?))
}

pub fn crowns_patch(offset: Option<u32>, original_rows: usize, rows: &[CrownRow]) -> Res<Option<InPlacePatch>> {
    flat_patch("monster crowns", offset, original_rows, rows, write_crown)
}

// ---- skill effects ----------------------------------------------------------------------------------------------

const SKILL_EFFECT_SIZE: usize = 0x1C;

fn read_skill_effect<R: Read>(r: &mut R) -> Result<SkillEffect> {
    Ok(SkillEffect {
        skill_id: r.read_u32::<LittleEndian>()?,
        effect_type: r.read_u32::<LittleEndian>()?,
        effect_id: r.read_u32::<LittleEndian>()?,
        value: r.read_i32::<LittleEndian>()?,
        range_min: r.read_i32::<LittleEndian>()?,
        range_max: r.read_i32::<LittleEndian>()?,
        unused18: r.read_i32::<LittleEndian>()?,
    })
}

fn write_skill_effect(w: &mut Vec<u8>, e: &SkillEffect) -> Result<()> {
    w.write_u32::<LittleEndian>(e.skill_id)?;
    w.write_u32::<LittleEndian>(e.effect_type)?;
    w.write_u32::<LittleEndian>(e.effect_id)?;
    w.write_i32::<LittleEndian>(e.value)?;
    w.write_i32::<LittleEndian>(e.range_min)?;
    w.write_i32::<LittleEndian>(e.range_max)?;
    w.write_i32::<LittleEndian>(e.unused18)
}

pub fn read_skill_effects(buffer: &[u8]) -> Res<(u32, Vec<SkillEffect>)> {
    let name = "skill effects (header 0x594)";
    // the count is the u16 at +2 of the block behind header 0x988
    let host = header_target(buffer, name, SKILL_COUNT_HOST_PTR)?;
    let count = buffer.get(host + 2..host + 4).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize)
        .ok_or_else(|| format!("{name}: count at 0x{:X} is outside the {}-byte file", host + 2, buffer.len()))?;
    let offset = header_target(buffer, name, SKILL_EFFECTS_PTR)?;
    Ok((offset as u32, read_table(buffer, name, offset, SKILL_EFFECT_SIZE, count, read_skill_effect)?))
}

pub fn skill_effects_patch(offset: Option<u32>, original_rows: usize, rows: &[SkillEffect]) -> Res<Option<InPlacePatch>> {
    flat_patch("skill effects", offset, original_rows, rows, write_skill_effect)
}

// ---- GR menus and beginner quests -------------------------------------------------------------------------------

const GR_MENU_NAMES: [&str; 9] = ["G1", "G2", "G3", "G4", "G5", "G6", "G8", "G9", "G10"];

/// G1-G6 and G8-G10: header slot -> root blob -> first dword -> QuestItem stream. A menu whose chain is broken
/// is reported in the returned messages and left empty; the others stay editable.
pub fn read_gr_menus(buffer: &[u8]) -> (Vec<PointerBlock<QuestItem>>, Vec<String>) {
    let mut blocks: Vec<PointerBlock<QuestItem>> = Vec::new();
    let mut problems = Vec::new();
    let mut seen: Vec<(u32, usize)> = Vec::new();
    for (i, slot) in GR_MENU_PTRS.iter().enumerate() {
        let name = GR_MENU_NAMES[i];
        let what = format!("GR menu {name} (header 0x{slot:X})");
        let mut block = PointerBlock { name: name.to_string(), offset: 0, alias_of: None, rows: Vec::new() };
        let stream = header_target(buffer, &what, *slot).and_then(|root| {
            read_header_pointer(buffer, root as u32).filter(|&p| p != 0)
                .ok_or_else(|| format!("{what}: root blob at 0x{root:X} has no quest pointer"))
        });
        match stream {
            Ok(p) => {
                block.offset = p;
                if let Some(&(_, first)) = seen.iter().find(|(o, _)| *o == p) {
                    block.alias_of = Some(first);
                } else {
                    seen.push((p, i));
                    let count = get_gr_count(buffer, p as usize);
                    match read_table(buffer, &what, p as usize, 8, count, read_quest_item) {
                        Ok(rows) => block.rows = rows,
                        Err(e) => {
                            problems.push(e);
                            block.offset = 0;
                        }
                    }
                }
            }
            Err(e) => problems.push(e),
        }
        blocks.push(block);
    }
    (blocks, problems)
}

pub fn gr_menus_patches(blocks: &[PointerBlock<QuestItem>], original_rows: &[usize]) -> Res<Vec<InPlacePatch>> {
    block_patches("GR menu", blocks, original_rows, |w, q| write_quest_item(w, q))
}

pub fn read_beginner_quests(buffer: &[u8]) -> Res<(u32, Vec<u16>)> {
    let name = "beginner quest ids (header 0x1B8)";
    let count = important_count(buffer, name, BEGINNER_QUESTS_COUNT_INNER)?;
    let offset = header_target(buffer, name, BEGINNER_QUEST_IDS_PTR)?;
    Ok((offset as u32, read_table(buffer, name, offset, 2, count, |r| r.read_u16::<LittleEndian>())?))
}

pub fn beginner_quests_patch(offset: Option<u32>, original_rows: usize, rows: &[u16]) -> Res<Option<InPlacePatch>> {
    flat_patch("beginner quest ids", offset, original_rows, rows, |w, v| w.write_u16::<LittleEndian>(*v))
}

// ---- Caravan -----------------------------------------------------------------------------------------------------

const VOYAGE_ROUTE_SIZE: usize = 0xA1C;
const VOYAGE_LEG_COUNT: usize = 12;
const VOYAGE_LEG_SIZE: usize = 0xC4;
const RANKING_ROUTE_SIZE: usize = 0x10C;
const RANKING_GROUPS: usize = 16;

fn read_voyage_route<R: Read>(r: &mut R) -> Result<VoyageRoute> {
    let mut route = VoyageRoute {
        fee: r.read_u32::<LittleEndian>()?,
        reward_money: r.read_u32::<LittleEndian>()?,
        reward_cp: r.read_u32::<LittleEndian>()?,
        reward_item: r.read_u16::<LittleEndian>()?,
        reward_item_qty: r.read_u16::<LittleEndian>()?,
        reputation: r.read_u16::<LittleEndian>()?,
        unk12: r.read_u16::<LittleEndian>()?,
        hr_to_post: r.read_u16::<LittleEndian>()?,
        hr_to_join: r.read_u16::<LittleEndian>()?,
        cp_bonus: r.read_f32::<LittleEndian>()?,
        monster_hp: r.read_f32::<LittleEndian>()?,
        monster_atk: r.read_f32::<LittleEndian>()?,
        monster_def: r.read_f32::<LittleEndian>()?,
        final_trial: r.read_u16::<LittleEndian>()?,
        unk2a: r.read_u16::<LittleEndian>()?,
        title: read_vec_u8(r, 64)?,
        success: read_vec_u8(r, 64)?,
        failure: read_vec_u8(r, 64)?,
        legs: Vec::with_capacity(VOYAGE_LEG_COUNT),
    };
    for _ in 0..VOYAGE_LEG_COUNT {
        route.legs.push(VoyageLeg {
            quest_id: r.read_i32::<LittleEndian>()?,
            map: read_vec_u8(r, 64)?,
            time: read_vec_u8(r, 64)?,
            target: read_vec_u8(r, 64)?,
        });
    }
    Ok(route)
}

fn write_voyage_route(w: &mut Vec<u8>, v: &VoyageRoute) -> Result<()> {
    w.write_u32::<LittleEndian>(v.fee)?;
    w.write_u32::<LittleEndian>(v.reward_money)?;
    w.write_u32::<LittleEndian>(v.reward_cp)?;
    w.write_u16::<LittleEndian>(v.reward_item)?;
    w.write_u16::<LittleEndian>(v.reward_item_qty)?;
    w.write_u16::<LittleEndian>(v.reputation)?;
    w.write_u16::<LittleEndian>(v.unk12)?;
    w.write_u16::<LittleEndian>(v.hr_to_post)?;
    w.write_u16::<LittleEndian>(v.hr_to_join)?;
    w.write_f32::<LittleEndian>(v.cp_bonus)?;
    w.write_f32::<LittleEndian>(v.monster_hp)?;
    w.write_f32::<LittleEndian>(v.monster_atk)?;
    w.write_f32::<LittleEndian>(v.monster_def)?;
    w.write_u16::<LittleEndian>(v.final_trial)?;
    w.write_u16::<LittleEndian>(v.unk2a)?;
    write_fixed(w, &v.title, 64);
    write_fixed(w, &v.success, 64);
    write_fixed(w, &v.failure, 64);
    for i in 0..VOYAGE_LEG_COUNT {
        let leg = v.legs.get(i).cloned().unwrap_or_default();
        w.write_i32::<LittleEndian>(leg.quest_id)?;
        write_fixed(w, &leg.map, 64);
        write_fixed(w, &leg.time, 64);
        write_fixed(w, &leg.target, 64);
    }
    debug_assert_eq!(VOYAGE_LEG_SIZE, 4 + 3 * 64);
    Ok(())
}

fn read_voyage_level<R: Read>(r: &mut R) -> Result<VoyageLevel> {
    let mut level = VoyageLevel::default();
    for v in level.routes.iter_mut() {
        *v = r.read_i32::<LittleEndian>()?;
    }
    Ok(level)
}

fn write_voyage_level(w: &mut Vec<u8>, l: &VoyageLevel) -> Result<()> {
    for v in &l.routes {
        w.write_i32::<LittleEndian>(*v)?;
    }
    Ok(())
}

fn read_ranking_route<R: Read>(r: &mut R) -> Result<RankingRoute> {
    let mut route = RankingRoute {
        title_ptr: r.read_u32::<LittleEndian>()?,
        success_ptr: r.read_u32::<LittleEndian>()?,
        failure_ptr: r.read_u32::<LittleEndian>()?,
        route_number: r.read_u16::<LittleEndian>()?,
        group_number: r.read_u16::<LittleEndian>()?,
        fee: r.read_i32::<LittleEndian>()?,
        reward_money: r.read_i32::<LittleEndian>()?,
        reward_cp: r.read_i32::<LittleEndian>()?,
        reward_item: r.read_u16::<LittleEndian>()?,
        unk1e: r.read_u16::<LittleEndian>()?,
        unk20: r.read_u16::<LittleEndian>()?,
        reward_item_qty: r.read_u16::<LittleEndian>()?,
        unk24: r.read_u32::<LittleEndian>()?,
        rank_type: r.read_u16::<LittleEndian>()?,
        hr_to_post: r.read_u16::<LittleEndian>()?,
        hr_to_join: r.read_u16::<LittleEndian>()?,
        unk2e: r.read_u16::<LittleEndian>()?,
        cp_bonus_carve: r.read_f32::<LittleEndian>()?,
        cp_bonus_gather: r.read_f32::<LittleEndian>()?,
        hunter_title: r.read_i32::<LittleEndian>()?,
        extra_mission: r.read_i32::<LittleEndian>()?,
        monster_hp: r.read_f32::<LittleEndian>()?,
        monster_atk: r.read_f32::<LittleEndian>()?,
        monster_def: r.read_f32::<LittleEndian>()?,
        legs: Vec::with_capacity(12),
    };
    for _ in 0..12 {
        route.legs.push(RankingLeg {
            quest_id: r.read_i32::<LittleEndian>()?,
            map_ptr: r.read_u32::<LittleEndian>()?,
            time_ptr: r.read_u32::<LittleEndian>()?,
            target_ptr: r.read_u32::<LittleEndian>()?,
        });
    }
    Ok(route)
}

fn write_ranking_route(w: &mut Vec<u8>, v: &RankingRoute) -> Result<()> {
    w.write_u32::<LittleEndian>(v.title_ptr)?;
    w.write_u32::<LittleEndian>(v.success_ptr)?;
    w.write_u32::<LittleEndian>(v.failure_ptr)?;
    w.write_u16::<LittleEndian>(v.route_number)?;
    w.write_u16::<LittleEndian>(v.group_number)?;
    w.write_i32::<LittleEndian>(v.fee)?;
    w.write_i32::<LittleEndian>(v.reward_money)?;
    w.write_i32::<LittleEndian>(v.reward_cp)?;
    w.write_u16::<LittleEndian>(v.reward_item)?;
    w.write_u16::<LittleEndian>(v.unk1e)?;
    w.write_u16::<LittleEndian>(v.unk20)?;
    w.write_u16::<LittleEndian>(v.reward_item_qty)?;
    w.write_u32::<LittleEndian>(v.unk24)?;
    w.write_u16::<LittleEndian>(v.rank_type)?;
    w.write_u16::<LittleEndian>(v.hr_to_post)?;
    w.write_u16::<LittleEndian>(v.hr_to_join)?;
    w.write_u16::<LittleEndian>(v.unk2e)?;
    w.write_f32::<LittleEndian>(v.cp_bonus_carve)?;
    w.write_f32::<LittleEndian>(v.cp_bonus_gather)?;
    w.write_i32::<LittleEndian>(v.hunter_title)?;
    w.write_i32::<LittleEndian>(v.extra_mission)?;
    w.write_f32::<LittleEndian>(v.monster_hp)?;
    w.write_f32::<LittleEndian>(v.monster_atk)?;
    w.write_f32::<LittleEndian>(v.monster_def)?;
    for i in 0..12 {
        let leg = v.legs.get(i).cloned().unwrap_or_default();
        w.write_i32::<LittleEndian>(leg.quest_id)?;
        w.write_u32::<LittleEndian>(leg.map_ptr)?;
        w.write_u32::<LittleEndian>(leg.time_ptr)?;
        w.write_u32::<LittleEndian>(leg.target_ptr)?;
    }
    Ok(())
}

/// Offsets of the two flat Caravan tables, needed to write them back.
#[derive(Clone, Debug, Default)]
pub struct CaravanOffsets {
    pub voyage: Option<u32>,
    pub levels: Option<u32>,
}

/// Voyage routes (end on an empty title), the level index and the ranking groups. A table that does not fit
/// the file is reported in the messages and left empty.
pub fn read_caravan(buffer: &[u8]) -> (Caravan, CaravanOffsets, Vec<String>) {
    let mut caravan = Caravan::default();
    let mut offsets = CaravanOffsets::default();
    let mut problems = Vec::new();

    let name = "Caravan voyage routes (header 0x22C)";
    match header_target(buffer, name, VOYAGE_ROUTES_PTR).and_then(|o| {
        let count = count_rows(buffer, o, VOYAGE_ROUTE_SIZE, 4096, |r| r[0x2C] == 0);
        read_table(buffer, name, o, VOYAGE_ROUTE_SIZE, count, read_voyage_route).map(|rows| (o, rows))
    }) {
        Ok((o, rows)) => {
            offsets.voyage = Some(o as u32);
            caravan.voyage = rows;
        }
        Err(e) => problems.push(e),
    }

    let name = "Caravan level index (header 0x230)";
    match header_target(buffer, name, VOYAGE_LEVEL_INDEX_PTR)
        .and_then(|o| read_table(buffer, name, o, 48, 15, read_voyage_level).map(|rows| (o, rows)))
    {
        Ok((o, rows)) => {
            offsets.levels = Some(o as u32);
            caravan.levels = rows;
        }
        Err(e) => problems.push(e),
    }

    let name = "Caravan ranking routes (header 0x91C)";
    match header_target(buffer, name, RANKING_ROUTES_PTR).and_then(|o| {
        read_pointer_blocks(
            buffer, name, o, RANKING_GROUPS, RANKING_ROUTE_SIZE,
            |i| format!("group {i}"),
            // a group ends at the first record whose title pointer is 0 (at most 12 records)
            |b, p| count_rows(b, p, RANKING_ROUTE_SIZE, 12, |r| r[..4] == [0, 0, 0, 0]),
            read_ranking_route,
        )
    }) {
        Ok(groups) => caravan.ranking = groups,
        Err(e) => problems.push(e),
    }
    (caravan, offsets, problems)
}

pub fn caravan_patches(
    caravan: &Caravan,
    offsets: &CaravanOffsets,
    original_voyage_rows: usize,
    original_ranking_rows: &[usize],
) -> Res<Vec<InPlacePatch>> {
    let mut out = Vec::new();
    out.extend(flat_patch("Caravan voyage routes", offsets.voyage, original_voyage_rows, &caravan.voyage, write_voyage_route)?);
    out.extend(flat_patch("Caravan level index", offsets.levels, 15, &caravan.levels, write_voyage_level)?);
    out.extend(block_patches("Caravan ranking", &caravan.ranking, original_ranking_rows, write_ranking_route)?);
    Ok(out)
}

// ---- Hunting Road -----------------------------------------------------------------------------------------------

const ROAD_POINT_SIZE: usize = 16;
const ROAD_CAT_SIZE: usize = 8;
const ROAD_SHOP_SIZE: usize = 16;
const ROAD_SKILL_SIZE: usize = 0x28;
const ROAD_WEIGHT_WINDOW: usize = 16;

fn read_road_point<R: Read>(r: &mut R) -> Result<RoadMonsterPoint> {
    Ok(RoadMonsterPoint {
        monster_id: r.read_u16::<LittleEndian>()?,
        flags: r.read_u16::<LittleEndian>()?,
        points: read_array_u16::<5, _>(r)?,
        unk0e: r.read_u16::<LittleEndian>()?,
    })
}

fn write_road_point(w: &mut Vec<u8>, p: &RoadMonsterPoint) -> Result<()> {
    w.write_u16::<LittleEndian>(p.monster_id)?;
    w.write_u16::<LittleEndian>(p.flags)?;
    for v in &p.points {
        w.write_u16::<LittleEndian>(*v)?;
    }
    w.write_u16::<LittleEndian>(p.unk0e)
}

fn read_road_cat<R: Read>(r: &mut R) -> Result<RoadCatItem> {
    Ok(RoadCatItem { item_id: r.read_u16::<LittleEndian>()?, unk02: r.read_u32::<LittleEndian>()?, unk06: r.read_u16::<LittleEndian>()? })
}

fn write_road_cat(w: &mut Vec<u8>, c: &RoadCatItem) -> Result<()> {
    w.write_u16::<LittleEndian>(c.item_id)?;
    w.write_u32::<LittleEndian>(c.unk02)?;
    w.write_u16::<LittleEndian>(c.unk06)
}

fn read_road_shop<R: Read>(r: &mut R) -> Result<RoadShopRow> {
    Ok(RoadShopRow {
        unk00: r.read_u16::<LittleEndian>()?,
        unk02: r.read_u16::<LittleEndian>()?,
        unk04: r.read_u32::<LittleEndian>()?,
        unk08: r.read_u8()?,
        unk09: r.read_u8()?,
        unk0a: r.read_u8()?,
        unk0b: r.read_u8()?,
        unk0c: r.read_u32::<LittleEndian>()?,
    })
}

fn write_road_shop(w: &mut Vec<u8>, s: &RoadShopRow) -> Result<()> {
    w.write_u16::<LittleEndian>(s.unk00)?;
    w.write_u16::<LittleEndian>(s.unk02)?;
    w.write_u32::<LittleEndian>(s.unk04)?;
    w.write_u8(s.unk08)?;
    w.write_u8(s.unk09)?;
    w.write_u8(s.unk0a)?;
    w.write_u8(s.unk0b)?;
    w.write_u32::<LittleEndian>(s.unk0c)
}

fn read_road_skill<R: Read>(r: &mut R) -> Result<RoadSkillDef> {
    let mut d = RoadSkillDef {
        skill_id: r.read_u16::<LittleEndian>()?,
        level: r.read_u16::<LittleEndian>()?,
        category: r.read_u16::<LittleEndian>()?,
        desc_id: r.read_u16::<LittleEndian>()?,
        sort_key: r.read_u16::<LittleEndian>()?,
        unk0a: r.read_u16::<LittleEndian>()?,
        cost: r.read_u32::<LittleEndian>()?,
        road_rank_req: r.read_u16::<LittleEndian>()?,
        road_points: r.read_u16::<LittleEndian>()?,
        icon_id: r.read_u16::<LittleEndian>()?,
        prereqs: Default::default(),
        unk26: 0,
    };
    for p in d.prereqs.iter_mut() {
        *p = RoadSkillPrereq { skill_id: r.read_u16::<LittleEndian>()?, level: r.read_u16::<LittleEndian>()? };
    }
    d.unk26 = r.read_u16::<LittleEndian>()?;
    Ok(d)
}

fn write_road_skill(w: &mut Vec<u8>, d: &RoadSkillDef) -> Result<()> {
    w.write_u16::<LittleEndian>(d.skill_id)?;
    w.write_u16::<LittleEndian>(d.level)?;
    w.write_u16::<LittleEndian>(d.category)?;
    w.write_u16::<LittleEndian>(d.desc_id)?;
    w.write_u16::<LittleEndian>(d.sort_key)?;
    w.write_u16::<LittleEndian>(d.unk0a)?;
    w.write_u32::<LittleEndian>(d.cost)?;
    w.write_u16::<LittleEndian>(d.road_rank_req)?;
    w.write_u16::<LittleEndian>(d.road_points)?;
    w.write_u16::<LittleEndian>(d.icon_id)?;
    for p in &d.prereqs {
        w.write_u16::<LittleEndian>(p.skill_id)?;
        w.write_u16::<LittleEndian>(p.level)?;
    }
    w.write_u16::<LittleEndian>(d.unk26)
}

/// Offsets of the five Hunting Road tables.
#[derive(Clone, Debug, Default)]
pub struct RoadOffsets {
    pub points: Option<u32>,
    pub cat_shop: Option<u32>,
    pub shop: Option<u32>,
    pub skills: Option<u32>,
    pub weight: Option<u32>,
}

pub fn read_road(buffer: &[u8]) -> (RoadData, RoadOffsets, Vec<String>) {
    let mut road = RoadData::default();
    let mut offsets = RoadOffsets::default();
    let mut problems = Vec::new();
    let counters = header_target(buffer, "Hunting Road counters (header 0xB04)", ROAD_COUNTERS_PTR);
    let counter_at = |at: usize| buffer.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize);

    let name = "Hunting Road points per monster (header 0xB20)";
    match counters.clone().and_then(|c| counter_at(c + 8).ok_or_else(|| format!("{name}: count is outside the file")))
        .and_then(|count| header_target(buffer, name, ROAD_MONSTER_POINTS_PTR).map(|o| (o, count)))
        .and_then(|(o, count)| read_table(buffer, name, o, ROAD_POINT_SIZE, count, read_road_point).map(|r| (o, r)))
    {
        Ok((o, rows)) => { offsets.points = Some(o as u32); road.points = rows; }
        Err(e) => problems.push(e),
    }

    let name = "Hunting Road Felyne shop (header 0xB10)";
    match counters.and_then(|c| counter_at(c + 4).ok_or_else(|| format!("{name}: count is outside the file")))
        .and_then(|count| header_target(buffer, name, ROAD_CAT_SHOP_PTR).map(|o| (o, count)))
        .and_then(|(o, count)| read_table(buffer, name, o, ROAD_CAT_SIZE, count, read_road_cat).map(|r| (o, r)))
    {
        Ok((o, rows)) => { offsets.cat_shop = Some(o as u32); road.cat_shop = rows; }
        Err(e) => problems.push(e),
    }

    let name = "Hunting Road fallback shop (special shops -> ptRoadShop)";
    match header_target(buffer, name, SPECIAL_SHOPS_PTR)
        .and_then(|h| read_header_pointer(buffer, h as u32).filter(|&p| p != 0).ok_or_else(|| format!("{name}: ptRoadShop is null")))
        .and_then(|o| {
            // the sources only say "16 bytes per row": the table ends on an all-zero row
            let count = count_rows(buffer, o as usize, ROAD_SHOP_SIZE, 8192, |r| r.iter().all(|&b| b == 0));
            read_table(buffer, name, o as usize, ROAD_SHOP_SIZE, count, read_road_shop).map(|r| (o, r))
        })
    {
        Ok((o, rows)) => { offsets.shop = Some(o); road.shop = rows; }
        Err(e) => problems.push(e),
    }

    let name = "Hunting Road skills (header 0x930, meta 0x910)";
    match header_target(buffer, name, ROAD_SKILL_META_PTR)
        .and_then(|m| buffer.get(m + 8..m + 10).map(|b| u16::from_le_bytes([b[0], b[1]]) as usize).ok_or_else(|| format!("{name}: count is outside the file")))
        .and_then(|count| header_target(buffer, name, ROAD_SKILLS_PTR).map(|o| (o, count)))
        .and_then(|(o, count)| read_table(buffer, name, o, ROAD_SKILL_SIZE, count, read_road_skill).map(|r| (o, r)))
    {
        Ok((o, rows)) => { offsets.skills = Some(o as u32); road.skills = rows; }
        Err(e) => problems.push(e),
    }

    // header 0x928 -> +0x8C4: neither the shape nor the length is documented, so a window of raw dwords
    let name = "Hunting Road spawn weight bonus (header 0x928 + 0x8C4)";
    match header_target(buffer, name, ROAD_UNK_928_PTR)
        .and_then(|h| read_table(buffer, name, h + 0x8C4, 4, ROAD_WEIGHT_WINDOW, |r| r.read_u32::<LittleEndian>()).map(|r| (h + 0x8C4, r)))
    {
        Ok((o, rows)) => { offsets.weight = Some(o as u32); road.weight_window = rows; }
        Err(e) => problems.push(e),
    }
    (road, offsets, problems)
}

pub fn road_patches(road: &RoadData, offsets: &RoadOffsets, original_rows: [usize; 5]) -> Res<Vec<InPlacePatch>> {
    let mut out = Vec::new();
    out.extend(flat_patch("Hunting Road points", offsets.points, original_rows[0], &road.points, write_road_point)?);
    out.extend(flat_patch("Hunting Road Felyne shop", offsets.cat_shop, original_rows[1], &road.cat_shop, write_road_cat)?);
    out.extend(flat_patch("Hunting Road fallback shop", offsets.shop, original_rows[2], &road.shop, write_road_shop)?);
    out.extend(flat_patch("Hunting Road skills", offsets.skills, original_rows[3], &road.skills, write_road_skill)?);
    out.extend(flat_patch("Hunting Road weight window", offsets.weight, original_rows[4], &road.weight_window, |w, v| w.write_u32::<LittleEndian>(*v))?);
    Ok(out)
}

// ---- extra crafting tables ----------------------------------------------------------------------------------------

/// Forging tables the Shop tab does not edit (special weapon 0x7AC, armor premium 0x7B0, second transmog 0xABC):
/// ShopEntry rows that end on a first byte of 0xFF (crafting.hexpat CraftMatTable).
pub fn read_forging_rows(buffer: &[u8], name: &str, slot: u32) -> Res<(u32, Vec<ShopEntry>)> {
    let offset = header_target(buffer, name, slot)?;
    let size = size_of::<ShopEntry>();
    let mut rows = Vec::new();
    loop {
        let at = offset + rows.len() * size;
        let slice = buffer.get(at..at + size).ok_or_else(|| {
            format!("{name}: no terminator row before the end of the {}-byte file (table at 0x{offset:X})", buffer.len())
        })?;
        let entry = read_shop_entry(&mut Cursor::new(slice)).map_err(|e| format!("{name}: row {} at 0x{at:X}: {e}", rows.len()))?;
        if slice[0] == 0xFF {
            break;
        }
        rows.push(entry);
    }
    Ok((offset as u32, rows))
}

/// The terminator row that follows the entries is left as it is (it is not rewritten).
pub fn forging_rows_patch(table: &str, offset: Option<u32>, original_rows: usize, rows: &[ShopEntry]) -> Res<Option<InPlacePatch>> {
    flat_patch(table, offset, original_rows, rows, |w, e| write_shop_entry(w, e))
}

const COMBO_SHOP_SIZE: usize = 0x30;

fn read_combo_shop<R: Read>(r: &mut R) -> Result<ComboShopRow> {
    let mut row = ComboShopRow {
        item_id: r.read_u16::<LittleEndian>()?,
        combo_fee: r.read_u16::<LittleEndian>()?,
        recipe_no: r.read_u16::<LittleEndian>()?,
        recipes: Default::default(),
    };
    for rec in row.recipes.iter_mut() {
        *rec = ComboRecipe { min: r.read_u8()?, max: r.read_u8()?, item_ids: [0; 3], item_qtys: [0; 3] };
        for k in 0..3 {
            rec.item_ids[k] = r.read_u16::<LittleEndian>()?;
            rec.item_qtys[k] = r.read_u16::<LittleEndian>()?;
        }
    }
    Ok(row)
}

fn write_combo_shop(w: &mut Vec<u8>, row: &ComboShopRow) -> Result<()> {
    w.write_u16::<LittleEndian>(row.item_id)?;
    w.write_u16::<LittleEndian>(row.combo_fee)?;
    w.write_u16::<LittleEndian>(row.recipe_no)?;
    for rec in &row.recipes {
        w.write_u8(rec.min)?;
        w.write_u8(rec.max)?;
        for k in 0..3 {
            w.write_u16::<LittleEndian>(rec.item_ids[k])?;
            w.write_u16::<LittleEndian>(rec.item_qtys[k])?;
        }
    }
    Ok(())
}

pub fn read_combine_shop(buffer: &[u8]) -> Res<(u32, Vec<ComboShopRow>)> {
    let name = "combine shop (header 0xBAC)";
    let offset = header_target(buffer, name, COMBINE_SHOP_PTR)?;
    // shop.hexpat get_combo_shop_count: itemId != 0 and every item id <= 16700
    let count = count_rows(buffer, offset, COMBO_SHOP_SIZE, 5000, |r| {
        let id = |o: usize| u16::from_le_bytes([r[o], r[o + 1]]);
        id(0) == 0 || id(0) > 16700 || id(8) > 16700 || id(12) > 16700 || id(16) > 16700
    });
    Ok((offset as u32, read_table(buffer, name, offset, COMBO_SHOP_SIZE, count, read_combo_shop)?))
}

pub fn combine_shop_patch(offset: Option<u32>, original_rows: usize, rows: &[ComboShopRow]) -> Res<Option<InPlacePatch>> {
    flat_patch("combine shop", offset, original_rows, rows, write_combo_shop)
}

const TOWER_JEWEL_SIZE: usize = 0x20;

fn read_tower_jewel<R: Read>(r: &mut R) -> Result<TowerJewelRecipe> {
    let mut j = TowerJewelRecipe {
        weapon_id: r.read_u16::<LittleEndian>()?,
        level_min: r.read_u16::<LittleEndian>()?,
        level_max: r.read_u16::<LittleEndian>()?,
        stat_id: r.read_u8()?,
        equip_type: r.read_u8()?,
        mats: Default::default(),
    };
    for m in j.mats.iter_mut() {
        *m = TowerJewelMat { item: r.read_u16::<LittleEndian>()?, qty: r.read_u16::<LittleEndian>()?, unk04: r.read_u32::<LittleEndian>()? };
    }
    Ok(j)
}

fn write_tower_jewel(w: &mut Vec<u8>, j: &TowerJewelRecipe) -> Result<()> {
    w.write_u16::<LittleEndian>(j.weapon_id)?;
    w.write_u16::<LittleEndian>(j.level_min)?;
    w.write_u16::<LittleEndian>(j.level_max)?;
    w.write_u8(j.stat_id)?;
    w.write_u8(j.equip_type)?;
    for m in &j.mats {
        w.write_u16::<LittleEndian>(m.item)?;
        w.write_u16::<LittleEndian>(m.qty)?;
        w.write_u32::<LittleEndian>(m.unk04)?;
    }
    Ok(())
}

pub fn read_tower_jewels(buffer: &[u8]) -> Res<(u32, Vec<TowerJewelRecipe>)> {
    let name = "tower jewel recipes (header 0x938)";
    let count = important_count(buffer, name, TOWER_JEWEL_RECIPES_COUNT_INNER)?;
    let offset = header_target(buffer, name, TOWER_JEWEL_RECIPES_PTR)?;
    Ok((offset as u32, read_table(buffer, name, offset, TOWER_JEWEL_SIZE, count, read_tower_jewel)?))
}

pub fn tower_jewels_patch(offset: Option<u32>, original_rows: usize, rows: &[TowerJewelRecipe]) -> Res<Option<InPlacePatch>> {
    flat_patch("tower jewel recipes", offset, original_rows, rows, write_tower_jewel)
}

// ---- weapon parameter tables ---------------------------------------------------------------------------------------

const ACTION_DESCRIPTOR_SIZE: usize = 0x88;
const ACTION_DESCRIPTOR_COUNT: usize = 0x35;
const THRESHOLD_COUNT: usize = 0x34; // provisional in weapon_action_thresholds.hexpat
const COATING_COUNT: usize = 23;
const COATING_SIZE: usize = 0x18;
const TOWER_MODIFIER_COUNT: usize = 32;
const TOWER_MODIFIER_OFFSET: usize = 0x258;

fn read_action_descriptor<R: Read>(r: &mut R) -> Result<ActionDescriptor> {
    let mut d = ActionDescriptor {
        valid: r.read_u8()?,
        unk01: read_array_u8::<3, _>(r)?,
        primary_mv: read_array_u16::<12, _>(r)?,
        element_magnitude: read_array_u8::<12, _>(r)?,
        attr28: read_array_u8::<12, _>(r)?,
        attr34: read_array_u8::<12, _>(r)?,
        scale: [0.0; 12],
        secondary_mv: [0; 12],
    };
    for v in d.scale.iter_mut() {
        *v = r.read_f32::<LittleEndian>()?;
    }
    d.secondary_mv = read_array_u16::<12, _>(r)?;
    Ok(d)
}

fn write_action_descriptor(w: &mut Vec<u8>, d: &ActionDescriptor) -> Result<()> {
    w.write_u8(d.valid)?;
    w.extend_from_slice(&d.unk01);
    for v in &d.primary_mv {
        w.write_u16::<LittleEndian>(*v)?;
    }
    w.extend_from_slice(&d.element_magnitude);
    w.extend_from_slice(&d.attr28);
    w.extend_from_slice(&d.attr34);
    for v in &d.scale {
        w.write_f32::<LittleEndian>(*v)?;
    }
    for v in &d.secondary_mv {
        w.write_u16::<LittleEndian>(*v)?;
    }
    Ok(())
}

fn read_rasta_npc<R: Read>(r: &mut R) -> Result<RastaNpc> {
    Ok(RastaNpc { companion_type: r.read_u8()?, visual: read_array_u8::<3, _>(r)?, unk4: r.read_u8()?, unk5: r.read_u8()?, unk06: read_array_u8::<2, _>(r)? })
}

fn write_rasta_npc(w: &mut Vec<u8>, n: &RastaNpc) -> Result<()> {
    w.write_u8(n.companion_type)?;
    w.extend_from_slice(&n.visual);
    w.write_u8(n.unk4)?;
    w.write_u8(n.unk5)?;
    w.extend_from_slice(&n.unk06);
    Ok(())
}

fn read_rasta_weapon<R: Read>(r: &mut R) -> Result<RastaWeaponOption> {
    Ok(RastaWeaponOption {
        companion_type: r.read_u8()?,
        weapon_type: r.read_u8()?,
        level_min: r.read_u16::<LittleEndian>()?,
        level_max: r.read_u16::<LittleEndian>()?,
        weapon_id: r.read_u16::<LittleEndian>()?,
    })
}

fn write_rasta_weapon(w: &mut Vec<u8>, o: &RastaWeaponOption) -> Result<()> {
    w.write_u8(o.companion_type)?;
    w.write_u8(o.weapon_type)?;
    w.write_u16::<LittleEndian>(o.level_min)?;
    w.write_u16::<LittleEndian>(o.level_max)?;
    w.write_u16::<LittleEndian>(o.weapon_id)
}

fn read_rasta_armor<R: Read>(r: &mut R) -> Result<RastaArmorSet> {
    Ok(RastaArmorSet {
        companion_type: r.read_u8()?,
        subtype: r.read_u8()?,
        level_min: r.read_u16::<LittleEndian>()?,
        level_max: r.read_u16::<LittleEndian>()?,
        slot2_id: r.read_u16::<LittleEndian>()?,
        slot3_id: r.read_u16::<LittleEndian>()?,
        slot4_id: r.read_u16::<LittleEndian>()?,
        slot5_id: r.read_u16::<LittleEndian>()?,
        slot0_id: r.read_u16::<LittleEndian>()?,
    })
}

fn write_rasta_armor(w: &mut Vec<u8>, a: &RastaArmorSet) -> Result<()> {
    w.write_u8(a.companion_type)?;
    w.write_u8(a.subtype)?;
    w.write_u16::<LittleEndian>(a.level_min)?;
    w.write_u16::<LittleEndian>(a.level_max)?;
    w.write_u16::<LittleEndian>(a.slot2_id)?;
    w.write_u16::<LittleEndian>(a.slot3_id)?;
    w.write_u16::<LittleEndian>(a.slot4_id)?;
    w.write_u16::<LittleEndian>(a.slot5_id)?;
    w.write_u16::<LittleEndian>(a.slot0_id)
}

/// Offsets (file offsets, None = table could not be located) of every weapon parameter table.
#[derive(Clone, Debug, Default)]
pub struct WeaponParamOffsets {
    pub thresholds: Option<u32>,
    pub coating: Option<u32>,
    pub tower_modifiers: Option<u32>,
    pub melodies: Option<u32>,
    pub gl_shell_types: Option<u32>,
    pub gl_param_pairs: Option<u32>,
    pub sa_phials: Option<u32>,
    pub motion_primary: Option<u32>,
    pub motion_alt: Option<u32>,
    pub rasta_npcs: Option<u32>,
    pub rasta_weapons: Option<u32>,
    pub rasta_armors: Option<u32>,
}

pub fn read_weapon_params(buffer: &[u8]) -> (WeaponParams, WeaponParamOffsets, Vec<String>) {
    let mut p = WeaponParams::default();
    let mut o = WeaponParamOffsets::default();
    let mut problems = Vec::new();
    macro_rules! table {
        ($offset:expr, $rows:expr, $res:expr) => {
            match $res {
                Ok((at, rows)) => { $offset = Some(at as u32); $rows = rows; }
                Err(e) => problems.push(e),
            }
        };
    }

    let name = "weapon action thresholds (header 0xA0)";
    table!(o.thresholds, p.thresholds, header_target(buffer, name, WEAPON_ACTION_THRESHOLDS_PTR)
        .and_then(|at| read_table(buffer, name, at, 4, THRESHOLD_COUNT, |r| read_array_u8::<4, _>(r)).map(|r| (at, r))));

    let name = "weapon coating modifiers (header 0xB0)";
    table!(o.coating, p.coating, header_target(buffer, name, WEAPON_COATING_MODIFIERS_PTR).and_then(|at| {
        read_table(buffer, name, at, COATING_SIZE, COATING_COUNT, |r| {
            Ok(CoatingRow { delta_coating: r.read_i16::<LittleEndian>()?, unused: read_vec_u8(r, COATING_SIZE - 2)? })
        }).map(|r| (at, r))
    }));

    let name = "tower / special weapon modifiers (header 0xB0 + 0x258)";
    table!(o.tower_modifiers, p.tower_modifiers, header_target(buffer, name, WEAPON_COATING_MODIFIERS_PTR).and_then(|base| {
        let at = base + TOWER_MODIFIER_OFFSET;
        read_table(buffer, name, at, 8, TOWER_MODIFIER_COUNT, |r| {
            Ok(TowerModifierRow { delta_tower: r.read_i16::<LittleEndian>()?, unused: read_array_u8::<6, _>(r)? })
        }).map(|r| (at, r))
    }));

    let name = "hunting horn melodies (header 0x670)";
    table!(o.melodies, p.melodies, important_count(buffer, name, HORN_MELODIES_COUNT_INNER).and_then(|count| {
        let at = header_target(buffer, name, HUNTING_HORN_MELODIES_PTR)?;
        read_table(buffer, name, at, 5, count, |r| Ok(HornMelody { song_id: r.read_u8()?, notes: read_array_u8::<4, _>(r)? })).map(|r| (at, r))
    }));

    let name = "gunlance random shell types (header 0x4F0)";
    match header_target(buffer, name, GUNLANCE_RANDOM_SHELL_TYPES_PTR)
        .and_then(|at| read_table(buffer, name, at, 3, 1, |r| read_array_u8::<3, _>(r)).map(|r| (at, r)))
    {
        Ok((at, rows)) => { o.gl_shell_types = Some(at as u32); p.gl_shell_types = rows[0]; }
        Err(e) => problems.push(e),
    }

    let name = "gunlance random param pairs (header 0x4F8)";
    table!(o.gl_param_pairs, p.gl_param_pairs, header_target(buffer, name, GUNLANCE_RANDOM_PARAM_PAIRS_PTR)
        .and_then(|at| read_table(buffer, name, at, 2, 64, |r| read_array_u8::<2, _>(r)).map(|r| (at, r))));

    // milli = *(i16*)( *( *(mhfdat+0xA88) + 0x1C ) + phial*2 )
    let name = "switch axe phial strength (header 0xA88 -> +0x1C)";
    match header_target(buffer, name, SWITCH_AXE_PHIAL_STRENGTH_PTR)
        .and_then(|h| read_header_pointer(buffer, (h + 0x1C) as u32).filter(|&q| q != 0).ok_or_else(|| format!("{name}: pointer at struct+0x1C is null or outside the file")))
        .and_then(|at| read_table(buffer, name, at as usize, 10, 1, |r| {
            let mut a = [0i16; 5];
            for v in a.iter_mut() { *v = r.read_i16::<LittleEndian>()?; }
            Ok(a)
        }).map(|r| (at, r)))
    {
        Ok((at, rows)) => { o.sa_phials = Some(at); p.sa_phials = rows[0]; }
        Err(e) => problems.push(e),
    }

    let name = "weapon action descriptors (header 0x63C)";
    table!(o.motion_primary, p.motion_primary, header_target(buffer, name, WEAPON_ACTION_DESCRIPTORS_PTR)
        .and_then(|at| read_table(buffer, name, at, ACTION_DESCRIPTOR_SIZE, ACTION_DESCRIPTOR_COUNT, read_action_descriptor).map(|r| (at, r))));
    let name = "weapon action descriptors, alternate bank (header 0x74C)";
    table!(o.motion_alt, p.motion_alt, header_target(buffer, name, WEAPON_ACTION_DESCRIPTORS_ALT_PTR)
        .and_then(|at| read_table(buffer, name, at, ACTION_DESCRIPTOR_SIZE, ACTION_DESCRIPTOR_COUNT, read_action_descriptor).map(|r| (at, r))));

    let name = "Rasta NPC base entries (header 0x56C)";
    table!(o.rasta_npcs, p.rasta_npcs, important_count(buffer, name, RASTA_NPCS_COUNT_INNER).and_then(|count| {
        let at = header_target(buffer, name, RASTA_NPCS_PTR)?;
        read_table(buffer, name, at, 8, count, read_rasta_npc).map(|r| (at, r))
    }));
    let name = "Rasta weapon options (header 0x570)";
    table!(o.rasta_weapons, p.rasta_weapons, important_count(buffer, name, RASTA_WEAPONS_COUNT_INNER).and_then(|count| {
        let at = header_target(buffer, name, RASTA_WEAPONS_PTR)?;
        read_table(buffer, name, at, 8, count, read_rasta_weapon).map(|r| (at, r))
    }));
    let name = "Rasta armor sets (header 0x574)";
    table!(o.rasta_armors, p.rasta_armors, important_count(buffer, name, RASTA_ARMOR_SETS_COUNT_INNER).and_then(|count| {
        let at = header_target(buffer, name, RASTA_ARMOR_SETS_PTR)?;
        read_table(buffer, name, at, 16, count, read_rasta_armor).map(|r| (at, r))
    }));
    (p, o, problems)
}

/// `original` = the row counts when the tables were read: [thresholds, coating, tower modifiers, melodies,
/// GL pairs, motion primary, motion alt, rasta npcs, rasta weapons] then the rasta armor sets.
pub fn weapon_params_patches(p: &WeaponParams, o: &WeaponParamOffsets, original: &[usize; 10]) -> Res<Vec<InPlacePatch>> {
    let mut out = Vec::new();
    out.extend(flat_patch("weapon action thresholds", o.thresholds, original[0], &p.thresholds, |w, v| { w.extend_from_slice(v); Ok(()) })?);
    out.extend(flat_patch("weapon coating modifiers", o.coating, original[1], &p.coating, |w, c| {
        w.write_i16::<LittleEndian>(c.delta_coating)?;
        write_fixed(w, &c.unused, COATING_SIZE - 2);
        Ok(())
    })?);
    out.extend(flat_patch("tower modifiers", o.tower_modifiers, original[2], &p.tower_modifiers, |w, t| {
        w.write_i16::<LittleEndian>(t.delta_tower)?;
        w.extend_from_slice(&t.unused);
        Ok(())
    })?);
    out.extend(flat_patch("hunting horn melodies", o.melodies, original[3], &p.melodies, |w, m| {
        w.write_u8(m.song_id)?;
        w.extend_from_slice(&m.notes);
        Ok(())
    })?);
    if let Some(at) = o.gl_shell_types {
        out.push(InPlacePatch { table: "gunlance shell types".to_string(), offset: at as u64, bytes: p.gl_shell_types.to_vec() });
    }
    out.extend(flat_patch("gunlance param pairs", o.gl_param_pairs, original[4], &p.gl_param_pairs, |w, v| { w.extend_from_slice(v); Ok(()) })?);
    if let Some(at) = o.sa_phials {
        let mut bytes = Vec::new();
        for v in &p.sa_phials {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        out.push(InPlacePatch { table: "switch axe phial strength".to_string(), offset: at as u64, bytes });
    }
    out.extend(flat_patch("weapon action descriptors", o.motion_primary, original[5], &p.motion_primary, write_action_descriptor)?);
    out.extend(flat_patch("weapon action descriptors (alt)", o.motion_alt, original[6], &p.motion_alt, write_action_descriptor)?);
    out.extend(flat_patch("Rasta NPCs", o.rasta_npcs, original[7], &p.rasta_npcs, write_rasta_npc)?);
    out.extend(flat_patch("Rasta weapon options", o.rasta_weapons, original[8], &p.rasta_weapons, write_rasta_weapon)?);
    out.extend(flat_patch("Rasta armor sets", o.rasta_armors, original[9], &p.rasta_armors, write_rasta_armor)?);
    Ok(out)
}

// ---- arena and tutorial sets ----------------------------------------------------------------------------------------

fn read_item_block<R: Read>(r: &mut R) -> Result<ItemBlock> {
    Ok(ItemBlock { quantity: r.read_u16::<LittleEndian>()?, id: r.read_u16::<LittleEndian>()? })
}

fn write_item_block(w: &mut Vec<u8>, b: &ItemBlock) -> Result<()> {
    w.write_u16::<LittleEndian>(b.quantity)?;
    w.write_u16::<LittleEndian>(b.id)
}

fn read_def_block<R: Read>(r: &mut R) -> Result<DefBlock> {
    Ok(DefBlock {
        weapon_class: r.read_u16::<LittleEndian>()?,
        weapon_id: r.read_u16::<LittleEndian>()?,
        unk04: r.read_u16::<LittleEndian>()?,
        head: r.read_u16::<LittleEndian>()?,
        chest: r.read_u16::<LittleEndian>()?,
        arms: r.read_u16::<LittleEndian>()?,
        waist: r.read_u16::<LittleEndian>()?,
        legs: r.read_u16::<LittleEndian>()?,
        set_idx: r.read_u16::<LittleEndian>()?,
        multi: r.read_u16::<LittleEndian>()?,
    })
}

fn write_def_block(w: &mut Vec<u8>, d: &DefBlock) -> Result<()> {
    for v in [d.weapon_class, d.weapon_id, d.unk04, d.head, d.chest, d.arms, d.waist, d.legs, d.set_idx, d.multi] {
        w.write_u16::<LittleEndian>(v)?;
    }
    Ok(())
}

fn read_tutorial_set<R: Read>(r: &mut R) -> Result<TutorialSet> {
    Ok(TutorialSet {
        weapon_equip_type: r.read_u8()?,
        unk01: r.read_u8()?,
        weapon_id: r.read_u16::<LittleEndian>()?,
        weapon_level: r.read_u16::<LittleEndian>()?,
        head: r.read_u16::<LittleEndian>()?,
        chest: r.read_u16::<LittleEndian>()?,
        arms: r.read_u16::<LittleEndian>()?,
        waist: r.read_u16::<LittleEndian>()?,
        legs: r.read_u16::<LittleEndian>()?,
        unk10: read_array_u8::<4, _>(r)?,
    })
}

fn write_tutorial_set(w: &mut Vec<u8>, t: &TutorialSet) -> Result<()> {
    w.write_u8(t.weapon_equip_type)?;
    w.write_u8(t.unk01)?;
    for v in [t.weapon_id, t.weapon_level, t.head, t.chest, t.arms, t.waist, t.legs] {
        w.write_u16::<LittleEndian>(v)?;
    }
    w.extend_from_slice(&t.unk10);
    Ok(())
}

/// Offsets of the two stage id lists (the other arena tables are reached through pointers, see PointerBlock).
#[derive(Clone, Debug, Default)]
pub struct ArenaOffsets {
    pub stage_ids_0: Option<u32>,
    pub stage_ids_1: Option<u32>,
}

pub fn read_arena(buffer: &[u8]) -> (Arena, ArenaOffsets, Vec<String>) {
    let mut a = Arena::default();
    let mut o = ArenaOffsets::default();
    let mut problems = Vec::new();
    // an item list ends on a quantity of -1
    let item_rows = |b: &[u8], p: usize| count_rows(b, p, 4, 400, |r| r[0] == 0xFF && r[1] == 0xFF);

    let name = "arena items (header 0x144)";
    match header_target(buffer, name, ARENA_ITEMS_PTR).and_then(|at| {
        read_pointer_blocks(buffer, name, at, 68, 4, |i| format!("#{i}"), item_rows, read_item_block)
    }) {
        Ok(v) => a.items = v,
        Err(e) => problems.push(e),
    }

    let name = "arena equipment (header 0x148)";
    match header_target(buffer, name, ARENA_EQUIPMENT_PTR).and_then(|at| {
        read_pointer_blocks(buffer, name, at, 17, 20, |i| format!("#{i}"), |_, _| 4, read_def_block)
    }) {
        Ok(v) => a.equipment = v,
        Err(e) => problems.push(e),
    }

    let name = "arena ammo (header 0x158)";
    match header_target(buffer, name, ARENA_AMMO_PTR).and_then(|at| {
        read_pointer_blocks(buffer, name, at, 23, 4, |i| format!("#{i}"), item_rows, read_item_block)
    }) {
        Ok(v) => a.ammo = v,
        Err(e) => problems.push(e),
    }

    let name = "arena stage ids, chunk 0 (header 0x154)";
    match header_target(buffer, name, ARENA_STAGE_IDS_0_PTR)
        .and_then(|at| read_table(buffer, name, at, 2, 28, |r| r.read_u16::<LittleEndian>()).map(|r| (at, r)))
    {
        Ok((at, rows)) => { o.stage_ids_0 = Some(at as u32); a.stage_ids_0 = rows; }
        Err(e) => problems.push(e),
    }
    let name = "arena stage ids, chunk 1 (header 0x14C)";
    match header_target(buffer, name, ARENA_STAGE_IDS_1_PTR)
        .and_then(|at| read_table(buffer, name, at, 2, 34, |r| r.read_u16::<LittleEndian>()).map(|r| (at, r)))
    {
        Ok((at, rows)) => { o.stage_ids_1 = Some(at as u32); a.stage_ids_1 = rows; }
        Err(e) => problems.push(e),
    }

    let name = "tutorial equipment sets (header 0x15C)";
    match important_count(buffer, name, TUTORIAL_EQUIP_SETS_COUNT_INNER).and_then(|n| {
        let at = header_target(buffer, name, TUTORIAL_EQUIP_SETS_PTR)?;
        read_pointer_blocks(buffer, name, at, n, 20, |i| format!("#{i}"), |_, _| 1, read_tutorial_set)
    }) {
        Ok(v) => a.tutorial = v,
        Err(e) => problems.push(e),
    }
    (a, o, problems)
}

/// `original` = rows per list when read: items (68), ammo (23), tutorial sets; equipment blocks are always 4.
pub fn arena_patches(a: &Arena, o: &ArenaOffsets, original_items: &[usize], original_ammo: &[usize], original_tutorial: &[usize]) -> Res<Vec<InPlacePatch>> {
    let mut out = Vec::new();
    out.extend(block_patches("arena items", &a.items, original_items, write_item_block)?);
    let equipment_rows: Vec<usize> = a.equipment.iter().map(|_| 4).collect();
    out.extend(block_patches("arena equipment", &a.equipment, &equipment_rows, write_def_block)?);
    out.extend(block_patches("arena ammo", &a.ammo, original_ammo, write_item_block)?);
    out.extend(block_patches("tutorial equipment sets", &a.tutorial, original_tutorial, write_tutorial_set)?);
    out.extend(flat_patch("arena stage ids 0", o.stage_ids_0, 28, &a.stage_ids_0, |w, v| w.write_u16::<LittleEndian>(*v))?);
    out.extend(flat_patch("arena stage ids 1", o.stage_ids_1, 34, &a.stage_ids_1, |w, v| w.write_u16::<LittleEndian>(*v))?);
    Ok(out)
}

// ---- tower seals and tower upgrade costs -------------------------------------------------------------------------

const TOWER_SEAL_SIZE: usize = 0x14;
const TOWER_STEP_SIZE: usize = 24;

/// Tower seal recipes (header 0x944). They end on an item id of 0 (tower_seal_craft_fill_recipes).
pub fn read_tower_seals(buffer: &[u8]) -> Res<(u32, Vec<SigilTowerTable>)> {
    let name = "tower seal recipes (header 0x944)";
    let offset = header_target(buffer, name, TOWER_SEAL_FORGING_PTR)?;
    let count = count_rows(buffer, offset, TOWER_SEAL_SIZE, 20000, |r| r[0] == 0 && r[1] == 0);
    Ok((offset as u32, read_table(buffer, name, offset, TOWER_SEAL_SIZE, count, read_sigil_tower_entry)?))
}

pub fn tower_seals_patch(offset: Option<u32>, original_rows: usize, rows: &[SigilTowerTable]) -> Res<Option<InPlacePatch>> {
    flat_patch("tower seal recipes", offset, original_rows, rows, write_sigil_tower_table)
}

fn read_tower_step<R: Read>(r: &mut R) -> Result<TowerUpgradeStep> {
    let mut step = TowerUpgradeStep::default();
    for pair in step.ok_cost.iter_mut() {
        *pair = read_array_u8::<2, _>(r)?;
    }
    step.bow = read_array_u8::<4, _>(r)?;
    Ok(step)
}

fn write_tower_step(w: &mut Vec<u8>, s: &TowerUpgradeStep) -> Result<()> {
    for pair in &s.ok_cost {
        w.extend_from_slice(pair);
    }
    w.extend_from_slice(&s.bow);
    Ok(())
}

/// Tower upgrade cost tables (header 0x93C): a list of pointers that ends on a 0, one block of steps per pointer.
pub fn read_tower_costs(buffer: &[u8]) -> Res<Vec<PointerBlock<TowerUpgradeStep>>> {
    let name = "tower upgrade cost tables (header 0x93C)";
    let at = header_target(buffer, name, TOWER_UPGRADE_COST_TABLES_PTR)?;
    let n = count_rows(buffer, at, 4, 4096, |r| r == [0, 0, 0, 0]);
    read_pointer_blocks(
        buffer, name, at, n, TOWER_STEP_SIZE,
        |i| format!("param {i}"),
        |b, p| count_rows(b, p, TOWER_STEP_SIZE, 4096, |r| r.iter().all(|&x| x == 0)),
        read_tower_step,
    )
}

pub fn tower_costs_patches(blocks: &[PointerBlock<TowerUpgradeStep>], original_rows: &[usize]) -> Res<Vec<InPlacePatch>> {
    block_patches("tower upgrade costs", blocks, original_rows, write_tower_step)
}
