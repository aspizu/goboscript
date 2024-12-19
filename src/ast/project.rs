use fxhash::FxHashMap;
use smol_str::SmolStr;

use super::sprite::Sprite;

#[derive(Debug)]
pub struct Project {
    pub stage: Sprite,
    pub sprites: FxHashMap<SmolStr, Sprite>,
}
