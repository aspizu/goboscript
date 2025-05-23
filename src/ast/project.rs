use fxhash::FxHashMap;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;

use super::sprite::Sprite;
use crate::misc::SmolStr;

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Project {
    pub stage: Sprite,
    pub sprites: FxHashMap<SmolStr, Sprite>,
}
