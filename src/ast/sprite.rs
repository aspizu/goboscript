use fxhash::{FxHashMap, FxHashSet};

use super::{
    costume::Costume, enum_::Enum, event::Event, list::List, proc::Proc, struct_::Struct, var::Var,
    Func,
};
use crate::misc::SmolStr;

#[derive(Debug, Default)]
pub struct Sprite {
    pub costumes: Vec<Costume>,
    pub procs: FxHashMap<SmolStr, Proc>,
    pub funcs: FxHashMap<SmolStr, Func>,
    pub enums: FxHashMap<SmolStr, Enum>,
    pub structs: FxHashMap<SmolStr, Struct>,
    pub vars: FxHashMap<SmolStr, Var>,
    pub lists: FxHashMap<SmolStr, List>,
    pub events: Vec<Event>,
    pub used_procs: FxHashSet<SmolStr>,
    pub used_funcs: FxHashSet<SmolStr>,
}
