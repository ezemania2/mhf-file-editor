use super::*;
use std::io::{Read, Seek, Write, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::core::packing::{compress_file, encrypt_file};
use crate::core::mhfdat::{
    write_melee_weapons_block, write_ranged_weapons_block, write_armors_block, write_items_block, write_transmog_data,
    write_mw_upgrades_block, write_rw_upgrades_block, write_deco_shop_block, write_automatic_skills_block,
    write_armor_names, write_armor_descriptions, write_item_names, write_item_descriptions, write_sharpness_data_block, write_bullet_sets_block,
    write_deco_ids_block, write_monster_descriptions_block
};
use crate::model::mhfdat_pointers::{
    MELEE_WEAPONS_PTR, RANGED_WEAPONS_PTR,
    MELEE_WEAPON_NAMES_PTR, RANGED_WEAPON_NAMES_PTR,
    MELEE_WEAPON_DESC_PTR, RANGED_WEAPON_DESC_PTR,
    HEAD_ARMOR_PTR, BODY_ARMOR_PTR, ARM_ARMOR_PTR, WAIST_ARMOR_PTR, LEG_ARMOR_PTR,
    HEAD_ARMOR_NAMES_PTR, BODY_ARMOR_NAMES_PTR, ARM_ARMOR_NAMES_PTR, WAIST_ARMOR_NAMES_PTR, LEG_ARMOR_NAMES_PTR,
    ITEM_DATA_PTR, ITEM_NAMES_PTR, ITEM_DESC_PTR, TRANSMOG_FORGING_PTR, WEAPON_FORGING_PTR, ARMOR_FORGING_PTR,
    G_RANK_WEAPON_SHOP_PTR, G_RANK_ARMOR_SHOP_PTR, ZENITH_WEAPON_FORGING_PTR, ZENITH_ARMOR_FORGING_PTR, TOWER_WEAPON_FORGING_PTR, TOWER_ARMOR_FORGING_PTR,
    DECO_SHOP_PTR, DECO_G_SHOP_PTR, CUFF_SHOP_PTR, CUFF_GR_SHOP_PTR,
    MELEE_WEAPON_UPGRADE_PATH_PTR, RANGED_WEAPON_UPGRADE_PATH_PTR,
    AUTOMATIC_SKILLS_TABLE_PTR, DECO_ID_PTR, ARMOR_UPGRADE_MATS_PTR, ARMOR_STAT_ARRAY_PTR, ARMOR_NAME_ARRAY_PTR, ARMOR_WEAPON_NAMES_ARRAY_PTR,
    MOSNTERS_DESCRIPTION_PTR, MOSNTERS_DESCRIPTION_COUNT_PTR, ARMOR_DESC_PTR,
    SHARPNESS_GREAT_SWORD_PTR, SHARPNESS_HAMMER_PTR, SHARPNESS_LANCE_PTR,
    SHARPNESS_SWORD_AND_SHIELD_PTR, SHARPNESS_DUAL_BLADES_PTR, SHARPNESS_LONG_SWORD_PTR,
    SHARPNESS_HUNTING_HORN_PTR, SHARPNESS_GUNLANCE_PTR,
    SHARPNESS_TONFA_PTR, SHARPNESS_SWITCH_AXE_PTR, SHARPNESS_MAGNET_SPIKE_PTR,
};
use crate::model::mhfdat::{ArmorStatPointers, ArmorNamePointers, ArmorWeaponNamePointers};
use std::ptr;

impl MhfdatApp {

    // Helper function to find the next known offset after a given offset
    // by checking all original offsets in the structure
    fn find_next_known_offset(&self, current_offset: u32) -> Option<u32> {
        let mut all_offsets = Vec::new();
        
        // Collect all original offsets
        all_offsets.push(self.original_melee_weapons_offset);
        all_offsets.push(self.original_ranged_weapons_offset);
        all_offsets.push(self.original_head_armors_offset);
        all_offsets.push(self.original_body_armors_offset);
        all_offsets.push(self.original_arms_armors_offset);
        all_offsets.push(self.original_waist_armors_offset);
        all_offsets.push(self.original_legs_armors_offset);
        all_offsets.push(self.original_items_offset);
        all_offsets.push(self.original_transmog_offset);
        all_offsets.push(self.original_weapon_forging_offset);
        all_offsets.push(self.original_armor_forging_offset);
        all_offsets.push(self.original_weapon_forging_gr_offset);
        all_offsets.push(self.original_armor_forging_gr_offset);
        all_offsets.push(self.original_weapon_forging_zenith_offset);
        all_offsets.push(self.original_armor_forging_zenith_offset);
        all_offsets.push(self.original_tower_weapon_forging_offset);
        all_offsets.push(self.original_tower_armor_forging_offset);
        all_offsets.push(self.original_deco_shop_hr_offset);
        all_offsets.push(self.original_deco_shop_gr_offset);
        all_offsets.push(self.original_cuff_shop_offset);
        all_offsets.push(self.original_cuff_gr_shop_offset);
        all_offsets.push(self.original_automatic_skills_offset);
        all_offsets.push(self.original_deco_ids_offset);
        all_offsets.push(self.original_mw_upgrades_offset);
        all_offsets.push(self.original_rw_upgrades_offset);
        all_offsets.push(self.original_melee_weapon_names_offset);
        all_offsets.push(self.original_melee_weapon_descriptions_offset);
        all_offsets.push(self.original_ranged_weapon_names_offset);
        all_offsets.push(self.original_ranged_weapon_descriptions_offset);
        all_offsets.push(self.original_head_armor_names_offset);
        all_offsets.push(self.original_body_armor_names_offset);
        all_offsets.push(self.original_arms_armor_names_offset);
        all_offsets.push(self.original_waist_armor_names_offset);
        all_offsets.push(self.original_legs_armor_names_offset);
        all_offsets.push(self.original_item_names_offset);
        all_offsets.push(self.original_item_descriptions_offset);
        all_offsets.push(self.original_armor_descriptions_offset);
        all_offsets.push(self.original_hr_quests_offset);
        all_offsets.push(self.original_gr_quests_offset);
        all_offsets.push(self.original_g50_melee_weapon_upgrades_offset);
        all_offsets.push(self.original_g50_ranged_weapon_upgrades_offset);
        all_offsets.push(self.original_armor_upgrade_mats_offset);
        all_offsets.push(self.original_carve_parts_offset);
        all_offsets.push(self.original_part_break_parts_offset);
        all_offsets.push(self.original_monster_descriptions_offset);
        all_offsets.push(self.original_sigil_recipes_offset);
        all_offsets.push(self.original_sigil_probabilities_offset);
        all_offsets.push(self.original_sigil_blacklists_offset);
        
        // Add sharpness offsets
        for offset in &self.original_sharpness_offsets {
            all_offsets.push(*offset);
        }
        
        // Add tower params offsets
        for offset in &self.original_g50_tower_params_offsets {
            all_offsets.push(*offset);
        }
        
        // Find the smallest offset that is greater than current_offset
        all_offsets
            .into_iter()
            .flatten()
            .filter(|&off| off > current_offset && off > 0)
            .min()
    }
    
    // Helper function to check if we can overwrite at original offset
    // by comparing the new entry count with the original entry count
    // If counts match, we can overwrite even if values changed (number of entries didn't change)
    fn can_overwrite_at_offset(&self, original_offset: Option<u32>, original_count: Option<usize>, new_count: usize) -> bool {
        eprintln!("[DEBUG] can_overwrite_at_offset: offset={:?}, orig_count={:?}, new_count={}", original_offset, original_count, new_count);
        
        if let Some(off) = original_offset {
            if off as usize >= self.buffer.len() {
                eprintln!("[DEBUG] can_overwrite: offset out of bounds");
                return false; // Offset out of bounds
            }
            
            // Use provided original entry count if available
            if let Some(orig_count) = original_count {
                // Can overwrite if new entry count matches original entry count exactly
                // This means the number of entries hasn't changed
                // We can overwrite even if the values have changed, as long as the count is the same
                let can_overwrite = new_count == orig_count && new_count > 0;
                eprintln!("[DEBUG] can_overwrite: orig_count={}, new_count={}, result={}", orig_count, new_count, can_overwrite);
                return can_overwrite;
            }
            
            // No original count available, cannot safely overwrite
            eprintln!("[DEBUG] can_overwrite: no original count available");
            false
        } else {
            eprintln!("[DEBUG] can_overwrite: no original offset");
            false
        }
    }
    
    // Helper function to write a block with overwrite optimization
    // Returns the offset where the block was written and whether pointer needs updating
    fn write_block_with_overwrite<W: Read + Seek + Write>(
        &self,
        writer: &mut W,
        original_offset: Option<u32>,
        original_count: Option<usize>,
        new_count: usize,
        new_block: &[u8],
    ) -> std::io::Result<(u32, bool)> {
        // If entry count matches (number of entries didn't change), overwrite at original offset
        // Even if values changed, we can overwrite in place
        if self.can_overwrite_at_offset(original_offset, original_count, new_count) {
            let original_offset = original_offset.unwrap() as u64;
            writer.seek(SeekFrom::Start(original_offset))?;
            writer.write_all(new_block)?;
            writer.seek(SeekFrom::End(0))?; // Return to end of file
            // Pointer stays the same, no need to update it
            Ok((original_offset as u32, false))
        } else {
            // Count changed (number of entries changed), write at end and update pointer
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            writer.write_all(new_block)?;
            // Pointer needs to be updated later
            Ok((offset, true))
        }
    }

    pub fn save_modified_data(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.current_file {
            let path = path.clone(); // Clone to avoid borrow conflicts
            
            // CRITICAL: Refresh all counts from actual data before writing
            // This ensures the counts match the real number of items being written
            self.refresh_weapon_counts_from_entries();
            self.refresh_equipment_counts_from_entries();
            
            // Ouvrir le fichier en mode read+write pour ajouter à la fin sans copier
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(&path)?;
            
            // Écrire les modifications directement à la fin du fichier
            self.save_modified_data_to_writer(file)?;

            // 6. IMPORTANT: Remplacer complètement le buffer avec le fichier sauvegardé
            // pour que les données écrites à la fin soient accessibles
            let saved_file_data = std::fs::read(&path)?;
            self.buffer = saved_file_data.clone();
            
            // 6b. Update armor descriptions offset after buffer reload
            if self.armor_descriptions_modified {
                if self.buffer.len() >= ARMOR_DESC_PTR as usize + 4 {
                    let off = u32::from_le_bytes(self.buffer[ARMOR_DESC_PTR as usize..(ARMOR_DESC_PTR + 4) as usize].try_into().unwrap());
                    self.original_armor_descriptions_offset = Some(off);
                }
                self.armor_descriptions_modified = false;
            }
            
            // 7. IMPORTANT: Mettre à jour les original_entry_counts après la sauvegarde
            // pour que les prochains saves puissent détecter si le nombre d'entries a changé
            if self.melee_weapons_modified {
                self.original_entry_counts.insert("melee_weapons".to_string(), self.melee_weapons.len());
            }
            if self.ranged_weapons_modified {
                self.original_entry_counts.insert("ranged_weapons".to_string(), self.ranged_weapons.len());
            }
            if self.head_armors_modified {
                self.original_entry_counts.insert("head_armors".to_string(), self.head_armors.len());
            }
            if self.body_armors_modified {
                self.original_entry_counts.insert("body_armors".to_string(), self.body_armors.len());
            }
            if self.arms_armors_modified {
                self.original_entry_counts.insert("arms_armors".to_string(), self.arms_armors.len());
            }
            if self.waist_armors_modified {
                self.original_entry_counts.insert("waist_armors".to_string(), self.waist_armors.len());
            }
            if self.legs_armors_modified {
                self.original_entry_counts.insert("legs_armors".to_string(), self.legs_armors.len());
            }
            if self.items_modified {
                self.original_entry_counts.insert("items".to_string(), self.items.len());
            }
            if self.transmog_modified {
                self.original_entry_counts.insert("transmog".to_string(), self.transmog_entries.len());
            }
            if self.weapon_forging_modified {
                self.original_entry_counts.insert("weapon_forging".to_string(), self.weapon_forging_entries.len());
            }
            if self.armor_forging_modified {
                self.original_entry_counts.insert("armor_forging".to_string(), self.armor_forging_entries.len());
            }
            if self.weapon_forging_gr_modified {
                self.original_entry_counts.insert("weapon_forging_gr".to_string(), self.weapon_forging_gr_entries.len());
            }
            if self.armor_forging_gr_modified {
                self.original_entry_counts.insert("armor_forging_gr".to_string(), self.armor_forging_gr_entries.len());
            }
            if self.weapon_forging_zenith_modified {
                self.original_entry_counts.insert("weapon_forging_zenith".to_string(), self.weapon_forging_zenith_entries.len());
            }
            if self.armor_forging_zenith_modified {
                self.original_entry_counts.insert("armor_forging_zenith".to_string(), self.armor_forging_zenith_entries.len());
            }
            if self.tower_weapon_forging_modified {
                self.original_entry_counts.insert("tower_weapon_forging".to_string(), self.tower_weapon_forging_entries.len());
            }
            if self.tower_armor_forging_modified {
                self.original_entry_counts.insert("tower_armor_forging".to_string(), self.tower_armor_forging_entries.len());
            }
            if self.deco_shop_hr_modified {
                self.original_entry_counts.insert("deco_shop_hr".to_string(), self.deco_shop_hr_entries.len());
            }
            if self.deco_shop_gr_modified {
                self.original_entry_counts.insert("deco_shop_gr".to_string(), self.deco_shop_gr_entries.len());
            }
            if self.deco_ids_modified {
                self.original_entry_counts.insert("deco_ids".to_string(), self.deco_ids.len());
            }
            if self.automatic_skills_modified {
                self.original_entry_counts.insert("automatic_skills".to_string(), self.automatic_skills.len());
            }
            if self.mw_upgrades_modified {
                self.original_entry_counts.insert("mw_upgrades".to_string(), self.mw_upgrade_entries.len());
            }
            if self.rw_upgrades_modified {
                self.original_entry_counts.insert("rw_upgrades".to_string(), self.rw_upgrade_entries.len());
            }
            if self.g50_melee_weapon_upgrades_modified {
                self.original_entry_counts.insert("g50_melee_weapon_upgrades".to_string(), self.g50_melee_weapon_upgrades.len());
            }
            if self.g50_ranged_weapon_upgrades_modified {
                self.original_entry_counts.insert("g50_ranged_weapon_upgrades".to_string(), self.g50_ranged_weapon_upgrades.len());
            }
            if self.melee_weapon_names_modified {
                self.original_entry_counts.insert("melee_weapon_names".to_string(), self.melee_weapon_names.len());
            }
            if self.melee_weapon_descriptions_modified {
                self.original_entry_counts.insert("melee_weapon_descriptions".to_string(), self.melee_weapon_descriptions.len());
            }
            if self.ranged_weapon_names_modified {
                self.original_entry_counts.insert("ranged_weapon_names".to_string(), self.ranged_weapon_names.len());
            }
            if self.ranged_weapon_descriptions_modified {
                self.original_entry_counts.insert("ranged_weapon_descriptions".to_string(), self.ranged_weapon_descriptions.len());
            }
            if self.head_armor_names_modified {
                self.original_entry_counts.insert("head_armor_names".to_string(), self.head_armor_names.len());
            }
            if self.body_armor_names_modified {
                self.original_entry_counts.insert("body_armor_names".to_string(), self.body_armor_names.len());
            }
            if self.arms_armor_names_modified {
                self.original_entry_counts.insert("arms_armor_names".to_string(), self.arms_armor_names.len());
            }
            if self.waist_armor_names_modified {
                self.original_entry_counts.insert("waist_armor_names".to_string(), self.waist_armor_names.len());
            }
            if self.legs_armor_names_modified {
                self.original_entry_counts.insert("legs_armor_names".to_string(), self.legs_armor_names.len());
            }
            if self.item_names_modified {
                self.original_entry_counts.insert("item_names".to_string(), self.item_names.len());
            }
            if self.item_descriptions_modified {
                self.original_entry_counts.insert("item_descriptions".to_string(), self.item_descriptions.len());
            }
            if self.armor_descriptions_modified {
                self.original_entry_counts.insert("armor_descriptions".to_string(), self.armor_descriptions.len());
            }
            if self.bullet_sets_modified {
                self.original_entry_counts.insert("bullet_sets".to_string(), self.bullet_sets.len());
            }
            if self.monster_descriptions_modified {
                self.original_entry_counts.insert("monster_descriptions".to_string(), self.monster_descriptions.len());
            }
            if self.carve_parts_modified {
                self.original_entry_counts.insert("carve_parts".to_string(), self.carve_parts.tables.len());
            }
            if self.part_break_parts_modified {
                self.original_entry_counts.insert("part_break_parts".to_string(), self.part_break_parts.tables.len());
            }
            if self.armor_upgrade_mats_modified {
                self.original_entry_counts.insert("armor_upgrade_mats".to_string(), self.armor_upgrade_mats.tables.len());
            }
            
            // Mettre à jour les original offsets pour les données modifiées
            use crate::model::mhfdat_pointers::*;
            
            // Mettre à jour les pointeurs d'armes seulement si modifiés
            if self.melee_weapons_modified && saved_file_data.len() >= (MELEE_WEAPONS_PTR + 4) as usize {
                self.buffer[MELEE_WEAPONS_PTR as usize..(MELEE_WEAPONS_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[MELEE_WEAPONS_PTR as usize..(MELEE_WEAPONS_PTR + 4) as usize]);
            }
            if self.ranged_weapons_modified && saved_file_data.len() >= (RANGED_WEAPONS_PTR + 4) as usize {
                self.buffer[RANGED_WEAPONS_PTR as usize..(RANGED_WEAPONS_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[RANGED_WEAPONS_PTR as usize..(RANGED_WEAPONS_PTR + 4) as usize]);
            }
            
            // Mettre à jour les pointeurs de noms/descriptions d'armes seulement si modifiés
            if self.melee_weapon_names_modified && saved_file_data.len() >= (MELEE_WEAPON_NAMES_PTR + 4) as usize {
                self.buffer[MELEE_WEAPON_NAMES_PTR as usize..(MELEE_WEAPON_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[MELEE_WEAPON_NAMES_PTR as usize..(MELEE_WEAPON_NAMES_PTR + 4) as usize]);
            }
            if self.melee_weapon_descriptions_modified && saved_file_data.len() >= (MELEE_WEAPON_DESC_PTR + 4) as usize {
                self.buffer[MELEE_WEAPON_DESC_PTR as usize..(MELEE_WEAPON_DESC_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[MELEE_WEAPON_DESC_PTR as usize..(MELEE_WEAPON_DESC_PTR + 4) as usize]);
            }
            if self.ranged_weapon_names_modified && saved_file_data.len() >= (RANGED_WEAPON_NAMES_PTR + 4) as usize {
                self.buffer[RANGED_WEAPON_NAMES_PTR as usize..(RANGED_WEAPON_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[RANGED_WEAPON_NAMES_PTR as usize..(RANGED_WEAPON_NAMES_PTR + 4) as usize]);
            }
            if self.ranged_weapon_descriptions_modified && saved_file_data.len() >= (RANGED_WEAPON_DESC_PTR + 4) as usize {
                self.buffer[RANGED_WEAPON_DESC_PTR as usize..(RANGED_WEAPON_DESC_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[RANGED_WEAPON_DESC_PTR as usize..(RANGED_WEAPON_DESC_PTR + 4) as usize]);
            }
            
            // Mettre à jour les pointeurs d'armures seulement si modifiés
            if self.head_armors_modified && saved_file_data.len() >= (HEAD_ARMOR_PTR + 4) as usize {
                self.buffer[HEAD_ARMOR_PTR as usize..(HEAD_ARMOR_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[HEAD_ARMOR_PTR as usize..(HEAD_ARMOR_PTR + 4) as usize]);
            }
            if self.body_armors_modified && saved_file_data.len() >= (BODY_ARMOR_PTR + 4) as usize {
                self.buffer[BODY_ARMOR_PTR as usize..(BODY_ARMOR_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[BODY_ARMOR_PTR as usize..(BODY_ARMOR_PTR + 4) as usize]);
            }
            if self.arms_armors_modified && saved_file_data.len() >= (ARM_ARMOR_PTR + 4) as usize {
                self.buffer[ARM_ARMOR_PTR as usize..(ARM_ARMOR_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ARM_ARMOR_PTR as usize..(ARM_ARMOR_PTR + 4) as usize]);
            }
            if self.waist_armors_modified && saved_file_data.len() >= (WAIST_ARMOR_PTR + 4) as usize {
                self.buffer[WAIST_ARMOR_PTR as usize..(WAIST_ARMOR_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[WAIST_ARMOR_PTR as usize..(WAIST_ARMOR_PTR + 4) as usize]);
            }
            if self.legs_armors_modified && saved_file_data.len() >= (LEG_ARMOR_PTR + 4) as usize {
                self.buffer[LEG_ARMOR_PTR as usize..(LEG_ARMOR_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[LEG_ARMOR_PTR as usize..(LEG_ARMOR_PTR + 4) as usize]);
            }

            // Mettre à jour les pointeurs de noms d'armures seulement si modifiés
            if self.head_armor_names_modified && saved_file_data.len() >= (HEAD_ARMOR_NAMES_PTR + 4) as usize {
                self.buffer[HEAD_ARMOR_NAMES_PTR as usize..(HEAD_ARMOR_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[HEAD_ARMOR_NAMES_PTR as usize..(HEAD_ARMOR_NAMES_PTR + 4) as usize]);
            }
            if self.body_armor_names_modified && saved_file_data.len() >= (BODY_ARMOR_NAMES_PTR + 4) as usize {
                self.buffer[BODY_ARMOR_NAMES_PTR as usize..(BODY_ARMOR_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[BODY_ARMOR_NAMES_PTR as usize..(BODY_ARMOR_NAMES_PTR + 4) as usize]);
            }
            if self.arms_armor_names_modified && saved_file_data.len() >= (ARM_ARMOR_NAMES_PTR + 4) as usize {
                self.buffer[ARM_ARMOR_NAMES_PTR as usize..(ARM_ARMOR_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ARM_ARMOR_NAMES_PTR as usize..(ARM_ARMOR_NAMES_PTR + 4) as usize]);
            }
            if self.waist_armor_names_modified && saved_file_data.len() >= (WAIST_ARMOR_NAMES_PTR + 4) as usize {
                self.buffer[WAIST_ARMOR_NAMES_PTR as usize..(WAIST_ARMOR_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[WAIST_ARMOR_NAMES_PTR as usize..(WAIST_ARMOR_NAMES_PTR + 4) as usize]);
            }
            if self.legs_armor_names_modified && saved_file_data.len() >= (LEG_ARMOR_NAMES_PTR + 4) as usize {
                self.buffer[LEG_ARMOR_NAMES_PTR as usize..(LEG_ARMOR_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[LEG_ARMOR_NAMES_PTR as usize..(LEG_ARMOR_NAMES_PTR + 4) as usize]);
            }
            
            // Mettre à jour les pointeurs d'objets seulement si modifiés
            if self.items_modified && saved_file_data.len() >= (ITEM_DATA_PTR + 4) as usize {
                self.buffer[ITEM_DATA_PTR as usize..(ITEM_DATA_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ITEM_DATA_PTR as usize..(ITEM_DATA_PTR + 4) as usize]);
            }
            if self.item_names_modified && saved_file_data.len() >= (ITEM_NAMES_PTR + 4) as usize {
                self.buffer[ITEM_NAMES_PTR as usize..(ITEM_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ITEM_NAMES_PTR as usize..(ITEM_NAMES_PTR + 4) as usize]);
            }
            if self.item_descriptions_modified && saved_file_data.len() >= (ITEM_DESC_PTR + 4) as usize {
                self.buffer[ITEM_DESC_PTR as usize..(ITEM_DESC_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ITEM_DESC_PTR as usize..(ITEM_DESC_PTR + 4) as usize]);
            }
            if self.monster_descriptions_modified && saved_file_data.len() >= (MOSNTERS_DESCRIPTION_PTR + 4) as usize {
                self.buffer[MOSNTERS_DESCRIPTION_PTR as usize..(MOSNTERS_DESCRIPTION_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[MOSNTERS_DESCRIPTION_PTR as usize..(MOSNTERS_DESCRIPTION_PTR + 4) as usize]);
            }
            if self.transmog_modified && saved_file_data.len() >= (TRANSMOG_FORGING_PTR + 4) as usize {
                self.buffer[TRANSMOG_FORGING_PTR as usize..(TRANSMOG_FORGING_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[TRANSMOG_FORGING_PTR as usize..(TRANSMOG_FORGING_PTR + 4) as usize]);
            }
            if self.weapon_forging_modified && saved_file_data.len() >= (WEAPON_FORGING_PTR + 4) as usize {
                self.buffer[WEAPON_FORGING_PTR as usize..(WEAPON_FORGING_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[WEAPON_FORGING_PTR as usize..(WEAPON_FORGING_PTR + 4) as usize]);
            }
            if self.armor_forging_modified && saved_file_data.len() >= (ARMOR_FORGING_PTR + 4) as usize {
                self.buffer[ARMOR_FORGING_PTR as usize..(ARMOR_FORGING_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ARMOR_FORGING_PTR as usize..(ARMOR_FORGING_PTR + 4) as usize]);
            }
            if self.weapon_forging_gr_modified && saved_file_data.len() >= (G_RANK_WEAPON_SHOP_PTR + 4) as usize {
                self.buffer[G_RANK_WEAPON_SHOP_PTR as usize..(G_RANK_WEAPON_SHOP_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[G_RANK_WEAPON_SHOP_PTR as usize..(G_RANK_WEAPON_SHOP_PTR + 4) as usize]);
            }
            if self.armor_forging_gr_modified && saved_file_data.len() >= (G_RANK_ARMOR_SHOP_PTR + 4) as usize {
                self.buffer[G_RANK_ARMOR_SHOP_PTR as usize..(G_RANK_ARMOR_SHOP_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[G_RANK_ARMOR_SHOP_PTR as usize..(G_RANK_ARMOR_SHOP_PTR + 4) as usize]);
            }
            if self.weapon_forging_zenith_modified && saved_file_data.len() >= (ZENITH_WEAPON_FORGING_PTR + 4) as usize {
                self.buffer[ZENITH_WEAPON_FORGING_PTR as usize..(ZENITH_WEAPON_FORGING_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ZENITH_WEAPON_FORGING_PTR as usize..(ZENITH_WEAPON_FORGING_PTR + 4) as usize]);
            }
            if self.tower_weapon_forging_modified && saved_file_data.len() >= (TOWER_WEAPON_FORGING_PTR + 4) as usize {
                self.buffer[TOWER_WEAPON_FORGING_PTR as usize..(TOWER_WEAPON_FORGING_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[TOWER_WEAPON_FORGING_PTR as usize..(TOWER_WEAPON_FORGING_PTR + 4) as usize]);
            }
            if self.tower_armor_forging_modified && saved_file_data.len() >= (TOWER_ARMOR_FORGING_PTR + 4) as usize {
                self.buffer[TOWER_ARMOR_FORGING_PTR as usize..(TOWER_ARMOR_FORGING_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[TOWER_ARMOR_FORGING_PTR as usize..(TOWER_ARMOR_FORGING_PTR + 4) as usize]);
            }
            if self.armor_forging_zenith_modified && saved_file_data.len() >= (ZENITH_ARMOR_FORGING_PTR + 4) as usize {
                self.buffer[ZENITH_ARMOR_FORGING_PTR as usize..(ZENITH_ARMOR_FORGING_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ZENITH_ARMOR_FORGING_PTR as usize..(ZENITH_ARMOR_FORGING_PTR + 4) as usize]);
            }
            
            // Mettre à jour les pointeurs de boutiques seulement si modifiés
            if self.deco_shop_hr_modified && saved_file_data.len() >= (DECO_SHOP_PTR + 4) as usize {
                self.buffer[DECO_SHOP_PTR as usize..(DECO_SHOP_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[DECO_SHOP_PTR as usize..(DECO_SHOP_PTR + 4) as usize]);
            }
            if self.deco_shop_gr_modified && saved_file_data.len() >= (DECO_G_SHOP_PTR + 4) as usize {
                self.buffer[DECO_G_SHOP_PTR as usize..(DECO_G_SHOP_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[DECO_G_SHOP_PTR as usize..(DECO_G_SHOP_PTR + 4) as usize]);
            }
            if self.cuff_shop_modified && saved_file_data.len() >= (CUFF_SHOP_PTR + 4) as usize {
                self.buffer[CUFF_SHOP_PTR as usize..(CUFF_SHOP_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[CUFF_SHOP_PTR as usize..(CUFF_SHOP_PTR + 4) as usize]);
            }
            if self.cuff_gr_shop_modified && saved_file_data.len() >= (CUFF_GR_SHOP_PTR + 4) as usize {
                self.buffer[CUFF_GR_SHOP_PTR as usize..(CUFF_GR_SHOP_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[CUFF_GR_SHOP_PTR as usize..(CUFF_GR_SHOP_PTR + 4) as usize]);
            }
            if self.automatic_skills_modified && saved_file_data.len() >= (AUTOMATIC_SKILLS_TABLE_PTR + 4) as usize {
                self.buffer[AUTOMATIC_SKILLS_TABLE_PTR as usize..(AUTOMATIC_SKILLS_TABLE_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[AUTOMATIC_SKILLS_TABLE_PTR as usize..(AUTOMATIC_SKILLS_TABLE_PTR + 4) as usize]);
            }
            if self.deco_ids_modified && saved_file_data.len() >= (DECO_ID_PTR + 4) as usize {
                self.buffer[DECO_ID_PTR as usize..(DECO_ID_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[DECO_ID_PTR as usize..(DECO_ID_PTR + 4) as usize]);
            }
            if self.armor_upgrade_mats_modified && saved_file_data.len() >= (ARMOR_UPGRADE_MATS_PTR + 4) as usize {
                self.buffer[ARMOR_UPGRADE_MATS_PTR as usize..(ARMOR_UPGRADE_MATS_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ARMOR_UPGRADE_MATS_PTR as usize..(ARMOR_UPGRADE_MATS_PTR + 4) as usize]);
            }
            if self.carve_parts_modified && saved_file_data.len() >= (crate::model::mhfdat_pointers::CARVE_PARTS_PTR + 4) as usize {
                self.buffer[crate::model::mhfdat_pointers::CARVE_PARTS_PTR as usize..(crate::model::mhfdat_pointers::CARVE_PARTS_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[crate::model::mhfdat_pointers::CARVE_PARTS_PTR as usize..(crate::model::mhfdat_pointers::CARVE_PARTS_PTR + 4) as usize]);
            }
            if self.carve_parts_count_modified && saved_file_data.len() >= (crate::model::mhfdat_pointers::CARVE_PARTS_COUNT_PTR + 2) as usize {
                self.buffer[crate::model::mhfdat_pointers::CARVE_PARTS_COUNT_PTR as usize..(crate::model::mhfdat_pointers::CARVE_PARTS_COUNT_PTR + 2) as usize]
                    .copy_from_slice(&saved_file_data[crate::model::mhfdat_pointers::CARVE_PARTS_COUNT_PTR as usize..(crate::model::mhfdat_pointers::CARVE_PARTS_COUNT_PTR + 2) as usize]);
            }
            if self.part_break_parts_modified && saved_file_data.len() >= (crate::model::mhfdat_pointers::PART_BREAK_DROP_PTR + 4) as usize {
                self.buffer[crate::model::mhfdat_pointers::PART_BREAK_DROP_PTR as usize..(crate::model::mhfdat_pointers::PART_BREAK_DROP_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[crate::model::mhfdat_pointers::PART_BREAK_DROP_PTR as usize..(crate::model::mhfdat_pointers::PART_BREAK_DROP_PTR + 4) as usize]);
            }
            if self.part_break_parts_count_modified && saved_file_data.len() >= (crate::model::mhfdat_pointers::PART_BREAK_DROP_COUNT_PTR + 2) as usize {
                self.buffer[crate::model::mhfdat_pointers::PART_BREAK_DROP_COUNT_PTR as usize..(crate::model::mhfdat_pointers::PART_BREAK_DROP_COUNT_PTR + 2) as usize]
                    .copy_from_slice(&saved_file_data[crate::model::mhfdat_pointers::PART_BREAK_DROP_COUNT_PTR as usize..(crate::model::mhfdat_pointers::PART_BREAK_DROP_COUNT_PTR + 2) as usize]);
            }
            if self.seasonal_events_modified && saved_file_data.len() >= (crate::model::mhfdat_pointers::SEASONAL_EVENT_PTR + 4) as usize {
                self.buffer[crate::model::mhfdat_pointers::SEASONAL_EVENT_PTR as usize..(crate::model::mhfdat_pointers::SEASONAL_EVENT_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[crate::model::mhfdat_pointers::SEASONAL_EVENT_PTR as usize..(crate::model::mhfdat_pointers::SEASONAL_EVENT_PTR + 4) as usize]);
            }
            if self.seasonal_events_count_modified && saved_file_data.len() >= (crate::model::mhfdat_pointers::SEASONAL_EVENT_COUNTER + 2) as usize {
                self.buffer[crate::model::mhfdat_pointers::SEASONAL_EVENT_COUNTER as usize..(crate::model::mhfdat_pointers::SEASONAL_EVENT_COUNTER + 2) as usize]
                    .copy_from_slice(&saved_file_data[crate::model::mhfdat_pointers::SEASONAL_EVENT_COUNTER as usize..(crate::model::mhfdat_pointers::SEASONAL_EVENT_COUNTER + 2) as usize]);
            }
            
            // Mettre à jour les pointeurs d'upgrades seulement si modifiés
            if self.mw_upgrades_modified && saved_file_data.len() >= (MELEE_WEAPON_UPGRADE_PATH_PTR + 4) as usize {
                self.buffer[MELEE_WEAPON_UPGRADE_PATH_PTR as usize..(MELEE_WEAPON_UPGRADE_PATH_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[MELEE_WEAPON_UPGRADE_PATH_PTR as usize..(MELEE_WEAPON_UPGRADE_PATH_PTR + 4) as usize]);
            }
            if self.rw_upgrades_modified && saved_file_data.len() >= (RANGED_WEAPON_UPGRADE_PATH_PTR + 4) as usize {
                self.buffer[RANGED_WEAPON_UPGRADE_PATH_PTR as usize..(RANGED_WEAPON_UPGRADE_PATH_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[RANGED_WEAPON_UPGRADE_PATH_PTR as usize..(RANGED_WEAPON_UPGRADE_PATH_PTR + 4) as usize]);
            }
            
            // Mettre à jour les original offsets depuis le buffer mis à jour
            if self.armor_forging_modified {
                let off = u32::from_le_bytes(self.buffer[ARMOR_FORGING_PTR as usize..(ARMOR_FORGING_PTR + 4) as usize].try_into().unwrap());
                self.original_armor_forging_offset = Some(off);
                self.armor_forging_modified = false;
            }
            if self.weapon_forging_modified {
                let off = u32::from_le_bytes(self.buffer[WEAPON_FORGING_PTR as usize..(WEAPON_FORGING_PTR + 4) as usize].try_into().unwrap());
                self.original_weapon_forging_offset = Some(off);
                self.weapon_forging_modified = false;
            }
            if self.armor_forging_gr_modified {
                let off = u32::from_le_bytes(self.buffer[G_RANK_ARMOR_SHOP_PTR as usize..(G_RANK_ARMOR_SHOP_PTR + 4) as usize].try_into().unwrap());
                self.original_armor_forging_gr_offset = Some(off);
                self.armor_forging_gr_modified = false;
            }
            if self.weapon_forging_gr_modified {
                let off = u32::from_le_bytes(self.buffer[G_RANK_WEAPON_SHOP_PTR as usize..(G_RANK_WEAPON_SHOP_PTR + 4) as usize].try_into().unwrap());
                self.original_weapon_forging_gr_offset = Some(off);
                self.weapon_forging_gr_modified = false;
            }
            if self.armor_forging_zenith_modified {
                let off = u32::from_le_bytes(self.buffer[ZENITH_ARMOR_FORGING_PTR as usize..(ZENITH_ARMOR_FORGING_PTR + 4) as usize].try_into().unwrap());
                self.original_armor_forging_zenith_offset = Some(off);
                self.armor_forging_zenith_modified = false;
            }
            if self.tower_weapon_forging_modified {
                let off = u32::from_le_bytes(self.buffer[TOWER_WEAPON_FORGING_PTR as usize..(TOWER_WEAPON_FORGING_PTR + 4) as usize].try_into().unwrap());
                self.original_tower_weapon_forging_offset = Some(off);
                self.tower_weapon_forging_modified = false;
            }
            if self.tower_armor_forging_modified {
                let off = u32::from_le_bytes(self.buffer[TOWER_ARMOR_FORGING_PTR as usize..(TOWER_ARMOR_FORGING_PTR + 4) as usize].try_into().unwrap());
                self.original_tower_armor_forging_offset = Some(off);
                self.tower_armor_forging_modified = false;
            }
            if self.weapon_forging_zenith_modified {
                let off = u32::from_le_bytes(self.buffer[ZENITH_WEAPON_FORGING_PTR as usize..(ZENITH_WEAPON_FORGING_PTR + 4) as usize].try_into().unwrap());
                self.original_weapon_forging_zenith_offset = Some(off);
                self.weapon_forging_zenith_modified = false;
            }
            if self.deco_ids_modified {
                let off = u32::from_le_bytes(self.buffer[DECO_ID_PTR as usize..(DECO_ID_PTR + 4) as usize].try_into().unwrap());
                self.original_deco_ids_offset = Some(off);
                self.deco_ids_modified = false;
            }
            if self.armor_upgrade_mats_modified {
                let off = u32::from_le_bytes(self.buffer[ARMOR_UPGRADE_MATS_PTR as usize..(ARMOR_UPGRADE_MATS_PTR + 4) as usize].try_into().unwrap());
                self.original_armor_upgrade_mats_offset = Some(off);
                // Update table count from actual number of tables written
                self.armor_upgrade_mats_table_count = self.armor_upgrade_mats.tables.len();
                self.armor_upgrade_mats_modified = false;
            }
            if self.carve_parts_modified {
                use crate::model::mhfdat_pointers::{CARVE_PARTS_PTR, CARVE_PARTS_COUNT_PTR};
                let off = u32::from_le_bytes(self.buffer[CARVE_PARTS_PTR as usize..(CARVE_PARTS_PTR + 4) as usize].try_into().unwrap());
                self.original_carve_parts_offset = Some(off);
                // Update count from the buffer after save (it should match the actual number of tables written)
                if self.buffer.len() >= CARVE_PARTS_COUNT_PTR as usize + 2 {
                    self.carve_parts_count = u16::from_le_bytes(
                        self.buffer[CARVE_PARTS_COUNT_PTR as usize..CARVE_PARTS_COUNT_PTR as usize + 2]
                            .try_into().unwrap()
                    );
                }
                self.carve_parts_modified = false;
            }
            if self.carve_parts_count_modified {
                self.carve_parts_count_modified = false;
            }
            if self.part_break_parts_modified {
                use crate::model::mhfdat_pointers::{PART_BREAK_DROP_PTR, PART_BREAK_DROP_COUNT_PTR};
                let off = u32::from_le_bytes(self.buffer[PART_BREAK_DROP_PTR as usize..(PART_BREAK_DROP_PTR + 4) as usize].try_into().unwrap());
                self.original_part_break_parts_offset = Some(off);
                // Update count from the buffer after save (it should match the actual number of tables written)
                if self.buffer.len() >= PART_BREAK_DROP_COUNT_PTR as usize + 2 {
                    self.part_break_parts_count = u16::from_le_bytes(
                        self.buffer[PART_BREAK_DROP_COUNT_PTR as usize..PART_BREAK_DROP_COUNT_PTR as usize + 2]
                            .try_into().unwrap()
                    );
                }
                self.part_break_parts_modified = false;
            }
            if self.part_break_parts_count_modified {
                self.part_break_parts_count_modified = false;
            }
            if self.monster_descriptions_modified {
                let off = u32::from_le_bytes(self.buffer[MOSNTERS_DESCRIPTION_PTR as usize..(MOSNTERS_DESCRIPTION_PTR + 4) as usize].try_into().unwrap());
                self.original_monster_descriptions_offset = Some(off);
                if self.buffer.len() >= MOSNTERS_DESCRIPTION_COUNT_PTR as usize + 2 {
                    self.monster_descriptions_count = u16::from_le_bytes(
                        self.buffer[MOSNTERS_DESCRIPTION_COUNT_PTR as usize..MOSNTERS_DESCRIPTION_COUNT_PTR as usize + 2]
                            .try_into().unwrap()
                    );
                }
                self.monster_descriptions_modified = false;
            }
            if self.monster_descriptions_count_modified {
                self.monster_descriptions_count_modified = false;
            }
            // Update sharpness offsets and reset modified flags
            {
                let sharpness_ptrs: [u32; 11] = [
                    SHARPNESS_GREAT_SWORD_PTR, SHARPNESS_HAMMER_PTR, SHARPNESS_LANCE_PTR,
                    SHARPNESS_SWORD_AND_SHIELD_PTR, SHARPNESS_DUAL_BLADES_PTR, SHARPNESS_LONG_SWORD_PTR,
                    SHARPNESS_HUNTING_HORN_PTR, SHARPNESS_GUNLANCE_PTR,
                    SHARPNESS_TONFA_PTR, SHARPNESS_SWITCH_AXE_PTR, SHARPNESS_MAGNET_SPIKE_PTR,
                ];
                for (i, &ptr_addr) in sharpness_ptrs.iter().enumerate() {
                    if self.sharpness_modified[i] && self.buffer.len() >= ptr_addr as usize + 4 {
                        let off = u32::from_le_bytes(
                            self.buffer[ptr_addr as usize..ptr_addr as usize + 4].try_into().unwrap()
                        );
                        self.original_sharpness_offsets[i] = Some(off);
                    }
                }
                self.sharpness_modified = [false; 11];
            }

            if self.sigil_recipes_modified {
                use crate::model::mhfdat_pointers::{
                    SIGIL_CRAFTING_RECIPES_PTR, SIGIL_SKILL_PROBABILITIES_PTR, SIGIL_SKILL_BLACKLISTS_PTR,
                };
                self.original_sigil_recipes_offset = Some(u32::from_le_bytes(
                    self.buffer[SIGIL_CRAFTING_RECIPES_PTR as usize..(SIGIL_CRAFTING_RECIPES_PTR + 4) as usize].try_into().unwrap(),
                ));
                self.original_sigil_probabilities_offset = Some(u32::from_le_bytes(
                    self.buffer[SIGIL_SKILL_PROBABILITIES_PTR as usize..(SIGIL_SKILL_PROBABILITIES_PTR + 4) as usize].try_into().unwrap(),
                ));
                self.original_sigil_blacklists_offset = Some(u32::from_le_bytes(
                    self.buffer[SIGIL_SKILL_BLACKLISTS_PTR as usize..(SIGIL_SKILL_BLACKLISTS_PTR + 4) as usize].try_into().unwrap(),
                ));
                self.original_entry_counts.insert("sigil_recipes".to_string(), self.sigil_recipes.len());
                self.sigil_recipes_modified = false;
            }

            if self.seasonal_events_modified {
                use crate::model::mhfdat_pointers::{SEASONAL_EVENT_PTR, SEASONAL_EVENT_COUNTER};
                let off = u32::from_le_bytes(self.buffer[SEASONAL_EVENT_PTR as usize..(SEASONAL_EVENT_PTR + 4) as usize].try_into().unwrap());
                self.original_seasonal_events_offset = Some(off);
                if self.buffer.len() >= SEASONAL_EVENT_COUNTER as usize + 2 {
                    self.seasonal_events_count = u16::from_le_bytes(
                        self.buffer[SEASONAL_EVENT_COUNTER as usize..SEASONAL_EVENT_COUNTER as usize + 2]
                            .try_into().unwrap()
                    );
                }
                self.seasonal_events_modified = false;
            }
            if self.seasonal_events_count_modified {
                self.seasonal_events_count_modified = false;
            }
        }
        Ok(())
    }

    pub fn save_modified_data_to_writer<W: Read + Seek + Write >(&self, mut writer: W) -> std::io::Result<()> {
        // On se place directement à la fin du fichier pour ajouter SEULEMENT les nouveaux blocs
        // SANS réécrire les données existantes
        writer.seek(SeekFrom::End(0))?;

        // NE PAS réécrire les blocs de données existants (weapons, armor, items, etc.)
        // Ces blocs sont déjà dans le fichier et on ne les modifie pas
        
        // On ajoute SEULEMENT les nouvelles tables de noms/descriptions
        
        // 1) Melee weapons data block - écrire seulement si modifié
        let (melee_data_offset, melee_ptr_needs_update) = if self.melee_weapons_modified {
            let melee_block = write_melee_weapons_block(&self.melee_weapons)?;
            let original_count = self.original_entry_counts.get("melee_weapons").copied();
            let new_count = self.melee_weapons.len();
            self.write_block_with_overwrite(
                &mut writer,
                self.original_melee_weapons_offset,
                original_count,
                new_count,
                &melee_block,
            )?
        } else {
            (self.original_melee_weapons_offset.unwrap_or(0), false)
        };

        // 2) Ranged weapons data block - écrire seulement si modifié
        let (ranged_data_offset, ranged_ptr_needs_update) = if self.ranged_weapons_modified {
            let ranged_block = write_ranged_weapons_block(&self.ranged_weapons)?;
            let original_count = self.original_entry_counts.get("ranged_weapons").copied();
            let new_count = self.ranged_weapons.len();
            self.write_block_with_overwrite(
                &mut writer,
                self.original_ranged_weapons_offset,
                original_count,
                new_count,
                &ranged_block,
            )?
        } else {
            (self.original_ranged_weapons_offset.unwrap_or(0), false)
        };

        // 3) Head armor block - écrire seulement si modifié
        let (head_armor_offset, head_ptr_needs_update) = if self.head_armors_modified {
            let head_block = write_armors_block(&self.head_armors)?;
            let original_count = self.original_entry_counts.get("head_armors").copied();
            let new_count = self.head_armors.len();
            self.write_block_with_overwrite(
                &mut writer,
                self.original_head_armors_offset,
                original_count,
                new_count,
                &head_block,
            )?
        } else {
            (self.original_head_armors_offset.unwrap_or(0), false)
        };

        // 4) Body armor block - écrire seulement si modifié
        let (body_armor_offset, body_ptr_needs_update) = if self.body_armors_modified {
            let body_block = write_armors_block(&self.body_armors)?;
            let original_count = self.original_entry_counts.get("body_armors").copied();
            let new_count = self.body_armors.len();
            self.write_block_with_overwrite(
                &mut writer,
                self.original_body_armors_offset,
                original_count,
                new_count,
                &body_block,
            )?
        } else {
            (self.original_body_armors_offset.unwrap_or(0), false)
        };

        // 5) Arms armor block - écrire seulement si modifié
        let (arms_armor_offset, arms_ptr_needs_update) = if self.arms_armors_modified {
            let arms_block = write_armors_block(&self.arms_armors)?;
            let original_count = self.original_entry_counts.get("arms_armors").copied();
            let new_count = self.arms_armors.len();
            self.write_block_with_overwrite(
                &mut writer,
                self.original_arms_armors_offset,
                original_count,
                new_count,
                &arms_block,
            )?
        } else {
            (self.original_arms_armors_offset.unwrap_or(0), false)
        };

        // 6) Waist armor block - écrire seulement si modifié
        let (waist_armor_offset, waist_ptr_needs_update) = if self.waist_armors_modified {
            let waist_block = write_armors_block(&self.waist_armors)?;
            let original_count = self.original_entry_counts.get("waist_armors").copied();
            let new_count = self.waist_armors.len();
            self.write_block_with_overwrite(
                &mut writer,
                self.original_waist_armors_offset,
                original_count,
                new_count,
                &waist_block,
            )?
        } else {
            (self.original_waist_armors_offset.unwrap_or(0), false)
        };

        // 7) Legs armor block - écrire seulement si modifié
        let (legs_armor_offset, legs_ptr_needs_update) = if self.legs_armors_modified {
            let legs_block = write_armors_block(&self.legs_armors)?;
            let original_count = self.original_entry_counts.get("legs_armors").copied();
            let new_count = self.legs_armors.len();
            self.write_block_with_overwrite(
                &mut writer,
                self.original_legs_armors_offset,
                original_count,
                new_count,
                &legs_block,
            )?
        } else {
            (self.original_legs_armors_offset.unwrap_or(0), false)
        };

        // 8) Items data block - écrire seulement si modifié
        let (item_data_offset, items_ptr_needs_update) = if self.items_modified {
            let items_block = write_items_block(&self.items)?;
            let original_count = self.original_entry_counts.get("items").copied();
            let new_count = self.items.len();
            self.write_block_with_overwrite(
                &mut writer,
                self.original_items_offset,
                original_count,
                new_count,
                &items_block,
            )?
        } else {
            (self.original_items_offset.unwrap_or(0), false)
        };

        // 9) Transmog shop data block - écrire seulement si modifié
        let (transmog_data_offset, transmog_ptr_needs_update) = if self.transmog_modified {
            let transmog_block = write_transmog_data(&self.transmog_entries)?;
            let original_count = self.original_entry_counts.get("transmog").copied();
            let new_count = self.transmog_entries.len();
            self.write_block_with_overwrite(
                &mut writer,
                self.original_transmog_offset,
                original_count,
                new_count,
                &transmog_block,
            )?
        } else {
            (self.original_transmog_offset.unwrap_or(0), false)
        };

        // 9a) Weapon forging shop data block - écrire seulement si modifié
        let (weapon_forging_data_offset, weapon_forging_ptr_needs_update) = if self.weapon_forging_modified {
            let weapon_forging_block = write_transmog_data(&self.weapon_forging_entries)?;
            let original_count = self.original_entry_counts.get("weapon_forging").copied();
            let new_count = self.weapon_forging_entries.len();
            self.write_block_with_overwrite(
                &mut writer,
                self.original_weapon_forging_offset,
                original_count,
                new_count,
                &weapon_forging_block,
            )?
        } else {
            (self.original_weapon_forging_offset.unwrap_or(0), false)
        };

        // 9a2) Armor forging shop data block - écrire seulement si modifié
        let (armor_forging_data_offset, armor_forging_ptr_needs_update) = if self.armor_forging_modified {
            let armor_forging_block = write_transmog_data(&self.armor_forging_entries)?;
            let original_count = self.original_entry_counts.get("armor_forging").copied();
            let new_count = self.armor_forging_entries.len();
            self.write_block_with_overwrite(
                &mut writer,
                self.original_armor_forging_offset,
                original_count,
                new_count,
                &armor_forging_block,
            )?
        } else {
            (self.original_armor_forging_offset.unwrap_or(0), false)
        };

        // 9a3) G-Rank Weapon forging shop data block - écrire seulement si modifié
        let weapon_forging_gr_data_offset = if self.weapon_forging_gr_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let weapon_forging_gr_block = write_transmog_data(&self.weapon_forging_gr_entries)?;
            writer.write_all(&weapon_forging_gr_block)?;
            offset
        } else {
            self.original_weapon_forging_gr_offset.unwrap_or(0)
        };

        // 9a4) G-Rank Armor forging shop data block - écrire seulement si modifié
        let armor_forging_gr_data_offset = if self.armor_forging_gr_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let armor_forging_gr_block = write_transmog_data(&self.armor_forging_gr_entries)?;
            writer.write_all(&armor_forging_gr_block)?;
            offset
        } else {
            self.original_armor_forging_gr_offset.unwrap_or(0)
        };

        // 9a5) Zenith Weapon forging shop data block - écrire seulement si modifié
        let weapon_forging_zenith_data_offset = if self.weapon_forging_zenith_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let weapon_forging_zenith_block = write_transmog_data(&self.weapon_forging_zenith_entries)?;
            writer.write_all(&weapon_forging_zenith_block)?;
            offset
        } else {
            self.original_weapon_forging_zenith_offset.unwrap_or(0)
        };

        // 9a6) Zenith Armor forging shop data block - écrire seulement si modifié
        let armor_forging_zenith_data_offset = if self.armor_forging_zenith_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let armor_forging_zenith_block = write_transmog_data(&self.armor_forging_zenith_entries)?;
            writer.write_all(&armor_forging_zenith_block)?;
            offset
        } else {
            self.original_armor_forging_zenith_offset.unwrap_or(0)
        };

        // 9a7) Tower Weapon forging shop data block - écrire seulement si modifié
        let tower_weapon_forging_data_offset = if self.tower_weapon_forging_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let tower_weapon_forging_block = write_transmog_data(&self.tower_weapon_forging_entries)?;
            writer.write_all(&tower_weapon_forging_block)?;
            offset
        } else {
            self.original_tower_weapon_forging_offset.unwrap_or(0)
        };

        // 9a8) Tower Armor forging shop data block - écrire seulement si modifié
        let tower_armor_forging_data_offset = if self.tower_armor_forging_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let tower_armor_forging_block = write_transmog_data(&self.tower_armor_forging_entries)?;
            writer.write_all(&tower_armor_forging_block)?;
            offset
        } else {
            self.original_tower_armor_forging_offset.unwrap_or(0)
        };

        // 9b) Deco shops - écrire seulement si modifié
        let deco_hr_offset = if self.deco_shop_hr_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let deco_hr_block = write_deco_shop_block(&self.deco_shop_hr_entries)?;
            writer.write_all(&deco_hr_block)?;
            offset
        } else {
            self.original_deco_shop_hr_offset.unwrap_or(0)
        };
        
        let deco_gr_offset = if self.deco_shop_gr_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let deco_gr_block = write_deco_shop_block(&self.deco_shop_gr_entries)?;
            writer.write_all(&deco_gr_block)?;
            offset
        } else {
            self.original_deco_shop_gr_offset.unwrap_or(0)
        };
        
        let cuff_offset = if self.cuff_shop_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let cuff_block = write_deco_shop_block(&self.cuff_shop_entries)?;
            writer.write_all(&cuff_block)?;
            offset
        } else {
            self.original_cuff_shop_offset.unwrap_or(0)
        };
        
        let cuff_gr_offset = if self.cuff_gr_shop_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let cuff_gr_block = write_deco_shop_block(&self.cuff_gr_shop_entries)?;
            writer.write_all(&cuff_gr_block)?;
            offset
        } else {
            self.original_cuff_gr_shop_offset.unwrap_or(0)
        };


        let automatic_skills_offset = if self.automatic_skills_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let automatic_skills_block = write_automatic_skills_block(&self.automatic_skills)?;
            writer.write_all(&automatic_skills_block)?;
            offset
        } else {
            self.original_automatic_skills_offset.unwrap_or(0)
        };

        // Deco IDs block - écrire seulement si modifié
        let deco_ids_offset = if self.deco_ids_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let deco_ids_block = write_deco_ids_block(&self.deco_ids)?;
            writer.write_all(&deco_ids_block)?;
            offset
        } else {
            self.original_deco_ids_offset.unwrap_or(0)
        };

        // 10) Melee weapon upgrade paths - écrire seulement si modifié
        let mw_upgrades_offset = if self.mw_upgrades_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let mw_block = write_mw_upgrades_block(&self.mw_upgrade_entries)?;
            writer.write_all(&mw_block)?;
            offset
        } else {
            self.original_mw_upgrades_offset.unwrap_or(0)
        };

        // 11) Ranged weapon upgrade paths - écrire seulement si modifié
        let rw_upgrades_offset = if self.rw_upgrades_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let rw_block = write_rw_upgrades_block(&self.rw_upgrade_entries)?;
            writer.write_all(&rw_block)?;
            offset
        } else {
            self.original_rw_upgrades_offset.unwrap_or(0)
        };

        // 12) Armor upgrades removed

        // 12a) Sharpness data blocks (11 melee weapon types, no bow)
        let sharpness_fields: [&Vec<crate::model::mhfdat::SharpnessItem>; 11] = [
            &self.sharpness.great_sword, &self.sharpness.hammer, &self.sharpness.lance,
            &self.sharpness.sword_and_shield, &self.sharpness.dual_blades, &self.sharpness.long_sword,
            &self.sharpness.hunting_horn, &self.sharpness.gunlance,
            &self.sharpness.tonfa, &self.sharpness.switch_axe, &self.sharpness.magnet_spike,
        ];
        let mut sharpness_offsets = [0u32; 11];
        for i in 0..11 {
            if self.sharpness_modified[i] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(sharpness_fields[i])?;
                writer.write_all(&block)?;
                sharpness_offsets[i] = offset;
            } else {
                sharpness_offsets[i] = self.original_sharpness_offsets[i].unwrap_or(0);
            }
        }

        // Update sharpness pointers for modified weapon types
        let sharpness_ptrs: [u32; 11] = [
            SHARPNESS_GREAT_SWORD_PTR, SHARPNESS_HAMMER_PTR, SHARPNESS_LANCE_PTR,
            SHARPNESS_SWORD_AND_SHIELD_PTR, SHARPNESS_DUAL_BLADES_PTR, SHARPNESS_LONG_SWORD_PTR,
            SHARPNESS_HUNTING_HORN_PTR, SHARPNESS_GUNLANCE_PTR,
            SHARPNESS_TONFA_PTR, SHARPNESS_SWITCH_AXE_PTR, SHARPNESS_MAGNET_SPIKE_PTR,
        ];
        for (i, &ptr_addr) in sharpness_ptrs.iter().enumerate() {
            if self.sharpness_modified[i] {
                writer.seek(SeekFrom::Start(ptr_addr as u64))?;
                writer.write_all(&sharpness_offsets[i].to_le_bytes())?;
            }
        }

        // 12b) Bullet Sets data block - write only if modified
        let bullet_sets_offset = if self.bullet_sets_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let block = write_bullet_sets_block(&self.bullet_sets)?;
            writer.write_all(&block)?;
            offset
        } else {
            self.original_bullet_sets_offset.unwrap_or(0)
        };

        // 12c) HR Quests data block - write only if modified
        use crate::core::mhfdat::{write_hr_quests_block, write_gr_quests_block};
        let (hr_quests_offset, hr_offsets) = if self.hr_quests_modified {
            let base_offset = writer.seek(SeekFrom::Current(0))? as u32;
            let (block, rel_offsets) = write_hr_quests_block(&self.hr_quests)?;
            writer.write_all(&block)?;
            // Calculate absolute offsets
            let abs_offsets: [u32; 6] = [
                base_offset + rel_offsets[0],
                base_offset + rel_offsets[1],
                base_offset + rel_offsets[2],
                base_offset + rel_offsets[3],
                base_offset + rel_offsets[4],
                base_offset + rel_offsets[5],
            ];
            (base_offset, abs_offsets)
        } else {
            (self.original_hr_quests_offset.unwrap_or(0), [0u32; 6])
        };

        // 12d) GR Quests data block - write only if modified
        let gr_quests_offset = if self.gr_quests_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let block = write_gr_quests_block(&self.gr_quests, offset)?;
            writer.write_all(&block)?;
            offset
        } else {
            self.original_gr_quests_offset.unwrap_or(0)
        };

        // 12e) G50 Melee Weapon Upgrades - write only if modified
        use crate::core::mhfdat::write_g50_weapon_upgrades_block;
        let g50_melee_offset = if self.g50_melee_weapon_upgrades_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let block = write_g50_weapon_upgrades_block(&self.g50_melee_weapon_upgrades)?;
            writer.write_all(&block)?;
            offset
        } else {
            self.original_g50_melee_weapon_upgrades_offset.unwrap_or(0)
        };

        // 12f) G50 Ranged Weapon Upgrades - write only if modified
        let g50_ranged_offset = if self.g50_ranged_weapon_upgrades_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let block = write_g50_weapon_upgrades_block(&self.g50_ranged_weapon_upgrades)?;
            writer.write_all(&block)?;
            offset
        } else {
            self.original_g50_ranged_weapon_upgrades_offset.unwrap_or(0)
        };

        // 13) Weapon names and descriptions tables - écrire seulement si modifié
        // Melee names
        let melee_names_count = self.melee_weapons.len().min(self.melee_weapon_names.len()).min(self.melee_weapon_descriptions.len());
        let melee_names_table_offset = if self.melee_weapon_names_modified && melee_names_count > 0 {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            let _ = crate::core::mhfdat::write_weapon_names(&mut writer, &self.melee_weapon_names[..melee_names_count])?;
            current_pos
        } else { 
            self.original_melee_weapon_names_offset.unwrap_or(0)
        };

        // Melee descriptions: table of pointers (4 per entry: 3 descriptions + 1 null) followed by SJIS strings
        let melee_desc_table_offset = if self.melee_weapon_descriptions_modified && melee_names_count > 0 {
            let table_start = writer.seek(SeekFrom::Current(0))? as u32;
            let num_ptrs = melee_names_count * 4;
            // Only 3 strings per weapon, 4th pointer is always null
            let strings_start = table_start + (num_ptrs as u32) * 4;
            // Build pointer values and string blob in memory
            let mut ptr_values: Vec<u32> = Vec::with_capacity(num_ptrs);
            let mut strings_blob: Vec<u8> = Vec::new();
            for descs in &self.melee_weapon_descriptions[..melee_names_count] {
                // Write 3 description pointers
                for desc in descs.iter().take(3) {
                    let desc_str: String = desc.chars().take(28).collect();
                    let (sjis_bytes, _, _) = encoding_rs::SHIFT_JIS.encode(&desc_str);
                    let absolute_ptr = strings_start + strings_blob.len() as u32;
                    ptr_values.push(absolute_ptr);
                    strings_blob.extend_from_slice(&sjis_bytes);
                    strings_blob.push(0);
                }
                // 4th pointer is always null (0x00000000)
                ptr_values.push(0);
            }
            // Write pointer table
            for p in ptr_values { writer.write_all(&p.to_le_bytes())?; }
            // Write strings
            writer.write_all(&strings_blob)?;
            table_start
        } else { 
            self.original_melee_weapon_descriptions_offset.unwrap_or(0)
        };

        // Ranged names
        let ranged_names_count = self.ranged_weapons.len().min(self.ranged_weapon_names.len()).min(self.ranged_weapon_descriptions.len());
        let ranged_names_table_offset = if self.ranged_weapon_names_modified && ranged_names_count > 0 {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            let _ = crate::core::mhfdat::write_ranged_weapon_names(&mut writer, &self.ranged_weapon_names[..ranged_names_count])?;
            current_pos
        } else { 
            self.original_ranged_weapon_names_offset.unwrap_or(0)
        };

        // Ranged descriptions: table of pointers (4 per entry: 3 descriptions + 1 null) followed by SJIS strings
        let ranged_desc_table_offset = if self.ranged_weapon_descriptions_modified && ranged_names_count > 0 {
            let table_start = writer.seek(SeekFrom::Current(0))? as u32;
            let num_ptrs = ranged_names_count * 4;
            // Only 3 strings per weapon, 4th pointer is always null
            let strings_start = table_start + (num_ptrs as u32) * 4;
            let mut ptr_values: Vec<u32> = Vec::with_capacity(num_ptrs);
            let mut strings_blob: Vec<u8> = Vec::new();
            for descs in &self.ranged_weapon_descriptions[..ranged_names_count] {
                // Write 3 description pointers
                for desc in descs.iter().take(3) {
                    let desc_str: String = desc.chars().take(28).collect();
                    let (sjis_bytes, _, _) = encoding_rs::SHIFT_JIS.encode(&desc_str);
                    let absolute_ptr = strings_start + strings_blob.len() as u32;
                    ptr_values.push(absolute_ptr);
                    strings_blob.extend_from_slice(&sjis_bytes);
                    strings_blob.push(0);
                }
                // 4th pointer is always null (0x00000000)
                ptr_values.push(0);
            }
            for p in ptr_values { writer.write_all(&p.to_le_bytes())?; }
            writer.write_all(&strings_blob)?;
            table_start
        } else { 
            self.original_ranged_weapon_descriptions_offset.unwrap_or(0)
        };

        // 14) Armor names tables - écrire seulement si modifié
        let head_armor_names_offset = if self.head_armor_names_modified && !self.head_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_armor_names(&mut writer, &self.head_armor_names)?;
            current_pos
        } else { 
            self.original_head_armor_names_offset.unwrap_or(0)
        };

        let body_armor_names_offset = if self.body_armor_names_modified && !self.body_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_armor_names(&mut writer, &self.body_armor_names)?;
            current_pos
        } else { 
            self.original_body_armor_names_offset.unwrap_or(0)
        };

        let arms_armor_names_offset = if self.arms_armor_names_modified && !self.arms_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_armor_names(&mut writer, &self.arms_armor_names)?;
            current_pos
        } else { 
            self.original_arms_armor_names_offset.unwrap_or(0)
        };

        let waist_armor_names_offset = if self.waist_armor_names_modified && !self.waist_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_armor_names(&mut writer, &self.waist_armor_names)?;
            current_pos
        } else { 
            self.original_waist_armor_names_offset.unwrap_or(0)
        };

        let legs_armor_names_offset = if self.legs_armor_names_modified && !self.legs_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_armor_names(&mut writer, &self.legs_armor_names)?;
            current_pos
        } else { 
            self.original_legs_armor_names_offset.unwrap_or(0)
        };

        // 15) Item names and descriptions - écrire seulement si modifié
        let item_names_count = self.items.len().min(self.item_names.len());
        let item_names_offset = if self.item_names_modified && item_names_count > 0 {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_item_names(&mut writer, &self.item_names[..item_names_count])?;
            current_pos
        } else { 
            self.original_item_names_offset.unwrap_or(0)
        };

        let item_desc_count = self.items.len().min(self.item_descriptions.len());
        let item_desc_offset = if self.item_descriptions_modified && item_desc_count > 0 {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_item_descriptions(&mut writer, &self.item_descriptions[..item_desc_count])?;
            current_pos
        } else { 
            self.original_item_descriptions_offset.unwrap_or(0)
        };
        
        
        let armor_desc_count = self.armor_descriptions.len();
        // Vérifier si au moins une description n'est pas vide
        let has_non_empty_descriptions = self.armor_descriptions.iter().any(|desc_array| {
            desc_array.iter().any(|s| !s.trim().is_empty())
        });
        
        // Calculer le nombre réel d'armures pour s'assurer qu'on n'écrit pas plus de descriptions que nécessaire
        let actual_armor_count = self.head_armors.len() + self.body_armors.len() + 
                                  self.arms_armors.len() + self.waist_armors.len() + self.legs_armors.len();
        
        // Seulement écrire les descriptions si:
        // 1. Elles ont été explicitement modifiées ET
        // 2. Il y a au moins une description non vide ET
        // 3. Le nombre de descriptions ne dépasse pas le nombre réel d'armures
        let should_write_descriptions = self.armor_descriptions_modified && 
                                         armor_desc_count > 0 && 
                                         has_non_empty_descriptions &&
                                         armor_desc_count <= actual_armor_count;
        
        let armor_desc_offset = if should_write_descriptions {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_armor_descriptions(&mut writer, &self.armor_descriptions[..armor_desc_count])?;
            current_pos
        } else {
            self.original_armor_descriptions_offset.unwrap_or(0)
        };

        // Patch header pointers - seulement si modifié
        
        if self.melee_weapons_modified && melee_ptr_needs_update {
            writer.seek(SeekFrom::Start(MELEE_WEAPONS_PTR as u64))?;
            writer.write_all(&melee_data_offset.to_le_bytes())?;
        }

        if self.ranged_weapons_modified && ranged_ptr_needs_update {
            writer.seek(SeekFrom::Start(RANGED_WEAPONS_PTR as u64))?;
            writer.write_all(&ranged_data_offset.to_le_bytes())?;
        }

        // Patch names/desc pointers
        if self.melee_weapon_names_modified {
            writer.seek(SeekFrom::Start(MELEE_WEAPON_NAMES_PTR as u64))?;
            writer.write_all(&melee_names_table_offset.to_le_bytes())?;
        }
        if self.melee_weapon_descriptions_modified {
            writer.seek(SeekFrom::Start(MELEE_WEAPON_DESC_PTR as u64))?;
            writer.write_all(&melee_desc_table_offset.to_le_bytes())?;
        }
        if self.ranged_weapon_names_modified {
            writer.seek(SeekFrom::Start(RANGED_WEAPON_NAMES_PTR as u64))?;
            writer.write_all(&ranged_names_table_offset.to_le_bytes())?;
        }
        if self.ranged_weapon_descriptions_modified {
            writer.seek(SeekFrom::Start(RANGED_WEAPON_DESC_PTR as u64))?;
            writer.write_all(&ranged_desc_table_offset.to_le_bytes())?;
        }

        if self.head_armors_modified && head_ptr_needs_update {
            writer.seek(SeekFrom::Start(HEAD_ARMOR_PTR as u64))?;
            writer.write_all(&head_armor_offset.to_le_bytes())?;
        }

        if self.body_armors_modified && body_ptr_needs_update {
            writer.seek(SeekFrom::Start(BODY_ARMOR_PTR as u64))?;
            writer.write_all(&body_armor_offset.to_le_bytes())?;
        }

        if self.arms_armors_modified && arms_ptr_needs_update {
            writer.seek(SeekFrom::Start(ARM_ARMOR_PTR as u64))?;
            writer.write_all(&arms_armor_offset.to_le_bytes())?;
        }

        if self.waist_armors_modified && waist_ptr_needs_update {
            writer.seek(SeekFrom::Start(WAIST_ARMOR_PTR as u64))?;
            writer.write_all(&waist_armor_offset.to_le_bytes())?;
        }

        if self.legs_armors_modified && legs_ptr_needs_update {
            writer.seek(SeekFrom::Start(LEG_ARMOR_PTR as u64))?;
            writer.write_all(&legs_armor_offset.to_le_bytes())?;
        }

        // Update ARMOR_STAT_ARRAY_PTR structure if any armor pointer was modified
        if self.head_armors_modified || self.body_armors_modified || self.arms_armors_modified ||
           self.waist_armors_modified || self.legs_armors_modified {
            // Read the pointer to ArmorStatPointers structure
            if ARMOR_STAT_ARRAY_PTR as usize + 4 <= self.buffer.len() {
                let armor_stat_array_ptr_offset = u32::from_le_bytes([
                    self.buffer[ARMOR_STAT_ARRAY_PTR as usize],
                    self.buffer[ARMOR_STAT_ARRAY_PTR as usize + 1],
                    self.buffer[ARMOR_STAT_ARRAY_PTR as usize + 2],
                    self.buffer[ARMOR_STAT_ARRAY_PTR as usize + 3],
                ]);
                
                // Read the current ArmorStatPointers structure
                if armor_stat_array_ptr_offset as usize + std::mem::size_of::<ArmorStatPointers>() <= self.buffer.len() {
                    let mut armor_stat_pointers = unsafe {
                        ptr::read_unaligned(
                            self.buffer[armor_stat_array_ptr_offset as usize..]
                                .as_ptr() as *const ArmorStatPointers
                        )
                    };
                    
                    // Update the corresponding fields based on which armor parts were modified
                    if self.legs_armors_modified {
                        armor_stat_pointers.legs = legs_armor_offset;
                    }
                    if self.head_armors_modified {
                        armor_stat_pointers.head1 = head_armor_offset;
                        armor_stat_pointers.head2 = head_armor_offset;
                    }
                    if self.body_armors_modified {
                        armor_stat_pointers.body = body_armor_offset;
                    }
                    if self.arms_armors_modified {
                        armor_stat_pointers.arm = arms_armor_offset;
                    }
                    if self.waist_armors_modified {
                        armor_stat_pointers.waist = waist_armor_offset;
                    }
                    
                    // Write the updated structure back
                    writer.seek(SeekFrom::Start(armor_stat_array_ptr_offset as u64))?;
                    let struct_bytes = unsafe {
                        std::slice::from_raw_parts(
                            &armor_stat_pointers as *const ArmorStatPointers as *const u8,
                            std::mem::size_of::<ArmorStatPointers>()
                        )
                    };
                    writer.write_all(struct_bytes)?;
                }
            }
        }

        if self.head_armor_names_modified {
            writer.seek(SeekFrom::Start(HEAD_ARMOR_NAMES_PTR as u64))?;
            writer.write_all(&head_armor_names_offset.to_le_bytes())?;
        }

        if self.body_armor_names_modified {
            writer.seek(SeekFrom::Start(BODY_ARMOR_NAMES_PTR as u64))?;
            writer.write_all(&body_armor_names_offset.to_le_bytes())?;
        }

        if self.arms_armor_names_modified {
            writer.seek(SeekFrom::Start(ARM_ARMOR_NAMES_PTR as u64))?;
            writer.write_all(&arms_armor_names_offset.to_le_bytes())?;
        }

        if self.waist_armor_names_modified {
            writer.seek(SeekFrom::Start(WAIST_ARMOR_NAMES_PTR as u64))?;
            writer.write_all(&waist_armor_names_offset.to_le_bytes())?;
        }

        if self.legs_armor_names_modified {
            writer.seek(SeekFrom::Start(LEG_ARMOR_NAMES_PTR as u64))?;
            writer.write_all(&legs_armor_names_offset.to_le_bytes())?;
        }

        // Update ARMOR_NAME_ARRAY_PTR structure if any armor name pointer was modified
        if self.head_armor_names_modified || self.body_armor_names_modified || self.arms_armor_names_modified ||
           self.waist_armor_names_modified || self.legs_armor_names_modified {
            // Read the pointer to ArmorNamePointers structure
            if ARMOR_NAME_ARRAY_PTR as usize + 4 <= self.buffer.len() {
                let armor_name_array_ptr_offset = u32::from_le_bytes([
                    self.buffer[ARMOR_NAME_ARRAY_PTR as usize],
                    self.buffer[ARMOR_NAME_ARRAY_PTR as usize + 1],
                    self.buffer[ARMOR_NAME_ARRAY_PTR as usize + 2],
                    self.buffer[ARMOR_NAME_ARRAY_PTR as usize + 3],
                ]);
                
                // Read the current ArmorNamePointers structure
                if armor_name_array_ptr_offset as usize + std::mem::size_of::<ArmorNamePointers>() <= self.buffer.len() {
                    let mut armor_name_pointers = unsafe {
                        ptr::read_unaligned(
                            self.buffer[armor_name_array_ptr_offset as usize..]
                                .as_ptr() as *const ArmorNamePointers
                        )
                    };
                    
                    // Update the corresponding fields based on which armor name pointers were modified
                    if self.head_armor_names_modified {
                        armor_name_pointers.head = head_armor_names_offset;
                    }
                    if self.body_armor_names_modified {
                        armor_name_pointers.body = body_armor_names_offset;
                    }
                    if self.arms_armor_names_modified {
                        armor_name_pointers.arm = arms_armor_names_offset;
                    }
                    if self.waist_armor_names_modified {
                        armor_name_pointers.waist = waist_armor_names_offset;
                    }
                    if self.legs_armor_names_modified {
                        armor_name_pointers.legs = legs_armor_names_offset;
                    }
                    
                    // Write the updated structure back
                    writer.seek(SeekFrom::Start(armor_name_array_ptr_offset as u64))?;
                    let struct_bytes = unsafe {
                        std::slice::from_raw_parts(
                            &armor_name_pointers as *const ArmorNamePointers as *const u8,
                            std::mem::size_of::<ArmorNamePointers>()
                        )
                    };
                    writer.write_all(struct_bytes)?;
                }
            }
        }

        // Update ARMOR_WEAPON_NAMES_ARRAY_PTR structure if any armor name or weapon name pointer was modified
        if self.head_armor_names_modified || self.body_armor_names_modified || self.arms_armor_names_modified ||
           self.waist_armor_names_modified || self.legs_armor_names_modified ||
           self.melee_weapon_names_modified || self.ranged_weapon_names_modified {
            // Read the pointer to ArmorWeaponNamePointers structure
            if ARMOR_WEAPON_NAMES_ARRAY_PTR as usize + 4 <= self.buffer.len() {
                let armor_weapon_names_array_ptr_offset = u32::from_le_bytes([
                    self.buffer[ARMOR_WEAPON_NAMES_ARRAY_PTR as usize],
                    self.buffer[ARMOR_WEAPON_NAMES_ARRAY_PTR as usize + 1],
                    self.buffer[ARMOR_WEAPON_NAMES_ARRAY_PTR as usize + 2],
                    self.buffer[ARMOR_WEAPON_NAMES_ARRAY_PTR as usize + 3],
                ]);
                
                // Read the current ArmorWeaponNamePointers structure
                if armor_weapon_names_array_ptr_offset as usize + std::mem::size_of::<ArmorWeaponNamePointers>() <= self.buffer.len() {
                    let mut armor_weapon_name_pointers = unsafe {
                        ptr::read_unaligned(
                            self.buffer[armor_weapon_names_array_ptr_offset as usize..]
                                .as_ptr() as *const ArmorWeaponNamePointers
                        )
                    };
                    
                    // Update the corresponding fields based on which pointers were modified
                    if self.legs_armor_names_modified {
                        armor_weapon_name_pointers.legs = legs_armor_names_offset;
                    }
                    // unknown1 is not modified - keep original value
                    if self.head_armor_names_modified {
                        armor_weapon_name_pointers.head = head_armor_names_offset;
                    }
                    if self.body_armor_names_modified {
                        armor_weapon_name_pointers.body = body_armor_names_offset;
                    }
                    if self.arms_armor_names_modified {
                        armor_weapon_name_pointers.arm = arms_armor_names_offset;
                    }
                    if self.waist_armor_names_modified {
                        armor_weapon_name_pointers.waist = waist_armor_names_offset;
                    }
                    if self.melee_weapon_names_modified {
                        armor_weapon_name_pointers.melee = melee_names_table_offset;
                    }
                    if self.ranged_weapon_names_modified {
                        armor_weapon_name_pointers.ranged = ranged_names_table_offset;
                    }
                    
                    // Write the updated structure back
                    writer.seek(SeekFrom::Start(armor_weapon_names_array_ptr_offset as u64))?;
                    let struct_bytes = unsafe {
                        std::slice::from_raw_parts(
                            &armor_weapon_name_pointers as *const ArmorWeaponNamePointers as *const u8,
                            std::mem::size_of::<ArmorWeaponNamePointers>()
                        )
                    };
                    writer.write_all(struct_bytes)?;
                }
            }
        }

        if self.items_modified && items_ptr_needs_update {
            writer.seek(SeekFrom::Start(ITEM_DATA_PTR as u64))?;
            writer.write_all(&item_data_offset.to_le_bytes())?;
        }

        if self.item_names_modified {
            writer.seek(SeekFrom::Start(ITEM_NAMES_PTR as u64))?;
            writer.write_all(&item_names_offset.to_le_bytes())?;
        }

        if self.item_descriptions_modified {
            writer.seek(SeekFrom::Start(ITEM_DESC_PTR as u64))?;
            writer.write_all(&item_desc_offset.to_le_bytes())?;
        }
        
        // Monster Descriptions - write only if modified
        if self.monster_descriptions_modified {
            use crate::model::mhfdat_pointers::{MOSNTERS_DESCRIPTION_PTR, MOSNTERS_DESCRIPTION_COUNT_PTR};
            
            // Calculate count from actual number of descriptions
            let count = self.monster_descriptions.len() as u16;
            let block = write_monster_descriptions_block(&self.monster_descriptions)?;
            
            // If table count hasn't changed, overwrite at original offset
            if count == self.monster_descriptions_count && self.original_monster_descriptions_offset.is_some() {
                let original_offset = self.original_monster_descriptions_offset.unwrap() as u64;
                writer.seek(SeekFrom::Start(original_offset))?;
                writer.write_all(&block)?;
                // Pointer stays the same, no need to update it
                // Count stays the same, no need to update it
            } else {
                // Table count changed, write at end and update pointer
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                writer.write_all(&block)?;
                
                // Update pointer
                writer.seek(SeekFrom::Start(MOSNTERS_DESCRIPTION_PTR as u64))?;
                writer.write_all(&offset.to_le_bytes())?;
                
                // Update count to match actual number of descriptions
                writer.seek(SeekFrom::Start(MOSNTERS_DESCRIPTION_COUNT_PTR as u64))?;
                writer.write_all(&count.to_le_bytes())?;
                
                writer.seek(SeekFrom::End(0))?;
            }
        }
        
        if should_write_descriptions {
            writer.seek(SeekFrom::Start(ARMOR_DESC_PTR as u64))?;
            writer.write_all(&armor_desc_offset.to_le_bytes())?;
        }

        if self.transmog_modified && transmog_ptr_needs_update {
            writer.seek(SeekFrom::Start(TRANSMOG_FORGING_PTR as u64))?;
            writer.write_all(&transmog_data_offset.to_le_bytes())?;
        }

        if self.weapon_forging_modified && weapon_forging_ptr_needs_update {
            writer.seek(SeekFrom::Start(WEAPON_FORGING_PTR as u64))?;
            writer.write_all(&weapon_forging_data_offset.to_le_bytes())?;
        }

        if self.armor_forging_modified && armor_forging_ptr_needs_update {
            writer.seek(SeekFrom::Start(ARMOR_FORGING_PTR as u64))?;
            writer.write_all(&armor_forging_data_offset.to_le_bytes())?;
        }

        if self.weapon_forging_gr_modified {
            writer.seek(SeekFrom::Start(G_RANK_WEAPON_SHOP_PTR as u64))?;
            writer.write_all(&weapon_forging_gr_data_offset.to_le_bytes())?;
        }

        if self.armor_forging_gr_modified {
            writer.seek(SeekFrom::Start(G_RANK_ARMOR_SHOP_PTR as u64))?;
            writer.write_all(&armor_forging_gr_data_offset.to_le_bytes())?;
        }

        if self.weapon_forging_zenith_modified {
            writer.seek(SeekFrom::Start(ZENITH_WEAPON_FORGING_PTR as u64))?;
            writer.write_all(&weapon_forging_zenith_data_offset.to_le_bytes())?;
        }

        if self.armor_forging_zenith_modified {
            writer.seek(SeekFrom::Start(ZENITH_ARMOR_FORGING_PTR as u64))?;
            writer.write_all(&armor_forging_zenith_data_offset.to_le_bytes())?;
        }

        if self.tower_armor_forging_modified {
            writer.seek(SeekFrom::Start(TOWER_ARMOR_FORGING_PTR as u64))?;
            writer.write_all(&tower_armor_forging_data_offset.to_le_bytes())?;
        }

        if self.tower_weapon_forging_modified {
            writer.seek(SeekFrom::Start(TOWER_WEAPON_FORGING_PTR as u64))?;
            writer.write_all(&tower_weapon_forging_data_offset.to_le_bytes())?;
        }

        // Patch deco shop pointers (HR/GR)
        if self.deco_shop_hr_modified {
            writer.seek(SeekFrom::Start(DECO_SHOP_PTR as u64))?;
            writer.write_all(&deco_hr_offset.to_le_bytes())?;
        }
        if self.deco_shop_gr_modified {
            writer.seek(SeekFrom::Start(DECO_G_SHOP_PTR as u64))?;
            writer.write_all(&deco_gr_offset.to_le_bytes())?;
        }
        if self.cuff_shop_modified {
            writer.seek(SeekFrom::Start(CUFF_SHOP_PTR as u64))?;
            writer.write_all(&cuff_offset.to_le_bytes())?;
        }
        if self.cuff_gr_shop_modified {
            writer.seek(SeekFrom::Start(CUFF_GR_SHOP_PTR as u64))?;
            writer.write_all(&cuff_gr_offset.to_le_bytes())?;
        }

        if self.automatic_skills_modified {
            writer.seek(SeekFrom::Start(AUTOMATIC_SKILLS_TABLE_PTR as u64))?;
            writer.write_all(&automatic_skills_offset.to_le_bytes())?;
        }

        // Update automatic skills count limiter if modified
        if self.automatic_skills_count_limiter_modified {
            use crate::model::mhfdat_pointers::AUTOMATIC_SKILLS_COUNT_LIMITER_PTR;
            writer.seek(SeekFrom::Start(AUTOMATIC_SKILLS_COUNT_LIMITER_PTR as u64))?;
            writer.write_all(&self.automatic_skills_count_limiter.to_le_bytes())?;
        }

        if self.mw_upgrades_modified {
            writer.seek(SeekFrom::Start(MELEE_WEAPON_UPGRADE_PATH_PTR as u64))?;
            writer.write_all(&mw_upgrades_offset.to_le_bytes())?;
        }

        if self.rw_upgrades_modified {
            writer.seek(SeekFrom::Start(RANGED_WEAPON_UPGRADE_PATH_PTR as u64))?;
            writer.write_all(&rw_upgrades_offset.to_le_bytes())?;
        }

        // armor upgrades pointer removed

        // Update bullet sets pointer if modified
        if self.bullet_sets_modified {
            use crate::model::mhfdat_pointers::BULLET_SETS_PTR;
            writer.seek(SeekFrom::Start(BULLET_SETS_PTR as u64))?;
            writer.write_all(&bullet_sets_offset.to_le_bytes())?;
        }

        // Update deco count limiter if modified
        if self.deco_id_count_limiter_modified {
            use crate::model::mhfdat_pointers::DECO_ID_COUNT_LIMITER_PTR;
            writer.seek(SeekFrom::Start(DECO_ID_COUNT_LIMITER_PTR as u64))?;
            writer.write_all(&self.deco_id_count_limiter.to_le_bytes())?;
        }

        // Update deco IDs pointer if modified
        if self.deco_ids_modified {
            writer.seek(SeekFrom::Start(DECO_ID_PTR as u64))?;
            writer.write_all(&deco_ids_offset.to_le_bytes())?;
        }

        // Update HR quests pointers if modified
        if self.hr_quests_modified {
            use crate::model::mhfdat_pointers::HR_QUEST_LIST_PTR;
            writer.seek(SeekFrom::Start(HR_QUEST_LIST_PTR as u64))?;
            for off in &hr_offsets {
                writer.write_all(&off.to_le_bytes())?;
            }
        }

        // Update GR quests pointer if modified
        // GR_QUEST_LIST_PTR contains the address of G7 data directly
        if self.gr_quests_modified {
            use crate::model::mhfdat_pointers::GR_QUEST_LIST_PTR;
            writer.seek(SeekFrom::Start(GR_QUEST_LIST_PTR as u64))?;
            writer.write_all(&gr_quests_offset.to_le_bytes())?;
        }

        // Update G50 Melee Weapon Upgrades pointer and limiter if modified
        if self.g50_melee_weapon_upgrades_modified {
            use crate::model::mhfdat_pointers::G50_MELEE_WEAPON_UPGRADE_PTR;
            writer.seek(SeekFrom::Start(G50_MELEE_WEAPON_UPGRADE_PTR as u64))?;
            writer.write_all(&g50_melee_offset.to_le_bytes())?;
        }
        if self.g50_melee_count_limiter_modified {
            use crate::model::mhfdat_pointers::G50_MELEE_WEAPON_UPGRADE_COUNT_LIMITER_PTR;
            writer.seek(SeekFrom::Start(G50_MELEE_WEAPON_UPGRADE_COUNT_LIMITER_PTR as u64))?;
            writer.write_all(&self.g50_melee_count_limiter.to_le_bytes())?;
        }

        // Update G50 Ranged Weapon Upgrades pointer and limiter if modified
        if self.g50_ranged_weapon_upgrades_modified {
            use crate::model::mhfdat_pointers::G50_RANGED_WEAPON_UPGRADE_PTR;
            writer.seek(SeekFrom::Start(G50_RANGED_WEAPON_UPGRADE_PTR as u64))?;
            writer.write_all(&g50_ranged_offset.to_le_bytes())?;
        }
        if self.g50_ranged_count_limiter_modified {
            use crate::model::mhfdat_pointers::G50_RANGED_WEAPON_UPGRADE_COUNT_LIMITER_PTR;
            writer.seek(SeekFrom::Start(G50_RANGED_WEAPON_UPGRADE_COUNT_LIMITER_PTR as u64))?;
            writer.write_all(&self.g50_ranged_count_limiter.to_le_bytes())?;
        }

        // G50 Tower Params - write data blocks and update pointers if modified
        use crate::model::mhfdat_pointers::*;
        use crate::core::mhfdat::write_tower_g50_weapon_type;
        
        let tower_ptrs = [
            SWORD_AND_SHIELD_G50_TOWER_PARAMS_PTR,
            DUAL_BLADES_G50_TOWER_PARAMS_PTR,
            GREAT_SWORD_G50_TOWER_PARAMS_PTR,
            LONG_SWORD_G50_TOWER_PARAMS_PTR,
            LANCE_G50_TOWER_PARAMS_PTR,
            GUNLANCE_G50_TOWER_PARAMS_PTR,
            HAMMER_G50_TOWER_PARAMS_PTR,
            HUNTING_HORN_G50_TOWER_PARAMS_PTR,
            HEAVY_BOWGUN_G50_TOWER_PARAMS_PTR,
            LIGHT_BOWGUN_G50_TOWER_PARAMS_PTR,
            BOW_G50_TOWER_PARAMS_PTR,
            TONFA_G50_TOWER_PARAMS_PTR,
            SWITCH_AXE_G50_TOWER_PARAMS_PTR,
            MAGNET_SPIKE_G50_TOWER_PARAMS_PTR,
        ];
        
        for i in 0..14 {
            if self.g50_tower_params_modified[i] {
                let base_offset = writer.seek(SeekFrom::Current(0))? as u32;
                let (ptr_table, data_block) = write_tower_g50_weapon_type(&self.g50_tower_params[i], base_offset)?;
                writer.write_all(&ptr_table)?;
                writer.write_all(&data_block)?;
                // Update main pointer
                writer.seek(SeekFrom::Start(tower_ptrs[i] as u64))?;
                writer.write_all(&base_offset.to_le_bytes())?;
                writer.seek(SeekFrom::End(0))?;
            }
        }

        // Armor Upgrade Materials - write only if modified
        if self.armor_upgrade_mats_modified {
            use crate::core::mhfdat::write_armor_upgrade_mats_block;
            
            let current_table_count = self.armor_upgrade_mats.tables.len();
            let block = write_armor_upgrade_mats_block(&self.armor_upgrade_mats)?;
            
            // If table count hasn't changed, overwrite at original offset
            if current_table_count == self.armor_upgrade_mats_table_count 
                && self.original_armor_upgrade_mats_offset.is_some() {
                let original_offset = self.original_armor_upgrade_mats_offset.unwrap() as u64;
                writer.seek(SeekFrom::Start(original_offset))?;
                writer.write_all(&block)?;
                // Pointer stays the same, no need to update it
            } else {
                // Table count changed, write at end and update pointer
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                writer.write_all(&block)?;
                
                // Update pointer
                writer.seek(SeekFrom::Start(ARMOR_UPGRADE_MATS_PTR as u64))?;
                writer.write_all(&offset.to_le_bytes())?;
                writer.seek(SeekFrom::End(0))?;
            }
        }

        // Carve Parts - write only if modified
        if self.carve_parts_modified {
            use crate::model::mhfdat_pointers::{CARVE_PARTS_PTR, CARVE_PARTS_COUNT_PTR};
            use crate::core::mhfdat::write_carve_parts_block;
            
            // Calculate count from actual number of tables
            let count = self.carve_parts.tables.len() as u16;
            let block = write_carve_parts_block(&self.carve_parts)?;
            
            // If table count hasn't changed, overwrite at original offset
            if count == self.carve_parts_count && self.original_carve_parts_offset.is_some() {
                let original_offset = self.original_carve_parts_offset.unwrap() as u64;
                writer.seek(SeekFrom::Start(original_offset))?;
                writer.write_all(&block)?;
                // Pointer stays the same, no need to update it
                // Count stays the same, no need to update it
            } else {
                // Table count changed, write at end and update pointer
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                writer.write_all(&block)?;
                
                // Update pointer
                writer.seek(SeekFrom::Start(CARVE_PARTS_PTR as u64))?;
                writer.write_all(&offset.to_le_bytes())?;
                
                // Update count to match actual number of tables
                writer.seek(SeekFrom::Start(CARVE_PARTS_COUNT_PTR as u64))?;
                writer.write_all(&count.to_le_bytes())?;
                
                writer.seek(SeekFrom::End(0))?;
            }
        }

        // Part Break Parts - write only if modified
        if self.part_break_parts_modified {
            use crate::model::mhfdat_pointers::{PART_BREAK_DROP_PTR, PART_BREAK_DROP_COUNT_PTR};
            use crate::core::mhfdat::write_part_break_parts_block;
            
            // Calculate count from actual number of tables
            let count = self.part_break_parts.tables.len() as u16;
            let block = write_part_break_parts_block(&self.part_break_parts)?;
            
            // If table count hasn't changed, overwrite at original offset
            if count == self.part_break_parts_count && self.original_part_break_parts_offset.is_some() {
                let original_offset = self.original_part_break_parts_offset.unwrap() as u64;
                writer.seek(SeekFrom::Start(original_offset))?;
                writer.write_all(&block)?;
                // Pointer stays the same, no need to update it
                // Count stays the same, no need to update it
            } else {
                // Table count changed, write at end and update pointer
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                writer.write_all(&block)?;
                
                // Update pointer
                writer.seek(SeekFrom::Start(PART_BREAK_DROP_PTR as u64))?;
                writer.write_all(&offset.to_le_bytes())?;
                
                // Update count to match actual number of tables
                writer.seek(SeekFrom::Start(PART_BREAK_DROP_COUNT_PTR as u64))?;
                writer.write_all(&count.to_le_bytes())?;
                
                writer.seek(SeekFrom::End(0))?;
            }
        }

        // Sigil Crafting - write all three blocks if modified
        if self.sigil_recipes_modified {
            use crate::model::mhfdat_pointers::{
                SIGIL_CRAFTING_RECIPES_PTR,
                SIGIL_SKILL_PROBABILITIES_PTR,
                SIGIL_SKILL_BLACKLISTS_PTR,
            };
            use crate::core::mhfdat::{
                write_sigil_recipes_block, write_sigil_probabilities_block,
                write_sigil_blacklists_block,
            };

            // 1) Recipes
            let recipes_offset = writer.seek(SeekFrom::Current(0))? as u32;
            let recipes_block = write_sigil_recipes_block(&self.sigil_recipes)?;
            writer.write_all(&recipes_block)?;

            writer.seek(SeekFrom::Start(SIGIL_CRAFTING_RECIPES_PTR as u64))?;
            writer.write_all(&recipes_offset.to_le_bytes())?;
            writer.seek(SeekFrom::End(0))?;

            // 2) Probabilities
            let probs_offset = writer.seek(SeekFrom::Current(0))? as u32;
            let probs_block = write_sigil_probabilities_block(&self.sigil_probabilities)?;
            writer.write_all(&probs_block)?;

            writer.seek(SeekFrom::Start(SIGIL_SKILL_PROBABILITIES_PTR as u64))?;
            writer.write_all(&probs_offset.to_le_bytes())?;
            writer.seek(SeekFrom::End(0))?;

            // 3) Blacklists (pointer table + data, needs base address)
            let bl_base = writer.seek(SeekFrom::Current(0))? as u32;
            let bl_block = write_sigil_blacklists_block(&self.sigil_blacklists, bl_base)?;
            writer.write_all(&bl_block)?;

            writer.seek(SeekFrom::Start(SIGIL_SKILL_BLACKLISTS_PTR as u64))?;
            writer.write_all(&bl_base.to_le_bytes())?;
            writer.seek(SeekFrom::End(0))?;
        }

        // Seasonal Events - write only if modified
        if self.seasonal_events_modified {
            use crate::model::mhfdat_pointers::{SEASONAL_EVENT_PTR, SEASONAL_EVENT_COUNTER};
            use crate::core::mhfdat::write_seasonal_events_block;

            let count = self.seasonal_events.len() as u16;
            let block = write_seasonal_events_block(&self.seasonal_events)?;

            if count == self.seasonal_events_count && self.original_seasonal_events_offset.is_some() {
                let original_offset = self.original_seasonal_events_offset.unwrap() as u64;
                writer.seek(SeekFrom::Start(original_offset))?;
                writer.write_all(&block)?;
            } else {
                let offset = writer.seek(SeekFrom::End(0))? as u32;
                writer.write_all(&block)?;

                writer.seek(SeekFrom::Start(SEASONAL_EVENT_PTR as u64))?;
                writer.write_all(&offset.to_le_bytes())?;

                writer.seek(SeekFrom::Start(SEASONAL_EVENT_COUNTER as u64))?;
                writer.write_all(&count.to_le_bytes())?;

                writer.seek(SeekFrom::End(0))?;
            }
        }

        // Update Equipment Counts at the end
        // This updates the counters for melee weapons, ranged weapons, and all armor types
        use crate::core::mhfdat::read_equipment_counts;
        if let Some(counts) = read_equipment_counts(&self.buffer) {
            // The counts have already been updated in the buffer by refresh_weapon_counts_from_entries
            // and refresh_equipment_counts_from_entries, so we just need to write them to the file
            use crate::model::mhfdat_pointers::EQUIPEMENT_COUNT_PTR;

            writer.seek(SeekFrom::Start(EQUIPEMENT_COUNT_PTR as u64))?;
            let ptr = writer.read_u32::<LittleEndian>()?;

            writer.seek(SeekFrom::Start(ptr as u64))?;
            
            // Write the equipment counts structure
            writer.write_all(&counts.numLegA.to_le_bytes())?;
            writer.write_all(&counts.numUnk.to_le_bytes())?;
            writer.write_all(&counts.numHeadA.to_le_bytes())?;
            writer.write_all(&counts.numBodyA.to_le_bytes())?;
            writer.write_all(&counts.numArmA.to_le_bytes())?;
            writer.write_all(&counts.numWaistA.to_le_bytes())?;
            writer.write_all(&counts.numMeleeW.to_le_bytes())?;
            writer.write_all(&counts.numRangedW.to_le_bytes())?;
            
            writer.seek(SeekFrom::End(0))?;
        }

        Ok(())
    }

    pub fn compress_file(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.current_file {
            let path = path.clone(); // Cloner pour éviter les problèmes de borrow
            self.save_modified_data()?;
            let temp_path = path.with_extension("tmp");
            std::fs::copy(&path, &temp_path)?;
            compress_file(&temp_path, &path)?;
            
            // 5. Nettoyer le fichier temporaire
            let _ = std::fs::remove_file(&temp_path);
            
            // 6. Recharger le buffer depuis le fichier compressé
            self.buffer = std::fs::read(&path)?;
            
            // 7. Update armor descriptions offset after reload
            if self.armor_descriptions_modified {
                if self.buffer.len() >= ARMOR_DESC_PTR as usize + 4 {
                    let off = u32::from_le_bytes(self.buffer[ARMOR_DESC_PTR as usize..(ARMOR_DESC_PTR + 4) as usize].try_into().unwrap());
                    self.original_armor_descriptions_offset = Some(off);
                }
                self.armor_descriptions_modified = false;
            }
            
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file loaded"
            ))
        }
    }
    
    pub fn encrypt_file(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.current_file {
            let path = path.clone(); // Cloner pour éviter les problèmes de borrow
            let temp_path = path.with_extension("tmp");
            std::fs::copy(&path, &temp_path)?;
            encrypt_file(&temp_path, &path)?;
            let _ = std::fs::remove_file(&temp_path);
            
            // 5. Recharger le buffer depuis le fichier chiffré
            self.buffer = std::fs::read(&path)?;
            
            // 6. Update armor descriptions offset after reload
            if self.armor_descriptions_modified {
                if self.buffer.len() >= ARMOR_DESC_PTR as usize + 4 {
                    let off = u32::from_le_bytes(self.buffer[ARMOR_DESC_PTR as usize..(ARMOR_DESC_PTR + 4) as usize].try_into().unwrap());
                    self.original_armor_descriptions_offset = Some(off);
                }
                self.armor_descriptions_modified = false;
            }
            
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file loaded"
            ))
        }
    }

}