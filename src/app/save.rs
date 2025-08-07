use super::*;
use std::io::{Read, Seek, Write, SeekFrom, Cursor};
use std::fs::File;
use crate::core::packing::pack_file;
use crate::core::mhfdat::{write_equipment_data, write_data_with_padding, save, read_equipment_counts, write_equipment_counts};
use crate::model::mhfdat::MhfdatMeleeWeapon;
use tempfile;

impl MhfdatApp {
    pub fn add_new_melee_weapons(&mut self, count: usize) {
        let start_id = self.melee_weapons.len();
        for i in 0..count {
            let new_weapon = MhfdatMeleeWeapon::default();
            self.melee_weapons.push(new_weapon);
            self.melee_weapon_names.push(format!("New Weapon {}", start_id + i));
            self.melee_weapon_descriptions.push(["".to_string(), "".to_string(), "".to_string()]);
        }
        
        // Mettre à jour le compteur d'armes
        if let Some(mut equipment_counts) = read_equipment_counts(&self.buffer) {
            equipment_counts.numMeleeW = (start_id + count) as u16;
            write_equipment_counts(&mut self.buffer, &equipment_counts);
        }
    }

    pub fn save_modified_data(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.current_file {
            // 1. Créer un chemin temporaire
            let temp_path = path.with_extension("tmp");

            // 2. Copier le fichier original vers le temporaire
            std::fs::copy(path, &temp_path)?;

            // 3. Ouvrir le temporaire en écriture
            let file = std::fs::File::options().read(true).write(true).open(&temp_path)?;

            // 4. Écrire les modifications dans la copie
            self.save_modified_data_to_writer(file)?;

            // 5. Remplacer l'original par la copie
            std::fs::rename(&temp_path, path)?;
        }
        Ok(())
    }

    pub fn save_modified_data_to_writer<W: Read + Seek + Write >(&self, mut writer: W) -> std::io::Result<()> {
        // Lire le fichier existant
        let mut buffer = Vec::new();
        writer.seek(SeekFrom::Start(0))?;
        writer.read_to_end(&mut buffer)?;
        
        // Mettre à jour le compteur d'armes de mêlée dans le buffer
        if let Some(mut equipment_counts) = read_equipment_counts(&buffer) {
            let num_weapons = self.melee_weapons.len();
            equipment_counts.numMeleeW = num_weapons as u16;
            write_equipment_counts(&mut buffer, &equipment_counts);
        }
        
        // Écrire le buffer mis à jour d'abord
        writer.seek(SeekFrom::Start(0))?;
        writer.write_all(&buffer)?;
        
        // On se place à la fin du fichier pour ajouter les nouveaux blocs
        writer.seek(SeekFrom::End(0))?;

        // 1. Données d'équipement
        let melee_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let equipment_data = write_equipment_data(&self.melee_weapons, &Vec::new())?;
        write_data_with_padding(&mut writer, &equipment_data)?;

        // 2. Noms d'armes de mêlée
        let melee_names_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let mut melee_names_data = Vec::new();
        let mut melee_names_cursor = Cursor::new(&mut melee_names_data);
        write_weapon_names(&mut melee_names_cursor, &self.melee_weapon_names)?;
        write_data_with_padding(&mut writer, &melee_names_data)?;

        // 3. Descriptions d'armes de mêlée
        let melee_desc_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let mut melee_desc_data = Vec::new();
        let mut melee_desc_cursor = Cursor::new(&mut melee_desc_data);
        write_weapon_names(&mut melee_desc_cursor, &self.melee_weapon_descriptions.iter().map(|d| d[0].clone()).collect::<Vec<_>>())?;
        write_data_with_padding(&mut writer, &melee_desc_data)?;

        // 4. Noms d'armes à distance
        let ranged_names_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let mut ranged_names_data = Vec::new();
        let mut ranged_names_cursor = Cursor::new(&mut ranged_names_data);
        write_ranged_weapon_names(&mut ranged_names_cursor, &self.ranged_weapon_names)?;
        write_data_with_padding(&mut writer, &ranged_names_data)?;

        // 5. Descriptions d'armes à distance
        let ranged_desc_offset = writer.seek(SeekFrom::Current(0))? as u32;
        let mut ranged_desc_data = Vec::new();
        let mut ranged_desc_cursor = Cursor::new(&mut ranged_desc_data);
        write_weapon_names(&mut ranged_desc_cursor, &self.ranged_weapon_descriptions.iter().map(|d| d[0].clone()).collect::<Vec<_>>())?;
        write_data_with_padding(&mut writer, &ranged_desc_data)?;

        // Mettre à jour les pointeurs dans le fichier
        writer.seek(SeekFrom::Start(0x7C))?;
        writer.write_all(&melee_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(0x80))?;
        writer.write_all(&melee_names_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(0x84))?;
        writer.write_all(&ranged_names_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(0x88))?;
        writer.write_all(&melee_desc_offset.to_le_bytes())?;

        writer.seek(SeekFrom::Start(0x8C))?;
        writer.write_all(&ranged_desc_offset.to_le_bytes())?;
        Ok(())
    }

    pub fn save_with_packing(&mut self) -> std::io::Result<()> {
        if let Some(path) = &self.current_file {
            let temp_file = tempfile::NamedTempFile::new()?;
            self.save_modified_data_to_writer(&temp_file)?;
            
            pack_file(temp_file.path(), path, self.should_encrypt)?;
        }
        Ok(())
    }
} 

