// Utilitaires pour manipuler le champ equipable_by (bitfield) des armures
pub fn is_male_equip(val: u8) -> bool { val & 0b0000_0001 != 0 }
pub fn is_female_equip(val: u8) -> bool { val & 0b0000_0010 != 0 }
pub fn is_blade_equip(val: u8) -> bool { val & 0b0000_0100 != 0 }
pub fn is_gunner_equip(val: u8) -> bool { val & 0b0000_1000 != 0 }
pub fn is_bool1(val: u8) -> bool { val & 0b0001_0000 != 0 }
pub fn is_sp_equip(val: u8) -> bool { val & 0b0010_0000 != 0 }
pub fn is_bool3(val: u8) -> bool { val & 0b0100_0000 != 0 }
pub fn is_bool4(val: u8) -> bool { val & 0b1000_0000 != 0 }

pub fn set_flag(val: &mut u8, mask: u8, enabled: bool) {
    if enabled {
        *val |= mask;
    } else {
        *val &= !mask;
    }
}

use serde::{Serialize, Deserialize};

/// Equipment Type bitfield structure (based on hexpat EquipType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EquipType {
    pub sp: bool,                 // bit 0
    pub gou: bool,                // bit 1  
    pub evolution: bool,          // bit 2
    pub hc: bool,                 // bit 3
    pub random_weapon: bool,      // bit 4
    pub ravi: bool,               // bit 5
    pub g50: bool,                // bit 6
    pub unk_7: bool,              // bit 7
}

impl EquipType {
    pub fn from_u8(value: u8) -> Self {
        Self {
            sp: (value & 0x01) != 0,
            gou: (value & 0x02) != 0,
            evolution: (value & 0x04) != 0,
            hc: (value & 0x08) != 0,
            random_weapon: (value & 0x10) != 0,
            ravi: (value & 0x20) != 0,
            g50: (value & 0x40) != 0,
            unk_7: (value & 0x80) != 0,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        let mut value = 0u8;
        if self.sp { value |= 0x01; }
        if self.gou { value |= 0x02; }
        if self.evolution { value |= 0x04; }
        if self.hc { value |= 0x08; }
        if self.random_weapon { value |= 0x10; }
        if self.ravi { value |= 0x20; }
        if self.g50 { value |= 0x40; }
        if self.unk_7 { value |= 0x80; }
        value
    }
    
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        if self.sp { parts.push("SP"); }
        if self.gou { parts.push("Gou"); }
        if self.evolution { parts.push("Evolution"); }
        if self.hc { parts.push("HC"); }
        if self.random_weapon { parts.push("Random Weapon"); }
        if self.ravi { parts.push("Ravi"); }
        if self.g50 { parts.push("G50"); }
        if self.unk_7 { parts.push("Unknown_7"); }
        
        if parts.is_empty() {
            "General".to_string()
        } else {
            parts.join(" | ")
        }
    }
}

impl Default for EquipType {
    fn default() -> Self {
        Self {
            sp: false,
            gou: false,
            evolution: false,
            hc: false,
            random_weapon: false,
            ravi: false,
            g50: false,
            unk_7: false,
        }
    }
}

/// Weapon Type bitfield structure (based on hexpat WeaponType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeaponType {
    pub finess_base: bool,        // bit 0
    pub gou_hr1: bool,           // bit 1
    pub gou_hr2: bool,           // bit 2
    pub gr_gunner: bool,         // bit 3
    pub gou_gr1: bool,           // bit 4
    pub gou_gr2: bool,           // bit 5
    pub finess_ext: bool,        // bit 6
    pub tower: bool,             // bit 7
    pub gou_gr3: bool,           // bit 8
    pub exotique: bool,          // bit 9
    pub ravi_z: bool,            // bit 10
    pub prayer_base: bool,       // bit 11
    pub zenith: bool,            // bit 12
    pub ravi_gr_plus: bool,      // bit 14
    pub gr_simple_upgrade: bool, // bit 27
}

impl WeaponType {
    pub fn from_u32(value: u32) -> Self {
        Self {
            finess_base: (value & 0x00000001) != 0,
            gou_hr1: (value & 0x00000002) != 0,
            gou_hr2: (value & 0x00000004) != 0,
            gr_gunner: (value & 0x00000008) != 0,
            gou_gr1: (value & 0x00000010) != 0,
            gou_gr2: (value & 0x00000020) != 0,
            finess_ext: (value & 0x00000040) != 0,
            tower: (value & 0x00000080) != 0,
            gou_gr3: (value & 0x00000100) != 0,
            exotique: (value & 0x00000200) != 0,
            ravi_z: (value & 0x00000400) != 0,
            prayer_base: (value & 0x00000800) != 0,
            zenith: (value & 0x00001000) != 0,
            ravi_gr_plus: (value & 0x00004000) != 0,
            gr_simple_upgrade: (value & 0x08000000) != 0,
        }
    }
    
