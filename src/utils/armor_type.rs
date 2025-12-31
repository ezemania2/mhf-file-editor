/// Armor Type bitfield utilities
/// 
/// Represents the various armor types as bit flags in a u32 value

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArmorTypeFlags {
    pub gou: bool,           // bit 0
    pub grank: bool,         // bit 6
    pub supremacy: bool,     // bit 7
    pub premium_hr: bool,    // bit 8
    pub premium_gr: bool,    // bit 9 (used in GRANK_PREMIUM and ZENITH_PREMIUM)
    pub g_supremacy: bool,   // bit 14
    pub gf_supremacy: bool,  // bit 15
    pub gx_supremacy: bool,  // bit 16
    pub exotic: bool,        // bit 18
    pub zenith: bool,        // bit 19
}

impl Default for ArmorTypeFlags {
    fn default() -> Self {
        Self {
            gou: false,
            grank: false,
            supremacy: false,
            premium_hr: false,
            premium_gr: false,
            g_supremacy: false,
            gf_supremacy: false,
            gx_supremacy: false,
            exotic: false,
            zenith: false,
        }
    }
}

impl ArmorTypeFlags {
    /// Decode a u32 armor_type value into individual flags
    pub fn from_u32(value: u32) -> Self {
        Self {
            gou: (value & (1 << 0)) != 0,
            grank: (value & (1 << 6)) != 0,
            supremacy: (value & (1 << 7)) != 0,
            premium_hr: (value & (1 << 8)) != 0,
            premium_gr: (value & (1 << 9)) != 0,
            g_supremacy: (value & (1 << 14)) != 0,
            gf_supremacy: (value & (1 << 15)) != 0,
            gx_supremacy: (value & (1 << 16)) != 0,
            exotic: (value & (1 << 18)) != 0,
            zenith: (value & (1 << 19)) != 0,
        }
    }

    /// Encode the flags back into a u32 armor_type value
    pub fn to_u32(&self) -> u32 {
        let mut value = 0u32;
        
        if self.gou { value |= 1 << 0; }
        if self.grank { value |= 1 << 6; }
        if self.supremacy { value |= 1 << 7; }
        if self.premium_hr { value |= 1 << 8; }
        if self.premium_gr { value |= 1 << 9; }
        if self.g_supremacy { value |= 1 << 14; }
        if self.gf_supremacy { value |= 1 << 15; }
        if self.gx_supremacy { value |= 1 << 16; }
        if self.exotic { value |= 1 << 18; }
        if self.zenith { value |= 1 << 19; }
        
        value
    }

    /// Get a human-readable description of the armor type
    pub fn description(&self) -> String {
        let mut types = Vec::new();
        
        if self.gou { types.push("Gou"); }
        if self.grank { types.push("G-Rank"); }
        if self.supremacy { types.push("Supremacy"); }
        if self.premium_hr { types.push("Premium HR"); }
        if self.premium_gr { types.push("Premium GR"); }
        if self.g_supremacy { types.push("G Supremacy"); }
        if self.gf_supremacy { types.push("GF Supremacy"); }
        if self.gx_supremacy { types.push("GX Supremacy"); }
        if self.exotic { types.push("Exotic"); }
        if self.zenith { types.push("Zenith"); }
        
        if types.is_empty() {
            "None".to_string()
        } else {
            types.join(", ")
        }
    }

    /// Get a short description (for UI display)
    pub fn short_description(&self) -> String {
        let mut types = Vec::new();
        
        if self.gou { types.push("Gou"); }
        if self.grank { types.push("GR"); }
        if self.supremacy { types.push("Sup"); }
        if self.premium_hr { types.push("P-HR"); }
        if self.premium_gr { types.push("P-GR"); }
        if self.g_supremacy { types.push("G-Sup"); }
        if self.gf_supremacy { types.push("GF-Sup"); }
        if self.gx_supremacy { types.push("GX-Sup"); }
        if self.exotic { types.push("Exotic"); }
        if self.zenith { types.push("Zenith"); }
        
        if types.is_empty() {
            "None".to_string()
        } else {
            types.join("|")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let flags = ArmorTypeFlags {
            gou: true,
            grank: true,
            supremacy: false,
            premium_hr: false,
            premium_gr: false,
            g_supremacy: false,
            gf_supremacy: false,
            gx_supremacy: false,
            exotic: false,
            zenith: true,
        };
        
        let encoded = flags.to_u32();
        let decoded = ArmorTypeFlags::from_u32(encoded);
        
        assert_eq!(flags, decoded);
    }

    #[test]
    fn test_individual_bits() {
        // Test Gou (bit 0)
        let flags = ArmorTypeFlags { gou: true, ..Default::default() };
        assert_eq!(flags.to_u32(), 0b1);
        
        // Test GRANK (bit 6)
        let flags = ArmorTypeFlags { grank: true, ..Default::default() };
        assert_eq!(flags.to_u32(), 0b1000000);
        
        // Test ZENITH (bit 19)
        let flags = ArmorTypeFlags { zenith: true, ..Default::default() };
        assert_eq!(flags.to_u32(), 1 << 19);
    }
}

