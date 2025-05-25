use std::fmt::Display;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Default, Debug, Serialize, Deserialize)]
pub enum RotationStyle {
    LeftRight,
    #[default]
    AllAround,
    DoNotRotate,
}

impl Display for RotationStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RotationStyle::LeftRight => write!(f, "left-right"),
            RotationStyle::AllAround => write!(f, "all around"),
            RotationStyle::DoNotRotate => write!(f, "don't rotate"),
        }
    }
}
