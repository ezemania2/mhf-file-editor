// Structures for melee and ranged weapons in mhfdat

use serde::{Serialize, Deserialize};
use crate::utils::equip_flags::{EquipType, WeaponType, BulletTypes, EquipableBy};

fn default_padding_68() -> [u8; 68] {
    [0u8; 68]
}

// Sharpness structures
#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SharpnessItem {
    pub red: u16,
    pub orange: u16,
    pub yellow: u16,
    pub green: u16,
    pub blue: u16,
    pub white: u16,
    pub purple: u16,
    pub sky_blue: u16,
}

impl SharpnessItem {
    pub fn total(&self) -> u16 {
        self.red + self.orange + self.yellow + self.green + 
        self.blue + self.white + self.purple + self.sky_blue
    }
}

// One sharpness data set contains 128 entries (one per weapon)
pub type SharpnessData = Vec<SharpnessItem>;

// Bullet Set structure (100 bytes total: 32 u8 fields + 68 bytes padding)
#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BulletSet {
    pub normal_lv1_capacity: u8,
    pub normal_lv2_capacity: u8,
    pub normal_lv3_capacity: u8,
    pub pierce_lv1_capacity: u8,
    pub pierce_lv2_capacity: u8,
    pub pierce_lv3_capacity: u8,
    pub spread_lv1_capacity: u8,
    pub spread_lv2_capacity: u8,
    pub spread_lv3_capacity: u8,
    pub crag_lv1_capacity: u8,
    pub crag_lv2_capacity: u8,
    pub crag_lv3_capacity: u8,
    pub cluster_lv1_capacity: u8,
    pub cluster_lv2_capacity: u8,
    pub cluster_lv3_capacity: u8,
    pub fire_capacity: u8,
    pub water_capacity: u8,
    pub thunder_capacity: u8,
    pub ice_capacity: u8,
    pub dragon_capacity: u8,
    pub recovery_lv1_capacity: u8,
    pub recovery_lv2_capacity: u8,
    pub poison_lv1_capacity: u8,
    pub poison_lv2_capacity: u8,
    pub paralysis_lv1_capacity: u8,
    pub paralysis_lv2_capacity: u8,
    pub sleep_lv1_capacity: u8,
    pub sleep_lv2_capacity: u8,
    pub tranquilizer_capacity: u8,
    pub paint_capacity: u8,
    pub demon_capacity: u8,
    pub armor_capacity: u8,
    #[serde(skip, default = "default_padding_68")]
    pub _padding: [u8; 68], // Padding to make total size 100 bytes
}

