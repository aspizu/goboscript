use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Config {
    #[serde(default)]
    pub frame_rate: Option<u64>,
    #[serde(default)]
    pub max_clones: Option<u64>,
    #[serde(default = "dtrue")]
    pub miscellaneous_limits: bool,
    #[serde(default = "dtrue")]
    pub sprite_fencing: bool,
    #[serde(default)]
    pub frame_interpolation: bool,
    #[serde(default)]
    pub high_quality_pen: bool,
    #[serde(default)]
    pub stage_width: Option<u64>,
    #[serde(default)]
    pub stage_height: Option<u64>,
}

fn dtrue() -> bool {
    true
}

impl Config {
    pub fn is_default(&self) -> bool {
        self == &Config::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frame_rate: None,
            max_clones: None,
            miscellaneous_limits: true,
            sprite_fencing: true,
            frame_interpolation: false,
            high_quality_pen: false,
            stage_width: None,
            stage_height: None,
        }
    }
}
