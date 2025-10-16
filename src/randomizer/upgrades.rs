use std::io::{Cursor, Seek, SeekFrom};
use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::core::mhfdat::{
    read_mw_upgrade_until_sentinel,
    read_rw_upgrade_until_sentinel,
    write_mw_upgrade_path,
    write_rw_upgrade_path,
    read_item_offsets,
    read_mhfdat_offsets,
    read_melee_weapons_until_sentinel,
    read_ranged_weapons_until_sentinel,
};
use crate::model::mhfdat_pointers::{
    MELEE_WEAPON_UPGRADE_PATH_PTR,
    RANGED_WEAPON_UPGRADE_PATH_PTR,
    ITEM_DATA_PTR,
    MELEE_WEAPONS_PTR,
    RANGED_WEAPONS_PTR,
};
use crate::model::mhfdat::{MWUpgradePath, RWUpgradePath, MhfdatItem};

pub struct UpgradeOptions {
    pub randomize_materials: bool,
    pub randomize_targets: bool,
}

pub fn randomize_upgrades_buffer(buf: &mut Vec<u8>, rng: &mut ChaCha8Rng) -> Result<(usize, usize), String> {
    randomize_upgrades_buffer_with_options(buf, rng, &UpgradeOptions {
        randomize_materials: true,
        randomize_targets: true,
    })
}

pub fn randomize_upgrades_buffer_with_options(buf: &mut Vec<u8>, rng: &mut ChaCha8Rng, options: &UpgradeOptions) -> Result<(usize, usize), String> {
    let items = collect_obtainable_item_ids(buf);
    let mw_total = randomize_melee_upgrades_with_options(buf, &items, rng, options)?;
    let rw_total = randomize_ranged_upgrades_with_options(buf, &items, rng, options)?;
    Ok((mw_total, rw_total))
}

fn collect_obtainable_item_ids(buf: &[u8]) -> Vec<u16> {
    const MAX_ITEMS: usize = 16_700;
    // Use item data pointer or fallback to offsets; read until sentinel (0xFF 0xFF first bytes)
    let data_off = if buf.len() >= (ITEM_DATA_PTR as usize) + 4 {
        let mut off = u32::from_le_bytes([
            buf[ITEM_DATA_PTR as usize],
            buf[ITEM_DATA_PTR as usize + 1],
            buf[ITEM_DATA_PTR as usize + 2],
            buf[ITEM_DATA_PTR as usize + 3],
        ]);
        if (off as usize) >= buf.len() && off >= 0x0180_0000 { off -= 0x0180_0000; }
        off as usize
    } else {
        match read_item_offsets(buf) { Some((off, _, _)) => off as usize, None => return Vec::new() }
    };

    let mut ids = Vec::new();
    let mut cursor = data_off;
    let sz = std::mem::size_of::<MhfdatItem>();
    let bad_pattern: [u8; 36] = [
        0x00,0x00,0x03,0x63,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
    ];
    while cursor + sz <= buf.len() {
        let slice = &buf[cursor..cursor+sz];
        // Sentinel: first two bytes 0xFF 0xFF
        if slice.get(0) == Some(&0xFF) && slice.get(1) == Some(&0xFF) { break; }
        // Skip non-obtainable items matching the forbidden pattern
        if slice.len() >= bad_pattern.len() && &slice[..bad_pattern.len()] == &bad_pattern {
            cursor += sz; continue;
        }
        // Skip decorations (joyaux): deco_id != 0 at offset 0x16 (u16 LE)
        if slice.len() >= 0x18 {
            let deco_id = u16::from_le_bytes([slice[0x16], slice[0x17]]);
            if deco_id != 0 { cursor += sz; continue; }
        }
        let idx = ((cursor - data_off) / sz) as u16;
        if (idx as usize) < MAX_ITEMS {
            ids.push(idx);
        } else {
            break;
        }
        cursor += sz;
    }
    ids
}

fn randomize_melee_upgrades_with_options(buf: &mut Vec<u8>, item_ids: &[u16], rng: &mut ChaCha8Rng, options: &UpgradeOptions) -> Result<usize, String> {
    let off = if buf.len() >= (MELEE_WEAPON_UPGRADE_PATH_PTR as usize) + 4 {
        let mut p = u32::from_le_bytes([
            buf[MELEE_WEAPON_UPGRADE_PATH_PTR as usize],
            buf[MELEE_WEAPON_UPGRADE_PATH_PTR as usize + 1],
            buf[MELEE_WEAPON_UPGRADE_PATH_PTR as usize + 2],
            buf[MELEE_WEAPON_UPGRADE_PATH_PTR as usize + 3],
        ]);
        if (p as usize) >= buf.len() && p >= 0x0180_0000 { p -= 0x0180_0000; }
        p as u64
    } else {
        return Ok(0);
    };
    let mut cur = Cursor::new(&*buf);
    let entries = read_mw_upgrade_until_sentinel(&mut cur, off).map_err(|e| e.to_string())?;
    if entries.is_empty() { return Ok(0); }
    let base = off;
    let stride = std::mem::size_of::<MWUpgradePath>() as u64;
    let mut changed = 0usize;
    // Bound upgrade targets by actual melee weapon count
    let melee_count = count_melee_weapons(buf);
    for (i, mut row) in entries.into_iter().enumerate() {
        if i >= melee_count { break; }
        // Randomize materials from obtainable items
        if options.randomize_materials && !item_ids.is_empty() {
            row.upgrade_material1 = item_ids[rng.gen_range(0..item_ids.len())];
            row.upgrade_material2 = item_ids[rng.gen_range(0..item_ids.len())];
            row.upgrade_material3 = item_ids[rng.gen_range(0..item_ids.len())];
        }
        // Randomize upgrade targets within valid melee weapon indices
        if options.randomize_targets && melee_count > 0 {
            let max_idx = (melee_count - 1).min(0xFFFE as usize) as u16;
            row.upgrades_to1 = rng.gen_range(0u16..=max_idx);
            row.upgrades_to2 = rng.gen_range(0u16..=max_idx);
            row.upgrades_to3 = rng.gen_range(0u16..=max_idx);
            row.upgrades_to4 = rng.gen_range(0u16..=max_idx);
        }

        let pos = base + (i as u64) * stride;
        let mut c = Cursor::new(buf.as_mut_slice());
        c.seek(SeekFrom::Start(pos)).map_err(|e| e.to_string())?;
        write_mw_upgrade_path(&mut c, &row).map_err(|e| e.to_string())?;
        changed += 1;
    }
    Ok(changed)
}

