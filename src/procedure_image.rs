#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcedureFireImageConfig {
    pub detail_strength: f32,
    pub scroll_speed: f32,
    pub fire_height: f32,
    pub fire_width: f32,
    pub fire_sharpness: f32,
    pub intensity: f32,
}

impl Default for ProcedureFireImageConfig {
    fn default() -> Self {
        Self {
            detail_strength: 3.0,
            scroll_speed: 1.0,
            fire_height: 0.5,
            fire_width: 4.0,
            fire_sharpness: 1.0,
            intensity: 1.1,
        }
    }
}

/// Owned shareable image resource.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProcedureImage {
    Fire(ProcedureFireImageConfig, u32, u32),
    CheckBoard(u32, u32),
}