impl Default for BulletSet {
    fn default() -> Self {
        BulletSet {
            normal_lv1_capacity: 0,
            normal_lv2_capacity: 0,
            normal_lv3_capacity: 0,
            pierce_lv1_capacity: 0,
            pierce_lv2_capacity: 0,
            pierce_lv3_capacity: 0,
            spread_lv1_capacity: 0,
            spread_lv2_capacity: 0,
            spread_lv3_capacity: 0,
            crag_lv1_capacity: 0,
            crag_lv2_capacity: 0,
            crag_lv3_capacity: 0,
            cluster_lv1_capacity: 0,
            cluster_lv2_capacity: 0,
            cluster_lv3_capacity: 0,
            fire_capacity: 0,
            water_capacity: 0,
            thunder_capacity: 0,
            ice_capacity: 0,
            dragon_capacity: 0,
            recovery_lv1_capacity: 0,
            recovery_lv2_capacity: 0,
            poison_lv1_capacity: 0,
            poison_lv2_capacity: 0,
            paralysis_lv1_capacity: 0,
            paralysis_lv2_capacity: 0,
            sleep_lv1_capacity: 0,
            sleep_lv2_capacity: 0,
            tranquilizer_capacity: 0,
            paint_capacity: 0,
            demon_capacity: 0,
            armor_capacity: 0,
            _padding: [0; 68],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SharpnessCollection {
    pub great_sword: SharpnessData,      // 128 entries
    pub hammer: SharpnessData,           // 128 entries
    pub lance: SharpnessData,            // 128 entries
    pub sword_and_shield: SharpnessData, // 128 entries
    pub dual_blades: SharpnessData,      // 128 entries
    pub long_sword: SharpnessData,       // 128 entries
    pub hunting_horn: SharpnessData,     // 128 entries
    pub gunlance: SharpnessData,         // 128 entries
    pub tonfa: SharpnessData,            // 128 entries
    pub switch_axe: SharpnessData,       // 128 entries
    pub magnet_spike: SharpnessData,     // 128 entries
}

#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MhfdatMeleeWeapon {
    pub model_id: u16,
    pub rarity: u8,
    pub class_id: u8,
    pub zenny_cost: u32,
    pub sharpness_id: u8,
    pub sharpness_max: u8,
    pub raw_damage: u16,
    pub defense: u16,
    pub affinity: i8,
    pub element_id: u8,
    pub ele_damage: u8,
    pub ailment_id: u8,
    pub ail_damage: u8,
    pub slots: u8,
    pub weapon_attribute: u8,
    pub unk15: u8,
    pub upgrade_path: u16,
    pub other_model: u16,
    pub equip_type: u8,
    pub unk1b: u8,
    pub length: u32,
    pub weapon_type: u32,
    pub visual_effects: u16,
    pub tower_g50_param_id: u16,
    pub g_rank: u8,
    pub unk29: u8,
    pub unk2a: u8,
    pub zero_f: u8,
    pub unk2c: u32,
    pub zenith_skill: u16,
    pub _padding: [u8; 2],
}

impl Default for MhfdatMeleeWeapon {
    fn default() -> Self {
        Self {
            model_id: 0,
            rarity: 0,
            class_id: 0,
            zenny_cost: 0,
            sharpness_id: 0,
            sharpness_max: 0,
            raw_damage: 0,
            defense: 0,
            affinity: 0,
            element_id: 0,
            ele_damage: 0,
            ailment_id: 0,
            ail_damage: 0,
            slots: 0,
            weapon_attribute: 0,
            unk15: 0,
            upgrade_path: 0,
            other_model: 0,
            equip_type: 0,
            unk1b: 0,
            length: 0,
            weapon_type: 0,
            visual_effects: 0,
            tower_g50_param_id: 0,
            g_rank: 0,
            unk29: 0,
            unk2a: 0,
            zero_f: 0,
            unk2c: 0,
            zenith_skill: 0,
            _padding: [0; 2],
        }
    }
}

impl MhfdatMeleeWeapon {
    /// Get the equipment type as a structured bitfield
    pub fn get_equip_type(&self) -> EquipType {
        EquipType::from_u8(self.equip_type)
    }
    
    /// Set the equipment type from a structured bitfield
    pub fn set_equip_type(&mut self, equip_type: EquipType) {
        self.equip_type = equip_type.to_u8();
    }
    
    /// Get the weapon type as a structured bitfield
    pub fn get_weapon_type(&self) -> WeaponType {
        WeaponType::from_u32(self.weapon_type)
    }
    
    /// Set the weapon type from a structured bitfield
    pub fn set_weapon_type(&mut self, weapon_type: WeaponType) {
        self.weapon_type = weapon_type.to_u32();
    }
    
    /// Get a human-readable string of equipment type flags
    pub fn equip_type_string(&self) -> String {
        self.get_equip_type().to_string()
    }
    
    /// Get a human-readable string of weapon type flags
    pub fn weapon_type_string(&self) -> String {
        self.get_weapon_type().to_string()
    }
}

#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MhfdatRangedWeapon {
    pub model_id: u16,
    pub rarity: u8,
    pub max_slots_maybe: u8,
    pub class_id: u8,
    pub unk05: u8,
    pub equip_type: u8,
    pub unk07: u8,
    pub unk08: u8,
    pub unk09: u8,
    pub unk11: u8,
    pub unk12: u8,
    pub weapon_type: u32,
    pub unk10: u32,
    pub zenny_cost: u32,
    pub raw_damage: u16,
    pub defense: u16,
    pub recoil: u8,
    pub slots: u8,
    pub affinity: i8,
    pub sort_order_maybe: u8,
    pub weapon_attribute: u8,
    pub element_id: u8,
    pub ele_damage: u8,
    pub reload: u8,
    pub unk24: u16,
    pub unk26: u16,
    pub bullet: u32,
    pub tower_g50_param_id: u16,
    pub unk2e: u16,
    pub g_rank: u8,
    pub unk32: u8,
    pub unk34: u8,
    pub zero_f: u8,
    pub unk38: u16,
    pub zenith_skill: u16,
    pub unk42: u32,
}

impl MhfdatRangedWeapon {
    /// Get the equipment type as a structured bitfield
    pub fn get_equip_type(&self) -> EquipType {
        EquipType::from_u8(self.equip_type)
    }
    
    /// Set the equipment type from a structured bitfield
    pub fn set_equip_type(&mut self, equip_type: EquipType) {
        self.equip_type = equip_type.to_u8();
    }
    
    /// Get the weapon type as a structured bitfield
    pub fn get_weapon_type(&self) -> WeaponType {
        WeaponType::from_u32(self.weapon_type)
    }
    
    /// Set the weapon type from a structured bitfield
    pub fn set_weapon_type(&mut self, weapon_type: WeaponType) {
        self.weapon_type = weapon_type.to_u32();
    }
    
    /// Get the bullet types as a structured bitfield
    pub fn get_bullet_types(&self) -> BulletTypes {
        BulletTypes::from_u32(self.bullet)
    }
    
    /// Set the bullet types from a structured bitfield
    pub fn set_bullet_types(&mut self, bullet_types: BulletTypes) {
        self.bullet = bullet_types.to_u32();
    }
    
    /// Get a human-readable string of equipment type flags
    pub fn equip_type_string(&self) -> String {
        self.get_equip_type().to_string()
    }
    
    /// Get a human-readable string of weapon type flags
    pub fn weapon_type_string(&self) -> String {
        self.get_weapon_type().to_string()
    }
    
    /// Get a human-readable string of bullet types
    pub fn bullet_types_string(&self) -> String {
        self.get_bullet_types().to_string()
    }
}

/// Export structure for ranged weapons with decomposed bitfields for JSON export
#[derive(Debug, Serialize, Deserialize)]
pub struct RangedWeaponExport {
    // ID (starting from 0)
    pub id: usize,
    
    // Name and descriptions
    pub name: String,
    pub description_1: String,
    pub description_2: String,
    pub description_3: String,
    
    // Basic weapon data
    pub model_id: u16,
    pub rarity: u8,
    pub class_id: u8,
    pub zenny_cost: u32,
    pub raw_damage: u16,
    pub defense: u16,
    pub affinity: i8,
    pub recoil: u8,
    pub reload: u8,
    pub slots: u8,
    pub weapon_attribute: u8,
    pub element_id: u8,
    pub ele_damage: u8,
    pub tower_g50_param_id: u16,
    pub g_rank: u8,
    pub zenith_skill: u16,
    
    // Decomposed bitfields
    pub equipment_type: EquipType,
    pub weapon_type: WeaponType,
    pub bullet_types: BulletTypes,
    
