use fxhash::FxHashSet;

use crate::misc::SmolStr;

#[derive(Debug, Default)]
pub struct References {
    pub procs: FxHashSet<SmolStr>,
    pub funcs: FxHashSet<SmolStr>,
    pub names: FxHashSet<SmolStr>,
    pub structs: FxHashSet<SmolStr>,
    pub struct_fields: FxHashSet<(SmolStr, SmolStr)>,
    pub enum_variants: FxHashSet<(SmolStr, SmolStr)>,
}