    pub fn to_u32(&self) -> u32 {
        let mut value = 0u32;
        if self.finess_base { value |= 0x00000001; }
        if self.gou_hr1 { value |= 0x00000002; }
        if self.gou_hr2 { value |= 0x00000004; }
        if self.gr_gunner { value |= 0x00000008; }
        if self.gou_gr1 { value |= 0x00000010; }
        if self.gou_gr2 { value |= 0x00000020; }
        if self.finess_ext { value |= 0x00000040; }
        if self.tower { value |= 0x00000080; }
        if self.gou_gr3 { value |= 0x00000100; }
        if self.exotique { value |= 0x00000200; }
        if self.ravi_z { value |= 0x00000400; }
        if self.prayer_base { value |= 0x00000800; }
        if self.zenith { value |= 0x00001000; }
        if self.ravi_gr_plus { value |= 0x00004000; }
        if self.gr_simple_upgrade { value |= 0x08000000; }
        value
    }
    
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        
        // Special case for Prayer GR (combination of bits 11 + 3)
        if self.prayer_base && self.gr_gunner {
            parts.push("Prayer GR");
        } else {
            // Individual handling if not Prayer GR
            if self.prayer_base { parts.push("Prayer LR"); }
            if self.gr_gunner { parts.push("GR Gunner"); }
        }
        
        if self.finess_base { parts.push("Finess Base"); }
        if self.gou_hr1 { parts.push("Gou HR1"); }
        if self.gou_hr2 { parts.push("Gou HR2"); }
        if self.gou_gr1 { parts.push("Gou GR1"); }
        if self.gou_gr2 { parts.push("Gou GR2"); }
        if self.finess_ext { parts.push("Finess Ext"); }
        if self.tower { parts.push("Tower"); }
        if self.gou_gr3 { parts.push("Gou GR3"); }
        if self.exotique { parts.push("Exotic"); }
        if self.ravi_z { parts.push("Ravi Z"); }
        if self.zenith { parts.push("Zenith"); }
        if self.ravi_gr_plus { parts.push("Ravi GR+"); }
        if self.gr_simple_upgrade { parts.push("GR Simple upgrade"); }
        
        if parts.is_empty() {
            "General".to_string()
        } else {
            parts.join(" | ")
        }
    }
    
    pub fn is_prayer_gr(&self) -> bool {
        self.prayer_base && self.gr_gunner
    }
}

impl Default for WeaponType {
    fn default() -> Self {
        Self {
            finess_base: false,
            gou_hr1: false,
            gou_hr2: false,
            gr_gunner: false,
            gou_gr1: false,
            gou_gr2: false,
            finess_ext: false,
            tower: false,
            gou_gr3: false,
            exotique: false,
            ravi_z: false,
            prayer_base: false,
            zenith: false,
            ravi_gr_plus: false,
            gr_simple_upgrade: false,
        }
    }
}

/// Bullet Types bitfield structure (for ranged weapons)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulletTypes {
    pub normal_lv1: bool,       // bit 0
    pub normal_lv2: bool,       // bit 1
    pub normal_lv3: bool,       // bit 2
    pub pierce_lv1: bool,       // bit 3
    pub pierce_lv2: bool,       // bit 4
    pub pierce_lv3: bool,       // bit 5
    pub spread_lv1: bool,       // bit 6
    pub spread_lv2: bool,       // bit 7
    pub spread_lv3: bool,       // bit 8
    pub crag_lv1: bool,         // bit 9
    pub crag_lv2: bool,         // bit 10
    pub crag_lv3: bool,         // bit 11
    pub cluster_lv1: bool,      // bit 12
    pub cluster_lv2: bool,      // bit 13
    pub cluster_lv3: bool,      // bit 14
    pub fire: bool,             // bit 15
    pub water: bool,            // bit 16
    pub thunder: bool,          // bit 17
    pub ice: bool,              // bit 18
    pub dragon: bool,           // bit 19
    pub recovery_lv1: bool,     // bit 20
    pub recovery_lv2: bool,     // bit 21
    pub poison_lv1: bool,       // bit 22
    pub poison_lv2: bool,       // bit 23
    pub paralysis_lv1: bool,    // bit 24
    pub paralysis_lv2: bool,    // bit 25
    pub sleep_lv1: bool,        // bit 26
    pub sleep_lv2: bool,        // bit 27
    pub tranquilizer: bool,     // bit 28
    pub paint: bool,            // bit 29
    pub demon: bool,            // bit 30
    pub armor: bool,            // bit 31
}