    // Human-readable strings
    pub equipment_type_string: String,
    pub weapon_type_string: String,
    pub bullet_types_string: String,
    
    // Upgrade path (index-aligned: weapon i ↔ upgrade i)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade: Option<RWUpgradePath>,
}

impl RangedWeaponExport {
    pub fn from_weapon_with_data(
        weapon: &MhfdatRangedWeapon,
        name: &str,
        descriptions: &[String; 4],
        id: usize
    ) -> Self {
        Self::from_weapon_with_data_and_upgrade(weapon, name, descriptions, id, None)
    }
    
    pub fn from_weapon_with_data_and_upgrade(
        weapon: &MhfdatRangedWeapon,
        name: &str,
        descriptions: &[String; 4],
        id: usize,
        upgrade: Option<RWUpgradePath>
    ) -> Self {
        let equipment_type = weapon.get_equip_type();
        let weapon_type = weapon.get_weapon_type();
        let bullet_types = weapon.get_bullet_types();
        
        Self {
            // ID (starting from 0)
            id,
            
            // Name and descriptions
            name: name.to_string(),
            description_1: descriptions[0].clone(),
            description_2: descriptions[1].clone(),
            description_3: descriptions[2].clone(),
            
            // Basic weapon data
            model_id: weapon.model_id,
            rarity: weapon.rarity,
            class_id: weapon.class_id,
            zenny_cost: weapon.zenny_cost,
            raw_damage: weapon.raw_damage,
            defense: weapon.defense,
            affinity: weapon.affinity,
            recoil: weapon.recoil,
            reload: weapon.reload,
            slots: weapon.slots,
            weapon_attribute: weapon.weapon_attribute,
            element_id: weapon.element_id,
            ele_damage: weapon.ele_damage,
            tower_g50_param_id: weapon.tower_g50_param_id,
            g_rank: weapon.g_rank,
            zenith_skill: weapon.zenith_skill,
            
            // Decomposed bitfields
            equipment_type,
            weapon_type,
            bullet_types,
            
            // Human-readable strings
            equipment_type_string: equipment_type.to_string(),
            weapon_type_string: weapon_type.to_string(),
            bullet_types_string: bullet_types.to_string(),
            
            // Upgrade path
            upgrade,
        }
    }

    /// Convert RangedWeaponExport back to MhfdatRangedWeapon
    pub fn to_weapon(&self) -> MhfdatRangedWeapon {
        MhfdatRangedWeapon {
            model_id: self.model_id,
            rarity: self.rarity,
            max_slots_maybe: 0,
            class_id: self.class_id,
            unk05: 0,
            equip_type: self.equipment_type.to_u8(),
            unk07: 0,
            unk08: 0,
            unk09: 0,
            unk11: 0,
            unk12: 0,
            weapon_type: self.weapon_type.to_u32(),
            unk10: 0,
            zenny_cost: self.zenny_cost,
            raw_damage: self.raw_damage,
            defense: self.defense,
            recoil: self.recoil,
            slots: self.slots,
            affinity: self.affinity,
            sort_order_maybe: 0,
            weapon_attribute: self.weapon_attribute,
            element_id: self.element_id,
            ele_damage: self.ele_damage,
            reload: self.reload,
            unk24: 0,
            unk26: 0,
            bullet: self.bullet_types.to_u32(),
            tower_g50_param_id: self.tower_g50_param_id,
            unk2e: 0,
            g_rank: self.g_rank,
            unk32: 0,
            unk34: 0,
            zero_f: 0,
            unk38: 0,
            zenith_skill: self.zenith_skill,
            unk42: 0,
        }
    }
}

/// Export structure for melee weapons with decomposed bitfields for JSON export
#[derive(Debug, Serialize, Deserialize)]
pub struct MeleeWeaponExport {
    // ID (starting from 0)
    pub id: usize,
    
    // Name and descriptions
    pub name: String,
    pub description_1: String,
    pub description_2: String,
    pub description_3: String,
    
    // Basic weapon data
    pub model_id: u16,
    pub rarity: u8,
    pub class_id: u8,
    pub zenny_cost: u32,
    pub sharpness_id: u8,
    pub sharpness_max: u8,
    pub raw_damage: u16,
    pub defense: u16,
    pub affinity: i8,
    pub element_id: u8,
    pub ele_damage: u8,
    pub ailment_id: u8,
    pub ail_damage: u8,
    pub slots: u8,
    pub weapon_attribute: u8,
    pub upgrade_path: u16,
    pub other_model: u16,
    pub length: u32,
    pub visual_effects: u16,
    pub tower_g50_param_id: u16,
    pub g_rank: u8,
    pub zenith_skill: u16,
    
    // Decomposed bitfields
    pub equipment_type: EquipType,
    pub weapon_type: WeaponType,
    
    // Human-readable strings
    pub equipment_type_string: String,
    pub weapon_type_string: String,
    
    // Upgrade path (linked by upgrade_path index)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade: Option<MWUpgradePath>,
}

impl MeleeWeaponExport {
    pub fn from_weapon_with_data(
        weapon: &MhfdatMeleeWeapon,
        name: &str,
        descriptions: &[String; 4],
        id: usize
    ) -> Self {
        Self::from_weapon_with_data_and_upgrade(weapon, name, descriptions, id, None)
    }
    
