use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Copy)]
pub struct Config {
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

impl Config {
    #[rustfmt::skip]
    #[allow(clippy::bool_comparison)]
    pub fn is_default(&self) -> bool {
            !( self.frame_rate             .is_some_and(|it| it != 30   )
            || self.max_clones             .is_some_and(|it| it != 300.0)
            || self.no_miscellaneous_limits.is_some_and(|it| it != false)
            || self.no_sprite_fencing      .is_some_and(|it| it != false)
            || self.frame_interpolation    .is_some_and(|it| it != false)
            || self.high_quality_pen       .is_some_and(|it| it != false)
            || self.stage_width            .is_some_and(|it| it != 480  )
            || self.stage_height           .is_some_and(|it| it != 360  ))
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut comma = false;
        write!(f, "Configuration for https://turbowarp.org/\nYou can move, resize, and minimize this comment, but don't edit it by hand. This comment can be deleted to remove the stored settings.\n{{")?;
        if let Some(frame_rate) = self.frame_rate {
            if frame_rate != 30 {
                write!(f, r#""framerate":{}"#, frame_rate)?;
                comma = true;
            }
        }
        if let Some(frame_interpolation) = self.frame_interpolation {
            if frame_interpolation {
                if comma {
                    write!(f, ",")?;
                }
                write!(f, r#""interpolation":true"#)?;
                comma = true;
            }
        }
        if let Some(high_quality_pen) = self.high_quality_pen {
            if high_quality_pen {
                if comma {
                    write!(f, ",")?;
                }
                write!(f, r#""hq":true"#)?;
                comma = true;
            }
        }
        if let Some(stage_width) = self.stage_width {
            if stage_width != 480 {
                if comma {
                    write!(f, ",")?;
                }
                write!(f, r#""width":{}"#, stage_width)?;
                comma = true;
            }
        }
        if let Some(stage_height) = self.stage_height {
            if stage_height != 360 {
                if comma {
                    write!(f, ",")?;
                }
                write!(f, r#""height":{}"#, stage_height)?;
                comma = true;
            }
        }
        let mut runtime_options_opened = false;
        if let Some(max_clones) = self.max_clones {
            if max_clones.is_infinite() {
                if comma {
                    write!(f, ",")?;
                }
                write!(f, r#""runtimeOptions":{{"#)?;
                runtime_options_opened = true;
                write!(f, r#""maxClones":Infinity"#,)?;
                comma = true;
            } else if (max_clones as i64) != 300 {
                if comma {
                    write!(f, ",")?;
                }
                write!(f, r#""runtimeOptions":{{"#)?;
                runtime_options_opened = true;
                write!(f, r#""maxClones":{}"#, max_clones as i64)?;
                comma = true;
            }
        }
        if let Some(no_miscellaneous_limits) = self.no_miscellaneous_limits {
            if no_miscellaneous_limits {
                if comma {
                    write!(f, ",")?;
                }
                if !runtime_options_opened {
                    write!(f, r#""runtimeOptions":{{"#)?;
                    runtime_options_opened = true;
                }
                write!(f, r#""miscLimits":false"#)?;
                comma = true;
            }
        }
        if let Some(no_sprite_fencing) = self.no_sprite_fencing {
            if no_sprite_fencing {
                if comma {
                    write!(f, ",")?;
                }
                if !runtime_options_opened {
                    write!(f, r#""runtimeOptions":{{"#)?;
                    runtime_options_opened = true;
                }
                write!(f, r#""fencing":false"#)?;
            }
        }
        if runtime_options_opened {
            write!(f, "}}")?;
        }
        write!(f, "}} // _twconfig_")
    }
}
