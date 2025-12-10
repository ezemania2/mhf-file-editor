use super::*;
use std::io::{Read, Seek, Write, SeekFrom};
use crate::core::packing::{compress_file, encrypt_file};
use crate::core::mhfdat::{
    write_melee_weapons_block, write_ranged_weapons_block, write_armors_block, write_items_block, write_transmog_data,
    write_mw_upgrades_block, write_rw_upgrades_block, write_deco_shop_block, write_automatic_skills_block,
    write_armor_names, write_item_names, write_item_descriptions
};
use crate::model::mhfdat_pointers::{
    MELEE_WEAPONS_PTR, RANGED_WEAPONS_PTR,
    MELEE_WEAPON_NAMES_PTR, RANGED_WEAPON_NAMES_PTR,
    MELEE_WEAPON_DESC_PTR, RANGED_WEAPON_DESC_PTR,
    HEAD_ARMOR_PTR, BODY_ARMOR_PTR, ARM_ARMOR_PTR, WAIST_ARMOR_PTR, LEG_ARMOR_PTR,
    HEAD_ARMOR_NAMES_PTR, BODY_ARMOR_NAMES_PTR, ARM_ARMOR_NAMES_PTR, WAIST_ARMOR_NAMES_PTR, LEG_ARMOR_NAMES_PTR,
    ITEM_DATA_PTR, ITEM_NAMES_PTR, ITEM_DESC_PTR, TRANSMOG_FORGING_PTR, 
    DECO_SHOP_PTR, DECO_G_SHOP_PTR, CUFF_SHOP_PTR, CUFF_GR_SHOP_PTR,
    MELEE_WEAPON_UPGRADE_PATH_PTR, RANGED_WEAPON_UPGRADE_PATH_PTR,
    AUTOMATIC_SKILLS_TABLE_PTR,
};

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

            // 6. IMPORTANT: Mettre à jour seulement les pointeurs dans le buffer existant
            // Lire les nouveaux pointeurs depuis le fichier sauvegardé et les appliquer au buffer
            let saved_file_data = std::fs::read(path)?;
            
            // Copier seulement les pointeurs depuis le fichier sauvegardé vers notre buffer
            use crate::model::mhfdat_pointers::*;
            
            // Mettre à jour les pointeurs d'armes
            if saved_file_data.len() >= (MELEE_WEAPONS_PTR + 4) as usize {
                self.buffer[MELEE_WEAPONS_PTR as usize..(MELEE_WEAPONS_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[MELEE_WEAPONS_PTR as usize..(MELEE_WEAPONS_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (RANGED_WEAPONS_PTR + 4) as usize {
                self.buffer[RANGED_WEAPONS_PTR as usize..(RANGED_WEAPONS_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[RANGED_WEAPONS_PTR as usize..(RANGED_WEAPONS_PTR + 4) as usize]);
            }
            
            // Mettre à jour les pointeurs de noms/descriptions d'armes
            if saved_file_data.len() >= (MELEE_WEAPON_NAMES_PTR + 4) as usize {
                self.buffer[MELEE_WEAPON_NAMES_PTR as usize..(MELEE_WEAPON_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[MELEE_WEAPON_NAMES_PTR as usize..(MELEE_WEAPON_NAMES_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (MELEE_WEAPON_DESC_PTR + 4) as usize {
                self.buffer[MELEE_WEAPON_DESC_PTR as usize..(MELEE_WEAPON_DESC_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[MELEE_WEAPON_DESC_PTR as usize..(MELEE_WEAPON_DESC_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (RANGED_WEAPON_NAMES_PTR + 4) as usize {
                self.buffer[RANGED_WEAPON_NAMES_PTR as usize..(RANGED_WEAPON_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[RANGED_WEAPON_NAMES_PTR as usize..(RANGED_WEAPON_NAMES_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (RANGED_WEAPON_DESC_PTR + 4) as usize {
                self.buffer[RANGED_WEAPON_DESC_PTR as usize..(RANGED_WEAPON_DESC_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[RANGED_WEAPON_DESC_PTR as usize..(RANGED_WEAPON_DESC_PTR + 4) as usize]);
            }
            
            // Mettre à jour les pointeurs d'armures
            if saved_file_data.len() >= (HEAD_ARMOR_PTR + 4) as usize {
                self.buffer[HEAD_ARMOR_PTR as usize..(HEAD_ARMOR_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[HEAD_ARMOR_PTR as usize..(HEAD_ARMOR_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (BODY_ARMOR_PTR + 4) as usize {
                self.buffer[BODY_ARMOR_PTR as usize..(BODY_ARMOR_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[BODY_ARMOR_PTR as usize..(BODY_ARMOR_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (ARM_ARMOR_PTR + 4) as usize {
                self.buffer[ARM_ARMOR_PTR as usize..(ARM_ARMOR_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ARM_ARMOR_PTR as usize..(ARM_ARMOR_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (WAIST_ARMOR_PTR + 4) as usize {
                self.buffer[WAIST_ARMOR_PTR as usize..(WAIST_ARMOR_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[WAIST_ARMOR_PTR as usize..(WAIST_ARMOR_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (LEG_ARMOR_PTR + 4) as usize {
                self.buffer[LEG_ARMOR_PTR as usize..(LEG_ARMOR_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[LEG_ARMOR_PTR as usize..(LEG_ARMOR_PTR + 4) as usize]);
            }

            // Mettre à jour les pointeurs de noms d'armures
            if saved_file_data.len() >= (HEAD_ARMOR_NAMES_PTR + 4) as usize {
                self.buffer[HEAD_ARMOR_NAMES_PTR as usize..(HEAD_ARMOR_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[HEAD_ARMOR_NAMES_PTR as usize..(HEAD_ARMOR_NAMES_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (BODY_ARMOR_NAMES_PTR + 4) as usize {
                self.buffer[BODY_ARMOR_NAMES_PTR as usize..(BODY_ARMOR_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[BODY_ARMOR_NAMES_PTR as usize..(BODY_ARMOR_NAMES_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (ARM_ARMOR_NAMES_PTR + 4) as usize {
                self.buffer[ARM_ARMOR_NAMES_PTR as usize..(ARM_ARMOR_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ARM_ARMOR_NAMES_PTR as usize..(ARM_ARMOR_NAMES_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (WAIST_ARMOR_NAMES_PTR + 4) as usize {
                self.buffer[WAIST_ARMOR_NAMES_PTR as usize..(WAIST_ARMOR_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[WAIST_ARMOR_NAMES_PTR as usize..(WAIST_ARMOR_NAMES_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (LEG_ARMOR_NAMES_PTR + 4) as usize {
                self.buffer[LEG_ARMOR_NAMES_PTR as usize..(LEG_ARMOR_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[LEG_ARMOR_NAMES_PTR as usize..(LEG_ARMOR_NAMES_PTR + 4) as usize]);
            }
            
            // Mettre à jour les pointeurs d'objets et transmog
            if saved_file_data.len() >= (ITEM_DATA_PTR + 4) as usize {
                self.buffer[ITEM_DATA_PTR as usize..(ITEM_DATA_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ITEM_DATA_PTR as usize..(ITEM_DATA_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (ITEM_NAMES_PTR + 4) as usize {
                self.buffer[ITEM_NAMES_PTR as usize..(ITEM_NAMES_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ITEM_NAMES_PTR as usize..(ITEM_NAMES_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (ITEM_DESC_PTR + 4) as usize {
                self.buffer[ITEM_DESC_PTR as usize..(ITEM_DESC_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[ITEM_DESC_PTR as usize..(ITEM_DESC_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (TRANSMOG_FORGING_PTR + 4) as usize {
                self.buffer[TRANSMOG_FORGING_PTR as usize..(TRANSMOG_FORGING_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[TRANSMOG_FORGING_PTR as usize..(TRANSMOG_FORGING_PTR + 4) as usize]);
            }
            
            // Mettre à jour les pointeurs de boutiques de décorations
            if saved_file_data.len() >= (DECO_SHOP_PTR + 4) as usize {
                self.buffer[DECO_SHOP_PTR as usize..(DECO_SHOP_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[DECO_SHOP_PTR as usize..(DECO_SHOP_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (DECO_G_SHOP_PTR + 4) as usize {
                self.buffer[DECO_G_SHOP_PTR as usize..(DECO_G_SHOP_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[DECO_G_SHOP_PTR as usize..(DECO_G_SHOP_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (CUFF_SHOP_PTR + 4) as usize {
                self.buffer[CUFF_SHOP_PTR as usize..(CUFF_SHOP_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[CUFF_SHOP_PTR as usize..(CUFF_SHOP_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (CUFF_GR_SHOP_PTR + 4) as usize {
                self.buffer[CUFF_GR_SHOP_PTR as usize..(CUFF_GR_SHOP_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[CUFF_GR_SHOP_PTR as usize..(CUFF_GR_SHOP_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (AUTOMATIC_SKILLS_TABLE_PTR + 4) as usize {
                self.buffer[AUTOMATIC_SKILLS_TABLE_PTR as usize..(AUTOMATIC_SKILLS_TABLE_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[AUTOMATIC_SKILLS_TABLE_PTR as usize..(AUTOMATIC_SKILLS_TABLE_PTR + 4) as usize]);
            }
            
            // Mettre à jour les pointeurs d'upgrades d'armes
            if saved_file_data.len() >= (MELEE_WEAPON_UPGRADE_PATH_PTR + 4) as usize {
                self.buffer[MELEE_WEAPON_UPGRADE_PATH_PTR as usize..(MELEE_WEAPON_UPGRADE_PATH_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[MELEE_WEAPON_UPGRADE_PATH_PTR as usize..(MELEE_WEAPON_UPGRADE_PATH_PTR + 4) as usize]);
            }
            if saved_file_data.len() >= (RANGED_WEAPON_UPGRADE_PATH_PTR + 4) as usize {
                self.buffer[RANGED_WEAPON_UPGRADE_PATH_PTR as usize..(RANGED_WEAPON_UPGRADE_PATH_PTR + 4) as usize]
                    .copy_from_slice(&saved_file_data[RANGED_WEAPON_UPGRADE_PATH_PTR as usize..(RANGED_WEAPON_UPGRADE_PATH_PTR + 4) as usize]);
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
        
        // TODO: À corriger - ne pas réécrire les blocs existants
        // 1) Melee weapons data block with 0xFFFF sentinel and pointer at 0x7C
        let melee_data_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let melee_block = write_melee_weapons_block(&self.melee_weapons)?;
        writer.write_all(&melee_block)?;

        // 2) Ranged weapons data block with 0xFFFF sentinel and pointer at 0x80
        let ranged_data_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let ranged_block = write_ranged_weapons_block(&self.ranged_weapons)?;
        writer.write_all(&ranged_block)?;

        // 3) Head armor block (0x50)
        let head_armor_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let head_block = write_armors_block(&self.head_armors)?;
        writer.write_all(&head_block)?;

        // 4) Body armor block (0x54)
        let body_armor_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let body_block = write_armors_block(&self.body_armors)?;
        writer.write_all(&body_block)?;

        // 5) Arms armor block (0x58)
        let arms_armor_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let arms_block = write_armors_block(&self.arms_armors)?;
        writer.write_all(&arms_block)?;

        // 6) Waist armor block (0x5C)
        let waist_armor_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let waist_block = write_armors_block(&self.waist_armors)?;
        writer.write_all(&waist_block)?;

        // 7) Legs armor block (0x60)
        let legs_armor_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let legs_block = write_armors_block(&self.legs_armors)?;
        writer.write_all(&legs_block)?;

        // 8) Items data block with 0xFFFF sentinel and pointer at 0xFC
        let item_data_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let items_block = write_items_block(&self.items)?;
        writer.write_all(&items_block)?;

        // 9) Transmog shop data block with sentinel and pointer at TRANSMOG_FORGING_PTR
        let transmog_data_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let transmog_block = write_transmog_data(&self.transmog_entries)?;
        writer.write_all(&transmog_block)?;

        // 9b) Deco shops (HR and GR) – same pattern, separate tables
        let deco_hr_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let deco_hr_block = write_deco_shop_block(&self.deco_shop_hr_entries)?;
        writer.write_all(&deco_hr_block)?;
        let deco_gr_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let deco_gr_block = write_deco_shop_block(&self.deco_shop_gr_entries)?;
        writer.write_all(&deco_gr_block)?;
        // Cuff shop table (HR and GR share same pattern). If you separate HR/GR, write both.
        let cuff_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let cuff_block = write_deco_shop_block(&self.cuff_shop_entries)?;
        writer.write_all(&cuff_block)?;
        let cuff_gr_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let cuff_gr_block = write_deco_shop_block(&self.cuff_gr_shop_entries)?;
        writer.write_all(&cuff_gr_block)?;

        // 9c) Automatic Skills Table
        let automatic_skills_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let automatic_skills_block = write_automatic_skills_block(&self.automatic_skills)?;
        writer.write_all(&automatic_skills_block)?;

        // 10) Melee weapon upgrade paths block + sentinel, pointer at MELEE_WEAPON_UPGRADE_PATH_PTR
        let mw_upgrades_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let mw_block = write_mw_upgrades_block(&self.mw_upgrade_entries)?;
        writer.write_all(&mw_block)?;

        // 11) Ranged weapon upgrade paths block + sentinel, pointer at RANGED_WEAPON_UPGRADE_PATH_PTR
        let rw_upgrades_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let rw_block = write_rw_upgrades_block(&self.rw_upgrade_entries)?;
        writer.write_all(&rw_block)?;

        // 12) Armor upgrades removed

        // 13) Weapon names and descriptions tables (patch header pointers)
        // Melee names
        let melee_names_count = self.melee_weapons.len().min(self.melee_weapon_names.len()).min(self.melee_weapon_descriptions.len());
        let melee_names_table_offset = if melee_names_count > 0 {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            let _ = crate::core::mhfdat::write_weapon_names(&mut writer, &self.melee_weapon_names[..melee_names_count])?;
            current_pos
        } else { 0 };

        // Melee descriptions: table of pointers (4 per entry: 3 descriptions + 1 null) followed by SJIS strings
        let melee_desc_table_offset = if melee_names_count > 0 {
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
        } else { 0 };

        // Ranged names
        let ranged_names_count = self.ranged_weapons.len().min(self.ranged_weapon_names.len()).min(self.ranged_weapon_descriptions.len());
        let ranged_names_table_offset = if ranged_names_count > 0 {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            let _ = crate::core::mhfdat::write_ranged_weapon_names(&mut writer, &self.ranged_weapon_names[..ranged_names_count])?;
            current_pos
        } else { 0 };

        // Ranged descriptions: table of pointers (4 per entry: 3 descriptions + 1 null) followed by SJIS strings
        let ranged_desc_table_offset = if ranged_names_count > 0 {
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
        } else { 0 };

        // 14) Armor names tables
        let head_armor_names_offset = if !self.head_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_armor_names(&mut writer, &self.head_armor_names)?;
            current_pos
        } else { 0 };

        let body_armor_names_offset = if !self.body_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_armor_names(&mut writer, &self.body_armor_names)?;
            current_pos
        } else { 0 };

        let arms_armor_names_offset = if !self.arms_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_armor_names(&mut writer, &self.arms_armor_names)?;
            current_pos
        } else { 0 };

        let waist_armor_names_offset = if !self.waist_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_armor_names(&mut writer, &self.waist_armor_names)?;
            current_pos
        } else { 0 };

        let legs_armor_names_offset = if !self.legs_armor_names.is_empty() {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_armor_names(&mut writer, &self.legs_armor_names)?;
            current_pos
        } else { 0 };

        // 15) Item names and descriptions
        let item_names_count = self.items.len().min(self.item_names.len());
        let item_names_offset = if item_names_count > 0 {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_item_names(&mut writer, &self.item_names[..item_names_count])?;
            current_pos
        } else { 0 };

        let item_desc_count = self.items.len().min(self.item_descriptions.len());
        let item_desc_offset = if item_desc_count > 0 {
            let current_pos = writer.seek(SeekFrom::Current(0))? as u32;
            write_item_descriptions(&mut writer, &self.item_descriptions[..item_desc_count])?;
            current_pos
        } else { 0 };

        // Patch header pointers
        
        writer.seek(SeekFrom::Start(MELEE_WEAPONS_PTR as u64))?;
        writer.write_all(&melee_data_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(RANGED_WEAPONS_PTR as u64))?;
        writer.write_all(&ranged_data_offset.to_le_bytes())?;

        // Patch names/desc pointers
        if melee_names_table_offset != 0 {
            writer.seek(SeekFrom::Start(MELEE_WEAPON_NAMES_PTR as u64))?;
            writer.write_all(&melee_names_table_offset.to_le_bytes())?;
        }
        if melee_desc_table_offset != 0 {
            writer.seek(SeekFrom::Start(MELEE_WEAPON_DESC_PTR as u64))?;
            writer.write_all(&melee_desc_table_offset.to_le_bytes())?;
        }
        if ranged_names_table_offset != 0 {
            writer.seek(SeekFrom::Start(RANGED_WEAPON_NAMES_PTR as u64))?;
            writer.write_all(&ranged_names_table_offset.to_le_bytes())?;
        }
        if ranged_desc_table_offset != 0 {
            writer.seek(SeekFrom::Start(RANGED_WEAPON_DESC_PTR as u64))?;
            writer.write_all(&ranged_desc_table_offset.to_le_bytes())?;
        }

        writer.seek(SeekFrom::Start(HEAD_ARMOR_PTR as u64))?;
        writer.write_all(&head_armor_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(BODY_ARMOR_PTR as u64))?;
        writer.write_all(&body_armor_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(ARM_ARMOR_PTR as u64))?;
        writer.write_all(&arms_armor_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(WAIST_ARMOR_PTR as u64))?;
        writer.write_all(&waist_armor_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(LEG_ARMOR_PTR as u64))?;
        writer.write_all(&legs_armor_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(HEAD_ARMOR_NAMES_PTR as u64))?;
        writer.write_all(&head_armor_names_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(BODY_ARMOR_NAMES_PTR as u64))?;
        writer.write_all(&body_armor_names_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(ARM_ARMOR_NAMES_PTR as u64))?;
        writer.write_all(&arms_armor_names_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(WAIST_ARMOR_NAMES_PTR as u64))?;
        writer.write_all(&waist_armor_names_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(LEG_ARMOR_NAMES_PTR as u64))?;
        writer.write_all(&legs_armor_names_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(ITEM_DATA_PTR as u64))?;
        writer.write_all(&item_data_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(ITEM_NAMES_PTR as u64))?;
        writer.write_all(&item_names_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(ITEM_DESC_PTR as u64))?;
        writer.write_all(&item_desc_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(TRANSMOG_FORGING_PTR as u64))?;
        writer.write_all(&transmog_data_offset.to_le_bytes())?;

        // Patch deco shop pointers (HR/GR)
        writer.seek(SeekFrom::Start(DECO_SHOP_PTR as u64))?;
        writer.write_all(&deco_hr_offset.to_le_bytes())?;
        writer.seek(SeekFrom::Start(DECO_G_SHOP_PTR as u64))?;
        writer.write_all(&deco_gr_offset.to_le_bytes())?;
        writer.seek(SeekFrom::Start(CUFF_SHOP_PTR as u64))?;
        writer.write_all(&cuff_offset.to_le_bytes())?;
        writer.seek(SeekFrom::Start(CUFF_GR_SHOP_PTR as u64))?;
        writer.write_all(&cuff_gr_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(AUTOMATIC_SKILLS_TABLE_PTR as u64))?;
        writer.write_all(&automatic_skills_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(MELEE_WEAPON_UPGRADE_PATH_PTR as u64))?;
        writer.write_all(&mw_upgrades_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(RANGED_WEAPON_UPGRADE_PATH_PTR as u64))?;
        writer.write_all(&rw_upgrades_offset.to_le_bytes())?;

        // armor upgrades pointer removed

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