impl BulletTypes {
    pub fn from_u32(value: u32) -> Self {
        Self {
            normal_lv1: (value & 0x00000001) != 0,
            normal_lv2: (value & 0x00000002) != 0,
            normal_lv3: (value & 0x00000004) != 0,
            pierce_lv1: (value & 0x00000008) != 0,
            pierce_lv2: (value & 0x00000010) != 0,
            pierce_lv3: (value & 0x00000020) != 0,
            spread_lv1: (value & 0x00000040) != 0,
            spread_lv2: (value & 0x00000080) != 0,
            spread_lv3: (value & 0x00000100) != 0,
            crag_lv1: (value & 0x00000200) != 0,
            crag_lv2: (value & 0x00000400) != 0,
            crag_lv3: (value & 0x00000800) != 0,
            cluster_lv1: (value & 0x00001000) != 0,
            cluster_lv2: (value & 0x00002000) != 0,
            cluster_lv3: (value & 0x00004000) != 0,
            fire: (value & 0x00008000) != 0,
            water: (value & 0x00010000) != 0,
            thunder: (value & 0x00020000) != 0,
            ice: (value & 0x00040000) != 0,
            dragon: (value & 0x00080000) != 0,
            recovery_lv1: (value & 0x00100000) != 0,
            recovery_lv2: (value & 0x00200000) != 0,
            poison_lv1: (value & 0x00400000) != 0,
            poison_lv2: (value & 0x00800000) != 0,
            paralysis_lv1: (value & 0x01000000) != 0,
            paralysis_lv2: (value & 0x02000000) != 0,
            sleep_lv1: (value & 0x04000000) != 0,
            sleep_lv2: (value & 0x08000000) != 0,
            tranquilizer: (value & 0x10000000) != 0,
            paint: (value & 0x20000000) != 0,
            demon: (value & 0x40000000) != 0,
            armor: (value & 0x80000000) != 0,
        }
    }
    
    pub fn to_u32(&self) -> u32 {
        let mut value = 0u32;
        if self.normal_lv1 { value |= 0x00000001; }
        if self.normal_lv2 { value |= 0x00000002; }
        if self.normal_lv3 { value |= 0x00000004; }
        if self.pierce_lv1 { value |= 0x00000008; }
        if self.pierce_lv2 { value |= 0x00000010; }
        if self.pierce_lv3 { value |= 0x00000020; }
        if self.spread_lv1 { value |= 0x00000040; }
        if self.spread_lv2 { value |= 0x00000080; }
        if self.spread_lv3 { value |= 0x00000100; }
        if self.crag_lv1 { value |= 0x00000200; }
        if self.crag_lv2 { value |= 0x00000400; }
        if self.crag_lv3 { value |= 0x00000800; }
        if self.cluster_lv1 { value |= 0x00001000; }
        if self.cluster_lv2 { value |= 0x00002000; }
        if self.cluster_lv3 { value |= 0x00004000; }
        if self.fire { value |= 0x00008000; }
        if self.water { value |= 0x00010000; }
        if self.thunder { value |= 0x00020000; }
        if self.ice { value |= 0x00040000; }
        if self.dragon { value |= 0x00080000; }
        if self.recovery_lv1 { value |= 0x00100000; }
        if self.recovery_lv2 { value |= 0x00200000; }
        if self.poison_lv1 { value |= 0x00400000; }
        if self.poison_lv2 { value |= 0x00800000; }
        if self.paralysis_lv1 { value |= 0x01000000; }
        if self.paralysis_lv2 { value |= 0x02000000; }
        if self.sleep_lv1 { value |= 0x04000000; }
        if self.sleep_lv2 { value |= 0x08000000; }
        if self.tranquilizer { value |= 0x10000000; }
        if self.paint { value |= 0x20000000; }
        if self.demon { value |= 0x40000000; }
        if self.armor { value |= 0x80000000; }
        value
    }
    
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        
        if self.normal_lv1 { parts.push("Normal Lv1"); }
        if self.normal_lv2 { parts.push("Normal Lv2"); }
        if self.normal_lv3 { parts.push("Normal Lv3"); }
        if self.pierce_lv1 { parts.push("Pierce Lv1"); }
        if self.pierce_lv2 { parts.push("Pierce Lv2"); }
        if self.pierce_lv3 { parts.push("Pierce Lv3"); }
        if self.spread_lv1 { parts.push("Spread Lv1"); }
        if self.spread_lv2 { parts.push("Spread Lv2"); }
        if self.spread_lv3 { parts.push("Spread Lv3"); }
        if self.crag_lv1 { parts.push("Crag Lv1"); }
        if self.crag_lv2 { parts.push("Crag Lv2"); }
        if self.crag_lv3 { parts.push("Crag Lv3"); }
        if self.cluster_lv1 { parts.push("Cluster Lv1"); }
        if self.cluster_lv2 { parts.push("Cluster Lv2"); }
        if self.cluster_lv3 { parts.push("Cluster Lv3"); }
        if self.fire { parts.push("Fire"); }
        if self.water { parts.push("Water"); }
        if self.thunder { parts.push("Thunder"); }
        if self.ice { parts.push("Ice"); }
        if self.dragon { parts.push("Dragon"); }
        if self.recovery_lv1 { parts.push("Recovery Lv1"); }
        if self.recovery_lv2 { parts.push("Recovery Lv2"); }
        if self.poison_lv1 { parts.push("Poison Lv1"); }
        if self.poison_lv2 { parts.push("Poison Lv2"); }
        if self.paralysis_lv1 { parts.push("Paralysis Lv1"); }
        if self.paralysis_lv2 { parts.push("Paralysis Lv2"); }
        if self.sleep_lv1 { parts.push("Sleep Lv1"); }
        if self.sleep_lv2 { parts.push("Sleep Lv2"); }
        if self.tranquilizer { parts.push("Tranquilizer"); }
        if self.paint { parts.push("Paint"); }
        if self.demon { parts.push("Demon"); }
        if self.armor { parts.push("Armor"); }
        
