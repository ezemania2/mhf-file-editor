use std::io::{Cursor, Seek, SeekFrom};
use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::core::mhfdat::{
    read_equipments_until_sentinel,
    write_equipment,
};
use crate::model::mhfdat_pointers::{
    HEAD_ARMOR_PTR,
    BODY_ARMOR_PTR,
    ARM_ARMOR_PTR,
    WAIST_ARMOR_PTR,
    LEG_ARMOR_PTR,
};
use crate::model::mhfdat::MhfdatEquipment;
use crate::utils::skills::SKILL_LIST;

pub struct ArmorOptions {
    pub randomize_skills: bool,
    pub randomize_skill_points: bool,
    pub randomize_zenith_skills: bool,
    pub randomize_resistances: bool,
    pub randomize_slots: bool,
}

pub fn randomize_armor_buffer(buf: &mut Vec<u8>, rng: &mut ChaCha8Rng) -> Result<usize, String> {
    randomize_armor_buffer_with_options(buf, rng, &ArmorOptions {
        randomize_skills: true,
        randomize_skill_points: true,
        randomize_zenith_skills: true,
        randomize_resistances: true,
        randomize_slots: true,
    })
}

pub fn randomize_armor_buffer_with_options(buf: &mut Vec<u8>, rng: &mut ChaCha8Rng, options: &ArmorOptions) -> Result<usize, String> {
    let mut total_count = 0usize;
    
    // Randomize each armor type
    total_count += randomize_armor_type_with_options(buf, HEAD_ARMOR_PTR, "head", rng, options)?;
    total_count += randomize_armor_type_with_options(buf, BODY_ARMOR_PTR, "body", rng, options)?;
    total_count += randomize_armor_type_with_options(buf, ARM_ARMOR_PTR, "arms", rng, options)?;
    total_count += randomize_armor_type_with_options(buf, WAIST_ARMOR_PTR, "waist", rng, options)?;
    total_count += randomize_armor_type_with_options(buf, LEG_ARMOR_PTR, "legs", rng, options)?;
    
    Ok(total_count)
}

fn randomize_armor_type_with_options(buf: &mut Vec<u8>, ptr: u32, armor_type: &str, rng: &mut ChaCha8Rng, options: &ArmorOptions) -> Result<usize, String> {
    // Read armor offset from pointer, rebase VA if needed
    let armor_offset = {
        let at = ptr as usize;
        if buf.len() >= at + 4 {
            let mut off = u32::from_le_bytes([buf[at], buf[at+1], buf[at+2], buf[at+3]]);
            if (off as usize) >= buf.len() && off >= 0x0180_0000 { off -= 0x0180_0000; }
            off
        } else {
            return Err(format!("Cannot read {} armor pointer", armor_type));
        }
    };

    let mut cur = Cursor::new(&*buf);
    let armors = read_equipments_until_sentinel(&mut cur, armor_offset as u64)
        .map_err(|e| format!("read {} armor error: {}", armor_type, e))?;
    let count = armors.len();
    if count == 0 { return Ok(0); }

    let base = armor_offset as u64;
    let stride = std::mem::size_of::<MhfdatEquipment>() as u64;

    for (i, mut armor) in armors.into_iter().enumerate() {
        // Randomize skills and their points
        if options.randomize_skills || options.randomize_skill_points {
            randomize_armor_skills_with_options(&mut armor, rng, options);
        }
        
        // Randomize zenith skill
        if options.randomize_zenith_skills {
            let max_idx = crate::utils::weapon_patterns::ZENITH_SKILL_LIST.len();
            if max_idx > 0 {
                let idx = rng.gen_range(0..max_idx);
                armor.zenith_skill = crate::utils::weapon_patterns::ZENITH_SKILL_LIST[idx].0;
            }
        }
        
        // Randomize resistances (-10 to +5)
        if options.randomize_resistances {
            armor.fire_res = rng.gen_range(-10i8..=5i8);
            armor.water_res = rng.gen_range(-10i8..=5i8);
            armor.thunder_res = rng.gen_range(-10i8..=5i8);
            armor.dragon_res = rng.gen_range(-10i8..=5i8);
            armor.ice_res = rng.gen_range(-10i8..=5i8);
        }
        
        // Randomize slots (0-3)
        if options.randomize_slots {
            armor.base_slots = rng.gen_range(0u8..=3u8);
            armor.max_slots = armor.base_slots.max(rng.gen_range(armor.base_slots..=3u8));
        }

        let pos = base + (i as u64) * stride;
        let mut c = Cursor::new(buf.as_mut_slice());
        c.seek(SeekFrom::Start(pos)).map_err(|e| e.to_string())?;
        write_equipment(&mut c, &armor).map_err(|e| format!("write {} armor error: {}", armor_type, e))?;
    }
    
    Ok(count)
}

fn randomize_armor_skills_with_options(armor: &mut MhfdatEquipment, rng: &mut ChaCha8Rng, options: &ArmorOptions) {
    // Get valid skill IDs from SKILL_LIST (excluding "None" at index 0)
    let valid_skills: Vec<u8> = SKILL_LIST.iter()
        .filter(|(id, _)| *id != 0x00) // Exclude "None"
        .map(|(id, _)| *id)
        .collect();
    
    if valid_skills.is_empty() {
        return;
    }
    
    // Randomize skill 1
    if options.randomize_skills {
        armor.skill_id1 = valid_skills[rng.gen_range(0..valid_skills.len())];
    }
    if options.randomize_skill_points {
        armor.skill_pts1 = rng.gen_range(-2i8..=7i8);
    }
    
    // Randomize skill 2
    if options.randomize_skills {
        armor.skill_id2 = valid_skills[rng.gen_range(0..valid_skills.len())];
    }
    if options.randomize_skill_points {
        armor.skill_pts2 = rng.gen_range(-2i8..=7i8);
    }
    
    // Randomize skill 3
    if options.randomize_skills {
        armor.skill_id3 = valid_skills[rng.gen_range(0..valid_skills.len())];
    }
    if options.randomize_skill_points {
        armor.skill_pts3 = rng.gen_range(-2i8..=7i8);
    }
    
    // Randomize skill 4
    if options.randomize_skills {
        armor.skill_id4 = valid_skills[rng.gen_range(0..valid_skills.len())];
    }
    if options.randomize_skill_points {
        armor.skill_pts4 = rng.gen_range(-2i8..=7i8);
    }
    
    // Randomize skill 5
    if options.randomize_skills {
        armor.skill_id5 = valid_skills[rng.gen_range(0..valid_skills.len())];
    }
    if options.randomize_skill_points {
        armor.skill_pts5 = rng.gen_range(-2i8..=7i8);
    }
}
