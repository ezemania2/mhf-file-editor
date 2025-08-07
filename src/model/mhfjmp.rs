// Structures de données pour mhfjmp.bin

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MenuEntry {
    pub jump_id: u32,
    pub unk0c: u32,
    pub area_id: u16,
    pub area_id2: u16,
    pub area_id3: u16,
    pub area_id4: u16,
    pub player_pos_x: f32,
    pub player_pos_y: f32,
    pub player_pos_z: f32,
    pub rotation: u32,
    pub camera_pos_x: f32,
    pub camera_pos_y: f32,
    pub camera_pos_z: f32,
    pub rotation1: u32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AreaEntry {
    pub index: u16,
    pub flags: u16,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Area {
    pub p_entry_data: u32,
    pub len_entry_data: u32,
    pub p_stage_ids: u32,
    pub entries: Vec<AreaEntry>,
    pub stage_ids: Vec<u16>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StringEntry {
    pub id: i32,
    pub text: String,
}