    pub fn from_weapon_with_data_and_upgrade(
        weapon: &MhfdatMeleeWeapon,
        name: &str,
        descriptions: &[String; 4],
        id: usize,
        upgrade: Option<MWUpgradePath>
    ) -> Self {
        let equipment_type = weapon.get_equip_type();
        let weapon_type = weapon.get_weapon_type();
        
        Self {
            // ID (starting from 0)
            id,
            
            // Name and descriptions
            name: name.to_string(),
            description_1: descriptions[0].clone(),
            description_2: descriptions[1].clone(),
            description_3: descriptions[2].clone(),
            
            // Basic weapon data
            model_id: weapon.model_id,
            rarity: weapon.rarity,
            class_id: weapon.class_id,
            zenny_cost: weapon.zenny_cost,
            sharpness_id: weapon.sharpness_id,
            sharpness_max: weapon.sharpness_max,
            raw_damage: weapon.raw_damage,
            defense: weapon.defense,
            affinity: weapon.affinity,
            element_id: weapon.element_id,
            ele_damage: weapon.ele_damage,
            ailment_id: weapon.ailment_id,
            ail_damage: weapon.ail_damage,
            slots: weapon.slots,
            weapon_attribute: weapon.weapon_attribute,
            upgrade_path: weapon.upgrade_path,
            other_model: weapon.other_model,
            length: weapon.length,
            visual_effects: weapon.visual_effects,
            tower_g50_param_id: weapon.tower_g50_param_id,
            g_rank: weapon.g_rank,
            zenith_skill: weapon.zenith_skill,
            
            // Decomposed bitfields
            equipment_type,
            weapon_type,
            
            // Human-readable strings
            equipment_type_string: equipment_type.to_string(),
            weapon_type_string: weapon_type.to_string(),
            
            // Upgrade path
            upgrade,
        }
    }

