use fxhash::FxHashMap;

use super::sprite::Sprite;
use crate::misc::SmolStr;

#[derive(Debug)]
pub struct Project {
    pub stage: Sprite,
    pub sprites: FxHashMap<SmolStr, Sprite>,
}
