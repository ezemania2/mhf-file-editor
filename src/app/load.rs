use super::*;
use std::io::{Read, Seek, SeekFrom, Cursor};
use crate::model::mhfdat_pointers::*;
use crate::core::mhfdat::{*, parse_items, parse_item_names, parse_item_descriptions};
use crate::utils::equip_flags::*;
use std::mem::size_of;
use std::fs::File;
use std::io::Write;
use std::path::Path;

impl MhfdatApp {
    pub fn load_ranged_weapon_names(&mut self) {
        let count = self.ranged_weapons.len();
        let mut cursor = std::io::Cursor::new(&self.buffer);
        if let Ok(names) = extract_ranged_weapon_names(&mut cursor, RANGED_WEAPON_NAMES_PTR, count) {
            self.ranged_weapon_names = names;
        }
    }

    pub fn load_armor_data(&mut self, buffer: &[u8]) {
        let mut cursor = std::io::Cursor::new(buffer);
        
        // Load head armor
        self.armor_tab = ArmorTab::Head;
        if let Ok(()) = self.read_armor_data(&mut cursor, HEAD_ARMOR_PTR as u64) {
            cursor.seek(SeekFrom::Start(0)).unwrap();
            if let Ok(names) = extract_armor_names(&mut cursor, HEAD_ARMOR_NAMES_PTR, self.head_armors.len()) {
                self.head_armor_names = names;
            }
        }

        // Load body armor
        self.armor_tab = ArmorTab::Body;
        cursor.seek(SeekFrom::Start(0)).unwrap();
        if let Ok(()) = self.read_armor_data(&mut cursor, BODY_ARMOR_PTR as u64) {
            cursor.seek(SeekFrom::Start(0)).unwrap();
            if let Ok(names) = extract_armor_names(&mut cursor, BODY_ARMOR_NAMES_PTR, self.body_armors.len()) {
                self.body_armor_names = names;
            }
        }

        // Load arms armor
        self.armor_tab = ArmorTab::Arms;
        cursor.seek(SeekFrom::Start(0)).unwrap();
        if let Ok(()) = self.read_armor_data(&mut cursor, ARM_ARMOR_PTR as u64) {
            cursor.seek(SeekFrom::Start(0)).unwrap();
            if let Ok(names) = extract_armor_names(&mut cursor, ARM_ARMOR_NAMES_PTR, self.arms_armors.len()) {
                self.arms_armor_names = names;
            }
        }

        // Load waist armor
        self.armor_tab = ArmorTab::Waist;
        cursor.seek(SeekFrom::Start(0)).unwrap();
        if let Ok(()) = self.read_armor_data(&mut cursor, WAIST_ARMOR_PTR as u64) {
            cursor.seek(SeekFrom::Start(0)).unwrap();
            if let Ok(names) = extract_armor_names(&mut cursor, WAIST_ARMOR_NAMES_PTR, self.waist_armors.len()) {
                self.waist_armor_names = names;
            }
        }

        // Load legs armor
        self.armor_tab = ArmorTab::Legs;
        cursor.seek(SeekFrom::Start(0)).unwrap();
        if let Ok(()) = self.read_armor_data(&mut cursor, LEG_ARMOR_PTR as u64) {
            cursor.seek(SeekFrom::Start(0)).unwrap();
            if let Ok(names) = extract_armor_names(&mut cursor, LEG_ARMOR_NAMES_PTR, self.legs_armors.len()) {
                self.legs_armor_names = names;
            }
        }

        // Load armor descriptions
        cursor.seek(SeekFrom::Start(0)).unwrap();
        let total_armors = self.head_armors.len() + self.body_armors.len() + self.arms_armors.len() + 
                          self.waist_armors.len() + self.legs_armors.len();
        if let Ok(descriptions) = extract_armor_descriptions(&mut cursor, HEAD_ARMOR_DESC_PTR, total_armors) {
            self.armor_descriptions = descriptions;
        }

        // === LOG ARMOR DATA POINTERS AND FIRST 3 ENTRIES OF EACH TYPE ===
        let log_path = if let Some(current_file) = &self.current_file {
            let parent = Path::new(current_file).parent().unwrap_or_else(|| Path::new(""));
            parent.join("armor_data.log")
        } else {
            Path::new("armor_data.log").to_path_buf()
        };
        if let Ok(mut file) = File::create(log_path) {
            writeln!(file, "=== ARMOR DATA POINTERS ===").ok();
            let mut cursor = std::io::Cursor::new(&self.buffer);
            let mut read_offset = |offset: u32| -> u64 {
                cursor.seek(SeekFrom::Start(offset as u64)).ok();
                let mut bytes = [0u8; 4];
                cursor.read_exact(&mut bytes).ok();
                u32::from_le_bytes(bytes) as u64
            };
            writeln!(file, "Head Armor: 0x{:08X}", read_offset(HEAD_ARMOR_PTR)).ok();
            writeln!(file, "Body Armor: 0x{:08X}", read_offset(BODY_ARMOR_PTR)).ok();
            writeln!(file, "Arms Armor: 0x{:08X}", read_offset(ARM_ARMOR_PTR)).ok();
            writeln!(file, "Waist Armor: 0x{:08X}", read_offset(WAIST_ARMOR_PTR)).ok();
            writeln!(file, "Legs Armor: 0x{:08X}", read_offset(LEG_ARMOR_PTR)).ok();
            writeln!(file, "").ok();
            let log_armor = |file: &mut File, label: &str, armors: &Vec<MhfdatEquipment>| {
                writeln!(file, "=== {} ARMOR (First 3) ===", label).ok();
                for (i, armor) in armors.iter().take(3).enumerate() {
                    // Copy packed fields to local variables
                    let model_id_male = armor.model_id_male;
                    let model_id_female = armor.model_id_female;
                    let equipable_by = armor.equipable_by;
                    let rarity = armor.rarity;
                    let max_level = armor.max_level;
                    let zenny_cost = armor.zenny_cost;
                    let base_defense = armor.base_defense;
                    let fire_res = armor.fire_res;
                    let water_res = armor.water_res;
                    let thunder_res = armor.thunder_res;
                    let dragon_res = armor.dragon_res;
                    let ice_res = armor.ice_res;
                    let base_slots = armor.base_slots;
                    let max_slots = armor.max_slots;
                    let zenith_skill = armor.zenith_skill;
                    writeln!(file, "{}. Model IDs: Male=0x{:04X}, Female=0x{:04X}", i+1, model_id_male, model_id_female).ok();
                    writeln!(file, "   Flags: 0x{:02X}", equipable_by).ok();
                    writeln!(file, "   Rarity: {}", rarity).ok();
                    writeln!(file, "   Max Level: {}", max_level).ok();
                    writeln!(file, "   Zenny: {}", zenny_cost).ok();
                    writeln!(file, "   Defense: {}", base_defense).ok();
                    writeln!(file, "   Resistances: Fire={}, Water={}, Thunder={}, Dragon={}, Ice={}", fire_res, water_res, thunder_res, dragon_res, ice_res).ok();
                    writeln!(file, "   Slots: Base={}, Max={}", base_slots, max_slots).ok();
                    writeln!(file, "   Zenith Skill: 0x{:04X}", zenith_skill).ok();
                    writeln!(file, "").ok();
                }
            };
            log_armor(&mut file, "HEAD", &self.head_armors);
            log_armor(&mut file, "BODY", &self.body_armors);
            log_armor(&mut file, "ARMS", &self.arms_armors);
            log_armor(&mut file, "WAIST", &self.waist_armors);
            log_armor(&mut file, "LEGS", &self.legs_armors);
        }

        // Load items
        let items = parse_items(&self.buffer);
        self.items = items;
        
        // Load item names using the new parse function
        let names = parse_item_names(&self.buffer, self.items.len());
        self.item_names = names;
        
        // Load item descriptions using the new parse function
        let descriptions = parse_item_descriptions(&self.buffer, self.items.len());
        self.item_descriptions = descriptions;
    }

