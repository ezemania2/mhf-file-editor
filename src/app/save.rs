use super::*;
use std::io::{Read, Seek, Write, SeekFrom};
use crate::core::packing::{compress_file, encrypt_file};
use crate::core::mhfdat::{
    write_melee_weapons_block, write_ranged_weapons_block, write_armors_block, write_items_block, write_transmog_data,
    write_mw_upgrades_block, write_rw_upgrades_block, write_deco_shop_block, write_automatic_skills_block,
    write_armor_names, write_item_names, write_item_descriptions, write_sharpness_data_block, write_bullet_sets_block,
    write_deco_ids_block, write_monster_descriptions
};
use crate::model::mhfdat_pointers::{
    MELEE_WEAPONS_PTR, RANGED_WEAPONS_PTR,
    MELEE_WEAPON_NAMES_PTR, RANGED_WEAPON_NAMES_PTR,
    MELEE_WEAPON_DESC_PTR, RANGED_WEAPON_DESC_PTR,
    HEAD_ARMOR_PTR, BODY_ARMOR_PTR, ARM_ARMOR_PTR, WAIST_ARMOR_PTR, LEG_ARMOR_PTR,
    HEAD_ARMOR_NAMES_PTR, BODY_ARMOR_NAMES_PTR, ARM_ARMOR_NAMES_PTR, WAIST_ARMOR_NAMES_PTR, LEG_ARMOR_NAMES_PTR,
    ITEM_DATA_PTR, ITEM_NAMES_PTR, ITEM_DESC_PTR, TRANSMOG_FORGING_PTR, WEAPON_FORGING_PTR, ARMOR_FORGING_PTR,
    G_RANK_WEAPON_SHOP_PTR, G_RANK_ARMOR_SHOP_PTR, ZENITH_WEAPON_FORGING_PTR, ZENITH_ARMOR_FORGING_PTR,
    DECO_SHOP_PTR, DECO_G_SHOP_PTR, CUFF_SHOP_PTR, CUFF_GR_SHOP_PTR,
    MELEE_WEAPON_UPGRADE_PATH_PTR, RANGED_WEAPON_UPGRADE_PATH_PTR,
    AUTOMATIC_SKILLS_TABLE_PTR, DECO_ID_PTR, ARMOR_UPGRADE_MATS_PTR, ARMOR_STAT_ARRAY_PTR, ARMOR_NAME_ARRAY_PTR, ARMOR_WEAPON_NAMES_ARRAY_PTR,
    MOSNTERS_DESCRIPTION_PTR, MOSNTERS_DESCRIPTION_COUNT_PTR,
};
use crate::model::mhfdat::{ArmorStatPointers, ArmorNamePointers, ArmorWeaponNamePointers};
use std::ptr;

impl MhfdatApp {

    pub fn save_modified_data(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.current_file {
            // Ouvrir le fichier en mode read+write pour ajouter à la fin sans copier
            let file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)?;

            // Écrire les modifications directement à la fin du fichier
            self.save_modified_data_to_writer(file)?;

            // 6. IMPORTANT: Remplacer complètement le buffer avec le fichier sauvegardé
            // pour que les données écrites à la fin soient accessibles
            let saved_file_data = std::fs::read(path)?;
            self.buffer = saved_file_data.clone();
            
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
        let melee_data_offset = if self.melee_weapons_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let melee_block = write_melee_weapons_block(&self.melee_weapons)?;
            writer.write_all(&melee_block)?;
            offset
        } else {
            self.original_melee_weapons_offset.unwrap_or(0)
        };

