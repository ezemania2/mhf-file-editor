use super::*;
use std::io::{Read, Seek, SeekFrom, Cursor};
use crate::model::mhfdat_pointers::*;
use crate::core::mhfdat::{*, parse_items, parse_item_names, parse_item_descriptions, parse_monster_descriptions};
use std::mem::size_of;

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
            // Save original offsets
            if buffer.len() >= HEAD_ARMOR_PTR as usize + 4 {
                self.original_head_armors_offset = Some(u32::from_le_bytes(buffer[HEAD_ARMOR_PTR as usize..HEAD_ARMOR_PTR as usize+4].try_into().unwrap()));
                self.head_armors_modified = false;
            }
            if buffer.len() >= HEAD_ARMOR_NAMES_PTR as usize + 4 {
                self.original_head_armor_names_offset = Some(u32::from_le_bytes(buffer[HEAD_ARMOR_NAMES_PTR as usize..HEAD_ARMOR_NAMES_PTR as usize+4].try_into().unwrap()));
                self.head_armor_names_modified = false;
            }
            
            cursor.seek(SeekFrom::Start(0)).unwrap();
            if let Ok(names) = extract_armor_names(&mut cursor, HEAD_ARMOR_NAMES_PTR, self.head_armors.len()) {
                self.head_armor_names = names;
            }
        }

        // Load body armor
        self.armor_tab = ArmorTab::Body;
        cursor.seek(SeekFrom::Start(0)).unwrap();
        if let Ok(()) = self.read_armor_data(&mut cursor, BODY_ARMOR_PTR as u64) {
            if buffer.len() >= BODY_ARMOR_PTR as usize + 4 {
                self.original_body_armors_offset = Some(u32::from_le_bytes(buffer[BODY_ARMOR_PTR as usize..BODY_ARMOR_PTR as usize+4].try_into().unwrap()));
                self.body_armors_modified = false;
            }
            if buffer.len() >= BODY_ARMOR_NAMES_PTR as usize + 4 {
                self.original_body_armor_names_offset = Some(u32::from_le_bytes(buffer[BODY_ARMOR_NAMES_PTR as usize..BODY_ARMOR_NAMES_PTR as usize+4].try_into().unwrap()));
                self.body_armor_names_modified = false;
            }
            cursor.seek(SeekFrom::Start(0)).unwrap();
            if let Ok(names) = extract_armor_names(&mut cursor, BODY_ARMOR_NAMES_PTR, self.body_armors.len()) {
                self.body_armor_names = names;
            }
        }

        // Load arms armor
        self.armor_tab = ArmorTab::Arms;
        cursor.seek(SeekFrom::Start(0)).unwrap();
        if let Ok(()) = self.read_armor_data(&mut cursor, ARM_ARMOR_PTR as u64) {
            if buffer.len() >= ARM_ARMOR_PTR as usize + 4 {
                self.original_arms_armors_offset = Some(u32::from_le_bytes(buffer[ARM_ARMOR_PTR as usize..ARM_ARMOR_PTR as usize+4].try_into().unwrap()));
                self.arms_armors_modified = false;
            }
            if buffer.len() >= ARM_ARMOR_NAMES_PTR as usize + 4 {
                self.original_arms_armor_names_offset = Some(u32::from_le_bytes(buffer[ARM_ARMOR_NAMES_PTR as usize..ARM_ARMOR_NAMES_PTR as usize+4].try_into().unwrap()));
                self.arms_armor_names_modified = false;
            }
            cursor.seek(SeekFrom::Start(0)).unwrap();
            if let Ok(names) = extract_armor_names(&mut cursor, ARM_ARMOR_NAMES_PTR, self.arms_armors.len()) {
                self.arms_armor_names = names;
            }
        }

        // Load waist armor
        self.armor_tab = ArmorTab::Waist;
        cursor.seek(SeekFrom::Start(0)).unwrap();
        if let Ok(()) = self.read_armor_data(&mut cursor, WAIST_ARMOR_PTR as u64) {
            if buffer.len() >= WAIST_ARMOR_PTR as usize + 4 {
                self.original_waist_armors_offset = Some(u32::from_le_bytes(buffer[WAIST_ARMOR_PTR as usize..WAIST_ARMOR_PTR as usize+4].try_into().unwrap()));
                self.waist_armors_modified = false;
            }
            if buffer.len() >= WAIST_ARMOR_NAMES_PTR as usize + 4 {
                self.original_waist_armor_names_offset = Some(u32::from_le_bytes(buffer[WAIST_ARMOR_NAMES_PTR as usize..WAIST_ARMOR_NAMES_PTR as usize+4].try_into().unwrap()));
                self.waist_armor_names_modified = false;
            }
            cursor.seek(SeekFrom::Start(0)).unwrap();
            if let Ok(names) = extract_armor_names(&mut cursor, WAIST_ARMOR_NAMES_PTR, self.waist_armors.len()) {
                self.waist_armor_names = names;
            }
        }

        // Load legs armor
        self.armor_tab = ArmorTab::Legs;
        cursor.seek(SeekFrom::Start(0)).unwrap();
        if let Ok(()) = self.read_armor_data(&mut cursor, LEG_ARMOR_PTR as u64) {
            if buffer.len() >= LEG_ARMOR_PTR as usize + 4 {
                self.original_legs_armors_offset = Some(u32::from_le_bytes(buffer[LEG_ARMOR_PTR as usize..LEG_ARMOR_PTR as usize+4].try_into().unwrap()));
                self.legs_armors_modified = false;
            }
            if buffer.len() >= LEG_ARMOR_NAMES_PTR as usize + 4 {
                self.original_legs_armor_names_offset = Some(u32::from_le_bytes(buffer[LEG_ARMOR_NAMES_PTR as usize..LEG_ARMOR_NAMES_PTR as usize+4].try_into().unwrap()));
                self.legs_armor_names_modified = false;
            }
            cursor.seek(SeekFrom::Start(0)).unwrap();
            if let Ok(names) = extract_armor_names(&mut cursor, LEG_ARMOR_NAMES_PTR, self.legs_armors.len()) {
                self.legs_armor_names = names;
            }
        }

        // Load armor descriptions
        cursor.seek(SeekFrom::Start(0)).unwrap();
        let total_armors = self.head_armors.len() + self.body_armors.len() + self.arms_armors.len() + 
                          self.waist_armors.len() + self.legs_armors.len();
        if buffer.len() >= ARMOR_DESC_PTR as usize + 4 {
            self.original_armor_descriptions_offset = Some(
                u32::from_le_bytes(buffer[ARMOR_DESC_PTR as usize..ARMOR_DESC_PTR as usize + 4].try_into().unwrap())
            );
            self.armor_descriptions_modified = false;
        }
        if let Ok(descriptions) = extract_armor_descriptions(&mut cursor, ARMOR_DESC_PTR, total_armors) {
            self.armor_descriptions = descriptions;
        }

        // Load items
        let items = parse_items(&self.buffer);
        self.items = items;
        
        // Save original offsets for items
        if buffer.len() >= ITEM_DATA_PTR as usize + 4 {
            self.original_items_offset = Some(u32::from_le_bytes(buffer[ITEM_DATA_PTR as usize..ITEM_DATA_PTR as usize+4].try_into().unwrap()));
            self.items_modified = false;
        }
        if buffer.len() >= ITEM_NAMES_PTR as usize + 4 {
            self.original_item_names_offset = Some(u32::from_le_bytes(buffer[ITEM_NAMES_PTR as usize..ITEM_NAMES_PTR as usize+4].try_into().unwrap()));
            self.item_names_modified = false;
        }
        if buffer.len() >= ITEM_DESC_PTR as usize + 4 {
            self.original_item_descriptions_offset = Some(u32::from_le_bytes(buffer[ITEM_DESC_PTR as usize..ITEM_DESC_PTR as usize+4].try_into().unwrap()));
            self.item_descriptions_modified = false;
        }
        
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
        
        // Save original offset
        self.original_transmog_offset = Some(data_offset as u32);
        self.transmog_modified = false;
        
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

    pub fn load_weapon_forging_entries(&mut self) {
        let valid_types = [0x06, 0x07]; // Only Melee (0x06) and Ranged (0x07)
        let ptr_offset = WEAPON_FORGING_PTR as usize;
        if self.buffer.len() < ptr_offset + 4 { return; }
        let data_offset = u32::from_le_bytes(self.buffer[ptr_offset..ptr_offset+4].try_into().unwrap()) as usize;
        
        // Save original offset
        self.original_weapon_forging_offset = Some(data_offset as u32);
        self.weapon_forging_modified = false;
        
        if data_offset == 0 || data_offset >= self.buffer.len() { return; }
        let entry_size = std::mem::size_of::<ShopEntry>();
        let mut cursor = data_offset;
        let mut entries = Vec::new();
        while cursor + entry_size <= self.buffer.len() {
            let equip_type = self.buffer[cursor];
            if !valid_types.contains(&equip_type) { break; }
            let entry = unsafe { std::ptr::read_unaligned(self.buffer.as_ptr().add(cursor) as *const ShopEntry) };
            entries.push(entry);
            cursor += entry_size;
        }
        self.weapon_forging_entries = entries;
    }

    pub fn load_armor_forging_entries(&mut self) {
        let valid_types = [0x00, 0x02, 0x03, 0x04, 0x05]; // Head, Body, Arms, Waist, Legs
        let ptr_offset = ARMOR_FORGING_PTR as usize;
        if self.buffer.len() < ptr_offset + 4 { return; }
        let data_offset = u32::from_le_bytes(self.buffer[ptr_offset..ptr_offset+4].try_into().unwrap()) as usize;
        
        // Save original offset
        self.original_armor_forging_offset = Some(data_offset as u32);
        self.armor_forging_modified = false;
        
        if data_offset == 0 || data_offset >= self.buffer.len() { return; }
        let entry_size = std::mem::size_of::<ShopEntry>();
        let mut cursor = data_offset;
        let mut entries = Vec::new();
        while cursor + entry_size <= self.buffer.len() {
            let equip_type = self.buffer[cursor];
            if !valid_types.contains(&equip_type) { break; }
            let entry = unsafe { std::ptr::read_unaligned(self.buffer.as_ptr().add(cursor) as *const ShopEntry) };
            entries.push(entry);
            cursor += entry_size;
        }
        self.armor_forging_entries = entries;
    }

    pub fn load_weapon_forging_gr_entries(&mut self) {
        let valid_types = [0x06, 0x07]; // Melee, Ranged
        let ptr_offset = G_RANK_WEAPON_SHOP_PTR as usize;
        if self.buffer.len() < ptr_offset + 4 { return; }
        let data_offset = u32::from_le_bytes(self.buffer[ptr_offset..ptr_offset+4].try_into().unwrap()) as usize;
        
        // Save original offset
        self.original_weapon_forging_gr_offset = Some(data_offset as u32);
        self.weapon_forging_gr_modified = false;
        
        if data_offset == 0 || data_offset >= self.buffer.len() { return; }
        let entry_size = std::mem::size_of::<ShopEntry>();
        let mut cursor = data_offset;
        let mut entries = Vec::new();
        while cursor + entry_size <= self.buffer.len() {
            let equip_type = self.buffer[cursor];
            if !valid_types.contains(&equip_type) { break; }
            let entry = unsafe { std::ptr::read_unaligned(self.buffer.as_ptr().add(cursor) as *const ShopEntry) };
            entries.push(entry);
            cursor += entry_size;
        }
        self.weapon_forging_gr_entries = entries;
    }

    pub fn load_armor_forging_gr_entries(&mut self) {
        let valid_types = [0x00, 0x02, 0x03, 0x04, 0x05]; // Head, Body, Arms, Waist, Legs
        let ptr_offset = G_RANK_ARMOR_SHOP_PTR as usize;
        if self.buffer.len() < ptr_offset + 4 { return; }
        let data_offset = u32::from_le_bytes(self.buffer[ptr_offset..ptr_offset+4].try_into().unwrap()) as usize;
        
        // Save original offset
        self.original_armor_forging_gr_offset = Some(data_offset as u32);
        self.armor_forging_gr_modified = false;
        
        if data_offset == 0 || data_offset >= self.buffer.len() { return; }
        let entry_size = std::mem::size_of::<ShopEntry>();
        let mut cursor = data_offset;
        let mut entries = Vec::new();
        while cursor + entry_size <= self.buffer.len() {
            let equip_type = self.buffer[cursor];
            if !valid_types.contains(&equip_type) { break; }
            let entry = unsafe { std::ptr::read_unaligned(self.buffer.as_ptr().add(cursor) as *const ShopEntry) };
            entries.push(entry);
            cursor += entry_size;
        }
        self.armor_forging_gr_entries = entries;
    }

    pub fn load_weapon_forging_zenith_entries(&mut self) {
        let valid_types = [0x06, 0x07]; // Melee, Ranged
        let ptr_offset = ZENITH_WEAPON_FORGING_PTR as usize;
        if self.buffer.len() < ptr_offset + 4 { return; }
        let data_offset = u32::from_le_bytes(self.buffer[ptr_offset..ptr_offset+4].try_into().unwrap()) as usize;
        
        // Save original offset
        self.original_weapon_forging_zenith_offset = Some(data_offset as u32);
        self.weapon_forging_zenith_modified = false;
        
        if data_offset == 0 || data_offset >= self.buffer.len() { return; }
        let entry_size = std::mem::size_of::<ShopEntry>();
        let mut cursor = data_offset;
        let mut entries = Vec::new();
        while cursor + entry_size <= self.buffer.len() {
            let equip_type = self.buffer[cursor];
            if !valid_types.contains(&equip_type) { break; }
            let entry = unsafe { std::ptr::read_unaligned(self.buffer.as_ptr().add(cursor) as *const ShopEntry) };
            entries.push(entry);
            cursor += entry_size;
        }
        self.weapon_forging_zenith_entries = entries;
    }

    pub fn load_armor_forging_zenith_entries(&mut self) {
        let valid_types = [0x00, 0x02, 0x03, 0x04, 0x05]; // Head, Body, Arms, Waist, Legs
        let ptr_offset = ZENITH_ARMOR_FORGING_PTR as usize;
        if self.buffer.len() < ptr_offset + 4 { return; }
        let data_offset = u32::from_le_bytes(self.buffer[ptr_offset..ptr_offset+4].try_into().unwrap()) as usize;
        
        // Save original offset
        self.original_armor_forging_zenith_offset = Some(data_offset as u32);
        self.armor_forging_zenith_modified = false;
        
        if data_offset == 0 || data_offset >= self.buffer.len() { return; }
        let entry_size = std::mem::size_of::<ShopEntry>();
        let mut cursor = data_offset;
        let mut entries = Vec::new();
        while cursor + entry_size <= self.buffer.len() {
            let equip_type = self.buffer[cursor];
            if !valid_types.contains(&equip_type) { break; }
            let entry = unsafe { std::ptr::read_unaligned(self.buffer.as_ptr().add(cursor) as *const ShopEntry) };
            entries.push(entry);
            cursor += entry_size;
        }
        self.armor_forging_zenith_entries = entries;
    }

    pub fn load_tower_weapon_forging_entries(&mut self) {
        let valid_types = [0x06, 0x07]; // Melee, Ranged
        let ptr_offset = TOWER_WEAPON_FORGING_PTR as usize;
        if self.buffer.len() < ptr_offset + 4 { return; }
        let data_offset = u32::from_le_bytes(self.buffer[ptr_offset..ptr_offset+4].try_into().unwrap()) as usize;
        
        // Save original offset
        self.original_tower_weapon_forging_offset = Some(data_offset as u32);
        self.tower_weapon_forging_modified = false;
        
        if data_offset == 0 || data_offset >= self.buffer.len() { return; }
        let entry_size = std::mem::size_of::<ShopEntry>();
        let mut cursor = data_offset;
        let mut entries = Vec::new();
        while cursor + entry_size <= self.buffer.len() {
            let equip_type = self.buffer[cursor];
            if !valid_types.contains(&equip_type) { break; }
            let entry = unsafe { std::ptr::read_unaligned(self.buffer.as_ptr().add(cursor) as *const ShopEntry) };
            entries.push(entry);
            cursor += entry_size;
        }
        self.tower_weapon_forging_entries = entries;
    }

    pub fn load_tower_armor_forging_entries(&mut self) {
        let valid_types = [0x00, 0x02, 0x03, 0x04, 0x05]; // Head, Body, Arms, Waist, Legs
        let ptr_offset = TOWER_ARMOR_FORGING_PTR as usize;
        if self.buffer.len() < ptr_offset + 4 { return; }
        let data_offset = u32::from_le_bytes(self.buffer[ptr_offset..ptr_offset+4].try_into().unwrap()) as usize;
        
        // Save original offset
        self.original_tower_armor_forging_offset = Some(data_offset as u32);
        self.tower_armor_forging_modified = false;
        
        if data_offset == 0 || data_offset >= self.buffer.len() { return; }
        
        // Try using sentinel first (like other shop entries)
        let mut cursor = std::io::Cursor::new(&self.buffer);
        if let Ok(entries) = crate::core::mhfdat::read_shop_entries_until_sentinel(&mut cursor, data_offset as u64) {
            // Filter to only include valid armor types
            self.tower_armor_forging_entries = entries.into_iter()
                .filter(|e| valid_types.contains(&e.equip_type))
                .collect();
            return;
        }
        
        // Fallback: manual reading if sentinel method fails
        let entry_size = std::mem::size_of::<ShopEntry>();
        let mut cursor = data_offset;
        let mut entries = Vec::new();
        while cursor + entry_size <= self.buffer.len() {
            let equip_type = self.buffer[cursor];
            if !valid_types.contains(&equip_type) { break; }
            let entry = unsafe { std::ptr::read_unaligned(self.buffer.as_ptr().add(cursor) as *const ShopEntry) };
            entries.push(entry);
            cursor += entry_size;
        }
        self.tower_armor_forging_entries = entries;
    }

    pub fn read_armor_data(&mut self, cursor: &mut Cursor<&[u8]>, offset: u64) -> Result<(), std::io::Error> {
        // Read the actual data offset from the pointer location
        cursor.seek(SeekFrom::Start(offset))?;
        let mut data_offset_bytes = [0u8; 4];
        cursor.read_exact(&mut data_offset_bytes)?;
        let data_offset = u32::from_le_bytes(data_offset_bytes) as u64;
        
        // Use the centralized read_equipments_until_sentinel function
        let armors = read_equipments_until_sentinel(cursor, data_offset)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        // Add to appropriate armor list based on type
        match self.armor_tab {
            ArmorTab::Head => self.head_armors = armors,
            ArmorTab::Body => self.body_armors = armors,
            ArmorTab::Arms => self.arms_armors = armors,
            ArmorTab::Waist => self.waist_armors = armors,
            ArmorTab::Legs => self.legs_armors = armors,
            ArmorTab::ArmorUpgrade => {}
        }
        
        Ok(())
    }

    pub fn load_sharpness_data(&mut self) {
        fn read_ptr(buffer: &[u8], ptr_offset: u32) -> Option<u32> {
            if buffer.len() >= ptr_offset as usize + 4 {
                Some(u32::from_le_bytes(buffer[ptr_offset as usize..ptr_offset as usize+4].try_into().unwrap()))
            } else {
                None
            }
        }

        // Load all 11 weapon type sharpness data (melee only, no bowguns/bow)
        // Index 0: Great Sword
        if let Some(off) = read_ptr(&self.buffer, SHARPNESS_GREAT_SWORD_PTR) {
            self.original_sharpness_offsets[0] = Some(off);
            self.sharpness_modified[0] = false;
            self.sharpness.great_sword = read_sharpness_data(&self.buffer, off as usize);
        }
        // Index 1: Hammer
        if let Some(off) = read_ptr(&self.buffer, SHARPNESS_HAMMER_PTR) {
            self.original_sharpness_offsets[1] = Some(off);
            self.sharpness_modified[1] = false;
            self.sharpness.hammer = read_sharpness_data(&self.buffer, off as usize);
        }
        // Index 2: Lance
        if let Some(off) = read_ptr(&self.buffer, SHARPNESS_LANCE_PTR) {
            self.original_sharpness_offsets[2] = Some(off);
            self.sharpness_modified[2] = false;
            self.sharpness.lance = read_sharpness_data(&self.buffer, off as usize);
        }
        // Index 3: Sword and Shield
        if let Some(off) = read_ptr(&self.buffer, SHARPNESS_SWORD_AND_SHIELD_PTR) {
            self.original_sharpness_offsets[3] = Some(off);
            self.sharpness_modified[3] = false;
            self.sharpness.sword_and_shield = read_sharpness_data(&self.buffer, off as usize);
        }
        // Index 4: Dual Blades
        if let Some(off) = read_ptr(&self.buffer, SHARPNESS_DUAL_BLADES_PTR) {
            self.original_sharpness_offsets[4] = Some(off);
            self.sharpness_modified[4] = false;
            self.sharpness.dual_blades = read_sharpness_data(&self.buffer, off as usize);
        }
        // Index 5: Long Sword
        if let Some(off) = read_ptr(&self.buffer, SHARPNESS_LONG_SWORD_PTR) {
            self.original_sharpness_offsets[5] = Some(off);
            self.sharpness_modified[5] = false;
            self.sharpness.long_sword = read_sharpness_data(&self.buffer, off as usize);
        }
        // Index 6: Hunting Horn
        if let Some(off) = read_ptr(&self.buffer, SHARPNESS_HUNTING_HORN_PTR) {
            self.original_sharpness_offsets[6] = Some(off);
            self.sharpness_modified[6] = false;
            self.sharpness.hunting_horn = read_sharpness_data(&self.buffer, off as usize);
        }
        // Index 7: Gunlance
        if let Some(off) = read_ptr(&self.buffer, SHARPNESS_GUNLANCE_PTR) {
            self.original_sharpness_offsets[7] = Some(off);
            self.sharpness_modified[7] = false;
            self.sharpness.gunlance = read_sharpness_data(&self.buffer, off as usize);
        }
        // Index 8: Tonfa
        if let Some(off) = read_ptr(&self.buffer, SHARPNESS_TONFA_PTR) {
            self.original_sharpness_offsets[8] = Some(off);
            self.sharpness_modified[8] = false;
            self.sharpness.tonfa = read_sharpness_data(&self.buffer, off as usize);
        }
        // Index 9: Switch Axe
        if let Some(off) = read_ptr(&self.buffer, SHARPNESS_SWITCH_AXE_PTR) {
            self.original_sharpness_offsets[9] = Some(off);
            self.sharpness_modified[9] = false;
            self.sharpness.switch_axe = read_sharpness_data(&self.buffer, off as usize);
        }
        // Index 10: Magnet Spike
        if let Some(off) = read_ptr(&self.buffer, SHARPNESS_MAGNET_SPIKE_PTR) {
            self.original_sharpness_offsets[10] = Some(off);
            self.sharpness_modified[10] = false;
            self.sharpness.magnet_spike = read_sharpness_data(&self.buffer, off as usize);
        }
    }

    pub fn load_bullet_sets(&mut self) {
        use crate::model::mhfdat_pointers::BULLET_SETS_PTR;
        use crate::core::mhfdat::read_bullet_sets;
        
        fn read_ptr(buffer: &[u8], ptr_offset: u32) -> Option<u32> {
            if buffer.len() >= ptr_offset as usize + 4 {
                Some(u32::from_le_bytes(buffer[ptr_offset as usize..ptr_offset as usize+4].try_into().unwrap()))
            } else {
                None
            }
        }

        // Load bullet sets (44 entries)
        if let Some(off) = read_ptr(&self.buffer, BULLET_SETS_PTR) {
            self.original_bullet_sets_offset = Some(off);
            self.bullet_sets_modified = false;
            self.bullet_sets = read_bullet_sets(&self.buffer, off as usize, 44);
        }
    }

    pub fn load_quests(&mut self) {
        use crate::model::mhfdat_pointers::{HR_QUEST_LIST_PTR, GR_QUEST_LIST_PTR};
        use crate::core::mhfdat::{read_hr_quests, read_gr_quests};
        
        fn read_ptr(buffer: &[u8], ptr_offset: u32) -> Option<u32> {
            if buffer.len() >= ptr_offset as usize + 4 {
                Some(u32::from_le_bytes(buffer[ptr_offset as usize..ptr_offset as usize+4].try_into().unwrap()))
            } else {
                None
            }
        }

        // Load HR Quests
        if let Some(off) = read_ptr(&self.buffer, HR_QUEST_LIST_PTR) {
            self.original_hr_quests_offset = Some(off);
            self.hr_quests_modified = false;
            self.hr_quests = read_hr_quests(&self.buffer, off);
        }

        // Load GR Quests
        if let Some(off) = read_ptr(&self.buffer, GR_QUEST_LIST_PTR) {
            self.original_gr_quests_offset = Some(off);
            self.gr_quests_modified = false;
            self.gr_quests = read_gr_quests(&self.buffer, off);
        }
    }

    pub fn load_g50_weapon_upgrades(&mut self) {
        use crate::model::mhfdat_pointers::{
            G50_MELEE_WEAPON_UPGRADE_PTR, G50_RANGED_WEAPON_UPGRADE_PTR,
            G50_MELEE_WEAPON_UPGRADE_COUNT_LIMITER_PTR, G50_RANGED_WEAPON_UPGRADE_COUNT_LIMITER_PTR
        };
        use crate::core::mhfdat::read_g50_weapon_until_sentinel;
        use std::io::Cursor;
        
        fn read_ptr(buffer: &[u8], ptr_offset: u32) -> Option<u32> {
            if buffer.len() >= ptr_offset as usize + 4 {
                Some(u32::from_le_bytes(buffer[ptr_offset as usize..ptr_offset as usize+4].try_into().unwrap()))
            } else {
                None
            }
        }
        
        fn read_u16_val(buffer: &[u8], offset: u32) -> u16 {
            if buffer.len() >= offset as usize + 2 {
                u16::from_le_bytes(buffer[offset as usize..offset as usize+2].try_into().unwrap())
            } else {
                0
            }
        }

        // Load G50 Melee Weapon Upgrades
        if let Some(off) = read_ptr(&self.buffer, G50_MELEE_WEAPON_UPGRADE_PTR) {
            self.original_g50_melee_weapon_upgrades_offset = Some(off);
            self.g50_melee_weapon_upgrades_modified = false;
            // Read counter first to know how many entries to read
            let count = read_u16_val(&self.buffer, G50_MELEE_WEAPON_UPGRADE_COUNT_LIMITER_PTR) as usize;
            self.g50_melee_count_limiter = count as u16;
            self.g50_melee_count_limiter_modified = false;
            
            let mut cursor = Cursor::new(self.buffer.clone());
            if let Ok(entries) = crate::core::mhfdat::read_g50_weapon_by_count(&mut cursor, off as u64, count) {
                self.g50_melee_weapon_upgrades = entries;
            }
        }

        // Load G50 Ranged Weapon Upgrades
        if let Some(off) = read_ptr(&self.buffer, G50_RANGED_WEAPON_UPGRADE_PTR) {
            self.original_g50_ranged_weapon_upgrades_offset = Some(off);
            self.g50_ranged_weapon_upgrades_modified = false;
            // Read counter first to know how many entries to read
            let count = read_u16_val(&self.buffer, G50_RANGED_WEAPON_UPGRADE_COUNT_LIMITER_PTR) as usize;
            self.g50_ranged_count_limiter = count as u16;
            self.g50_ranged_count_limiter_modified = false;
            
            let mut cursor = Cursor::new(self.buffer.clone());
            if let Ok(entries) = crate::core::mhfdat::read_g50_weapon_by_count(&mut cursor, off as u64, count) {
                self.g50_ranged_weapon_upgrades = entries;
            }
        }
    }

    pub fn load_g50_tower_params(&mut self) {
        use crate::model::mhfdat_pointers::*;
        use crate::core::mhfdat::read_tower_g50_weapon_type;
        
        let ptrs = [
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
        
        for (i, &ptr_offset) in ptrs.iter().enumerate() {
            if self.buffer.len() >= ptr_offset as usize + 4 {
                let ptr_table_offset = u32::from_le_bytes(
                    self.buffer[ptr_offset as usize..ptr_offset as usize + 4].try_into().unwrap()
                );
                self.original_g50_tower_params_offsets[i] = Some(ptr_table_offset);
                self.g50_tower_params_modified[i] = false;
                self.g50_tower_params[i] = read_tower_g50_weapon_type(&self.buffer, ptr_table_offset as usize);
            }
        }
    }
    
    pub fn load_armor_upgrade_mats(&mut self) {
        use crate::core::mhfdat::read_armor_upgrade_mats;
        
        if self.buffer.len() >= ARMOR_UPGRADE_MATS_PTR as usize + 4 {
            let offset = u32::from_le_bytes(
                self.buffer[ARMOR_UPGRADE_MATS_PTR as usize..ARMOR_UPGRADE_MATS_PTR as usize + 4].try_into().unwrap()
            );
            self.original_armor_upgrade_mats_offset = Some(offset);
            self.armor_upgrade_mats_modified = false;
            self.armor_upgrade_mats = read_armor_upgrade_mats(&self.buffer, ARMOR_UPGRADE_MATS_PTR);
            self.armor_upgrade_mats_table_count = self.armor_upgrade_mats.tables.len();
        }
    }
    
    pub fn load_carve_parts(&mut self) {
        use crate::model::mhfdat_pointers::{CARVE_PARTS_PTR, CARVE_PARTS_COUNT_PTR};
        use crate::core::mhfdat::read_carve_parts;
        
        // Read count from CARVE_PARTS_COUNT_PTR first
        let count = if self.buffer.len() >= CARVE_PARTS_COUNT_PTR as usize + 2 {
            u16::from_le_bytes(
                self.buffer[CARVE_PARTS_COUNT_PTR as usize..CARVE_PARTS_COUNT_PTR as usize + 2]
                    .try_into().unwrap()
            )
        } else {
            0
        };
        self.carve_parts_count = count;
        self.carve_parts_count_modified = false;
        
        // Read carve parts data using the count
        if self.buffer.len() >= CARVE_PARTS_PTR as usize + 4 {
            let offset = u32::from_le_bytes(
                self.buffer[CARVE_PARTS_PTR as usize..CARVE_PARTS_PTR as usize + 4]
                    .try_into().unwrap()
            );
            self.original_carve_parts_offset = Some(offset);
            self.carve_parts_modified = false;
            self.carve_parts = read_carve_parts(&self.buffer, CARVE_PARTS_PTR, count as usize);
        }
    }
    
    pub fn load_part_break_parts(&mut self) {
        use crate::model::mhfdat_pointers::{PART_BREAK_DROP_PTR, PART_BREAK_DROP_COUNT_PTR};
        use crate::core::mhfdat::read_part_break_parts;
        
        // Read count from PART_BREAK_DROP_COUNT_PTR first
        let count = if self.buffer.len() >= PART_BREAK_DROP_COUNT_PTR as usize + 2 {
            u16::from_le_bytes(
                self.buffer[PART_BREAK_DROP_COUNT_PTR as usize..PART_BREAK_DROP_COUNT_PTR as usize + 2]
                    .try_into().unwrap()
            )
        } else {
            0
        };
        self.part_break_parts_count = count;
        self.part_break_parts_count_modified = false;
        
        // Read part break parts data using the count
        if self.buffer.len() >= PART_BREAK_DROP_PTR as usize + 4 {
            let offset = u32::from_le_bytes(
                self.buffer[PART_BREAK_DROP_PTR as usize..PART_BREAK_DROP_PTR as usize + 4]
                    .try_into().unwrap()
            );
            self.original_part_break_parts_offset = Some(offset);
            self.part_break_parts_modified = false;
            self.part_break_parts = read_part_break_parts(&self.buffer, PART_BREAK_DROP_PTR, count as usize);
        }
    }
    
    pub fn load_monster_descriptions(&mut self) {
        use crate::model::mhfdat_pointers::{MOSNTERS_DESCRIPTION_PTR, MOSNTERS_DESCRIPTION_COUNT_PTR};
        
        // Read count from MOSNTERS_DESCRIPTION_COUNT_PTR first
        let count = if self.buffer.len() >= MOSNTERS_DESCRIPTION_COUNT_PTR as usize + 2 {
            u16::from_le_bytes(
                self.buffer[MOSNTERS_DESCRIPTION_COUNT_PTR as usize..MOSNTERS_DESCRIPTION_COUNT_PTR as usize + 2]
                    .try_into().unwrap()
            )
        } else {
            0
        };
        self.monster_descriptions_count = count;
        self.monster_descriptions_count_modified = false;
        
        // Save original offset
        if self.buffer.len() >= MOSNTERS_DESCRIPTION_PTR as usize + 4 {
            self.original_monster_descriptions_offset = Some(
                u32::from_le_bytes(
                    self.buffer[MOSNTERS_DESCRIPTION_PTR as usize..MOSNTERS_DESCRIPTION_PTR as usize + 4]
                        .try_into().unwrap()
                )
            );
            self.monster_descriptions_modified = false;
        }
        
        // Load monster descriptions using the count
        let descriptions = parse_monster_descriptions(&self.buffer, count as usize);
        self.monster_descriptions = descriptions;
    }

    pub fn load_sigil_data(&mut self) {
        use crate::model::mhfdat_pointers::{
            SIGIL_CRAFTING_RECIPES_PTR,
            SIGIL_SKILL_PROBABILITIES_PTR,
            SIGIL_SKILL_BLACKLISTS_PTR,
        };
        use crate::core::mhfdat::{read_sigil_recipes, read_sigil_skill_probabilities, read_sigil_blacklists};

        fn read_ptr(buffer: &[u8], ptr_offset: u32) -> Option<u32> {
            if buffer.len() >= ptr_offset as usize + 4 {
                Some(u32::from_le_bytes(
                    buffer[ptr_offset as usize..ptr_offset as usize + 4].try_into().unwrap(),
                ))
            } else {
                None
            }
        }

        // Recipes
        if let Some(off) = read_ptr(&self.buffer, SIGIL_CRAFTING_RECIPES_PTR) {
            self.original_sigil_recipes_offset = Some(off);
            self.sigil_recipes_modified = false;
            self.sigil_recipes = read_sigil_recipes(&self.buffer, off as usize);
        }

        let recipe_count = self.sigil_recipes.len();

        // Probabilities (same count as recipes)
        if let Some(off) = read_ptr(&self.buffer, SIGIL_SKILL_PROBABILITIES_PTR) {
            self.original_sigil_probabilities_offset = Some(off);
            self.sigil_probabilities = read_sigil_skill_probabilities(&self.buffer, off as usize, recipe_count);
        }

        // Blacklists (same count as recipes, each entry is a u32 pointer)
        if let Some(off) = read_ptr(&self.buffer, SIGIL_SKILL_BLACKLISTS_PTR) {
            self.original_sigil_blacklists_offset = Some(off);
            self.sigil_blacklists = read_sigil_blacklists(&self.buffer, off as usize, recipe_count);
        }
    }
}
