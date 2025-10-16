use std::io::{Cursor, Seek, SeekFrom};
use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::core::mhfdat::{
    read_mhfdat_offsets,
    read_melee_weapons_until_sentinel,
    read_ranged_weapons_until_sentinel,
    write_melee_weapon,
    write_ranged_weapon,
};
use crate::model::mhfdat_pointers::{
    MELEE_WEAPONS_PTR,
    RANGED_WEAPONS_PTR,
};
use crate::model::mhfdat::{MhfdatMeleeWeapon, MhfdatRangedWeapon};
use crate::utils::equip_flags::BulletTypes;

pub fn randomize_melee_buffer(buf: &mut Vec<u8>, rng: &mut ChaCha8Rng) -> Result<usize, String> {
    randomize_melee_buffer_with_options(buf, rng, &MeleeWeaponOptions {
        randomize_elements: true,
        randomize_status: true,
        randomize_sharpness: true,
        randomize_zenith_skills: true,
    })
}

pub struct MeleeWeaponOptions {
    pub randomize_elements: bool,
    pub randomize_status: bool,
    pub randomize_sharpness: bool,
    pub randomize_zenith_skills: bool,
}

pub fn randomize_melee_buffer_with_options(buf: &mut Vec<u8>, rng: &mut ChaCha8Rng, options: &MeleeWeaponOptions) -> Result<usize, String> {
    // Prefer reading the melee weapons pointer directly from header, rebasing VA if needed
    let melee_offset = {
        let at = MELEE_WEAPONS_PTR as usize;
        if buf.len() >= at + 4 {
            let mut off = u32::from_le_bytes([buf[at], buf[at+1], buf[at+2], buf[at+3]]);
            if (off as usize) >= buf.len() && off >= 0x0180_0000 { off -= 0x0180_0000; }
            off
        } else {
            // Fallback to computed offsets
            read_mhfdat_offsets(buf).ok_or("Offsets not found")?.0
        }
    };
    let mut cur = Cursor::new(&*buf);
    let weapons = read_melee_weapons_until_sentinel(&mut cur, melee_offset as u64)
        .map_err(|e| format!("read error: {}", e))?;
    let count = weapons.len();
    if count == 0 { return Ok(0); }

    let base = melee_offset as u64;
    let stride = std::mem::size_of::<MhfdatMeleeWeapon>() as u64;

    for (i, mut w) in weapons.into_iter().enumerate() {
        // randomize element kind only (keep damage)
        if options.randomize_elements {
            w.element_id = rng.gen_range(0u8..=5u8);
            // Randomize elemental damage (cap 100). If 0, force no element
            w.ele_damage = rng.gen_range(0u8..=100u8);
            if w.ele_damage == 0 { w.element_id = 0; }
        }
        
        // randomize status kind (only valid ids 0..=4) and set ailment damage within allowed caps
        if options.randomize_status {
            w.ailment_id = rng.gen_range(0u8..=4u8);
            // poison<=80, paralysis<=40, sleep<=100, blast<=100; else no ailment => 0
            let max_ail: u8 = match w.ailment_id {
                1 => 80,  // Poison
                2 => 40,  // Paralysis
                3 => 100, // Sleep
                4 => 100, // Blast/Explosion
                _ => 0,   // None/unknown -> zero damage
            };
            w.ail_damage = if max_ail == 0 { 0 } else { rng.gen_range(0u8..=max_ail) };
        }
        
        // sharpness id/max
        if options.randomize_sharpness {
            w.sharpness_id = rng.gen_range(0u8..=128u8);
            w.sharpness_max = rng.gen_range(0u8..=4u8);
        }
        
        // zenith skill within list
        if options.randomize_zenith_skills {
            let max_idx = crate::utils::weapon_patterns::ZENITH_SKILL_LIST.len();
            if max_idx > 0 {
                let idx = rng.gen_range(0..max_idx);
                w.zenith_skill = crate::utils::weapon_patterns::ZENITH_SKILL_LIST[idx].0;
            }
        }

        let pos = base + (i as u64)*stride;
        let mut c = Cursor::new(buf.as_mut_slice());
        c.seek(SeekFrom::Start(pos)).map_err(|e| e.to_string())?;
        write_melee_weapon(&mut c, &w).map_err(|e| format!("write error: {}", e))?;
    }
    Ok(count)
}

pub struct RangedWeaponOptions {
    pub randomize_elements: bool,
    pub randomize_bullet_types: bool,
    pub randomize_weapon_attributes: bool,
    pub randomize_zenith_skills: bool,
}

pub fn randomize_ranged_buffer(buf: &mut Vec<u8>, rng: &mut ChaCha8Rng) -> Result<usize, String> {
    randomize_ranged_buffer_with_options(buf, rng, &RangedWeaponOptions {
        randomize_elements: true,
        randomize_bullet_types: true,
        randomize_weapon_attributes: true,
        randomize_zenith_skills: true,
    })
}