fn randomize_ranged_upgrades_with_options(buf: &mut Vec<u8>, item_ids: &[u16], rng: &mut ChaCha8Rng, options: &UpgradeOptions) -> Result<usize, String> {
    let off = if buf.len() >= (RANGED_WEAPON_UPGRADE_PATH_PTR as usize) + 4 {
        let mut p = u32::from_le_bytes([
            buf[RANGED_WEAPON_UPGRADE_PATH_PTR as usize],
            buf[RANGED_WEAPON_UPGRADE_PATH_PTR as usize + 1],
            buf[RANGED_WEAPON_UPGRADE_PATH_PTR as usize + 2],
            buf[RANGED_WEAPON_UPGRADE_PATH_PTR as usize + 3],
        ]);
        if (p as usize) >= buf.len() && p >= 0x0180_0000 { p -= 0x0180_0000; }
        p as u64
    } else { return Ok(0); };
    let mut cur = Cursor::new(&*buf);
    let entries = read_rw_upgrade_until_sentinel(&mut cur, off).map_err(|e| e.to_string())?;
    if entries.is_empty() { return Ok(0); }
    let base = off;
    let stride = std::mem::size_of::<RWUpgradePath>() as u64;
    let mut changed = 0usize;
    // Bound upgrade targets by actual ranged weapon count
    let ranged_count = count_ranged_weapons(buf);
    for (i, mut row) in entries.into_iter().enumerate() {
        if i >= ranged_count { break; }
        if options.randomize_materials && !item_ids.is_empty() {
            row.upgrade_material1 = item_ids[rng.gen_range(0..item_ids.len())];
            row.upgrade_material2 = item_ids[rng.gen_range(0..item_ids.len())];
            row.upgrade_material3 = item_ids[rng.gen_range(0..item_ids.len())];
        }
        if options.randomize_targets && ranged_count > 0 {
            let max_idx = (ranged_count - 1).min(0xFFFE as usize) as u16;
            row.upgrades_to1 = rng.gen_range(0u16..=max_idx);
            row.upgrades_to2 = rng.gen_range(0u16..=max_idx);
            row.upgrades_to3 = rng.gen_range(0u16..=max_idx);
            row.upgrades_to4 = rng.gen_range(0u16..=max_idx);
        }

        let pos = base + (i as u64) * stride;
        let mut c = Cursor::new(buf.as_mut_slice());
        c.seek(SeekFrom::Start(pos)).map_err(|e| e.to_string())?;
        write_rw_upgrade_path(&mut c, &row).map_err(|e| e.to_string())?;
        changed += 1;
    }
    Ok(changed)
}

fn count_melee_weapons(buf: &[u8]) -> usize {
    let melee_offset = {
        let at = MELEE_WEAPONS_PTR as usize;
        if buf.len() >= at + 4 {
            let mut off = u32::from_le_bytes([buf[at], buf[at+1], buf[at+2], buf[at+3]]);
            if (off as usize) >= buf.len() && off >= 0x0180_0000 { off -= 0x0180_0000; }
            off
        } else {
            match read_mhfdat_offsets(buf) { Some((melee_off, _)) => melee_off, None => 0 }
        }
    };
    if melee_offset == 0 { return 0; }
    let mut cur = Cursor::new(buf);
    match read_melee_weapons_until_sentinel(&mut cur, melee_offset as u64) { Ok(v) => v.len(), Err(_) => 0 }
}

fn count_ranged_weapons(buf: &[u8]) -> usize {
    let ranged_offset = {
        let at = RANGED_WEAPONS_PTR as usize;
        if buf.len() >= at + 4 {
            let mut off = u32::from_le_bytes([buf[at], buf[at+1], buf[at+2], buf[at+3]]);
            if (off as usize) >= buf.len() && off >= 0x0180_0000 { off -= 0x0180_0000; }
            off
        } else {
            match read_mhfdat_offsets(buf) { Some((_, ranged_off)) => ranged_off, None => 0 }
        }
    };
    if ranged_offset == 0 { return 0; }
    let mut cur = Cursor::new(buf);
    match read_ranged_weapons_until_sentinel(&mut cur, ranged_offset as u64) { Ok(v) => v.len(), Err(_) => 0 }
}