    /// Convert MeleeWeaponExport back to MhfdatMeleeWeapon
    pub fn to_weapon(&self) -> MhfdatMeleeWeapon {
        MhfdatMeleeWeapon {
            model_id: self.model_id,
            rarity: self.rarity,
            class_id: self.class_id,
            zenny_cost: self.zenny_cost,
            sharpness_id: self.sharpness_id,
            sharpness_max: self.sharpness_max,
            raw_damage: self.raw_damage,
            defense: self.defense,
            affinity: self.affinity,
            element_id: self.element_id,
            ele_damage: self.ele_damage,
            ailment_id: self.ailment_id,
            ail_damage: self.ail_damage,
            slots: self.slots,
            weapon_attribute: self.weapon_attribute,
            unk15: 0,
            upgrade_path: self.upgrade_path,
            other_model: self.other_model,
            equip_type: self.equipment_type.to_u8(),
            unk1b: 0,
            length: self.length,
            weapon_type: self.weapon_type.to_u32(),
            visual_effects: self.visual_effects,
            tower_g50_param_id: self.tower_g50_param_id,
            g_rank: self.g_rank,
            unk29: 0,
            unk2a: 0,
            zero_f: 0,
            unk2c: 0,
            zenith_skill: self.zenith_skill,
            _padding: [0; 2],
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MhfdatEquipment {
    pub model_id_male: u16,
    pub model_id_female: u16,
    pub equipable_by: u8,
    pub rarity: u8,
    pub max_level: u8,
    pub unk07: u8,
    pub unk08: u16,
    pub unk0A: u16,
    pub zenny_cost: u32,
    pub unk10: u16,
    pub base_defense: u16,
    pub fire_res: i8,
    pub water_res: i8,
    pub thunder_res: i8,
    pub dragon_res: i8,
    pub ice_res: i8,
    pub coef_upgrade: u8,
    pub unk1A: u8,
    pub base_slots: u8,
    pub max_slots: u8,
    pub post_festi: u8,
    pub show_next_level: u16,
    pub equip_id: u16,
    pub unk22: u16,
    pub unk24: u32,
    pub unk28: u16,
    pub skill_id1: u8,
    pub skill_pts1: i8,
    pub skill_id2: u8,
    pub skill_pts2: i8,
    pub skill_id3: u8,
    pub skill_pts3: i8,
    pub skill_id4: u8,
    pub skill_pts4: i8,
    pub skill_id5: u8,
    pub skill_pts5: i8,
    pub armor_type: u32,
    pub weap_hiden: u16,
    pub deco_item_id: u16,
    pub towerslots: u16,
    pub g_rank: u8,
    pub zero_f: u8,
    pub app_price: u32,
    pub unk44: u16,
    pub zenith_skill: u16,
}

impl MhfdatEquipment {
    /// Get the equipable by flags as a structured bitfield
    pub fn get_equipable_by(&self) -> EquipableBy {
        EquipableBy::from_u8(self.equipable_by)
    }
    
    /// Set the equipable by flags from a structured bitfield
    pub fn set_equipable_by(&mut self, equipable_by: EquipableBy) {
        self.equipable_by = equipable_by.to_u8();
    }
    
    /// Get a human-readable string of equipable by flags
    pub fn equipable_by_string(&self) -> String {
        self.get_equipable_by().to_string()
    }
}

/// Export structure for armor pieces with decomposed bitfields for JSON export
#[derive(Debug, Serialize, Deserialize)]
pub struct ArmorExport {
    // ID (starting from 0)
    pub id: usize,
    
    // Name only
    pub name: String,
    
    // Descriptions (4 lines)
    pub description_1: String,
    pub description_2: String,
    pub description_3: String,
    pub description_4: String,
    
    // Basic armor data
    pub model_id_male: u16,
    pub model_id_female: u16,
    pub rarity: u8,
    pub max_level: u8,
    pub unk07: u8,
    pub unk08: u16,
    pub unk0A: u16,
    pub zenny_cost: u32,
    pub unk10: u16,
    pub base_defense: u16,
    pub fire_res: i8,
    pub water_res: i8,
    pub thunder_res: i8,
    pub dragon_res: i8,
    pub ice_res: i8,
    pub coef_upgrade: u8,
    pub unk1A: u8,
    pub base_slots: u8,
    pub max_slots: u8,
    pub post_festi: u8,
    pub show_next_level: u16,
    pub equip_id: u16,
    pub unk22: u16,
    pub unk24: u32,
    pub unk28: u16,
    
    // Skills (MUST be before armor_type to match MhfdatEquipment structure)
    pub skill_id1: u8,
    pub skill_pts1: i8,
    pub skill_id2: u8,
    pub skill_pts2: i8,
    pub skill_id3: u8,
    pub skill_pts3: i8,
    pub skill_id4: u8,
    pub skill_pts4: i8,
    pub skill_id5: u8,
    pub skill_pts5: i8,
    
    pub armor_type: u32,
    pub weap_hiden: u16,
    pub deco_item_id: u16,
    pub towerslots: u16,
    pub g_rank: u8,
    pub zero_f: u8,
    pub app_price: u32,
    pub unk44: u16,
    pub zenith_skill: u16,
    
    // Decomposed bitfields
    pub equipable_by: EquipableBy,
    
    // Human-readable strings
    pub equipable_by_string: String,
}

impl ArmorExport {
    pub fn from_armor_with_data(
        armor: &MhfdatEquipment,
        name: &str,
        descriptions: &[String; 4],
        id: usize
    ) -> Self {
        let equipable_by = armor.get_equipable_by();
        
        Self {
            // ID (starting from 0)
            id,
            
            // Name only
            name: name.to_string(),
            
            // Descriptions (4 lines)
            description_1: descriptions[0].clone(),
            description_2: descriptions[1].clone(),
            description_3: descriptions[2].clone(),
            description_4: descriptions[3].clone(),
            
            // Basic armor data
            model_id_male: armor.model_id_male,
            model_id_female: armor.model_id_female,
            rarity: armor.rarity,
            max_level: armor.max_level,
            unk07: armor.unk07,
            unk08: armor.unk08,
            unk0A: armor.unk0A,
            zenny_cost: armor.zenny_cost,
            unk10: armor.unk10,
            base_defense: armor.base_defense,
            fire_res: armor.fire_res,
            water_res: armor.water_res,
            thunder_res: armor.thunder_res,
            dragon_res: armor.dragon_res,
            ice_res: armor.ice_res,
            coef_upgrade: armor.coef_upgrade,
            unk1A: armor.unk1A,
            base_slots: armor.base_slots,
            max_slots: armor.max_slots,
            post_festi: armor.post_festi,
            show_next_level: armor.show_next_level,
            equip_id: armor.equip_id,
            unk22: armor.unk22,
            unk24: armor.unk24,
            unk28: armor.unk28,
            
            // Skills (MUST be before armor_type to match MhfdatEquipment structure)
            skill_id1: armor.skill_id1,
            skill_pts1: armor.skill_pts1,
            skill_id2: armor.skill_id2,
            skill_pts2: armor.skill_pts2,
            skill_id3: armor.skill_id3,
            skill_pts3: armor.skill_pts3,
            skill_id4: armor.skill_id4,
            skill_pts4: armor.skill_pts4,
            skill_id5: armor.skill_id5,
            skill_pts5: armor.skill_pts5,
            
            armor_type: armor.armor_type,
            weap_hiden: armor.weap_hiden,
            towerslots: armor.towerslots,
            deco_item_id: armor.deco_item_id,
            g_rank: armor.g_rank,
            zero_f: armor.zero_f,
            app_price: armor.app_price,
            unk44: armor.unk44,
            zenith_skill: armor.zenith_skill,
            
            // Decomposed bitfields
            equipable_by,
            
            // Human-readable strings
            equipable_by_string: equipable_by.to_string(),
        }
    }

    /// Convert ArmorExport back to MhfdatEquipment
    pub fn to_armor(&self) -> MhfdatEquipment {
        MhfdatEquipment {
            model_id_male: self.model_id_male,
            model_id_female: self.model_id_female,
            equipable_by: self.equipable_by.to_u8(),
            rarity: self.rarity,
            max_level: self.max_level,
            unk07: self.unk07,
            unk08: self.unk08,
            unk0A: self.unk0A,
            zenny_cost: self.zenny_cost,
            unk10: self.unk10,
            base_defense: self.base_defense,
            fire_res: self.fire_res,
            water_res: self.water_res,
            thunder_res: self.thunder_res,
            dragon_res: self.dragon_res,
            ice_res: self.ice_res,
            coef_upgrade: self.coef_upgrade,
            unk1A: self.unk1A,
            base_slots: self.base_slots,
            max_slots: self.max_slots,
            post_festi: self.post_festi,
            show_next_level: self.show_next_level,
            equip_id: self.equip_id,
            unk22: self.unk22,
            unk24: self.unk24,
            unk28: self.unk28,
            skill_id1: self.skill_id1,
            skill_pts1: self.skill_pts1,
            skill_id2: self.skill_id2,
            skill_pts2: self.skill_pts2,
            skill_id3: self.skill_id3,
            skill_pts3: self.skill_pts3,
            skill_id4: self.skill_id4,
            skill_pts4: self.skill_pts4,
            skill_id5: self.skill_id5,
            skill_pts5: self.skill_pts5,
            armor_type: self.armor_type,
            weap_hiden: self.weap_hiden,
            deco_item_id: self.deco_item_id,
            towerslots: self.towerslots,
            g_rank: self.g_rank,
            zero_f: self.zero_f,
            app_price: self.app_price,
            unk44: self.unk44,
            zenith_skill: self.zenith_skill,
        }
    }
}

/// Structure représentant un objet/item dans mhfdat
#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MhfdatItem {
    pub unk00: u8,        // interaction_type (1 = carryable, 2 = gun ammo, 3 = coating)
    pub unk01: u8,        // is_usable. bitflag & 0x10 == eatable
    pub rarity: u8,       // rarity (add 1 to display)
    pub max_stack: u8,    // max stack
    pub unk04: u8,        // data_bitset 0x20 sale, 6 mytra? 0x4 and 0x2 diva related, 0x40 = calculate tower points
    pub icon: u8,         // ItemIcon
    pub icon_color: u8,   // Icon color
    pub unk07: u8,
    pub bottle: u16,      // pax says the internal name for this is bottle?
    pub unk0A: u16,       // related to hc carbes/eggs/infos books, probably depreciated
    pub buy_price: u32,   // buy price
    pub sell_price: u32,  // sell price
    pub item_type: u16,   // Item Type (04 = Consumable, 05 = Placeable)
    pub deco_id: u16,     // deco_id
    pub unk18: u16,
    pub unk1A: u8,
    pub unk1B: u8,
    pub equip_type: u16,  // equip_type is giving the item type, for sigils for example
    pub is_gz: u8,        // is_gz
    pub unk1F: u8,
    pub unk20: u16,
    pub unk22: u16,
}

/// Export structure for items with names and descriptions for JSON export
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemExport {
    // ID (starting from 0)
    pub id: usize,
    
    // Name and descriptions
    pub name: String,
    pub description: String,
    
    // Basic item data
    pub unk00: u8,        // interaction_type
    pub unk01: u8,        // is_usable
    pub rarity: u8,       // rarity (add 1 to display)
    pub max_stack: u8,    // max stack
    pub unk04: u8,        // data_bitset
    pub icon: u8,         // ItemIcon
    pub icon_color: u8,   // Icon color
    pub unk07: u8,
    pub bottle: u16,      // bottle
    pub unk0A: u16,       // related to hc carbes/eggs/infos books
    pub buy_price: u32,   // buy price
    pub sell_price: u32,  // sell price
    pub item_type: u16,   // Item Type
    pub deco_id: u16,     // deco_id
    pub unk18: u16,
    pub unk1A: u8,
    pub unk1B: u8,
    pub equip_type: u16,  // equip_type
    pub is_gz: u8,        // is_gz
    pub unk1F: u8,
    pub unk20: u16,
    pub unk22: u16,
}

impl ItemExport {
    pub fn from_item_with_data(
        item: &MhfdatItem,
        name: &str,
        description: &str,
        id: usize
    ) -> Self {
        Self {
            // ID (starting from 0)
            id,
            
            // Name and description
            name: name.to_string(),
            description: description.to_string(),
            
            // Basic item data
            unk00: item.unk00,
            unk01: item.unk01,
            rarity: item.rarity,
            max_stack: item.max_stack,
            unk04: item.unk04,
            icon: item.icon,
            icon_color: item.icon_color,
            unk07: item.unk07,
            bottle: item.bottle,
            unk0A: item.unk0A,
            buy_price: item.buy_price,
            sell_price: item.sell_price,
            item_type: item.item_type,
            deco_id: item.deco_id,
            unk18: item.unk18,
            unk1A: item.unk1A,
            unk1B: item.unk1B,
            equip_type: item.equip_type,
            is_gz: item.is_gz,
            unk1F: item.unk1F,
            unk20: item.unk20,
            unk22: item.unk22,
        }
    }

    /// Convert ItemExport back to MhfdatItem
    pub fn to_item(&self) -> MhfdatItem {
        MhfdatItem {
            unk00: self.unk00,
            unk01: self.unk01,
            rarity: self.rarity,
            max_stack: self.max_stack,
            unk04: self.unk04,
            icon: self.icon,
            icon_color: self.icon_color,
            unk07: self.unk07,
            bottle: self.bottle,
            unk0A: self.unk0A,
            buy_price: self.buy_price,
            sell_price: self.sell_price,
            item_type: self.item_type,
            deco_id: self.deco_id,
            unk18: self.unk18,
            unk1A: self.unk1A,
            unk1B: self.unk1B,
            equip_type: self.equip_type,
            is_gz: self.is_gz,
            unk1F: self.unk1F,
            unk20: self.unk20,
            unk22: self.unk22,
        }
    }
}

/// Structure représentant une entrée de shop/crafting (basée sur le pattern hexpat CraftMatReq)
#[repr(C, packed)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShopEntry {
    pub equip_type: u8,
    pub purchaseable: u8,
    pub equip_id: u16,           // ID de l'équipement
    pub material_id1: u16,       // Item ID 1
    pub material_amnt1: u16,     // Quantité 1
    pub padding1: [u8; 4],       // Padding pour alignement
    pub material_id2: u16,       // Item ID 2
    pub material_amnt2: u16,     // Quantité 2
    pub padding2: [u8; 4],       // Padding pour alignement
    pub material_id3: u16,       // Item ID 3
    pub material_amnt3: u16,     // Quantité 3
    pub padding3: [u8; 4],       // Padding pour alignement
    pub material_id4: u16,       // Item ID 4
    pub material_amnt4: u16,     // Quantité 4
    pub padding4: [u8; 4],       // Padding pour alignement
    pub unk24: u16,              // Flag inconnu
    pub padding5: [u8; 2],       // Padding pour alignement
    pub hr_req: u16,             // HR requis
    pub unk2a: u16,              // Flag inconnu
    pub preview_able: bool,      // Peut être prévisualisé
    pub padding6: [u8; 3],
    pub footer: u8,              // 0x0F
    pub padding7: [u8; 3],       // Padding pour alignement
    pub unk34: u8,               // Type spécial (exotic, festi, diva, etc.)
    pub padding8: [u8; 3],       // Padding pour alignement
}

#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct DecoShop {
    pub deco_item_id: u16,
    pub receipt_category: u16,
    pub item_id1: u16,
    pub item_qty1: u8,
    pub item_unlock_flag1: u8,
    pub item_id2: u16,
    pub item_qty2: u8,
    pub item_unlock_flag2: u8,
    pub item_id3: u16,
    pub item_qty3: u8,
    pub item_unlock_flag3: u8,
    pub item_id4: u16,
    pub item_qty4: u8,
    pub item_unlock_flag4: u8,
}

/// Structure représentant une entrée de la table AutomaticSkills
/// Cette table associe des équipements spécifiques à des skills automatiques
#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AutomaticSkill {
    pub is_armor: bool,              // True if the skill is for an armor, false if it is for a weapon
    pub eq_type: u8,            // Equipment type: 0=Legs, 2=Head, 3=Chest, 4=Arms, 5=Waist, 6=Melee, 7=Ranged
    pub equip_id: u16,          // ID of the equipment (weapon or armor)
    pub skill_id: u16,          // Automatic skill ID
    pub padding: [u8; 2],       // Padding bytes
}