pub fn randomize_ranged_buffer_with_options(buf: &mut Vec<u8>, rng: &mut ChaCha8Rng, options: &RangedWeaponOptions) -> Result<usize, String> {
    // Read ranged weapons pointer (rebase VA if needed), fallback to offsets table
    let ranged_offset = {
        let at = RANGED_WEAPONS_PTR as usize;
        if buf.len() >= at + 4 {
            let mut off = u32::from_le_bytes([buf[at], buf[at+1], buf[at+2], buf[at+3]]);
            if (off as usize) >= buf.len() && off >= 0x0180_0000 { off -= 0x0180_0000; }
            off
        } else {
            read_mhfdat_offsets(buf).ok_or("Offsets not found")?.1
        }
    };

    let mut cur = Cursor::new(&*buf);
    let weapons = read_ranged_weapons_until_sentinel(&mut cur, ranged_offset as u64)
        .map_err(|e| format!("read error: {}", e))?;
    let count = weapons.len();
    if count == 0 { return Ok(0); }

    let base = ranged_offset as u64;
    let stride = std::mem::size_of::<MhfdatRangedWeapon>() as u64;

    for (i, mut w) in weapons.into_iter().enumerate() {
        // Common: randomize zenith skill like melee
        if options.randomize_zenith_skills {
            let max_idx = crate::utils::weapon_patterns::ZENITH_SKILL_LIST.len();
            if max_idx > 0 {
                let idx = rng.gen_range(0..max_idx);
                w.zenith_skill = crate::utils::weapon_patterns::ZENITH_SKILL_LIST[idx].0;
            }
        }

        // Class-based behavior: class_id 0x01 HBG, 0x05 LBG, 0x0A Bow
        match w.class_id {
            0x01 | 0x05 => {
                // Heavy/Light Bowgun: randomize WeaponAttribute between 0..=44
                if options.randomize_weapon_attributes {
                    w.weapon_attribute = rng.gen_range(0u8..=44u8);
                }
                // Randomize bullet types arbitrarily
                if options.randomize_bullet_types {
                    let bt = random_bullet_types(rng);
                    w.bullet = bt.to_u32();
                }
            }
            0x0A => {
                // Bow: randomize element kind only (keep damage)
                if options.randomize_elements {
                    w.element_id = rng.gen_range(0u8..=5u8);
                    // Randomize elemental damage (cap 100). If 0, force no element
                    w.ele_damage = rng.gen_range(0u8..=100u8);
                    if w.ele_damage == 0 { w.element_id = 0; }
                }
                // Randomize bullet types as well (arbitrary flags)
                if options.randomize_bullet_types {
                    let bt = random_bullet_types(rng);
                    w.bullet = bt.to_u32();
                }
            }
            _ => {
                // Other ranged classes (if any): still update zenith above, and randomize bullet types
                if options.randomize_bullet_types {
                    let bt = random_bullet_types(rng);
                    w.bullet = bt.to_u32();
                }
            }
        }

        // Write back in-place without reallocating pointers
        let pos = base + (i as u64) * stride;
        let mut c = Cursor::new(buf.as_mut_slice());
        c.seek(SeekFrom::Start(pos)).map_err(|e| e.to_string())?;
        write_ranged_weapon(&mut c, &w).map_err(|e| format!("write error: {}", e))?;
    }

    Ok(count)
}

fn random_bullet_types(rng: &mut ChaCha8Rng) -> BulletTypes {
    // Flip each bullet flag randomly, ensuring at least one bullet type is set
    let mut bt = BulletTypes::default();
    let mut any = false;
    let mut set_flag = |flag: &mut bool| { let v = rng.gen_bool(0.5); *flag = v; if v { any = true; } };
    set_flag(&mut bt.normal_lv1);
    set_flag(&mut bt.normal_lv2);
    set_flag(&mut bt.normal_lv3);
    set_flag(&mut bt.pierce_lv1);
    set_flag(&mut bt.pierce_lv2);
    set_flag(&mut bt.pierce_lv3);
    set_flag(&mut bt.spread_lv1);
    set_flag(&mut bt.spread_lv2);
    set_flag(&mut bt.spread_lv3);
    set_flag(&mut bt.crag_lv1);
    set_flag(&mut bt.crag_lv2);
    set_flag(&mut bt.crag_lv3);
    set_flag(&mut bt.cluster_lv1);
    set_flag(&mut bt.cluster_lv2);
    set_flag(&mut bt.cluster_lv3);
    set_flag(&mut bt.fire);
    set_flag(&mut bt.water);
    set_flag(&mut bt.thunder);
    set_flag(&mut bt.ice);
    set_flag(&mut bt.dragon);
    set_flag(&mut bt.recovery_lv1);
    set_flag(&mut bt.recovery_lv2);
    set_flag(&mut bt.poison_lv1);
    set_flag(&mut bt.poison_lv2);
    set_flag(&mut bt.paralysis_lv1);
    set_flag(&mut bt.paralysis_lv2);
    set_flag(&mut bt.sleep_lv1);
    set_flag(&mut bt.sleep_lv2);
    set_flag(&mut bt.tranquilizer);
    set_flag(&mut bt.paint);
    set_flag(&mut bt.demon);
    set_flag(&mut bt.armor);
    if !any {
        // Ensure at least one flag is set
        bt.normal_lv1 = true;
    }
    bt
}
