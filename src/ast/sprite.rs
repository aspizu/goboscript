use fxhash::{
    FxHashMap,
    FxHashSet,
};

use super::*;
use crate::misc::SmolStr;

#[derive(Debug, Default)]
pub struct Sprite {
    pub costumes: Vec<Costume>,
    pub procs: FxHashMap<SmolStr, Proc>,
    pub proc_definitions: FxHashMap<SmolStr, Vec<Stmt>>,
    pub proc_references: FxHashMap<SmolStr, References>,
    pub proc_used_args: FxHashMap<SmolStr, FxHashSet<SmolStr>>,
    pub funcs: FxHashMap<SmolStr, Func>,
    pub func_definitions: FxHashMap<SmolStr, Vec<Stmt>>,
    pub func_references: FxHashMap<SmolStr, References>,
    pub func_used_args: FxHashMap<SmolStr, FxHashSet<SmolStr>>,
    pub enums: FxHashMap<SmolStr, Enum>,
    pub structs: FxHashMap<SmolStr, Struct>,
    pub vars: FxHashMap<SmolStr, Var>,
    pub lists: FxHashMap<SmolStr, List>,
    pub events: Vec<Event>,
    pub used_procs: FxHashSet<SmolStr>,
    pub used_funcs: FxHashSet<SmolStr>,
    pub volume: Option<ConstExpr>,
    pub layer_order: Option<ConstExpr>,
    pub x_position: Option<ConstExpr>,
    pub y_position: Option<ConstExpr>,
    pub size: Option<ConstExpr>,
    pub direction: Option<ConstExpr>,
    pub rotation_style: RotationStyle,
    pub hidden: bool,
}