#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ArmorUpgradeRow {
    pub item_id: u16,
    pub lv1_upgrade: u16,
    pub lv2_upgrade: u16,
    pub lv3_upgrade: u16,
    pub lv4_upgrade: u16,
    pub lv5_upgrade: u16,
    pub lv6_upgrade: u16,
    pub lv7_upgrade: u16,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArmorUpgradeTable {
    pub rows: Vec<ArmorUpgradeRow>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ArmorUpgradeMats {
    pub tables: Vec<ArmorUpgradeTable>,
}

// Carve drop structure
#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CarveDrop {
    pub percentage: u16,
    pub item_id: u16,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CarveDropTable {
    pub carves: Vec<CarveDrop>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct CarveParts {
    pub tables: Vec<CarveDropTable>,
}

// Part break drop structure
#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PartBreakDrop {
    pub percentage: u16,
    pub item_id: u16,
    pub number: u16,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PartBreakDropTable {
    pub break_drops: Vec<PartBreakDrop>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PartBreakParts {
    pub tables: Vec<PartBreakDropTable>,
}

#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SigilTowerTable {
    pub item_id: u16,
    pub receipt_category: u16,
    pub item_craft1: u16,
    pub item_qty1: u8,
    pub item_unlock_flag1: u8,
    pub item_craft2: u16,
    pub item_qty2: u8,
    pub item_unlock_flag2: u8,
    pub item_craft3: u16,
    pub item_qty3: u8,
    pub item_unlock_flag3: u8,
    pub item_craft4: u16,
    pub item_qty4: u8,
    pub item_unlock_flag4: u8,
}

// G50 Tower Weapon Parameters - 16 bytes per entry
// Structure: Each weapon type has 130 weapon pointers, each pointing to 50 level entries
#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TowerG50WeaponParams {
    pub model_id: u16,
    pub sharpness_id: u8,
    pub max_sharpness: u8,
    pub weapon_raw: u16,
    pub element_id: u8,
    pub ele_damage: u8,
    pub ailment_id: u8,
    pub ail_damage: u8,
    pub defense: u16,
    pub chance_rate: u8,  // Chance to get upgraded on low cost upgrade?
    pub unk_0d: u8,
    pub upgrade_path: u16,
}

// A single weapon's G50 data: 50 level entries
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct G50WeaponLevels {
    pub levels: Vec<TowerG50WeaponParams>, // 50 entries per weapon
}

// A weapon type's G50 data: 130 weapons
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct G50WeaponTypeData {
    pub weapons: Vec<G50WeaponLevels>, // 130 weapons per type
}

#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct G50WUpgrade {
    pub weapon_id: u16,
    pub level1: u16,
    pub level2: u16,
    pub full_succ_rate: u16,
    pub zenny_cost: u32,
    pub upgrade_material1: u16,
    pub num_material1: u8,
    pub padding1: [u8; 5],
    pub upgrade_material2: u16,
    pub num_material2: u8,
    pub padding2: [u8; 5],
    pub upgrade_material3: u16,
    pub num_material3: u8,
    pub padding3: [u8; 9],
}

