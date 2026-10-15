use super::*;
use std::io::{Read, Seek, Write, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::core::packing::{compress_file, encrypt_file};
use crate::core::mhfdat::{
    read_header_pointer, write_melee_weapons_block, write_ranged_weapons_block, write_armors_block, write_items_block, write_transmog_data,
    write_mw_upgrades_block, write_rw_upgrades_block, write_deco_shop_block, write_automatic_skills_block,
    write_armor_names, write_armor_descriptions, write_item_names, write_item_descriptions, write_sharpness_data_block, write_bullet_sets_block,
    write_deco_ids_block, write_monster_descriptions_block,
    Plan, read_crowns, crowns_plan, read_skill_effects, skill_effects_plan, read_gr_menus, gr_menus_plan,
    read_beginner_quests, beginner_quests_plan, read_caravan, caravan_plan, read_road, road_plan,
    read_forging_rows, forging_rows_plan, read_combine_shop, combine_shop_plan, read_tower_jewels, tower_jewels_plan,
    read_tower_seals, tower_seals_plan, read_tower_costs, tower_costs_plan,
    read_horn_melodies, horn_melodies_plan, read_arena, arena_plan,
    read_skill_tree, skill_tree_plan, read_vampirism, vampirism_plan, read_crit_conversion, crit_conversion_plan,
    read_weapon_params, weapon_params_plan, read_weapon_bonus_slots, weapon_bonus_slots_plan,
    read_coating_modifiers, coating_modifiers_plan,
    rush_plan, obscurity_plan, obscurity_up_plan,
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
    MOSNTERS_DESCRIPTION_PTR, ARMOR_DESC_PTR,
    SPECIAL_WEAPON_FORGING_PTR, ARMOR_PREMIUM_FORGING_PTR,
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
        
        if let Some(off) = original_offset {
            if off as usize >= self.buffer.len() {
                return false; // Offset out of bounds
            }
            
            // Use provided original entry count if available
            if let Some(orig_count) = original_count {
                // Can overwrite if new entry count matches original entry count exactly
                // This means the number of entries hasn't changed
                // We can overwrite even if the values have changed, as long as the count is the same
                let can_overwrite = new_count == orig_count && new_count > 0;
                return can_overwrite;
            }
            
            // No original count available, cannot safely overwrite
            false
        } else {
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
        // Even if values changed, we can overwrite in place. The block must end inside the file: a table
        // that would run past EOF (or grow) is appended instead of overwriting whatever follows.
        let file_len = writer.seek(SeekFrom::End(0))?;
        let fits = original_offset
            .map(|o| o as u64 + new_block.len() as u64 <= file_len)
            .unwrap_or(false);
        if fits && self.can_overwrite_at_offset(original_offset, original_count, new_count) {
            let original_offset = original_offset.unwrap() as u64;
            writer.seek(SeekFrom::Start(original_offset))?;
            writer.write_all(new_block)?;
            writer.seek(SeekFrom::End(0))?; // Return to end of file
            // Pointer stays the same, no need to update it
            Ok((original_offset as u32, false))
        } else {
            // Count changed (number of entries changed), write at end and update pointer
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            writer.write_all(new_block)?;
            // Pointer needs to be updated later
            Ok((offset, true))
        }
    }

    pub fn save_modified_data(&mut self) -> std::io::Result<()> {
        let Some(path) = self.current_file.clone() else { return Ok(()) };

        // CRITICAL: Refresh all counts from actual data before writing
        // This ensures the counts match the real number of items being written
        self.refresh_weapon_counts_from_entries();
        self.refresh_equipment_counts_from_entries();

        // Everything that can be refused is refused here, before a single byte is written.
        let plan = self.table_plan()?;

        // Work on a copy and replace the original only when the whole save went through, so a failure
        // half-way never leaves a half-written mhfdat behind.
        let tmp = PathBuf::from(format!("{}.saving", path.display()));
        std::fs::copy(&path, &tmp)?;
        let written = (|| -> std::io::Result<()> {
            let file = std::fs::OpenOptions::new().read(true).write(true).open(&tmp)?;
            self.save_modified_data_to_writer(file)?;
            if !plan.patches.is_empty() || !plan.moves.is_empty() {
                let mut file = std::fs::OpenOptions::new().read(true).write(true).open(&tmp)?;
                for patch in &plan.patches {
                    file.seek(SeekFrom::Start(patch.offset))?;
                    file.write_all(&patch.bytes)?;
                }
                // A table whose number of rows changed is written again at the end of the file (aligned to 0x10), then
                // every pointer to it and its count are updated. The old copy stays where it was.
                for m in &plan.moves {
                    let end = file.seek(SeekFrom::End(0))?;
                    let pad = (0x10 - end % 0x10) % 0x10;
                    file.write_all(&vec![0u8; pad as usize])?;
                    let at = (end + pad) as u32;
                    file.write_all(&m.bytes)?;
                    for cell in &m.pointers_at {
                        file.seek(SeekFrom::Start(*cell))?;
                        file.write_all(&at.to_le_bytes())?;
                    }
                    if let Some((count_at, n)) = m.count {
                        file.seek(SeekFrom::Start(count_at))?;
                        file.write_all(&n.to_le_bytes())?;
                    }
                }
            }
            Ok(())
        })();
        if let Err(e) = written {
            let _ = std::fs::remove_file(&tmp);
            return Err(e);
        }
        if let Err(e) = std::fs::rename(&tmp, &path) {
            let _ = std::fs::remove_file(&tmp);
            return Err(e);
        }

        // The file on disk is the truth: reload it, then re-sync every original offset / count and
        // clear every modified flag (otherwise a second save would target a stale block).
        self.buffer = std::fs::read(&path)?;
        self.after_save_sync();
        Ok(())
    }

    /// What the save does for the tables of the editor that have their own header slot: bytes to write over a table where
    /// it is (same number of rows), and tables to write again at the end of the file (another number of rows). Anything
    /// that does not fit (a table that cannot change length, a pointer outside the file, two tables on the same bytes) is
    /// refused here, before anything is written. Positions come from the file as it is on disk, not from the edited copy.
    fn table_plan(&self) -> std::io::Result<Plan> {
        let bad = |e: String| std::io::Error::new(std::io::ErrorKind::InvalidData, e);
        let buf = &self.buffer;
        let mut plan = Plan::default();

        // HR quests keep the old save (the tiers are written again) but a row with quest id 0 would end its tier
        if self.hr_quests_modified {
            let tiers = [
                ("HR1", &self.hr_quests.one_star), ("HR2", &self.hr_quests.two_stars), ("HR3", &self.hr_quests.three_stars),
                ("HR4", &self.hr_quests.four_stars), ("HR5", &self.hr_quests.five_stars), ("HR6", &self.hr_quests.six_stars),
            ];
            for (name, list) in tiers {
                if let Some(i) = list.iter().position(|q| q.quest_id == 0) {
                    return Err(bad(format!("HR quests {name}: row {i} has quest id 0, which ends the tier. Set a quest id first")));
                }
            }
        }

        // the crowns and the monster descriptions are counted by the same number (important_nums + 0x22)
        if self.crowns_modified || self.monster_descriptions_count_modified {
            let original = read_crowns(buf).map(|(_, rows)| rows.len()).unwrap_or(self.crowns.len());
            let changed = self.crowns.len() != original || self.monster_descriptions_count_modified;
            if changed && !self.crowns.is_empty() && self.crowns.len() != self.monster_descriptions.len() {
                return Err(bad(format!(
                    "The crown table has {} rows but there are {} monster descriptions. They share one counter (important_nums + 0x22), so both must have the same number of monsters",
                    self.crowns.len(), self.monster_descriptions.len()
                )));
            }
        }

        if self.crowns_modified {
            let (off, rows) = read_crowns(buf).map_err(bad)?;
            plan.extend(crowns_plan(Some(off), rows.len(), &self.crowns).map_err(bad)?);
        }
        if self.skill_effects_modified {
            let (off, rows) = read_skill_effects(buf).map_err(bad)?;
            plan.extend(skill_effects_plan(buf, Some(off), rows.len(), &self.skill_effects).map_err(bad)?);
        }
        if self.gr_menus_modified {
            let (menus, _) = read_gr_menus(buf);
            let rows: Vec<usize> = menus.iter().map(|m| m.rows.len()).collect();
            plan.extend(gr_menus_plan(buf, &self.gr_menus, &rows).map_err(bad)?);
        }
        if self.beginner_quests_modified {
            let (off, rows) = read_beginner_quests(buf).map_err(bad)?;
            plan.extend(beginner_quests_plan(buf, Some(off), rows.len(), &self.beginner_quests).map_err(bad)?);
        }
        if self.caravan_modified {
            let (orig, offsets, _) = read_caravan(buf);
            plan.extend(caravan_plan(buf, &self.caravan, &orig, &offsets).map_err(bad)?);
        }
        if self.road_modified {
            let (orig, offsets, _) = read_road(buf);
            let rows = [orig.points.len(), orig.cat_shop.len(), orig.skills.len()];
            plan.extend(road_plan(buf, &self.road, &offsets, rows).map_err(bad)?);
        }
        if self.craft_more_modified {
            for (slot, name, rows) in [
                (SPECIAL_WEAPON_FORGING_PTR, "special weapon forging", &self.special_weapon_forging),
                (ARMOR_PREMIUM_FORGING_PTR, "armor premium forging", &self.armor_premium_forging),
            ] {
                let (off, orig) = read_forging_rows(buf, name, slot).map_err(bad)?;
                plan.extend(forging_rows_plan(buf, name, slot, Some(off), orig.len(), rows).map_err(bad)?);
            }
            let (off, orig) = read_combine_shop(buf).map_err(bad)?;
            plan.extend(combine_shop_plan(Some(off), orig.len(), &self.combine_shop).map_err(bad)?);
            let (off, orig) = read_tower_jewels(buf).map_err(bad)?;
            plan.extend(tower_jewels_plan(buf, Some(off), orig.len(), &self.tower_jewels).map_err(bad)?);
            let (off, orig) = read_tower_seals(buf).map_err(bad)?;
            plan.extend(tower_seals_plan(Some(off), orig.len(), &self.sigil_tower_entries).map_err(bad)?);
            let orig = read_tower_costs(buf).map_err(bad)?;
            let rows: Vec<usize> = orig.iter().map(|b| b.rows.len()).collect();
            plan.extend(tower_costs_plan(&self.tower_costs, &rows).map_err(bad)?);
        }
        if self.horn_melodies_modified {
            let (off, rows) = read_horn_melodies(buf).map_err(bad)?;
            plan.extend(horn_melodies_plan(buf, Some(off), rows.len(), &self.horn_melodies).map_err(bad)?);
        }
        if self.skill_tree_modified {
            let (off, rows) = read_skill_tree(buf).map_err(bad)?;
            plan.extend(skill_tree_plan(buf, Some(off), rows.len(), &self.skill_tree).map_err(bad)?);
        }
        if self.vampirism_modified {
            let (off, rows) = read_vampirism(buf).map_err(bad)?;
            plan.extend(vampirism_plan(Some(off), rows.len(), &self.vampirism).map_err(bad)?);
        }
        if self.crit_conversion_modified {
            let (off, _) = read_crit_conversion(buf).map_err(bad)?;
            plan.extend(crit_conversion_plan(Some(off), &self.crit_conversion).map_err(bad)?);
        }
        if self.rush_modified {
            plan.extend(rush_plan(buf, &self.rush).map_err(bad)?);
        }
        if self.obscurity_modified {
            plan.extend(obscurity_plan(buf, &self.obscurity).map_err(bad)?);
        }
        if self.obscurity_up_modified {
            plan.extend(obscurity_up_plan(buf, &self.obscurity_up).map_err(bad)?);
        }
        if self.weapon_params_modified {
            let (off, _) = read_weapon_params(buf).map_err(bad)?;
            plan.extend(weapon_params_plan(Some(off), &self.weapon_params).map_err(bad)?);
        }
        if self.weapon_bonus_slots_modified {
            let (off, _) = read_weapon_bonus_slots(buf).map_err(bad)?;
            plan.extend(weapon_bonus_slots_plan(Some(off), &self.weapon_bonus_slots).map_err(bad)?);
        }
        if self.coating_modifiers_modified {
            let (off, _) = read_coating_modifiers(buf).map_err(bad)?;
            plan.extend(coating_modifiers_plan(Some(off), &self.coating_modifiers).map_err(bad)?);
        }
        if self.arena_modified {
            let (orig, offsets, _) = read_arena(buf);
            let items: Vec<usize> = orig.items.iter().map(|b| b.rows.len()).collect();
            let ammo: Vec<usize> = orig.ammo.iter().map(|b| b.rows.len()).collect();
            let tutorial: Vec<usize> = orig.tutorial.iter().map(|b| b.rows.len()).collect();
            plan.extend(arena_plan(buf, &self.arena, &offsets, &items, &ammo, &tutorial).map_err(bad)?);
        }

        // Everything that is written over the file must be inside it.
        let len = buf.len() as u64;
        let inside = |what: &str, at: u64, size: u64| -> std::io::Result<()> {
            if at.checked_add(size).map_or(true, |end| end > len) {
                return Err(bad(format!("{what}: {size} bytes at offset 0x{at:X} do not fit in the {len}-byte file; nothing was saved")));
            }
            Ok(())
        };
        for p in &plan.patches {
            inside(&p.table, p.offset, p.bytes.len() as u64)?;
        }
        for m in &plan.moves {
            for cell in &m.pointers_at {
                inside(&m.table, *cell, 4)?;
            }
            if let Some((at, _)) = m.count {
                inside(&m.table, at, 2)?;
            }
        }
        // What is identical to the file is not rewritten (a table that was opened and not changed stays byte for byte).
        plan.patches.retain(|p| buf.get(p.offset as usize..p.offset as usize + p.bytes.len()) != Some(&p.bytes[..]));

        // Two tables must never claim the same bytes (pointer cells and counts of the moved tables included).
        let mut spans: Vec<(u64, u64, String)> = plan.patches.iter().map(|p| (p.offset, p.offset + p.bytes.len() as u64, p.table.clone())).collect();
        for m in &plan.moves {
            for cell in &m.pointers_at {
                spans.push((*cell, *cell + 4, format!("{} (pointer)", m.table)));
            }
            if let Some((at, _)) = m.count {
                spans.push((at, at + 2, format!("{} (count)", m.table)));
            }
        }
        spans.sort();
        for w in spans.windows(2) {
            if w[1].0 < w[0].1 {
                return Err(bad(format!(
                    "{} (0x{:X}..0x{:X}) and {} (0x{:X}..0x{:X}) overlap; nothing was saved",
                    w[0].2, w[0].0, w[0].1, w[1].2, w[1].0, w[1].1
                )));
            }
        }
        Ok(plan)
    }

    /// Called once the file on disk has been rewritten: the in-place tables are read again from it.
    fn table_edits_saved(&mut self) {
        self.load_inplace_tables();
    }

    /// Reads a header slot from the current buffer.
    fn slot_value(&self, slot: u32) -> Option<u32> {
        read_header_pointer(&self.buffer, slot)
    }

    /// After a successful save: point every `original_*_offset` at what the header now says, reset
    /// every `*_modified` flag and re-record entry counts.
    fn after_save_sync(&mut self) {
        use crate::model::mhfdat_pointers::*;
        macro_rules! sync {
            ($slot:expr => $field:expr) => {
                if let Some(v) = self.slot_value($slot) { $field = Some(v); }
            };
        }
        sync!(MELEE_WEAPONS_PTR => self.original_melee_weapons_offset);
        sync!(RANGED_WEAPONS_PTR => self.original_ranged_weapons_offset);
        sync!(HEAD_ARMOR_PTR => self.original_head_armors_offset);
        sync!(BODY_ARMOR_PTR => self.original_body_armors_offset);
        sync!(ARM_ARMOR_PTR => self.original_arms_armors_offset);
        sync!(WAIST_ARMOR_PTR => self.original_waist_armors_offset);
        sync!(LEG_ARMOR_PTR => self.original_legs_armors_offset);
        sync!(ITEM_DATA_PTR => self.original_items_offset);
        sync!(TRANSMOG_FORGING_PTR => self.original_transmog_offset);
        sync!(WEAPON_FORGING_PTR => self.original_weapon_forging_offset);
        sync!(ARMOR_FORGING_PTR => self.original_armor_forging_offset);
        sync!(G_RANK_WEAPON_SHOP_PTR => self.original_weapon_forging_gr_offset);
        sync!(G_RANK_ARMOR_SHOP_PTR => self.original_armor_forging_gr_offset);
        sync!(ZENITH_WEAPON_FORGING_PTR => self.original_weapon_forging_zenith_offset);
        sync!(ZENITH_ARMOR_FORGING_PTR => self.original_armor_forging_zenith_offset);
        sync!(TOWER_WEAPON_FORGING_PTR => self.original_tower_weapon_forging_offset);
        sync!(TOWER_ARMOR_FORGING_PTR => self.original_tower_armor_forging_offset);
        sync!(DECO_SHOP_PTR => self.original_deco_shop_hr_offset);
        sync!(DECO_G_SHOP_PTR => self.original_deco_shop_gr_offset);
        sync!(CUFF_SHOP_PTR => self.original_cuff_shop_offset);
        sync!(CUFF_GR_SHOP_PTR => self.original_cuff_gr_shop_offset);
        sync!(AUTOMATIC_SKILLS_TABLE_PTR => self.original_automatic_skills_offset);
        sync!(DECO_ID_PTR => self.original_deco_ids_offset);
        sync!(MELEE_WEAPON_UPGRADE_PATH_PTR => self.original_mw_upgrades_offset);
        sync!(RANGED_WEAPON_UPGRADE_PATH_PTR => self.original_rw_upgrades_offset);
        sync!(MELEE_WEAPON_NAMES_PTR => self.original_melee_weapon_names_offset);
        sync!(MELEE_WEAPON_DESC_PTR => self.original_melee_weapon_descriptions_offset);
        sync!(RANGED_WEAPON_NAMES_PTR => self.original_ranged_weapon_names_offset);
        sync!(RANGED_WEAPON_DESC_PTR => self.original_ranged_weapon_descriptions_offset);
        sync!(HEAD_ARMOR_NAMES_PTR => self.original_head_armor_names_offset);
        sync!(BODY_ARMOR_NAMES_PTR => self.original_body_armor_names_offset);
        sync!(ARM_ARMOR_NAMES_PTR => self.original_arms_armor_names_offset);
        sync!(WAIST_ARMOR_NAMES_PTR => self.original_waist_armor_names_offset);
        sync!(LEG_ARMOR_NAMES_PTR => self.original_legs_armor_names_offset);
        sync!(ITEM_NAMES_PTR => self.original_item_names_offset);
        sync!(ITEM_DESC_PTR => self.original_item_descriptions_offset);
        sync!(ARMOR_DESC_PTR => self.original_armor_descriptions_offset);
        sync!(HR_QUEST_LIST_PTR => self.original_hr_quests_offset);
        sync!(G50_MELEE_WEAPON_UPGRADE_PTR => self.original_g50_melee_weapon_upgrades_offset);
        sync!(G50_RANGED_WEAPON_UPGRADE_PTR => self.original_g50_ranged_weapon_upgrades_offset);
        sync!(ARMOR_UPGRADE_MATS_PTR => self.original_armor_upgrade_mats_offset);
        sync!(CARVE_PARTS_PTR => self.original_carve_parts_offset);
        sync!(PART_BREAK_DROP_PTR => self.original_part_break_parts_offset);
        sync!(MOSNTERS_DESCRIPTION_PTR => self.original_monster_descriptions_offset);
        sync!(SIGIL_CRAFTING_RECIPES_PTR => self.original_sigil_recipes_offset);
        sync!(SIGIL_SKILL_PROBABILITIES_PTR => self.original_sigil_probabilities_offset);
        sync!(SIGIL_SKILL_BLACKLISTS_PTR => self.original_sigil_blacklists_offset);
        sync!(SEASONAL_EVENT_PTR => self.original_seasonal_events_offset);
        sync!(BULLET_SETS_PTR => self.original_bullet_sets_offset);
        for i in 0..self.original_sharpness_offsets.len() {
            let v = self.sharpness_slot(i).and_then(|slot| read_header_pointer(&self.buffer, slot as u32));
            if v.is_some() { self.original_sharpness_offsets[i] = v; }
        }
        let tower_slots = [
            SWORD_AND_SHIELD_G50_TOWER_PARAMS_PTR, DUAL_BLADES_G50_TOWER_PARAMS_PTR, GREAT_SWORD_G50_TOWER_PARAMS_PTR,
            LONG_SWORD_G50_TOWER_PARAMS_PTR, LANCE_G50_TOWER_PARAMS_PTR, GUNLANCE_G50_TOWER_PARAMS_PTR,
            HAMMER_G50_TOWER_PARAMS_PTR, HUNTING_HORN_G50_TOWER_PARAMS_PTR, HEAVY_BOWGUN_G50_TOWER_PARAMS_PTR,
            LIGHT_BOWGUN_G50_TOWER_PARAMS_PTR, BOW_G50_TOWER_PARAMS_PTR, TONFA_G50_TOWER_PARAMS_PTR,
            SWITCH_AXE_G50_TOWER_PARAMS_PTR, MAGNET_SPIKE_G50_TOWER_PARAMS_PTR,
        ];
        for (i, slot) in tower_slots.iter().enumerate() {
            if let Some(v) = self.slot_value(*slot) { self.original_g50_tower_params_offsets[i] = Some(v); }
        }

        // counts that live in important_nums
        self.carve_parts_count = self.imp_count(CARVE_PARTS_COUNT_INNER);
        self.part_break_parts_count = self.imp_count(PART_BREAK_DROP_COUNT_INNER);
        self.monster_descriptions_count = self.imp_count(MOSNTERS_DESCRIPTION_COUNT_INNER);
        self.seasonal_events_count = self.imp_count(SEASONAL_EVENT_COUNT_INNER);
        self.armor_upgrade_mats_table_count = self.armor_upgrade_mats.tables.len();

        // nothing is pending any more
        self.melee_weapons_modified = false;
        self.ranged_weapons_modified = false;
        self.head_armors_modified = false;
        self.body_armors_modified = false;
        self.arms_armors_modified = false;
        self.waist_armors_modified = false;
        self.legs_armors_modified = false;
        self.items_modified = false;
        self.transmog_modified = false;
        self.weapon_forging_modified = false;
        self.armor_forging_modified = false;
        self.weapon_forging_gr_modified = false;
        self.armor_forging_gr_modified = false;
        self.weapon_forging_zenith_modified = false;
        self.armor_forging_zenith_modified = false;
        self.tower_weapon_forging_modified = false;
        self.tower_armor_forging_modified = false;
        self.deco_shop_hr_modified = false;
        self.deco_shop_gr_modified = false;
        self.cuff_shop_modified = false;
        self.cuff_gr_shop_modified = false;
        self.mw_upgrades_modified = false;
        self.rw_upgrades_modified = false;
        self.melee_weapon_names_modified = false;
        self.melee_weapon_descriptions_modified = false;
        self.ranged_weapon_names_modified = false;
        self.ranged_weapon_descriptions_modified = false;
        self.head_armor_names_modified = false;
        self.body_armor_names_modified = false;
        self.arms_armor_names_modified = false;
        self.waist_armor_names_modified = false;
        self.legs_armor_names_modified = false;
        self.item_names_modified = false;
        self.item_descriptions_modified = false;
        self.armor_descriptions_modified = false;
        self.deco_ids_modified = false;
        self.deco_id_count_limiter_modified = false;
        self.automatic_skills_modified = false;
        self.automatic_skills_count_limiter_modified = false;
        self.sharpness_modified = [false; 11];
        self.bullet_sets_modified = false;
        self.hr_quests_modified = false;
        self.g50_melee_weapon_upgrades_modified = false;
        self.g50_ranged_weapon_upgrades_modified = false;
        self.g50_melee_count_limiter_modified = false;
        self.g50_ranged_count_limiter_modified = false;
        self.g50_tower_params_modified = [false; 14];
        self.armor_upgrade_mats_modified = false;
        self.carve_parts_modified = false;
        self.carve_parts_count_modified = false;
        self.part_break_parts_modified = false;
        self.part_break_parts_count_modified = false;
        self.monster_descriptions_modified = false;
        self.monster_descriptions_count_modified = false;
        self.sigil_recipes_modified = false;
        self.seasonal_events_modified = false;
        self.seasonal_events_count_modified = false;

        self.store_all_entry_counts();
        self.table_edits_saved();
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
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let weapon_forging_gr_block = write_transmog_data(&self.weapon_forging_gr_entries)?;
            writer.write_all(&weapon_forging_gr_block)?;
            offset
        } else {
            self.original_weapon_forging_gr_offset.unwrap_or(0)
        };

        // 9a4) G-Rank Armor forging shop data block - écrire seulement si modifié
        let armor_forging_gr_data_offset = if self.armor_forging_gr_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let armor_forging_gr_block = write_transmog_data(&self.armor_forging_gr_entries)?;
            writer.write_all(&armor_forging_gr_block)?;
            offset
        } else {
            self.original_armor_forging_gr_offset.unwrap_or(0)
        };

        // 9a5) Zenith Weapon forging shop data block - écrire seulement si modifié
        let weapon_forging_zenith_data_offset = if self.weapon_forging_zenith_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let weapon_forging_zenith_block = write_transmog_data(&self.weapon_forging_zenith_entries)?;
            writer.write_all(&weapon_forging_zenith_block)?;
            offset
        } else {
            self.original_weapon_forging_zenith_offset.unwrap_or(0)
        };

        // 9a6) Zenith Armor forging shop data block - écrire seulement si modifié
        let armor_forging_zenith_data_offset = if self.armor_forging_zenith_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let armor_forging_zenith_block = write_transmog_data(&self.armor_forging_zenith_entries)?;
            writer.write_all(&armor_forging_zenith_block)?;
            offset
        } else {
            self.original_armor_forging_zenith_offset.unwrap_or(0)
        };

        // 9a7) Tower Weapon forging shop data block - écrire seulement si modifié
        let tower_weapon_forging_data_offset = if self.tower_weapon_forging_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let tower_weapon_forging_block = write_transmog_data(&self.tower_weapon_forging_entries)?;
            writer.write_all(&tower_weapon_forging_block)?;
            offset
        } else {
            self.original_tower_weapon_forging_offset.unwrap_or(0)
        };

        // 9a8) Tower Armor forging shop data block - écrire seulement si modifié
        let tower_armor_forging_data_offset = if self.tower_armor_forging_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let tower_armor_forging_block = write_transmog_data(&self.tower_armor_forging_entries)?;
            writer.write_all(&tower_armor_forging_block)?;
            offset
        } else {
            self.original_tower_armor_forging_offset.unwrap_or(0)
        };

        // 9b) Deco shops - écrire seulement si modifié
        let deco_hr_offset = if self.deco_shop_hr_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let deco_hr_block = write_deco_shop_block(&self.deco_shop_hr_entries)?;
            writer.write_all(&deco_hr_block)?;
            offset
        } else {
            self.original_deco_shop_hr_offset.unwrap_or(0)
        };
        
        let deco_gr_offset = if self.deco_shop_gr_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let deco_gr_block = write_deco_shop_block(&self.deco_shop_gr_entries)?;
            writer.write_all(&deco_gr_block)?;
            offset
        } else {
            self.original_deco_shop_gr_offset.unwrap_or(0)
        };
        
        let cuff_offset = if self.cuff_shop_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let cuff_block = write_deco_shop_block(&self.cuff_shop_entries)?;
            writer.write_all(&cuff_block)?;
            offset
        } else {
            self.original_cuff_shop_offset.unwrap_or(0)
        };
        
        let cuff_gr_offset = if self.cuff_gr_shop_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let cuff_gr_block = write_deco_shop_block(&self.cuff_gr_shop_entries)?;
            writer.write_all(&cuff_gr_block)?;
            offset
        } else {
            self.original_cuff_gr_shop_offset.unwrap_or(0)
        };


        let automatic_skills_offset = if self.automatic_skills_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let automatic_skills_block = write_automatic_skills_block(&self.automatic_skills)?;
            writer.write_all(&automatic_skills_block)?;
            offset
        } else {
            self.original_automatic_skills_offset.unwrap_or(0)
        };

        // Deco IDs block - écrire seulement si modifié
        let deco_ids_offset = if self.deco_ids_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let deco_ids_block = write_deco_ids_block(&self.deco_ids)?;
            writer.write_all(&deco_ids_block)?;
            offset
        } else {
            self.original_deco_ids_offset.unwrap_or(0)
        };

        // 10) Melee weapon upgrade paths - écrire seulement si modifié
        let mw_upgrades_offset = if self.mw_upgrades_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let mw_block = write_mw_upgrades_block(&self.mw_upgrade_entries)?;
            writer.write_all(&mw_block)?;
            offset
        } else {
            self.original_mw_upgrades_offset.unwrap_or(0)
        };

        // 11) Ranged weapon upgrade paths - écrire seulement si modifié
        let rw_upgrades_offset = if self.rw_upgrades_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
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
                let offset = writer.seek(SeekFrom::End(0))? as u32;
                let block = write_sharpness_data_block(sharpness_fields[i])?;
                writer.write_all(&block)?;
                sharpness_offsets[i] = offset;
            } else {
                sharpness_offsets[i] = self.original_sharpness_offsets[i].unwrap_or(0);
            }
        }

        // Update sharpness pointers for modified weapon types (slots are found through header slot 0xAC)
        for i in 0..11 {
            if self.sharpness_modified[i] {
                let slot = self.sharpness_slot(i).ok_or_else(|| std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "sharpness pointer array (header slot 0xAC) is missing or outside the file",
                ))?;
                writer.seek(SeekFrom::Start(slot as u64))?;
                writer.write_all(&sharpness_offsets[i].to_le_bytes())?;
                writer.seek(SeekFrom::End(0))?;
            }
        }

        // 12b) Bullet Sets data block - write only if modified
        let bullet_sets_offset = if self.bullet_sets_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let block = write_bullet_sets_block(&self.bullet_sets)?;
            writer.write_all(&block)?;
            offset
        } else {
            self.original_bullet_sets_offset.unwrap_or(0)
        };

        // 12c) HR Quests data block - write only if modified
        use crate::core::mhfdat::write_hr_quests_block;
        let (hr_quests_offset, hr_offsets) = if self.hr_quests_modified {
            let base_offset = writer.seek(SeekFrom::End(0))? as u32;
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

        // 12e) G50 Melee Weapon Upgrades - write only if modified
        use crate::core::mhfdat::write_g50_weapon_upgrades_block;
        let g50_melee_offset = if self.g50_melee_weapon_upgrades_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let block = write_g50_weapon_upgrades_block(&self.g50_melee_weapon_upgrades)?;
            writer.write_all(&block)?;
            offset
        } else {
            self.original_g50_melee_weapon_upgrades_offset.unwrap_or(0)
        };

        // 12f) G50 Ranged Weapon Upgrades - write only if modified
        let g50_ranged_offset = if self.g50_ranged_weapon_upgrades_modified {
            let offset = writer.seek(SeekFrom::End(0))? as u32;
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
            let current_pos = writer.seek(SeekFrom::End(0))? as u32;
            let _ = crate::core::mhfdat::write_weapon_names(&mut writer, &self.melee_weapon_names[..melee_names_count])?;
            current_pos
        } else { 
            self.original_melee_weapon_names_offset.unwrap_or(0)
        };

        // Melee descriptions: table of pointers (4 per entry: 3 descriptions + 1 null) followed by SJIS strings
        let melee_desc_table_offset = if self.melee_weapon_descriptions_modified && melee_names_count > 0 {
            let table_start = writer.seek(SeekFrom::End(0))? as u32;
            let num_ptrs = melee_names_count * 4;
            // Only 3 strings per weapon, 4th pointer is always null
            let strings_start = table_start + (num_ptrs as u32) * 4;
            // Build pointer values and string blob in memory
            let mut ptr_values: Vec<u32> = Vec::with_capacity(num_ptrs);
            let mut strings_blob: Vec<u8> = Vec::new();
            for descs in &self.melee_weapon_descriptions[..melee_names_count] {
                // Write 3 description pointers
                for desc in descs.iter().take(3) {
                    // no truncation: long descriptions (colour codes included) exist in the original data
                    let desc_str: &str = desc.as_str();
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
            let current_pos = writer.seek(SeekFrom::End(0))? as u32;
            let _ = crate::core::mhfdat::write_ranged_weapon_names(&mut writer, &self.ranged_weapon_names[..ranged_names_count])?;
            current_pos
        } else { 
            self.original_ranged_weapon_names_offset.unwrap_or(0)
        };

        // Ranged descriptions: table of pointers (4 per entry: 3 descriptions + 1 null) followed by SJIS strings
        let ranged_desc_table_offset = if self.ranged_weapon_descriptions_modified && ranged_names_count > 0 {
            let table_start = writer.seek(SeekFrom::End(0))? as u32;
            let num_ptrs = ranged_names_count * 4;
            // Only 3 strings per weapon, 4th pointer is always null
            let strings_start = table_start + (num_ptrs as u32) * 4;
            let mut ptr_values: Vec<u32> = Vec::with_capacity(num_ptrs);
            let mut strings_blob: Vec<u8> = Vec::new();
            for descs in &self.ranged_weapon_descriptions[..ranged_names_count] {
                // Write 3 description pointers
                for desc in descs.iter().take(3) {
                    // no truncation: long descriptions (colour codes included) exist in the original data
                    let desc_str: &str = desc.as_str();
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
            let current_pos = writer.seek(SeekFrom::End(0))? as u32;
            write_armor_names(&mut writer, &self.head_armor_names)?;
            current_pos
        } else { 
            self.original_head_armor_names_offset.unwrap_or(0)
        };

        let body_armor_names_offset = if self.body_armor_names_modified && !self.body_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::End(0))? as u32;
            write_armor_names(&mut writer, &self.body_armor_names)?;
            current_pos
        } else { 
            self.original_body_armor_names_offset.unwrap_or(0)
        };

        let arms_armor_names_offset = if self.arms_armor_names_modified && !self.arms_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::End(0))? as u32;
            write_armor_names(&mut writer, &self.arms_armor_names)?;
            current_pos
        } else { 
            self.original_arms_armor_names_offset.unwrap_or(0)
        };

        let waist_armor_names_offset = if self.waist_armor_names_modified && !self.waist_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::End(0))? as u32;
            write_armor_names(&mut writer, &self.waist_armor_names)?;
            current_pos
        } else { 
            self.original_waist_armor_names_offset.unwrap_or(0)
        };

        let legs_armor_names_offset = if self.legs_armor_names_modified && !self.legs_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::End(0))? as u32;
            write_armor_names(&mut writer, &self.legs_armor_names)?;
            current_pos
        } else { 
            self.original_legs_armor_names_offset.unwrap_or(0)
        };

        // 15) Item names and descriptions - écrire seulement si modifié
        let item_names_count = self.items.len().min(self.item_names.len());
        let item_names_offset = if self.item_names_modified && item_names_count > 0 {
            let current_pos = writer.seek(SeekFrom::End(0))? as u32;
            write_item_names(&mut writer, &self.item_names[..item_names_count])?;
            current_pos
        } else { 
            self.original_item_names_offset.unwrap_or(0)
        };

        let item_desc_count = self.items.len().min(self.item_descriptions.len());
        let item_desc_offset = if self.item_descriptions_modified && item_desc_count > 0 {
            let current_pos = writer.seek(SeekFrom::End(0))? as u32;
            // The 0x60 bytes in front of the description table are 24 pointers to real UI messages
            // ("Select a recipient.", ...). They must be carried over, not zeroed.
            let preface: Vec<u8> = self.original_item_descriptions_offset
                .and_then(|o| self.buffer.get(o as usize..o as usize + 0x60))
                .map(|b| b.to_vec())
                .unwrap_or_else(|| vec![0u8; 0x60]);
            write_item_descriptions(&mut writer, &self.item_descriptions[..item_desc_count], &preface)?;
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
            let current_pos = writer.seek(SeekFrom::End(0))? as u32;
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
        
        // Monster Descriptions - write only if modified. The block is variable-size (strings), so it is
        // always appended with absolute pointers and the header slot is repointed; rewriting it in place
        // could overrun the data that follows it.
        if self.monster_descriptions_modified {
            use crate::model::mhfdat_pointers::{MOSNTERS_DESCRIPTION_PTR, MOSNTERS_DESCRIPTION_COUNT_INNER};

            let count = self.monster_descriptions.len() as u16;
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let block = write_monster_descriptions_block(&self.monster_descriptions, offset)?;
            writer.write_all(&block)?;

            writer.seek(SeekFrom::Start(MOSNTERS_DESCRIPTION_PTR as u64))?;
            writer.write_all(&offset.to_le_bytes())?;
            writer.seek(SeekFrom::Start(self.imp_count_pos(MOSNTERS_DESCRIPTION_COUNT_INNER)?))?;
            writer.write_all(&count.to_le_bytes())?;
            writer.seek(SeekFrom::End(0))?;
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
            use crate::model::mhfdat_pointers::AUTOMATIC_SKILLS_COUNT_INNER;
            writer.seek(SeekFrom::Start(self.imp_count_pos(AUTOMATIC_SKILLS_COUNT_INNER)?))?;
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
            use crate::model::mhfdat_pointers::DECO_ID_COUNT_INNER;
            writer.seek(SeekFrom::Start(self.imp_count_pos(DECO_ID_COUNT_INNER)?))?;
            writer.write_all(&self.deco_id_count_limiter.to_le_bytes())?;
        }

        // Update deco IDs pointer if modified
        if self.deco_ids_modified {
            writer.seek(SeekFrom::Start(DECO_ID_PTR as u64))?;
            writer.write_all(&deco_ids_offset.to_le_bytes())?;
        }

        // HR quests: header slot 0xA98 points to a struct of six list pointers (1*..6*). Rewrite those six
        // pointers in place (24 bytes, same size); only when the struct is unusable, append a new one
        // and repoint the slot. The old code wrote the six offsets *over the header slots* starting at 0xA98.
        if self.hr_quests_modified {
            use crate::model::mhfdat_pointers::HR_QUEST_LIST_PTR;
            let file_len = writer.seek(SeekFrom::End(0))?;
            let in_place = self.original_hr_quests_offset
                .filter(|&o| o as u64 >= 0x1000 && o as u64 + 24 <= file_len)
                .map(|o| o as u64);
            let ptr_struct = match in_place {
                Some(o) => o,
                None => {
                    let at = writer.seek(SeekFrom::End(0))?;
                    writer.write_all(&[0u8; 24])?;
                    writer.seek(SeekFrom::Start(HR_QUEST_LIST_PTR as u64))?;
                    writer.write_all(&(at as u32).to_le_bytes())?;
                    at
                }
            };
            writer.seek(SeekFrom::Start(ptr_struct))?;
            for off in &hr_offsets {
                writer.write_all(&off.to_le_bytes())?;
            }
            writer.seek(SeekFrom::End(0))?;
        }

        // Update G50 Melee Weapon Upgrades pointer and limiter if modified
        if self.g50_melee_weapon_upgrades_modified {
            use crate::model::mhfdat_pointers::G50_MELEE_WEAPON_UPGRADE_PTR;
            writer.seek(SeekFrom::Start(G50_MELEE_WEAPON_UPGRADE_PTR as u64))?;
            writer.write_all(&g50_melee_offset.to_le_bytes())?;
        }
        if self.g50_melee_count_limiter_modified {
            use crate::model::mhfdat_pointers::G50_MELEE_WEAPON_UPGRADE_COUNT_INNER;
            writer.seek(SeekFrom::Start(self.imp_count_pos(G50_MELEE_WEAPON_UPGRADE_COUNT_INNER)?))?;
            writer.write_all(&self.g50_melee_count_limiter.to_le_bytes())?;
        }

        // Update G50 Ranged Weapon Upgrades pointer and limiter if modified
        if self.g50_ranged_weapon_upgrades_modified {
            use crate::model::mhfdat_pointers::G50_RANGED_WEAPON_UPGRADE_PTR;
            writer.seek(SeekFrom::Start(G50_RANGED_WEAPON_UPGRADE_PTR as u64))?;
            writer.write_all(&g50_ranged_offset.to_le_bytes())?;
        }
        if self.g50_ranged_count_limiter_modified {
            use crate::model::mhfdat_pointers::G50_RANGED_WEAPON_UPGRADE_COUNT_INNER;
            writer.seek(SeekFrom::Start(self.imp_count_pos(G50_RANGED_WEAPON_UPGRADE_COUNT_INNER)?))?;
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
                let base_offset = writer.seek(SeekFrom::End(0))? as u32;
                let (ptr_table, data_block) = write_tower_g50_weapon_type(&self.g50_tower_params[i], base_offset)?;
                writer.write_all(&ptr_table)?;
                writer.write_all(&data_block)?;
                // Update main pointer
                writer.seek(SeekFrom::Start(tower_ptrs[i] as u64))?;
                writer.write_all(&base_offset.to_le_bytes())?;
                writer.seek(SeekFrom::End(0))?;
            }
        }

        // Armor Upgrade Materials, Carve and Part-break drops are pointer tables over variable-length
        // sub-tables (and the original tables share storage), so each is rebuilt as one block appended to
        // the end of the file with absolute pointers, and the header slot is repointed. The old block is
        // left where it is: nothing that follows it is overwritten.
        if self.armor_upgrade_mats_modified {
            use crate::core::mhfdat::write_armor_upgrade_mats_block;
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let block = write_armor_upgrade_mats_block(&self.armor_upgrade_mats, offset)?;
            writer.write_all(&block)?;
            writer.seek(SeekFrom::Start(ARMOR_UPGRADE_MATS_PTR as u64))?;
            writer.write_all(&offset.to_le_bytes())?;
            writer.seek(SeekFrom::End(0))?;
        }

        if self.carve_parts_modified {
            use crate::model::mhfdat_pointers::{CARVE_PARTS_PTR, CARVE_PARTS_COUNT_INNER};
            use crate::core::mhfdat::write_carve_parts_block;
            let count = self.carve_parts.tables.len() as u16;
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let block = write_carve_parts_block(&self.carve_parts, offset)?;
            writer.write_all(&block)?;
            writer.seek(SeekFrom::Start(CARVE_PARTS_PTR as u64))?;
            writer.write_all(&offset.to_le_bytes())?;
            writer.seek(SeekFrom::Start(self.imp_count_pos(CARVE_PARTS_COUNT_INNER)?))?;
            writer.write_all(&count.to_le_bytes())?;
            writer.seek(SeekFrom::End(0))?;
        }

        if self.part_break_parts_modified {
            use crate::model::mhfdat_pointers::{PART_BREAK_DROP_PTR, PART_BREAK_DROP_COUNT_INNER};
            use crate::core::mhfdat::write_part_break_parts_block;
            let count = self.part_break_parts.tables.len() as u16;
            let offset = writer.seek(SeekFrom::End(0))? as u32;
            let block = write_part_break_parts_block(&self.part_break_parts, offset)?;
            writer.write_all(&block)?;
            writer.seek(SeekFrom::Start(PART_BREAK_DROP_PTR as u64))?;
            writer.write_all(&offset.to_le_bytes())?;
            writer.seek(SeekFrom::Start(self.imp_count_pos(PART_BREAK_DROP_COUNT_INNER)?))?;
            writer.write_all(&count.to_le_bytes())?;
            writer.seek(SeekFrom::End(0))?;
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
            let recipes_offset = writer.seek(SeekFrom::End(0))? as u32;
            let recipes_block = write_sigil_recipes_block(&self.sigil_recipes)?;
            writer.write_all(&recipes_block)?;

            writer.seek(SeekFrom::Start(SIGIL_CRAFTING_RECIPES_PTR as u64))?;
            writer.write_all(&recipes_offset.to_le_bytes())?;
            writer.seek(SeekFrom::End(0))?;

            // 2) Probabilities
            let probs_offset = writer.seek(SeekFrom::End(0))? as u32;
            let probs_block = write_sigil_probabilities_block(&self.sigil_probabilities)?;
            writer.write_all(&probs_block)?;

            writer.seek(SeekFrom::Start(SIGIL_SKILL_PROBABILITIES_PTR as u64))?;
            writer.write_all(&probs_offset.to_le_bytes())?;
            writer.seek(SeekFrom::End(0))?;

            // 3) Blacklists (pointer table + data, needs base address)
            let bl_base = writer.seek(SeekFrom::End(0))? as u32;
            let bl_block = write_sigil_blacklists_block(&self.sigil_blacklists, bl_base)?;
            writer.write_all(&bl_block)?;

            writer.seek(SeekFrom::Start(SIGIL_SKILL_BLACKLISTS_PTR as u64))?;
            writer.write_all(&bl_base.to_le_bytes())?;

            // important_nums + 0x23C is the number of recipes / chances / blacklists
            writer.seek(SeekFrom::Start(self.imp_count_pos(crate::model::mhfdat_pointers::SIGIL_RECIPES_COUNT_INNER)?))?;
            writer.write_all(&(self.sigil_recipes.len() as u16).to_le_bytes())?;
            writer.seek(SeekFrom::End(0))?;
        }

        // Seasonal Events - write only if modified
        if self.seasonal_events_modified {
            use crate::model::mhfdat_pointers::{SEASONAL_EVENT_PTR, SEASONAL_EVENT_COUNT_INNER};
            use crate::core::mhfdat::write_seasonal_events_block;

            let count = self.seasonal_events.len() as u16;
            let block = write_seasonal_events_block(&self.seasonal_events)?;
            let file_len = writer.seek(SeekFrom::End(0))?;
            let in_place = self.original_seasonal_events_offset
                .filter(|&o| count == self.seasonal_events_count && o as u64 + block.len() as u64 <= file_len);

            if let Some(original_offset) = in_place {
                // fixed 12-byte rows, same count: same size, safe to rewrite where it is
                writer.seek(SeekFrom::Start(original_offset as u64))?;
                writer.write_all(&block)?;
            } else {
                let offset = writer.seek(SeekFrom::End(0))? as u32;
                writer.write_all(&block)?;

                writer.seek(SeekFrom::Start(SEASONAL_EVENT_PTR as u64))?;
                writer.write_all(&offset.to_le_bytes())?;

                writer.seek(SeekFrom::Start(self.imp_count_pos(SEASONAL_EVENT_COUNT_INNER)?))?;
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

            let file_len = writer.seek(SeekFrom::End(0))?;
            writer.seek(SeekFrom::Start(EQUIPEMENT_COUNT_PTR as u64))?;
            let ptr = writer.read_u32::<LittleEndian>()?;
            if ptr == 0 || ptr as u64 + 16 > file_len {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("equipment counts pointer (header slot 0xE8) = 0x{ptr:X} does not leave room for 16 bytes in a {file_len}-byte file"),
                ));
            }

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
