// Logique spécifique au format mhfdat 

use std::fs::OpenOptions;
use std::io::{Write, Seek, SeekFrom, Result, Read, Cursor};
use std::fs::File;
// use crate::model::mhfdat::MhfdatBinEntry;
use crate::model::mhfdat::{MhfdatMeleeWeapon, MhfdatRangedWeapon, ShopEntry, DecoShop, SigilTowerTable, G50WUpgrade, MWUpgradePath, RWUpgradePath, EvoUpgrade, EvoUpgradeSub, MhfdatEquipment, EquipmentCounts, MhfdatItem};
use byteorder::{ReadBytesExt, LittleEndian};
use encoding_rs::SHIFT_JIS;
use std::env;
use std::path::PathBuf;
use std::mem::size_of;
use crate::model::mhfdat_pointers::EQUIPEMENT_COUNT_PTR;
use std::ptr;

// pub fn append_to_mhfdat_bin<P: AsRef<std::path::Path>>(path: P, entry: &MhfdatBinEntry) -> Result<()> {
//     let mut file = OpenOptions::new()
//         .append(true)
//         .open(path)?;
//     file.seek(SeekFrom::End(0))?;
//     file.write_all(&entry.id.to_le_bytes())?;
//     file.write_all(&entry.value.to_le_bytes())?;
//     Ok(())
// }

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
                // C'est un premier byte de kanji, vérifier le second byte
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
                // Byte ASCII ou katakana valide
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
    // 1. Seek to the pointer table offset (names_ptr)
    reader.seek(SeekFrom::Start(names_ptr as u64))?;
    // 2. Read the pointer to the real table of string pointers
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    
    // 3. Seek to the real table
    reader.seek(SeekFrom::Start(table_offset as u64))?;
    
    // 4. For each entry, read the pointer to the string, then read the string
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
    
    // First, write all the names and collect their offsets
    for name in names {
        let offset = name_data.len() as u32;
        name_offsets.push(offset);
        
        // Encoder en Shift-JIS avec vérification
        let (sjis_bytes, _, had_errors) = SHIFT_JIS.encode(name);
        if had_errors {
            println!("Warning: Shift-JIS encoding had errors for name: {}", name);
        }
        
        // Vérifier que les bytes sont valides en Shift-JIS
        let mut valid_bytes = Vec::new();
        let mut i = 0;
        while i < sjis_bytes.len() {
            let b = sjis_bytes[i];
            if is_valid_shift_jis_byte(b) {
                if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xEF) {
                    // C'est un premier byte de kanji, vérifier le second byte
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
                    // Byte ASCII ou katakana valide
                    valid_bytes.push(b);
                }
            }
            i += 1;
        }
        
        name_data.extend_from_slice(&valid_bytes);
        name_data.push(0); // null terminator
    }
    
    // Write the name offsets table
    let table_offset = writer.seek(SeekFrom::Current(0))? as u32;
    for offset in &name_offsets {
        writer.write_all(&(offset + table_offset + (names.len() as u32 * 4)).to_le_bytes())?;
    }
    
    // Write the actual name data
    writer.write_all(&name_data)?;
    
    Ok(table_offset)
}