    pub fn load_transmog_entries(&mut self) {
        let valid_types = [0x00, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let ptr_offset = TRANSMOG_FORGING_PTR as usize;
        if self.buffer.len() < ptr_offset + 4 { return; }
        let data_offset = u32::from_le_bytes(self.buffer[ptr_offset..ptr_offset+4].try_into().unwrap()) as usize;
        let entry_size = size_of::<ShopEntry>();
        let mut entries = Vec::new();
        let mut cursor = data_offset;
        while cursor + entry_size <= self.buffer.len() {
            let equip_type = self.buffer[cursor];
            if !valid_types.contains(&equip_type) { break; }
            let entry = unsafe { std::ptr::read_unaligned(self.buffer.as_ptr().add(cursor) as *const ShopEntry) };
            entries.push(entry);
            cursor += entry_size;
        }
        self.transmog_entries = entries;
    }



    pub fn read_armor_data(&mut self, cursor: &mut Cursor<&[u8]>, offset: u64) -> Result<(), std::io::Error> {
        // Read the actual data offset from the pointer location
        cursor.seek(SeekFrom::Start(offset))?;
        let mut data_offset_bytes = [0u8; 4];
        cursor.read_exact(&mut data_offset_bytes)?;
        let data_offset = u32::from_le_bytes(data_offset_bytes) as u64;
        
        // Log the pattern
        let log_path = if let Some(current_file) = &self.current_file {
            let parent = Path::new(current_file).parent().unwrap_or_else(|| Path::new(""));
            parent.join("armor_pattern.log")
        } else {
            Path::new("armor_pattern.log").to_path_buf()
        };
        
        if let Ok(mut file) = File::create(log_path) {
            writeln!(file, "=== ARMOR DATA PATTERN ===").ok();
            writeln!(file, "Pointer Offset: 0x{:08X}", offset).ok();
            writeln!(file, "Data Offset: 0x{:08X}", data_offset).ok();
            writeln!(file, "").ok();
            
            // Save current position
            let start_pos = cursor.position();
            
            // Seek to data start
            cursor.seek(SeekFrom::Start(data_offset))?;
            
            let mut entry_count = 0;
            loop {
                let entry_start = cursor.position();
                let mut raw_data = [0u8; 72];
                if cursor.read_exact(&mut raw_data).is_err() {
                    break;
                }
                
                // Check for sentinel
                if u16::from_le_bytes([raw_data[0], raw_data[1]]) == 0xFFFF && 
                   u16::from_le_bytes([raw_data[2], raw_data[3]]) == 0xFFFF {
                    break;
                }
                
                writeln!(file, "=== Entry {} at offset 0x{:08X} ===", entry_count, entry_start).ok();
                writeln!(file, "model_id_male: 0x{:04X}", u16::from_le_bytes([raw_data[0], raw_data[1]])).ok();
                writeln!(file, "model_id_female: 0x{:04X}", u16::from_le_bytes([raw_data[2], raw_data[3]])).ok();
                writeln!(file, "equipable_by: 0x{:02X}", raw_data[4]).ok();
                writeln!(file, "rarity: 0x{:02X}", raw_data[5]).ok();
                writeln!(file, "max_level: 0x{:02X}", raw_data[6]).ok();
                writeln!(file, "unk07: 0x{:02X}", raw_data[7]).ok();
                writeln!(file, "unk08: 0x{:04X}", u16::from_le_bytes([raw_data[8], raw_data[9]])).ok();
                writeln!(file, "unk0A: 0x{:04X}", u16::from_le_bytes([raw_data[10], raw_data[11]])).ok();
                writeln!(file, "zenny_cost: 0x{:08X}", u32::from_le_bytes([raw_data[12], raw_data[13], raw_data[14], raw_data[15]])).ok();
                writeln!(file, "unk10: 0x{:04X}", u16::from_le_bytes([raw_data[16], raw_data[17]])).ok();
                writeln!(file, "base_defense: 0x{:04X}", u16::from_le_bytes([raw_data[18], raw_data[19]])).ok();
                writeln!(file, "fire_res: 0x{:02X}", raw_data[20] as i8).ok();
                writeln!(file, "water_res: 0x{:02X}", raw_data[21] as i8).ok();
                writeln!(file, "thunder_res: 0x{:02X}", raw_data[22] as i8).ok();
                writeln!(file, "dragon_res: 0x{:02X}", raw_data[23] as i8).ok();
                writeln!(file, "ice_res: 0x{:02X}", raw_data[24] as i8).ok();
                writeln!(file, "unk19: 0x{:02X}", raw_data[25]).ok();
                writeln!(file, "unk1A: 0x{:02X}", raw_data[26]).ok();
                writeln!(file, "base_slots: 0x{:02X}", raw_data[27]).ok();
                writeln!(file, "max_slots: 0x{:02X}", raw_data[28]).ok();
                writeln!(file, "sth_event_crown: 0x{:02X}", raw_data[29]).ok();
                writeln!(file, "unk1E: 0x{:04X}", u16::from_le_bytes([raw_data[30], raw_data[31]])).ok();
                writeln!(file, "equip_id: 0x{:04X}", u16::from_le_bytes([raw_data[32], raw_data[33]])).ok();
                writeln!(file, "unk22: 0x{:04X}", u16::from_le_bytes([raw_data[34], raw_data[35]])).ok();
                writeln!(file, "unk24: 0x{:08X}", u32::from_le_bytes([raw_data[36], raw_data[37], raw_data[38], raw_data[39]])).ok();
                writeln!(file, "unk28: 0x{:04X}", u16::from_le_bytes([raw_data[40], raw_data[41]])).ok();
                writeln!(file, "skill_id1: 0x{:02X}", raw_data[42]).ok();
                writeln!(file, "skill_pts1: 0x{:02X}", raw_data[43] as i8).ok();
                writeln!(file, "skill_id2: 0x{:02X}", raw_data[44]).ok();
                writeln!(file, "skill_pts2: 0x{:02X}", raw_data[45] as i8).ok();
                writeln!(file, "skill_id3: 0x{:02X}", raw_data[46]).ok();
                writeln!(file, "skill_pts3: 0x{:02X}", raw_data[47] as i8).ok();
                writeln!(file, "skill_id4: 0x{:02X}", raw_data[48]).ok();
                writeln!(file, "skill_pts4: 0x{:02X}", raw_data[49] as i8).ok();
                writeln!(file, "skill_id5: 0x{:02X}", raw_data[50]).ok();
                writeln!(file, "skill_pts5: 0x{:02X}", raw_data[51] as i8).ok();
                writeln!(file, "sth_hidden: 0x{:08X}", u32::from_le_bytes([raw_data[52], raw_data[53], raw_data[54], raw_data[55]])).ok();
                writeln!(file, "unk38: 0x{:08X}", u32::from_le_bytes([raw_data[56], raw_data[57], raw_data[58], raw_data[59]])).ok();
                writeln!(file, "unk3C: 0x{:04X}", u16::from_le_bytes([raw_data[60], raw_data[61]])).ok();
                writeln!(file, "unk3E: 0x{:02X}", raw_data[62]).ok();
                writeln!(file, "zero_f: 0x{:02X}", raw_data[63]).ok();
                writeln!(file, "unk40: 0x{:08X}", u32::from_le_bytes([raw_data[64], raw_data[65], raw_data[66], raw_data[67]])).ok();
                writeln!(file, "unk44: 0x{:04X}", u16::from_le_bytes([raw_data[68], raw_data[69]])).ok();
                writeln!(file, "zenith_skill: 0x{:04X}", u16::from_le_bytes([raw_data[70], raw_data[71]])).ok();
                writeln!(file, "").ok();
                
                entry_count += 1;
                if entry_count >= 10 { // Limit to first 10 entries for readability
                    break;
                }
            }
            
            // Restore position
            cursor.seek(SeekFrom::Start(start_pos))?;
        }
        
        // Seek to the actual data location
        cursor.seek(SeekFrom::Start(data_offset))?;
        
        loop {
            let mut entry = MhfdatEquipment::default();
            
            // Read model IDs
            let mut model_id_male = [0u8; 2];
            let mut model_id_female = [0u8; 2];
            cursor.read_exact(&mut model_id_male)?;
            cursor.read_exact(&mut model_id_female)?;
            entry.model_id_male = u16::from_le_bytes(model_id_male);
            entry.model_id_female = u16::from_le_bytes(model_id_female);
            
            // Read bitfield for equipment flags
            let mut bitfield = [0u8; 1];
            cursor.read_exact(&mut bitfield)?;
            let bitfield = bitfield[0];
            
            // Set equipable_by based on all known flags
            let is_male = is_male_equip(bitfield);
            let is_female = is_female_equip(bitfield);
            let is_blade = is_blade_equip(bitfield);
            let is_gunner = is_gunner_equip(bitfield);
            let is_bool1 = is_bool1(bitfield);
            let is_sp = is_sp_equip(bitfield);
            let is_bool3 = is_bool3(bitfield);
            let is_bool4 = is_bool4(bitfield);
            
            // Set equipable_by based on all flags
            entry.equipable_by = match (is_male, is_female, is_blade, is_gunner, is_sp) {
                // Male only combinations
                (true, false, true, false, false) => 1,  // Male Blademaster only
                (true, false, false, true, false) => 2,  // Male Gunner only
                (true, false, true, true, false) => 3,   // Male Both
                
                // Female only combinations
                (false, true, true, false, false) => 4,  // Female Blademaster only
                (false, true, false, true, false) => 5,  // Female Gunner only
                (false, true, true, true, false) => 6,   // Female Both
                
                // Both genders combinations
                (true, true, true, false, false) => 7,   // Both Blademaster only
                (true, true, false, true, false) => 8,   // Both Gunner only
                (true, true, true, true, false) => 9,    // Both Both
                
                // Special equipment
                (_, _, _, _, true) => 10,                // Special equipment
                
                // Default case
                _ => 0,                                  // Unknown/Invalid combination
            };
            
            // Read rarity and other basic stats
            let mut rarity = [0u8; 1];
            cursor.read_exact(&mut rarity)?;
            entry.rarity = rarity[0];
            
            let mut max_level = [0u8; 1];
            cursor.read_exact(&mut max_level)?;
            entry.max_level = max_level[0];
            
            // Skip unknown bytes
            cursor.seek(SeekFrom::Current(5))?;
            
            // Read zenny cost
            let mut zenny_cost = [0u8; 4];
            cursor.read_exact(&mut zenny_cost)?;
            entry.zenny_cost = u32::from_le_bytes(zenny_cost);
            
            // Skip unknown short
            cursor.seek(SeekFrom::Current(2))?;
            
            // Read defense and resistances
            let mut base_defense = [0u8; 2];
            cursor.read_exact(&mut base_defense)?;
            entry.base_defense = u16::from_le_bytes(base_defense);
            
            // Read resistances (1 byte each)
            let mut fire_res = [0u8; 1];
            let mut water_res = [0u8; 1];
            let mut thunder_res = [0u8; 1];
            let mut dragon_res = [0u8; 1];
            let mut ice_res = [0u8; 1];
            
            cursor.read_exact(&mut fire_res)?;
            cursor.read_exact(&mut water_res)?;
            cursor.read_exact(&mut thunder_res)?;
            cursor.read_exact(&mut dragon_res)?;
            cursor.read_exact(&mut ice_res)?;
            
            entry.fire_res = fire_res[0] as i8;
            entry.water_res = water_res[0] as i8;
            entry.thunder_res = thunder_res[0] as i8;
            entry.dragon_res = dragon_res[0] as i8;
            entry.ice_res = ice_res[0] as i8;
            
            // Read unk19 and unk1A
            let mut unk19 = [0u8; 1];
            let mut unk1A = [0u8; 1];
            cursor.read_exact(&mut unk19)?;
            cursor.read_exact(&mut unk1A)?;
            
            // Read slots
            let mut base_slots = [0u8; 1];
            let mut max_slots = [0u8; 1];
            cursor.read_exact(&mut base_slots)?;
            cursor.read_exact(&mut max_slots)?;
            entry.base_slots = base_slots[0];
            entry.max_slots = max_slots[0];
            
            // Read sth_event_crown
            let mut sth_event_crown = [0u8; 1];
            cursor.read_exact(&mut sth_event_crown)?;
            
            // Read unk1E
            let mut unk1E = [0u8; 2];
            cursor.read_exact(&mut unk1E)?;
            
            // Read equip_id
            let mut equip_id = [0u8; 2];
            cursor.read_exact(&mut equip_id)?;
            
            // Read unk22
            let mut unk22 = [0u8; 2];
            cursor.read_exact(&mut unk22)?;
            
            // Read unk24
            let mut unk24 = [0u8; 4];
            cursor.read_exact(&mut unk24)?;
            
            // Read unk28
            let mut unk28 = [0u8; 2];
            cursor.read_exact(&mut unk28)?;
            
            // Read skills
            let mut skill_data = [0u8; 10];  // 5 pairs of skill_id and skill_pts
            cursor.read_exact(&mut skill_data)?;
            
            // Assign skills to entry
            entry.skill_id1 = skill_data[0];
            entry.skill_pts1 = skill_data[1] as i8;
            entry.skill_id2 = skill_data[2];
            entry.skill_pts2 = skill_data[3] as i8;
            entry.skill_id3 = skill_data[4];
            entry.skill_pts3 = skill_data[5] as i8;
            entry.skill_id4 = skill_data[6];
            entry.skill_pts4 = skill_data[7] as i8;
            entry.skill_id5 = skill_data[8];
            entry.skill_pts5 = skill_data[9] as i8;
            
            // Read remaining data
            let mut sth_hidden = [0u8; 4];
            let mut unk38 = [0u8; 4];
            let mut unk3C = [0u8; 2];
            let mut unk3E = [0u8; 1];
            let mut zero_f = [0u8; 1];
            let mut unk40 = [0u8; 4];
            let mut unk44 = [0u8; 2];
            let mut zenith_skill = [0u8; 2];
            
            cursor.read_exact(&mut sth_hidden)?;
            cursor.read_exact(&mut unk38)?;
            cursor.read_exact(&mut unk3C)?;
            cursor.read_exact(&mut unk3E)?;
            cursor.read_exact(&mut zero_f)?;
            cursor.read_exact(&mut unk40)?;
            cursor.read_exact(&mut unk44)?;
            cursor.read_exact(&mut zenith_skill)?;
            
            entry.zenith_skill = u16::from_le_bytes(zenith_skill);
            
            // Check for sentinel value (0xFFFF)
            if entry.model_id_male == 0xFFFF && entry.model_id_female == 0xFFFF {
                break;
            }
            
            // Add to appropriate armor list based on type
            match self.armor_tab {
                ArmorTab::Head => self.head_armors.push(entry),
                ArmorTab::Body => self.body_armors.push(entry),
                ArmorTab::Arms => self.arms_armors.push(entry),
                ArmorTab::Waist => self.waist_armors.push(entry),
                ArmorTab::Legs => self.legs_armors.push(entry),
            }
        }
        
        Ok(())
    }
}
