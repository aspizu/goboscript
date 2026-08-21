use rustc_hash::FxHashMap;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;

use super::sprite::Sprite;
use crate::misc::SmolStr;

#[derive(Debug, Tsify, Serialize, Deserialize)]
pub struct Project {
    pub stage: Sprite,
    pub sprites: FxHashMap<SmolStr, Sprite>,
}