pub fn extract_melee_weapon_descriptions<R: Read + Seek>(
    reader: &mut R,
    desc_ptr_offset: u64,
    count: usize,
    buffer_len: usize,
) -> std::io::Result<Vec<[String; 4]>> {
    // 1. Aller à l'offset 0x8C et lire le pointeur vers la table de descriptions
    reader.seek(SeekFrom::Start(desc_ptr_offset))?;
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let desc_table_offset = u32::from_le_bytes(buf);

    // 2. Aller à la table de descriptions
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

pub fn extract_ranged_weapon_names<R: Read + Seek>(
    reader: &mut R,
    names_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<String>> {
    // 1. Seek to the pointer table offset (names_ptr)
    reader.seek(SeekFrom::Start(names_ptr as u64))?;
    // 2. Read the pointer to the real table of string pointers
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    
    // 3. Seek to the real table
    reader.seek(SeekFrom::Start(table_offset as u64))?;
    
    // 4. For each entry, read the pointer to the string, then read the string
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
    
    // First, write all the names and collect their offsets
    for name in names {
        let offset = name_data.len() as u32;
        name_offsets.push(offset);
        
        // Encoder en Shift-JIS avec vérification
        let (sjis_bytes, _, had_errors) = SHIFT_JIS.encode(name);
        if had_errors {
            println!("Warning: Shift-JIS encoding had errors for name: {}", name);
        }
        
        // Vérifier que les bytes sont valides en Shift-JIS
        let mut valid_bytes = Vec::new();
        let mut i = 0;
        while i < sjis_bytes.len() {
            let b = sjis_bytes[i];
            if is_valid_shift_jis_byte(b) {
                if (b >= 0x81 && b <= 0x9F) || (b >= 0xE0 && b <= 0xEF) {
                    // C'est un premier byte de kanji, vérifier le second byte
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
                    // Byte ASCII ou katakana valide
                    valid_bytes.push(b);
                }
            }
            i += 1;
        }
        
        name_data.extend_from_slice(&valid_bytes);
        name_data.push(0); // null terminator
    }
    
    // Write the name offsets table
    let table_offset = writer.seek(SeekFrom::Current(0))? as u32;
    for offset in &name_offsets {
        writer.write_all(&(offset + table_offset + (names.len() as u32 * 4)).to_le_bytes())?;
    }
    
    // Write the actual name data
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
        unk19: r.read_u8()?,
        unk1A: r.read_u8()?,
        base_slots: r.read_u8()?,
        max_slots: r.read_u8()?,
        sth_event_crown: r.read_u8()?,
        unk1E: r.read_u16::<LittleEndian>()?,
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
        sth_hidden: r.read_u32::<LittleEndian>()?,
        unk38: r.read_u32::<LittleEndian>()?,
        unk3C: r.read_u16::<LittleEndian>()?,
        unk3E: r.read_u8()?,
        zero_f: r.read_u8()?,
        unk40: r.read_u32::<LittleEndian>()?,
        unk44: r.read_u16::<LittleEndian>()?,
        zenith_skill: r.read_u16::<LittleEndian>()?,
    })
}

