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
        let items = parse_items(buffer);
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
        let names = parse_item_names(buffer, self.items.len());
        self.item_names = names;
        
        // Load item descriptions using the new parse function
        let descriptions = parse_item_descriptions(buffer, self.items.len());
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
        
        // Same terminator as every forging table: a row whose first byte is 0xFF
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

    /// Value of an `important_nums` count (0 when the block cannot be reached). The block is found
    /// through header slot 0x10, not through a fixed address.
    pub(crate) fn imp_count(&self, inner: usize) -> u16 {
        read_important_count(&self.buffer, inner).unwrap_or(0)
    }

    /// File offset of an `important_nums` count, for writing it back.
    pub(crate) fn imp_count_pos(&self, inner: usize) -> std::io::Result<u64> {
        important_count_offset(&self.buffer, inner).map(|o| o as u64).ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("important_nums (header 0x10) + 0x{inner:X} is outside the {}-byte file", self.buffer.len()),
            )
        })
    }

    /// File offset of the pointer for sharpness weapon type `i` (0..11), inside the array behind
    /// header slot 0xAC.
    pub(crate) fn sharpness_slot(&self, i: usize) -> Option<usize> {
        let inner = *SHARPNESS_INNER_BY_INDEX.get(i)?;
        let base = read_header_pointer(&self.buffer, SHARPNESS_ARRAY_SLOT)? as usize;
        let slot = base.checked_add(inner)?;
        self.buffer.get(slot..slot + 4)?;
        Some(slot)
    }

    pub(crate) fn sharpness_data_mut(&mut self, i: usize) -> &mut crate::model::mhfdat::SharpnessData {
        match i {
            0 => &mut self.sharpness.great_sword,
            1 => &mut self.sharpness.hammer,
            2 => &mut self.sharpness.lance,
            3 => &mut self.sharpness.sword_and_shield,
            4 => &mut self.sharpness.dual_blades,
            5 => &mut self.sharpness.long_sword,
            6 => &mut self.sharpness.hunting_horn,
            7 => &mut self.sharpness.gunlance,
            8 => &mut self.sharpness.tonfa,
            9 => &mut self.sharpness.switch_axe,
            _ => &mut self.sharpness.magnet_spike,
        }
    }

    pub fn load_sharpness_data(&mut self) {
        // 11 melee weapon types (no bowguns / bow), see SHARPNESS_INNER_BY_INDEX
        for i in 0..SHARPNESS_INNER_BY_INDEX.len() {
            let off = self.sharpness_slot(i).and_then(|slot| read_header_pointer(&self.buffer, slot as u32));
            if let Some(off) = off {
                self.original_sharpness_offsets[i] = Some(off);
                self.sharpness_modified[i] = false;
                let data = read_sharpness_data(&self.buffer, off as usize);
                *self.sharpness_data_mut(i) = data;
            }
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
        use crate::model::mhfdat_pointers::HR_QUEST_LIST_PTR;
        use crate::core::mhfdat::read_hr_quests;
        
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

        // The GR lists (G1 to G10) are read with the other tables that have their own slot: see load_inplace_tables.
    }

    pub fn load_g50_weapon_upgrades(&mut self) {
        use crate::model::mhfdat_pointers::{
            G50_MELEE_WEAPON_UPGRADE_PTR, G50_RANGED_WEAPON_UPGRADE_PTR,
            G50_MELEE_WEAPON_UPGRADE_COUNT_INNER, G50_RANGED_WEAPON_UPGRADE_COUNT_INNER
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
        
        // Load G50 Melee Weapon Upgrades
        if let Some(off) = read_ptr(&self.buffer, G50_MELEE_WEAPON_UPGRADE_PTR) {
            self.original_g50_melee_weapon_upgrades_offset = Some(off);
            self.g50_melee_weapon_upgrades_modified = false;
            // Read counter first to know how many entries to read
            let count = self.imp_count(G50_MELEE_WEAPON_UPGRADE_COUNT_INNER) as usize;
            self.g50_melee_count_limiter = count as u16;
            self.g50_melee_count_limiter_modified = false;
            
            let mut cursor = Cursor::new(&self.buffer[..]);
            if let Ok(entries) = crate::core::mhfdat::read_g50_weapon_by_count(&mut cursor, off as u64, count) {
                self.g50_melee_weapon_upgrades = entries;
            }
        }

        // Load G50 Ranged Weapon Upgrades
        if let Some(off) = read_ptr(&self.buffer, G50_RANGED_WEAPON_UPGRADE_PTR) {
            self.original_g50_ranged_weapon_upgrades_offset = Some(off);
            self.g50_ranged_weapon_upgrades_modified = false;
            // Read counter first to know how many entries to read
            let count = self.imp_count(G50_RANGED_WEAPON_UPGRADE_COUNT_INNER) as usize;
            self.g50_ranged_count_limiter = count as u16;
            self.g50_ranged_count_limiter_modified = false;
            
            let mut cursor = Cursor::new(&self.buffer[..]);
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
        use crate::model::mhfdat_pointers::{CARVE_PARTS_PTR, CARVE_PARTS_COUNT_INNER};
        use crate::core::mhfdat::read_carve_parts;

        let count = self.imp_count(CARVE_PARTS_COUNT_INNER);
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
        use crate::model::mhfdat_pointers::{PART_BREAK_DROP_PTR, PART_BREAK_DROP_COUNT_INNER};
        use crate::core::mhfdat::read_part_break_parts;

        let count = self.imp_count(PART_BREAK_DROP_COUNT_INNER);
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
        use crate::model::mhfdat_pointers::{MOSNTERS_DESCRIPTION_PTR, MOSNTERS_DESCRIPTION_COUNT_INNER};

        let count = self.imp_count(MOSNTERS_DESCRIPTION_COUNT_INNER);
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

    pub fn load_seasonal_events(&mut self) {
        use crate::model::mhfdat_pointers::{SEASONAL_EVENT_PTR, SEASONAL_EVENT_COUNT_INNER};
        use crate::core::mhfdat::read_seasonal_events;

        let count = self.imp_count(SEASONAL_EVENT_COUNT_INNER);
        self.seasonal_events_count = count;
        self.seasonal_events_count_modified = false;

        // Read seasonal events data using the count
        if self.buffer.len() >= SEASONAL_EVENT_PTR as usize + 4 {
            let offset = u32::from_le_bytes(
                self.buffer[SEASONAL_EVENT_PTR as usize..SEASONAL_EVENT_PTR as usize + 4]
                    .try_into().unwrap()
            );
            self.original_seasonal_events_offset = Some(offset);
            self.seasonal_events_modified = false;
            self.seasonal_events = read_seasonal_events(&self.buffer, offset as usize, count as usize);
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// Tables that are rewritten in place. Each one is read through its header slot (core/mhfdat.rs); a table that
// cannot be read is reported on its tab and left empty, the others stay available.
// ---------------------------------------------------------------------------------------------------------
impl MhfdatApp {
    pub fn load_inplace_tables(&mut self) {
        self.clear_inplace_tables();

        // The header slots used below are documented for the 0xBC8-byte header only. Another build (the Vita file has
        // 0xBC4 at 0xC) puts other things behind them, so its tables are not read rather than shown wrong.
        let word = self.buffer.get(0xC..0x10).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
        if word != Some(0xBC8) {
            let why = format!(
                "this file's header word at 0xC is {}, not 0xBC8: the header layout of this build is not documented, so this tab is not read",
                word.map_or("missing".to_string(), |w| format!("0x{w:X}"))
            );
            for tab in ["quests", "crowns", "skills", "caravan", "road", "craft", "weapon", "arena"] {
                self.table_problems.push((tab, why.clone()));
            }
            return;
        }

        match read_crowns(&self.buffer) {
            Ok((_, rows)) => self.crowns = rows,
            Err(e) => self.table_problems.push(("crowns", e)),
        }
        match read_skill_effects(&self.buffer) {
            Ok((_, rows)) => self.skill_effects = rows,
            Err(e) => self.table_problems.push(("skills", e)),
        }

        let (menus, problems) = read_gr_menus(&self.buffer);
        self.gr_menus = menus;
        self.table_problems.extend(problems.into_iter().map(|e| ("quests", e)));
        match read_beginner_quests(&self.buffer) {
            Ok((_, rows)) => self.beginner_quests = rows,
            Err(e) => self.table_problems.push(("quests", e)),
        }

        let (caravan, _, problems) = read_caravan(&self.buffer);
        self.caravan = caravan;
        self.table_problems.extend(problems.into_iter().map(|e| ("caravan", e)));

        let (road, _, problems) = read_road(&self.buffer);
        self.road = road;
        self.table_problems.extend(problems.into_iter().map(|e| ("road", e)));

        for (slot, name) in [
            (SPECIAL_WEAPON_FORGING_PTR, "special weapon forging (header 0x7AC)"),
            (ARMOR_PREMIUM_FORGING_PTR, "armor premium forging (header 0x7B0)"),
        ] {
            match read_forging_rows(&self.buffer, name, slot) {
                Ok((_, rows)) => match slot {
                    SPECIAL_WEAPON_FORGING_PTR => self.special_weapon_forging = rows,
                    _ => self.armor_premium_forging = rows,
                },
                Err(e) => self.table_problems.push(("craft", e)),
            }
        }
        match read_combine_shop(&self.buffer) {
            Ok((_, rows)) => self.combine_shop = rows,
            Err(e) => self.table_problems.push(("craft", e)),
        }
        match read_tower_jewels(&self.buffer) {
            Ok((_, rows)) => self.tower_jewels = rows,
            Err(e) => self.table_problems.push(("craft", e)),
        }
        match read_tower_seals(&self.buffer) {
            Ok((_, rows)) => self.sigil_tower_entries = rows,
            Err(e) => self.table_problems.push(("craft", e)),
        }
        match read_tower_costs(&self.buffer) {
            Ok(blocks) => self.tower_costs = blocks,
            Err(e) => self.table_problems.push(("craft", e)),
        }

        match read_horn_melodies(&self.buffer) {
            Ok((_, rows)) => self.horn_melodies = rows,
            Err(e) => self.table_problems.push(("weapon", e)),
        }

        let (arena, _, problems) = read_arena(&self.buffer);
        self.arena = arena;
        self.table_problems.extend(problems.into_iter().map(|e| ("arena", e)));
    }

    /// Empties the in-place tables and clears their modified flags (file closed, or reloaded after a save).
    pub fn clear_inplace_tables(&mut self) {
        self.table_problems.clear();
        self.gr_menus.clear();
        self.beginner_quests.clear();
        self.crowns.clear();
        self.skill_effects.clear();
        self.caravan = Default::default();
        self.road = Default::default();
        self.special_weapon_forging.clear();
        self.armor_premium_forging.clear();
        self.combine_shop.clear();
        self.tower_jewels.clear();
        self.sigil_tower_entries.clear();
        self.tower_costs.clear();
        self.horn_melodies.clear();
        self.arena = Default::default();

        self.gr_menus_modified = false;
        self.beginner_quests_modified = false;
        self.crowns_modified = false;
        self.skill_effects_modified = false;
        self.caravan_modified = false;
        self.road_modified = false;
        self.craft_more_modified = false;
        self.horn_melodies_modified = false;
        self.arena_modified = false;

        self.crowns_page = 0;
        self.crowns_selected = None;
        self.skill_effects_page = 0;
        self.caravan_selected = None;
        self.caravan_group = 0;
        self.caravan_page = 0;
        self.road_page = 0;
        self.road_selected = None;
        self.craft_more_page = 0;
        self.craft_more_selected = None;
        self.horn_melodies_page = 0;
        self.horn_melodies_selected = None;
        self.arena_selected = 0;
    }
}
