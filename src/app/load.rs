use super::*;
use std::io::{Read, Seek, SeekFrom, Cursor};
use crate::model::mhfdat_pointers::*;
use crate::core::mhfdat::{*, parse_items, parse_item_names, parse_item_descriptions};
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
        }
        
        Ok(())
    }
}