        // 2) Ranged weapons data block - écrire seulement si modifié
        let ranged_data_offset = if self.ranged_weapons_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let ranged_block = write_ranged_weapons_block(&self.ranged_weapons)?;
            writer.write_all(&ranged_block)?;
            offset
        } else {
            self.original_ranged_weapons_offset.unwrap_or(0)
        };

        // 3) Head armor block - écrire seulement si modifié
        let head_armor_offset = if self.head_armors_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let head_block = write_armors_block(&self.head_armors)?;
            writer.write_all(&head_block)?;
            offset
        } else {
            self.original_head_armors_offset.unwrap_or(0)
        };

        // 4) Body armor block - écrire seulement si modifié
        let body_armor_offset = if self.body_armors_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let body_block = write_armors_block(&self.body_armors)?;
            writer.write_all(&body_block)?;
            offset
        } else {
            self.original_body_armors_offset.unwrap_or(0)
        };

        // 5) Arms armor block - écrire seulement si modifié
        let arms_armor_offset = if self.arms_armors_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let arms_block = write_armors_block(&self.arms_armors)?;
            writer.write_all(&arms_block)?;
            offset
        } else {
            self.original_arms_armors_offset.unwrap_or(0)
        };

        // 6) Waist armor block - écrire seulement si modifié
        let waist_armor_offset = if self.waist_armors_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let waist_block = write_armors_block(&self.waist_armors)?;
            writer.write_all(&waist_block)?;
            offset
        } else {
            self.original_waist_armors_offset.unwrap_or(0)
        };

        // 7) Legs armor block - écrire seulement si modifié
        let legs_armor_offset = if self.legs_armors_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let legs_block = write_armors_block(&self.legs_armors)?;
            writer.write_all(&legs_block)?;
            offset
        } else {
            self.original_legs_armors_offset.unwrap_or(0)
        };

        // 8) Items data block - écrire seulement si modifié
        let item_data_offset = if self.items_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let items_block = write_items_block(&self.items)?;
            writer.write_all(&items_block)?;
            offset
        } else {
            self.original_items_offset.unwrap_or(0)
        };

        // 9) Transmog shop data block - écrire seulement si modifié
        let transmog_data_offset = if self.transmog_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let transmog_block = write_transmog_data(&self.transmog_entries)?;
            writer.write_all(&transmog_block)?;
            offset
        } else {
            self.original_transmog_offset.unwrap_or(0)
        };

        // 9a) Weapon forging shop data block - écrire seulement si modifié
        let weapon_forging_data_offset = if self.weapon_forging_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let weapon_forging_block = write_transmog_data(&self.weapon_forging_entries)?;
            writer.write_all(&weapon_forging_block)?;
            offset
        } else {
            self.original_weapon_forging_offset.unwrap_or(0)
        };

        // 9a2) Armor forging shop data block - écrire seulement si modifié
        let armor_forging_data_offset = if self.armor_forging_modified {
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let armor_forging_block = write_transmog_data(&self.armor_forging_entries)?;
            writer.write_all(&armor_forging_block)?;
            offset
        } else {
            self.original_armor_forging_offset.unwrap_or(0)
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

        // 12a) Sharpness data blocks - write only if modified, WITHOUT updating pointers
        // (Pointers remain at their original locations)
        let _sharpness_offsets = [
            if self.sharpness_modified[0] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.great_sword)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[0].unwrap_or(0)
            },
            if self.sharpness_modified[1] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.hammer)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[1].unwrap_or(0)
            },
            if self.sharpness_modified[2] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.lance)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[2].unwrap_or(0)
            },
            if self.sharpness_modified[3] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.sword_and_shield)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[3].unwrap_or(0)
            },
            if self.sharpness_modified[4] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.dual_blades)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[4].unwrap_or(0)
            },
            if self.sharpness_modified[5] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.long_sword)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[5].unwrap_or(0)
            },
            if self.sharpness_modified[6] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.hunting_horn)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[6].unwrap_or(0)
            },
            if self.sharpness_modified[7] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.gunlance)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[7].unwrap_or(0)
            },
            if self.sharpness_modified[8] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.bow)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[8].unwrap_or(0)
            },
            if self.sharpness_modified[9] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.tonfa)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[9].unwrap_or(0)
            },
            if self.sharpness_modified[10] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.switch_axe)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[10].unwrap_or(0)
            },
            if self.sharpness_modified[11] {
                let offset = writer.seek(SeekFrom::Current(0))? as u32;
                let block = write_sharpness_data_block(&self.sharpness.magnet_spike)?;
                writer.write_all(&block)?;
                offset
            } else {
                self.original_sharpness_offsets[11].unwrap_or(0)
            },
        ];

        // NOTE: Sharpness pointers are NOT updated - only data blocks are written if modified

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
        
        let monster_desc_count = self.monster_descriptions.len();
        let monster_desc_offset = if self.monster_descriptions_modified && monster_desc_count > 0 {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_monster_descriptions(&mut writer, &self.monster_descriptions[..monster_desc_count])?;
            current_pos
        } else {
            self.original_monster_descriptions_offset.unwrap_or(0)
        };

        // Patch header pointers - seulement si modifié
        
        if self.melee_weapons_modified {
            writer.seek(SeekFrom::Start(MELEE_WEAPONS_PTR as u64))?;
            writer.write_all(&melee_data_offset.to_le_bytes())?;
        }

        if self.ranged_weapons_modified {
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

        if self.head_armors_modified {
            writer.seek(SeekFrom::Start(HEAD_ARMOR_PTR as u64))?;
            writer.write_all(&head_armor_offset.to_le_bytes())?;
        }

        if self.body_armors_modified {
            writer.seek(SeekFrom::Start(BODY_ARMOR_PTR as u64))?;
            writer.write_all(&body_armor_offset.to_le_bytes())?;
        }

        if self.arms_armors_modified {
            writer.seek(SeekFrom::Start(ARM_ARMOR_PTR as u64))?;
            writer.write_all(&arms_armor_offset.to_le_bytes())?;
        }

        if self.waist_armors_modified {
            writer.seek(SeekFrom::Start(WAIST_ARMOR_PTR as u64))?;
            writer.write_all(&waist_armor_offset.to_le_bytes())?;
        }

        if self.legs_armors_modified {
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

        if self.items_modified {
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
        
        if self.monster_descriptions_modified {
            writer.seek(SeekFrom::Start(MOSNTERS_DESCRIPTION_PTR as u64))?;
            writer.write_all(&monster_desc_offset.to_le_bytes())?;
            
            if self.monster_descriptions_count_modified {
                writer.seek(SeekFrom::Start(MOSNTERS_DESCRIPTION_COUNT_PTR as u64))?;
                writer.write_all(&self.monster_descriptions_count.to_le_bytes())?;
            }
        }

        if self.transmog_modified {
            writer.seek(SeekFrom::Start(TRANSMOG_FORGING_PTR as u64))?;
            writer.write_all(&transmog_data_offset.to_le_bytes())?;
        }

        if self.weapon_forging_modified {
            writer.seek(SeekFrom::Start(WEAPON_FORGING_PTR as u64))?;
            writer.write_all(&weapon_forging_data_offset.to_le_bytes())?;
        }

        if self.armor_forging_modified {
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
            
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let block = write_armor_upgrade_mats_block(&self.armor_upgrade_mats)?;
            writer.write_all(&block)?;
            
            // Update pointer
            writer.seek(SeekFrom::Start(ARMOR_UPGRADE_MATS_PTR as u64))?;
            writer.write_all(&offset.to_le_bytes())?;
            writer.seek(SeekFrom::End(0))?;
        }

        // Carve Parts - write only if modified
        if self.carve_parts_modified {
            use crate::model::mhfdat_pointers::{CARVE_PARTS_PTR, CARVE_PARTS_COUNT_PTR};
            use crate::core::mhfdat::write_carve_parts_block;
            
            // Calculate count from actual number of tables
            let count = self.carve_parts.tables.len() as u16;
            
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let block = write_carve_parts_block(&self.carve_parts)?;
            writer.write_all(&block)?;
            
            // Update pointer
            writer.seek(SeekFrom::Start(CARVE_PARTS_PTR as u64))?;
            writer.write_all(&offset.to_le_bytes())?;
            
            // Update count (always update to match actual number of tables)
            writer.seek(SeekFrom::Start(CARVE_PARTS_COUNT_PTR as u64))?;
            writer.write_all(&count.to_le_bytes())?;
            
            writer.seek(SeekFrom::End(0))?;
        }

        // Part Break Parts - write only if modified
        if self.part_break_parts_modified {
            use crate::model::mhfdat_pointers::{PART_BREAK_DROP_PTR, PART_BREAK_DROP_COUNT_PTR};
            use crate::core::mhfdat::write_part_break_parts_block;
            
            // Calculate count from actual number of tables
            let count = self.part_break_parts.tables.len() as u16;
            
            let offset = writer.seek(SeekFrom::Current(0))? as u32;
            let block = write_part_break_parts_block(&self.part_break_parts)?;
            writer.write_all(&block)?;
            
            // Update pointer
            writer.seek(SeekFrom::Start(PART_BREAK_DROP_PTR as u64))?;
            writer.write_all(&offset.to_le_bytes())?;
            
            // Update count (always update to match actual number of tables)
            writer.seek(SeekFrom::Start(PART_BREAK_DROP_COUNT_PTR as u64))?;
            writer.write_all(&count.to_le_bytes())?;
            
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
            
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No file loaded"
            ))
        }
    }

}