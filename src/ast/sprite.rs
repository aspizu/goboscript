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
    pub costumes: Vec<Asset>,
    pub sounds: Vec<Asset>,
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
    pub x_position: Option<(Value, Span)>,
    pub y_position: Option<(Value, Span)>,
    pub size: Option<(Value, Span)>,
    pub direction: Option<(Value, Span)>,
    pub rotation_style: RotationStyle,
    pub hidden: bool,
}

impl Sprite {
    pub(crate) fn add_var(&mut self, var: Var, diagnostics: &mut Vec<Diagnostic>) {
        let name = var.name.clone();
        if self.vars.contains_key(&name) || self.lists.contains_key(&name) {
            diagnostics.push(Diagnostic {
                kind: DiagnosticKind::VariableRedefinition(name),
                span: var.span.clone(),
            });
            return;
        }
        self.vars.insert(name, var);
    }

    pub(crate) fn add_list(&mut self, list: List, diagnostics: &mut Vec<Diagnostic>) {
        let name = list.name.clone();
        if self.vars.contains_key(&name) || self.lists.contains_key(&name) {
            diagnostics.push(Diagnostic {
                kind: DiagnosticKind::ListRedefinition(name),
                span: list.span.clone(),
            });
            return;
        }
        self.lists.insert(name, list);
    }

    pub(crate) fn add_struct(&mut self, struct_: Struct, diagnostics: &mut Vec<Diagnostic>) {
        let name = struct_.name.clone();
        let mut fields = FxHashSet::default();
        for field in &struct_.fields {
            if !fields.insert(field.name.clone()) {
                diagnostics.push(Diagnostic {
                    kind: DiagnosticKind::DuplicateField {
                        struct_name: name,
                        field_name: field.name.clone(),
                    },
                    span: field.span.clone(),
                });
                return;
            }
        }
        if self.structs.contains_key(&name) {
            diagnostics.push(Diagnostic {
                kind: DiagnosticKind::StructRedefinition(name),
                span: struct_.span.clone(),
            });
            return;
        }
        self.structs.insert(name, struct_);
    }

    pub(crate) fn add_enum(&mut self, enum_: Enum, diagnostics: &mut Vec<Diagnostic>) {
        let name = enum_.name.clone();
        let mut variants = FxHashSet::default();
        for variant in &enum_.variants {
            if !variants.insert(variant.name.clone()) {
                diagnostics.push(Diagnostic {
                    kind: DiagnosticKind::DuplicateEnumVariant {
                        enum_name: name,
                        variant_name: variant.name.clone(),
                    },
                    span: variant.span.clone(),
                });
                return;
            }
        }
        if self.enums.contains_key(&name) {
            diagnostics.push(Diagnostic {
                kind: DiagnosticKind::EnumRedefinition(name),
                span: enum_.span.clone(),
            });
            return;
        }
        self.enums.insert(name, enum_);
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_and_list_share_declaration_namespace() {
        let mut sprite = Sprite::default();
        let mut diagnostics = Vec::new();
        sprite.add_var(
            Var {
                name: "x".into(),
                span: 0..1,
                type_: Type::Value,
                default: None,
                is_cloud: false,
                is_used: false,
            },
            &mut diagnostics,
        );
        sprite.add_list(List::new("x".into(), 2..3, Type::Value), &mut diagnostics);
        assert!(matches!(
            diagnostics.first().map(|diagnostic| &diagnostic.kind),
            Some(DiagnosticKind::ListRedefinition(name)) if name == "x"
        ));
    }
}
