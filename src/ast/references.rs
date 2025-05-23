use fxhash::FxHashSet;
use serde::{
    Deserialize,
    Serialize,
};

use crate::misc::SmolStr;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct References {
    pub procs: FxHashSet<SmolStr>,
    pub funcs: FxHashSet<SmolStr>,
    pub names: FxHashSet<(SmolStr, Option<SmolStr>)>,
    pub structs: FxHashSet<SmolStr>,
    pub struct_fields: FxHashSet<(SmolStr, SmolStr)>,
    pub enum_variants: FxHashSet<(SmolStr, SmolStr)>,
}