        if parts.is_empty() {
            "None".to_string()
        } else {
            parts.join(", ")
        }
    }
}

impl Default for BulletTypes {
    fn default() -> Self {
        Self {
            normal_lv1: false,
            normal_lv2: false,
            normal_lv3: false,
            pierce_lv1: false,
            pierce_lv2: false,
            pierce_lv3: false,
            spread_lv1: false,
            spread_lv2: false,
            spread_lv3: false,
            crag_lv1: false,
            crag_lv2: false,
            crag_lv3: false,
            cluster_lv1: false,
            cluster_lv2: false,
            cluster_lv3: false,
            fire: false,
            water: false,
            thunder: false,
            ice: false,
            dragon: false,
            recovery_lv1: false,
            recovery_lv2: false,
            poison_lv1: false,
            poison_lv2: false,
            paralysis_lv1: false,
            paralysis_lv2: false,
            sleep_lv1: false,
            sleep_lv2: false,
            tranquilizer: false,
            paint: false,
            demon: false,
            armor: false,
        }
    }
}

/// Equipable By bitfield structure (for armor pieces)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EquipableBy {
    pub male: bool,       // bit 0
    pub female: bool,     // bit 1
    pub blade: bool,      // bit 2 - Blade user (melee)
    pub gunner: bool,     // bit 3 - Gunner user (ranged)
    pub bool1: bool,      // bit 4 - Unknown flag 1
    pub sp: bool,         // bit 5 - Special/SP equipment
    pub bool3: bool,      // bit 6 - Unknown flag 3
    pub bool4: bool,      // bit 7 - Unknown flag 4
}

impl EquipableBy {
    pub fn from_u8(value: u8) -> Self {
        Self {
            male: (value & 0x01) != 0,
            female: (value & 0x02) != 0,
            blade: (value & 0x04) != 0,
            gunner: (value & 0x08) != 0,
            bool1: (value & 0x10) != 0,
            sp: (value & 0x20) != 0,
            bool3: (value & 0x40) != 0,
            bool4: (value & 0x80) != 0,
        }
    }
    
    pub fn to_u8(&self) -> u8 {
        let mut value = 0u8;
        if self.male { value |= 0x01; }
        if self.female { value |= 0x02; }
        if self.blade { value |= 0x04; }
        if self.gunner { value |= 0x08; }
        if self.bool1 { value |= 0x10; }
        if self.sp { value |= 0x20; }
        if self.bool3 { value |= 0x40; }
        if self.bool4 { value |= 0x80; }
        value
    }
    
    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        if self.male { parts.push("Male"); }
        if self.female { parts.push("Female"); }
        if self.blade { parts.push("Blade"); }
        if self.gunner { parts.push("Gunner"); }
        if self.bool1 { parts.push("Bool1"); }
        if self.sp { parts.push("SP"); }
        if self.bool3 { parts.push("Bool3"); }
        if self.bool4 { parts.push("Bool4"); }
        
        if parts.is_empty() {
            "None".to_string()
        } else {
            parts.join(", ")
        }
    }
}