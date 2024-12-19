use std::fmt::{self, Display};

use crate::config::Config;

#[derive(Debug)]
pub struct TurbowarpConfig {
    pub frame_rate: u64,
    pub max_clones: f64,
    pub no_miscellaneous_limits: bool,
    pub no_sprite_fencing: bool,
    pub frame_interpolation: bool,
    pub high_quality_pen: bool,
    pub stage_width: u64,
    pub stage_height: u64,
}

impl Default for TurbowarpConfig {
    fn default() -> Self {
        Self {
            frame_rate: 30,
            max_clones: 300.0,
            no_miscellaneous_limits: false,
            no_sprite_fencing: false,
            frame_interpolation: false,
            high_quality_pen: false,
            stage_width: 480,
            stage_height: 360,
        }
    }
}

#[allow(clippy::write_with_newline)]
impl Display for TurbowarpConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration for https://turbowarp.org/\n")?;
        write!(
            f,
            "You can move, resize, and minimize this comment, but don't "
        )?;
        write!(
            f,
            "edit it by hand. This comment can be deleted to remove the "
        )?;
        write!(f, "stored settings.\n")?;
        write!(f, "{{")?;
        write!(f, r#""framerate":{}"#, self.frame_rate)?;
        write!(f, r#","interpolation":{}"#, self.frame_interpolation)?;
        write!(f, r#","hq":{}"#, self.high_quality_pen)?;
        write!(f, r#","width":{}"#, self.stage_width)?;
        write!(f, r#","height":{}"#, self.stage_height)?;
        write!(f, r#","runtimeOptions":{{"#)?;
        if self.max_clones.is_infinite() {
            write!(f, r#""maxClones":Infinity"#)?;
        } else {
            write!(f, r#""maxClones":{}"#, self.max_clones)?;
        }
        write!(f, r#","miscLimits":{}"#, !self.no_miscellaneous_limits)?;
        write!(f, r#","fencing":{}"#, !self.no_sprite_fencing)?;
        write!(f, r#"}}"#)?; // runtimeOptions
        write!(f, "}} // _twconfig_") // twconfig
    }
}

impl From<&Config> for TurbowarpConfig {
    fn from(config: &Config) -> Self {
        let default = Self::default();
        Self {
            frame_rate: config.frame_rate.unwrap_or(default.frame_rate),
            max_clones: config.max_clones.unwrap_or(default.max_clones),
            no_miscellaneous_limits: config
                .no_miscellaneous_limits
                .unwrap_or(default.no_miscellaneous_limits),
            no_sprite_fencing: config
                .no_sprite_fencing
                .unwrap_or(default.no_sprite_fencing),
            frame_interpolation: config
                .frame_interpolation
                .unwrap_or(default.frame_interpolation),
            high_quality_pen: config.high_quality_pen.unwrap_or(default.high_quality_pen),
            stage_width: config.stage_width.unwrap_or(default.stage_width),
            stage_height: config.stage_height.unwrap_or(default.stage_height),
        }
    }
}
