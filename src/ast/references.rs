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
    pub names: FxHashSet<NameReference>,
    pub structs: FxHashSet<SmolStr>,
    pub args: FxHashSet<NameReference>,
}

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct NameReference {
    pub name: SmolStr,
    pub field: Option<SmolStr>,
    pub proc: Option<SmolStr>,
    pub func: Option<SmolStr>,
}
