use fxhash::{
    FxHashMap,
    FxHashSet,
};
use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::*;
use crate::{
    diagnostic::{
        Diagnostic,
        DiagnosticKind,
    },
    misc::SmolStr,
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Sprite {
    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
    pub procs: FxHashMap<SmolStr, Proc>,
    pub proc_definitions: FxHashMap<SmolStr, Vec<Stmt>>,
    pub proc_references: FxHashMap<SmolStr, References>,
    pub proc_args: FxHashMap<SmolStr, Vec<Arg>>,
    pub funcs: FxHashMap<SmolStr, Func>,
    pub func_definitions: FxHashMap<SmolStr, Vec<Stmt>>,
    pub func_references: FxHashMap<SmolStr, References>,
    pub func_args: FxHashMap<SmolStr, Vec<Arg>>,
    pub enums: FxHashMap<SmolStr, Enum>,
    pub structs: FxHashMap<SmolStr, Struct>,
    pub vars: FxHashMap<SmolStr, Var>,
    pub proc_locals: FxHashMap<SmolStr, FxHashMap<SmolStr, Var>>,
    pub func_locals: FxHashMap<SmolStr, FxHashMap<SmolStr, Var>>,
    pub lists: FxHashMap<SmolStr, List>,
    pub events: Vec<Event>,
    pub used_procs: FxHashSet<SmolStr>,
    pub used_funcs: FxHashSet<SmolStr>,
    pub volume: Option<(Value, Span)>,
    pub layer_order: Option<(Value, Span)>,
    pub x_position: Option<(Value, Span)>,
    pub y_position: Option<(Value, Span)>,
    pub size: Option<(Value, Span)>,
    pub direction: Option<(Value, Span)>,
    pub rotation_style: RotationStyle,
    pub hidden: bool,
}

impl Sprite {
    pub fn add_proc(
        &mut self,
        proc: Proc,
        args: Vec<Arg>,
        stmts: Vec<Stmt>,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let name = proc.name.clone();
        if self.procs.contains_key(&name) {
            diagnostics.push(Diagnostic {
                kind: DiagnosticKind::ProcedureRedefinition(name),
                span: proc.span.clone(),
            });
            return;
        }
        self.procs.insert(name.clone(), proc);
        self.proc_args.insert(name.clone(), args);
        self.proc_definitions.insert(name.clone(), stmts);
        self.proc_references.insert(name, Default::default());
    }

    pub fn add_func(
        &mut self,
        func: Func,
        args: Vec<Arg>,
        stmts: Vec<Stmt>,
        diagnostics: &mut Vec<Diagnostic>,
    ) {
        let name = func.name.clone();
        if self.funcs.contains_key(&name) {
            diagnostics.push(Diagnostic {
                kind: DiagnosticKind::FunctionRedefinition(name),
                span: func.span.clone(),
            });
            return;
        }
        self.funcs.insert(name.clone(), func);
        self.func_args.insert(name.clone(), args);
        self.func_definitions.insert(name.clone(), stmts);
        self.func_references.insert(name, Default::default());
    }
}
