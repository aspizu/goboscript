use fxhash::FxHashMap;
use smol_str::SmolStr;

use super::{
    costume::Costume, enum_::Enum, event::Event, list::List, proc::Proc, struct_::Struct, var::Var,
    Func,
};

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
}
