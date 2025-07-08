use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct Config {
    #[serde(default)]
    pub pre_build: Option<String>,
    #[serde(default)]
    pub post_build: Option<String>,
    #[serde(default)]
    pub layers: Option<Vec<String>>,
    #[serde(default)]
    pub std: Option<String>,
    #[serde(default)]
    pub bitmap_resolution: Option<u64>,
    #[serde(default)]
    pub frame_rate: Option<u64>,
    #[serde(default)]
    pub max_clones: Option<f64>,
    #[serde(default)]
    pub no_miscellaneous_limits: Option<bool>,
    #[serde(default)]
    pub no_sprite_fencing: Option<bool>,
    #[serde(default)]
    pub frame_interpolation: Option<bool>,
    #[serde(default)]
    pub high_quality_pen: Option<bool>,
    #[serde(default)]
    pub stage_width: Option<u64>,
    #[serde(default)]
    pub stage_height: Option<u64>,
}