pub fn write_equipment<W: Write>(w: &mut W, eq: &MhfdatEquipment) -> Result<()> {
    w.write_all(&eq.model_id_male.to_le_bytes())?;
    w.write_all(&eq.model_id_female.to_le_bytes())?;
    w.write_all(&[eq.equipable_by])?;
    w.write_all(&[eq.rarity])?;
    w.write_all(&[eq.max_level])?;
    w.write_all(&[eq.unk07])?;
    w.write_all(&eq.unk08.to_le_bytes())?;
    w.write_all(&eq.unk0A.to_le_bytes())?;
    w.write_all(&eq.zenny_cost.to_le_bytes())?;
    w.write_all(&eq.unk10.to_le_bytes())?;
    w.write_all(&eq.base_defense.to_le_bytes())?;
    w.write_all(&[eq.fire_res as u8])?;
    w.write_all(&[eq.water_res as u8])?;
    w.write_all(&[eq.thunder_res as u8])?;
    w.write_all(&[eq.dragon_res as u8])?;
    w.write_all(&[eq.ice_res as u8])?;
    w.write_all(&[eq.unk19])?;
    w.write_all(&[eq.unk1A])?;
    w.write_all(&[eq.base_slots])?;
    w.write_all(&[eq.max_slots])?;
    w.write_all(&[eq.sth_event_crown])?;
    w.write_all(&eq.unk1E.to_le_bytes())?;
    w.write_all(&eq.equip_id.to_le_bytes())?;
    w.write_all(&eq.unk22.to_le_bytes())?;
    w.write_all(&eq.unk24.to_le_bytes())?;
    w.write_all(&eq.unk28.to_le_bytes())?;
    w.write_all(&[eq.skill_id1])?;
    w.write_all(&[eq.skill_pts1 as u8])?;
    w.write_all(&[eq.skill_id2])?;
    w.write_all(&[eq.skill_pts2 as u8])?;
    w.write_all(&[eq.skill_id3])?;
    w.write_all(&[eq.skill_pts3 as u8])?;
    w.write_all(&[eq.skill_id4])?;
    w.write_all(&[eq.skill_pts4 as u8])?;
    w.write_all(&[eq.skill_id5])?;
    w.write_all(&[eq.skill_pts5 as u8])?;
    w.write_all(&eq.sth_hidden.to_le_bytes())?;
    w.write_all(&eq.unk38.to_le_bytes())?;
    w.write_all(&eq.unk3C.to_le_bytes())?;
    w.write_all(&[eq.unk3E])?;
    w.write_all(&[eq.zero_f])?;
    w.write_all(&eq.unk40.to_le_bytes())?;
    w.write_all(&eq.unk44.to_le_bytes())?;
    w.write_all(&eq.zenith_skill.to_le_bytes())?;
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

/// Extraction des noms d'armures (head, chest, arms, waist, legs) via table de pointeurs, même format que pour les armes
pub fn extract_armor_names<R: Read + Seek>(
    reader: &mut R,
    names_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<String>> {
    // 1. Seek to the pointer table offset (names_ptr)
    reader.seek(SeekFrom::Start(names_ptr as u64))?;
    // 2. Read the pointer to the real table of string pointers
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    
    // 3. Seek to the real table
    reader.seek(SeekFrom::Start(table_offset as u64))?;
    
    // 4. For each entry, read the pointer to the string, then read the string
    let mut names = Vec::with_capacity(count);
    for _ in 0..count {
        let mut ptr_buf = [0u8; 4];
        reader.read_exact(&mut ptr_buf)?;
        let str_offset = u32::from_le_bytes(ptr_buf);
        
        if str_offset == 0 {
            names.push(String::new());
            continue;
        }
        // Save current position
        let cur = reader.seek(SeekFrom::Current(0))?;
        // Go to string offset
        reader.seek(SeekFrom::Start(str_offset as u64))?;
        // Read bytes until null terminator
        let mut bytes = Vec::new();
        let mut b = [0u8; 1];
        while reader.read_exact(&mut b).is_ok() && b[0] != 0 {
            bytes.push(b[0]);
        }
        // Clean and decode
        let cleaned_bytes = clean_shift_jis_bytes(&bytes);
        let (cow, _, _) = SHIFT_JIS.decode(&cleaned_bytes);
        names.push(cow.to_string());
        // Restore position
        reader.seek(SeekFrom::Start(cur))?;
    }
    Ok(names)
}

/// Extraction des descriptions d'armures (3 champs par entrée) via table de pointeurs, même format que pour les armes
pub fn extract_armor_descriptions<R: Read + Seek>(
    reader: &mut R,
    desc_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<[String; 3]>> {
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
        // Lire 3 pointeurs
        let mut field_ptrs = [0u32; 3];
        for i in 0..3 {
            let mut ptr_buf = [0u8; 4];
            reader.read_exact(&mut ptr_buf)?;
            field_ptrs[i] = u32::from_le_bytes(ptr_buf);
        }
        // Pour chaque champ, lire la chaîne
        let mut descs = [String::new(), String::new(), String::new()];
        for (i, &str_offset) in field_ptrs.iter().enumerate() {
            if str_offset == 0 {
                descs[i] = String::new();
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
            descs[i] = cow.to_string();
            reader.seek(SeekFrom::Start(cur))?;
        }
        all_descs.push(descs);
    }
    Ok(all_descs)
}

pub fn read_equipment_counts(buffer: &[u8]) -> Option<EquipmentCounts> {
    let mut cursor = std::io::Cursor::new(buffer);
    cursor.seek(SeekFrom::Start(EQUIPEMENT_COUNT_PTR as u64)).ok()?;
    
    let mut counts = EquipmentCounts::default();
    cursor.read_exact(unsafe { std::slice::from_raw_parts_mut(&mut counts as *mut _ as *mut u8, std::mem::size_of::<EquipmentCounts>()) }).ok()?;
    
    Some(counts)
}

pub fn write_equipment_counts(buffer: &mut [u8], counts: &EquipmentCounts) -> bool {
    let mut cursor = std::io::Cursor::new(buffer);
    // Convertir EQUIPEMENT_COUNT_PTR en u64 pour le seek
    let ptr_u64 = EQUIPEMENT_COUNT_PTR as u64;
    if cursor.seek(SeekFrom::Start(ptr_u64)).is_err() {
        return false;
    }
    
    // Écrire les données
    let data = unsafe { std::slice::from_raw_parts(counts as *const _ as *const u8, std::mem::size_of::<EquipmentCounts>()) };
    cursor.write_all(data).is_ok()
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

pub fn write_armor_names<W: Write + Seek>(writer: &mut W, names: &[String]) -> Result<u32> {
    let mut data = Vec::new();
    let mut cursor = Cursor::new(&mut data);
    
    // Write the pointer table
    let mut string_offsets = Vec::new();
    let mut strings_data = Vec::new();
    
    // Calculate string offsets
    let mut current_offset = 4 + (names.len() * 4) as u32; // 4 bytes for table pointer + 4 bytes per string pointer
    
    for name in names {
        string_offsets.push(current_offset);
        let encoded = SHIFT_JIS.encode(name).0;
        strings_data.extend_from_slice(&encoded);
        strings_data.push(0); // null terminator
        current_offset += encoded.len() as u32 + 1;
    }
    
    // Write the table pointer (points to the string pointer table)
    cursor.write_all(&(4u32).to_le_bytes())?;
    
    // Write string pointers
    for offset in string_offsets {
        cursor.write_all(&offset.to_le_bytes())?;
    }
    
    // Write the actual strings
    cursor.write_all(&strings_data)?;
    
    // Write the data to the writer
    writer.write_all(&data)?;
    
    Ok(4) // Return the offset where the table pointer was written
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

pub fn write_armor_descriptions<W: Write + Seek>(writer: &mut W, descriptions: &[[String; 3]]) -> Result<u32> {
    let mut data = Vec::new();
    let mut cursor = Cursor::new(&mut data);
    
    // Write the pointer table
    let mut field_offsets = Vec::new();
    let mut strings_data = Vec::new();
    
    // Calculate string offsets
    let mut current_offset = 4 + (descriptions.len() * 12) as u32; // 4 bytes for table pointer + 12 bytes per entry (3 pointers * 4 bytes)
    
    for desc in descriptions {
        let mut entry_offsets = Vec::new();
        for field in desc {
            entry_offsets.push(current_offset);
            let encoded = SHIFT_JIS.encode(field).0;
            strings_data.extend_from_slice(&encoded);
            strings_data.push(0); // null terminator
            current_offset += encoded.len() as u32 + 1;
        }
        field_offsets.extend(entry_offsets);
    }
    
    // Write the table pointer (points to the field pointer table)
    cursor.write_all(&(4u32).to_le_bytes())?;
    
    // Write field pointers for each entry
    for i in 0..descriptions.len() {
        for j in 0..3 {
            let offset = field_offsets[i * 3 + j];
            cursor.write_all(&offset.to_le_bytes())?;
        }
    }
    
    // Write the actual strings
    cursor.write_all(&strings_data)?;
    
    // Write the data to the writer
    writer.write_all(&data)?;
    
    Ok(4) // Return the offset where the table pointer was written
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

pub fn extract_item_names<R: Read + Seek>(
    reader: &mut R,
    names_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<String>> {
    // 1. Seek to the pointer table offset (names_ptr)
    reader.seek(SeekFrom::Start(names_ptr as u64))?;
    // 2. Read the pointer to the real table of string pointers
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    
    // 3. Seek to the real table
    reader.seek(SeekFrom::Start(table_offset as u64))?;
    
    // 4. For each entry, read the pointer to the string, then read the string
    let mut names = Vec::with_capacity(count);
    for _ in 0..count {
        let mut ptr_buf = [0u8; 4];
        reader.read_exact(&mut ptr_buf)?;
        let string_ptr = u32::from_le_bytes(ptr_buf);
        
        // Save current position
        let current_pos = reader.seek(SeekFrom::Current(0))?;
        
        // Seek to string and read it
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
        
        // Restore position
        reader.seek(SeekFrom::Start(current_pos))?;
    }
    
    Ok(names)
}

pub fn extract_item_descriptions<R: Read + Seek>(
    reader: &mut R,
    desc_ptr: u32,
    count: usize,
) -> std::io::Result<Vec<String>> {
    // 1. Seek to the pointer table offset (desc_ptr)
    reader.seek(SeekFrom::Start(desc_ptr as u64))?;
    // 2. Read the pointer to the real table of string pointers
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf)?;
    let table_offset = u32::from_le_bytes(buf);
    
    // 3. Seek to the real table (add 0x60 as per pattern)
    reader.seek(SeekFrom::Start((table_offset + 0x60) as u64))?;
    
    // 4. For each entry, read the pointer to the string, then read the string
    let mut descriptions = Vec::with_capacity(count);
    for _ in 0..count {
        let mut ptr_buf = [0u8; 4];
        reader.read_exact(&mut ptr_buf)?;
        let string_ptr = u32::from_le_bytes(ptr_buf);
        
        // Save current position
        let current_pos = reader.seek(SeekFrom::Current(0))?;
        
        // Seek to string and read it
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
        
        // Restore position
        reader.seek(SeekFrom::Start(current_pos))?;
    }
    
    Ok(descriptions)
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