use rand_chacha::ChaCha8Rng;
use rand::Rng;
use std::io::Result;
use crate::core::mhfdat::{read_automatic_skills, read_equipment_counts};
use crate::utils::automatic_skills::AUTO_SKILL_LIST;

pub fn randomize_auto_skills_buffer(
    buffer: &mut Vec<u8>,
    rng: &mut ChaCha8Rng,
    offset: usize,
) -> Result<usize> {
    let mut skills = read_automatic_skills(buffer, offset);
    
    let valid_skills: Vec<u16> = AUTO_SKILL_LIST.iter()
        .filter(|(id, _)| *id != 0 && *id <= 534)
        .map(|(id, _)| *id)
        .collect();
    
    if valid_skills.is_empty() {
        return Ok(0);
    }
    
    let counts = match read_equipment_counts(buffer.as_slice()) {
        Some(c) => c,
        None => {
            println!("Warning: Could not read equipment counts, aborting auto_skills randomization");
            return Ok(0);
        }
    };
    
    // Build list of valid eq_types with their max counts (only those with count > 0)
    let mut valid_eq_types: Vec<(u8, u16)> = Vec::new();
    if counts.numLegA > 0 { valid_eq_types.push((0, counts.numLegA)); }       // Legs
    if counts.numHeadA > 0 { valid_eq_types.push((2, counts.numHeadA)); }     // Head
    if counts.numBodyA > 0 { valid_eq_types.push((3, counts.numBodyA)); }     // Chest
    if counts.numArmA > 0 { valid_eq_types.push((4, counts.numArmA)); }       // Arms
    if counts.numWaistA > 0 { valid_eq_types.push((5, counts.numWaistA)); }   // Waist
    if counts.numMeleeW > 0 { valid_eq_types.push((6, counts.numMeleeW)); }   // Melee Weapon
    if counts.numRangedW > 0 { valid_eq_types.push((7, counts.numRangedW)); } // Ranged Weapon
    
    if valid_eq_types.is_empty() {
        println!("Warning: No valid equipment types found, aborting auto_skills randomization");
        return Ok(0);
    }
    
    let mut changed = 0;
    for skill in skills.iter_mut() {
        // Pick a random valid eq_type (one that has equipment)
        let (new_eq_type, max_id) = valid_eq_types[rng.gen_range(0..valid_eq_types.len())];
        skill.eq_type = new_eq_type;
        
        // Randomize equip_id within valid range [0, max_id)
        skill.equip_id = rng.gen_range(0..max_id);
        
        // Randomize skill_id
        if let Some(&skill_id) = valid_skills.get(rng.gen_range(0..valid_skills.len())) {
            skill.skill_id = skill_id;
            changed += 1;
        }
    }
    
    let mut cursor = offset;
    for skill in &skills {
        if cursor + 8 <= buffer.len() {
            buffer[cursor] = skill.is_armor as u8;
            buffer[cursor + 1] = skill.eq_type;
            buffer[cursor + 2..cursor + 4].copy_from_slice(&skill.equip_id.to_le_bytes());
            buffer[cursor + 4..cursor + 6].copy_from_slice(&skill.skill_id.to_le_bytes());
            buffer[cursor + 6..cursor + 8].copy_from_slice(&skill.padding);
            cursor += 8;
        }
    }
    
    Ok(changed)
}