#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MWUpgradePath {
    pub upgrade_material1: u16,
    pub num_material1: u16,
    pub padding1: [u8; 4],
    pub upgrade_material2: u16,
    pub num_material2: u16,
    pub padding2: [u8; 4],
    pub upgrade_material3: u16,
    pub num_material3: u16,
    pub padding3: [u8; 4],
    pub upgrades_to1: u16,
    pub upgrades_to2: u16,
    pub upgrades_to3: u16,
    pub upgrades_to4: u16,
    pub padding4: [u8; 4],
}

#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RWUpgradePath {
    pub upgrade_material1: u16,
    pub num_material1: u16,
    pub padding1: [u8; 4],
    pub upgrade_material2: u16,
    pub num_material2: u16,
    pub padding2: [u8; 4],
    pub upgrade_material3: u16,
    pub num_material3: u16,
    pub padding3: [u8; 4],
    pub upgrades_to1: u16,
    pub upgrades_to2: u16,
    pub upgrades_to3: u16,
    pub upgrades_to4: u16,
    pub padding4: [u8; 4],
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub struct EvoUpgradeSub {
    pub dmg: u16,
    pub unk_sub2: u16,
    pub ele_damage: u16,
    pub padding1: [u8; 2],
    pub unk_sub8: u16,
    pub padding2: [u8; 2],
}

#[repr(C, packed)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EvoUpgrade {
    pub g_cost: u32,
    pub unk04: u16,
    pub level: u16,
    pub sub: [EvoUpgradeSub; 11],
    pub unk8c: u16,
    pub unk8e: u8,
    pub padding1: [u8; 1],
    pub unk90: u16,
    pub padding2: [u8; 2],
    pub unk94: f32,
    pub unk98: f32,
    pub unk9c: f32,
    pub unka0: f32,
    pub unka4: f32,
    pub unka8: u16,
    pub unkaa: u16,
    pub unkac: u8,
    pub unkad: u8,
    pub padding3: [u8; 2],
}

impl Clone for EvoUpgrade {
    fn clone(&self) -> Self {
        let mut sub = [EvoUpgradeSub::default(); 11];
        for (i, s) in sub.iter_mut().enumerate() {
            *s = self.sub[i];
        }
        Self {
            g_cost: self.g_cost,
            unk04: self.unk04,
            level: self.level,
            sub,
            unk8c: self.unk8c,
            unk8e: self.unk8e,
            padding1: self.padding1,
            unk90: self.unk90,
            padding2: self.padding2,
            unk94: self.unk94,
            unk98: self.unk98,
            unk9c: self.unk9c,
            unka0: self.unka0,
            unka4: self.unka4,
            unka8: self.unka8,
            unkaa: self.unkaa,
            unkac: self.unkac,
            unkad: self.unkad,
            padding3: self.padding3,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EquipmentCounts {
    pub numLegA: u16,
    pub numUnk: u16,
    pub numHeadA: u16,
    pub numBodyA: u16,
    pub numArmA: u16,
    pub numWaistA: u16,
    pub numMeleeW: u16,
    pub numRangedW: u16,
}

#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct MhfdatDecoId {
    pub slot_nb: u8,
    pub flags: u16,
    pub price: u32,
    pub _pad0: u8,
    pub skill_id1: u8,
    pub skill_pts1: i8,
    pub skill_id2: u8,
    pub skill_pts2: i8,
    pub skill_id3: u8,
    pub skill_pts3: i8,
    pub skill_id4: u8,
    pub skill_pts4: i8,
    pub special_flags: u16,
    pub zenith_skill: u16,
}

// Quest structures
#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct QuestItem {
    pub quest_id: u16,
    pub quest_number: u16,
    pub key_quest: u8,
    pub urgent_quest: u8,
    pub unknown: u16,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HRQuests {
    pub one_star: Vec<QuestItem>,
    pub two_stars: Vec<QuestItem>,
    pub three_stars: Vec<QuestItem>,
    pub four_stars: Vec<QuestItem>,
    pub five_stars: Vec<QuestItem>,
    pub six_stars: Vec<QuestItem>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GRQuests {
    pub g1: Vec<QuestItem>,
    pub g2: Vec<QuestItem>,
    pub g3: Vec<QuestItem>,
    pub g4: Vec<QuestItem>,
    pub g5: Vec<QuestItem>,
    pub g6: Vec<QuestItem>,
    pub g7: Vec<QuestItem>,
}

// Armor Stat Pointers structure (6 u32 pointers: 24 bytes total)
#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ArmorStatPointers {
    pub legs: u32,  // Legs armor stats
    pub head1: u32, // Head armor stats (set 1)
    pub head2: u32, // Head armor stats (set 2)
    pub body: u32,  // Body armor stats
    pub arm: u32,   // Arm armor stats
    pub waist: u32, // Waist armor stats
}

// Armor Name Pointers structure (5 u32 pointers: 20 bytes total)
#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ArmorNamePointers {
    pub head: u32,  // Head armor names
    pub body: u32,  // Body armor names
    pub arm: u32,   // Arm armor names
    pub waist: u32, // Waist armor names
    pub legs: u32,  // Legs armor names
}

// Armor/Weapon Name Pointers structure (8 u32 pointers: 32 bytes total)
#[repr(C, packed)]
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ArmorWeaponNamePointers {
    pub legs: u32,     // Legs armor names
    pub unknown1: u32,  // Unknown/Reserved
    pub head: u32,     // Head armor names
    pub body: u32,     // Body armor names
    pub arm: u32,      // Arm armor names
    pub waist: u32,    // Waist armor names
    pub melee: u32,    // Melee weapon names
    pub ranged: u32,   // Ranged weapon names
}

// ── Sigil Crafting ──────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SigilMaterial {
    pub item: u16,
    pub percentage_filled: u8,
    pub unk: u8,
}

/// 0x28 (40) bytes per recipe
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SigilRecipe {
    pub unk0: u8,
    pub extra_skills_low: u8,
    pub extra_skills_high: u8,
    pub unk1: u8,
    pub cost: u16,
    pub padding: [u8; 2],
    pub key_materials: [SigilMaterial; 5],
    pub padding2: [u8; 12],
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SigilSkillProbability {
    pub skill: u16,
    pub percentage_chance: u16,
    pub low_points: i16,
    pub high_points: i16,
}

/// 8 probabilities per recipe = 64 bytes
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SigilSkillProbabilities {
    pub probabilities: [SigilSkillProbability; 8],
}