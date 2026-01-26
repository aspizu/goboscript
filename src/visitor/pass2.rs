use fxhash::FxHashMap;
use logos::Span;

use super::transformations::{
    self,
    keyword_arguments,
};
use crate::{
    ast::*,
    blocks::{
        Block,
        Repr,
    },
    codegen::sb3::D,
    diagnostic::{
        keys,
        DiagnosticKind,
        SpriteDiagnostics,
    },
    misc::SmolStr,
    visitor::switchcase::switchcase,
};

#[derive(Copy, Clone)]
pub struct S<'a> {
    pub func_args: &'a FxHashMap<SmolStr, Vec<Arg>>,
    pub proc_args: &'a FxHashMap<SmolStr, Vec<Arg>>,
    pub args: Option<&'a Vec<Arg>>,
    pub local_vars: Option<&'a FxHashMap<SmolStr, Var>>,
    pub vars: &'a FxHashMap<SmolStr, Var>,
    pub lists: &'a FxHashMap<SmolStr, List>,
    pub enums: &'a FxHashMap<SmolStr, Enum>,
    pub structs: &'a FxHashMap<SmolStr, Struct>,
    pub procs: &'a FxHashMap<SmolStr, Proc>,
    pub funcs: &'a FxHashMap<SmolStr, Func>,
    pub global_vars: Option<&'a FxHashMap<SmolStr, Var>>,
    pub global_lists: Option<&'a FxHashMap<SmolStr, List>>,
    pub global_enums: Option<&'a FxHashMap<SmolStr, Enum>>,
    pub global_structs: Option<&'a FxHashMap<SmolStr, Struct>>,
}

impl S<'_> {
    pub fn get_var(&self, name: &str) -> Option<&Var> {
        self.local_vars
            .and_then(|local_vars| local_vars.get(name))
            .or_else(|| self.vars.get(name))
            .or_else(|| {
                self.global_vars
                    .and_then(|global_vars| global_vars.get(name))
            })
    }

    pub fn get_list(&self, name: &str) -> Option<&List> {
        self.lists.get(name).or_else(|| {
            self.global_lists
                .and_then(|global_lists| global_lists.get(name))
        })
    }

    pub fn get_struct(&self, name: &str) -> Option<&Struct> {
        self.structs.get(name).or_else(|| {
            self.global_structs
                .and_then(|global_structs| global_structs.get(name))
        })
    }

    pub fn get_enum(&self, name: &str) -> Option<&Enum> {
        self.enums.get(name).or_else(|| {
            self.global_enums
                .and_then(|global_enums| global_enums.get(name))
        })
    }
}

pub fn visit_project(
    project: &mut Project,
    stage_diagnostics: &mut SpriteDiagnostics,
    sprites_diagnostics: &mut FxHashMap<SmolStr, SpriteDiagnostics>,
) {
    visit_sprite(&mut project.stage, None, stage_diagnostics);
    for (sprite_name, sprite) in &mut project.sprites {
        visit_sprite(
            sprite,
            Some(&project.stage),
            sprites_diagnostics.get_mut(sprite_name).unwrap(),
        );
    }
}

fn visit_sprite(sprite: &mut Sprite, stage: Option<&Sprite>, d: D) {
    for proc in sprite.procs.values() {
        let proc_definition = sprite.proc_definitions.get_mut(&proc.name).unwrap();
        visit_stmts(
            proc_definition,
            S {
                proc_args: &sprite.proc_args,
                func_args: &sprite.func_args,
                args: sprite.proc_args.get(&proc.name),
                local_vars: Some(&sprite.proc_locals[&proc.name]),
                vars: &sprite.vars,
                lists: &sprite.lists,
                enums: &sprite.enums,
                structs: &sprite.structs,
                procs: &sprite.procs,
                funcs: &sprite.funcs,
                global_vars: stage.map(|stage| &stage.vars),
                global_lists: stage.map(|stage| &stage.lists),
                global_enums: stage.map(|stage| &stage.enums),
                global_structs: stage.map(|stage| &stage.structs),
            },
            d,
            true,
        );
    }
    for func in sprite.funcs.values() {
        let func_definition = sprite.func_definitions.get_mut(&func.name).unwrap();
        visit_stmts(
            func_definition,
            S {
                proc_args: &sprite.proc_args,
                func_args: &sprite.func_args,
                args: sprite.func_args.get(&func.name),
                local_vars: Some(&sprite.func_locals[&func.name]),
                vars: &sprite.vars,
                lists: &sprite.lists,
                enums: &sprite.enums,
                structs: &sprite.structs,
                procs: &sprite.procs,
                funcs: &sprite.funcs,
                global_vars: stage.map(|stage| &stage.vars),
                global_lists: stage.map(|stage| &stage.lists),
                global_enums: stage.map(|stage| &stage.enums),
                global_structs: stage.map(|stage| &stage.structs),
            },
            d,
            true,
        );
    }
    for event in &mut sprite.events {
        visit_stmts(
            &mut event.body,
            S {
                proc_args: &sprite.proc_args,
                func_args: &sprite.func_args,
                args: None,
                local_vars: None,
                vars: &sprite.vars,
                lists: &sprite.lists,
                enums: &sprite.enums,
                structs: &sprite.structs,
                procs: &sprite.procs,
                funcs: &sprite.funcs,
                global_vars: stage.map(|stage| &stage.vars),
                global_lists: stage.map(|stage| &stage.lists),
                global_enums: stage.map(|stage| &stage.enums),
                global_structs: stage.map(|stage| &stage.structs),
            },
            d,
            true,
        );
    }
}

fn visit_stmts(stmts: &mut Vec<Stmt>, s: S, d: D, top_level: bool) {
    for stmt in &mut *stmts {
        visit_stmt(stmt, s, d);
    }
    let mut i = 0;
    while i < stmts.len() {
        let replace = match &stmts[i] {
            Stmt::SetVar {
                name,
                value,
                type_,
                is_local,
                is_cloud,
            } => visit_stmt_set_var(s, d, name, value, type_, is_local, is_cloud),
            Stmt::SetListIndex { name, index, value } => {
                visit_stmt_list_set(s, d, name, index, value)
            }
            Stmt::AddToList { name, value } => visit_stmt_list_add(s, d, name, value),
            Stmt::DeleteList(name) => visit_stmt_delete_list(s, name),
            Stmt::DeleteListIndex { name, index } => {
                visit_stmt_delete_list_index(s, d, name, index)
            }
            Stmt::InsertAtList { name, index, value } => {
                visit_stmt_insert_at_list(s, d, name, index, value)
            }
            Stmt::Return { value, .. } => {
                // Don't add stop_this_script after return stmt if it's the last stmt.
                if top_level && i == stmts.len() - 1 {
                    Some(vec![])
                } else {
                    visit_stmt_return(value)
                }
            }
            Stmt::Switch { value, cases, span } => Some(vec![switchcase(value, cases, span, d)]),
            _ => None,
        };
        if let Some(replace) = replace {
            let len = replace.len();
            stmts.remove(i);
            for replace in replace.into_iter().rev() {
                stmts.insert(i, replace);
            }
            i += len.saturating_sub(1);
        }
        i += 1;
    }
}

fn visit_stmt(stmt: &mut Stmt, s: S, d: D) {
    match stmt {
        Stmt::Repeat { times, body } => {
            visit_expr(times, s, d);
            visit_stmts(body, s, d, false);
        }
        Stmt::Forever { body, span: _ } => {
            visit_stmts(body, s, d, false);
        }
        Stmt::Branch {
            cond,
            if_body,
            else_body,
        } => {
            visit_expr(cond, s, d);
            visit_stmts(if_body, s, d, false);
            visit_stmts(else_body, s, d, false);
        }
        Stmt::Until { cond, body } => {
            visit_expr(cond, s, d);
            visit_stmts(body, s, d, false);
        }
        Stmt::SetVar {
            name: _,
            value,
            type_: _,
            is_local: _,
            is_cloud: _,
        } => {
            visit_expr(value, s, d);
        }
        Stmt::ChangeVar { name: _, value } => {
            visit_expr(value, s, d);
        }
        Stmt::Show(_) => {}
        Stmt::Hide(_) => {}
        Stmt::AddToList { name: _, value } => {
            visit_expr(value, s, d);
        }
        Stmt::DeleteList(_) => {}
        Stmt::DeleteListIndex { name: _, index } => {
            visit_expr(index, s, d);
        }
        Stmt::InsertAtList {
            name: _,
            index,
            value,
        } => {
            visit_expr(value, s, d);
            visit_expr(index, s, d);
        }
        Stmt::SetListIndex {
            name: _,
            index,
            value,
        } => {
            visit_expr(value, s, d);
            visit_expr(index, s, d);
        }

        Stmt::Block {
            block: _,
            span: _,
            args,
            kwargs,
        } => {
            for arg in args {
                visit_expr(arg, s, d);
            }
            for (_, arg) in kwargs.values_mut() {
                visit_expr(arg, s, d);
            }
        }
        Stmt::ProcCall {
            name,
            span: _,
            args,
            kwargs,
        } => {
            keyword_arguments(s.proc_args.get(name), args, kwargs, d);
            for arg in args {
                visit_expr(arg, s, d);
            }
            for (_, arg) in kwargs.values_mut() {
                visit_expr(arg, s, d);
            }
        }
        Stmt::FuncCall {
            name,
            span: _,
            args,
            kwargs,
        } => {
            keyword_arguments(s.func_args.get(name), args, kwargs, d);
            for arg in args {
                visit_expr(arg, s, d);
            }
            for (_, arg) in kwargs.values_mut() {
                visit_expr(arg, s, d);
            }
        }
        Stmt::Return { value, .. } => visit_expr(value, s, d),
        Stmt::Switch {
            value,
            cases,
            span: _,
        } => {
            visit_expr(value, s, d);
            for case in cases {
                visit_expr(&mut case.value, s, d);
                visit_stmts(&mut case.body, s, d, false);
            }
        }
    }
}

fn visit_expr(expr: &mut Expr, s: S, d: D) {
    match expr {
        Expr::Value { value: _, span: _ } => {}
        Expr::Name(_) => {}
        Expr::Arg(_) => {}
        Expr::Dot {
            lhs,
            rhs: _,
            rhs_span: _,
        } => {
            visit_expr(lhs, s, d);
        }
        Expr::Repr {
            repr,
            span: _,
            args,
        } => {
            if let Repr::KeyPressed = repr {
                if let Some(Expr::Value {
                    value: Value::String(keyname),
                    span: keyname_span,
                }) = args.first()
                {
                    if !keys::is_key(keyname) {
                        d.report(
                            DiagnosticKind::UnrecognizedKey(keyname.clone()),
                            keyname_span,
                        );
                    }
                }
            }
            for arg in args {
                visit_expr(arg, s, d);
            }
        }
        Expr::FuncCall {
            name,
            span: _,
            args,
            kwargs,
        } => {
            keyword_arguments(s.func_args.get(name), args, kwargs, d);
            for arg in args {
                visit_expr(arg, s, d);
            }
            for (_, arg) in kwargs.values_mut() {
                visit_expr(arg, s, d);
            }
        }
        Expr::UnOp { opr, .. } => {
            visit_expr(opr, s, d);
        }
        Expr::BinOp { lhs, rhs, .. } => {
            visit_expr(lhs, s, d);
            visit_expr(rhs, s, d);
        }
        Expr::StructLiteral { name, fields, span } => {
            struct_literal(s, d, name, span, fields);
            for field in fields {
                visit_expr(&mut field.value, s, d);
            }
        }
        Expr::Property { object, .. } => {
            visit_expr(object, s, d);
        }
    }
    transformations::apply(expr, transformations::minus);
    transformations::apply(expr, transformations::less_than_equal);
    transformations::apply(expr, transformations::greater_than_equal);
    transformations::apply(expr, transformations::not_equal);
    transformations::apply(expr, transformations::floor_div);
    transformations::apply(expr, transformations::add_zero_left);
    transformations::apply(expr, transformations::add_zero_right);
    transformations::apply(expr, transformations::sub_zero);
    transformations::apply(expr, transformations::mul_one_left);
    transformations::apply(expr, transformations::mul_one_right);
    transformations::apply(expr, transformations::div_one);
    transformations::apply(expr, transformations::mul_zero_left);
    transformations::apply(expr, transformations::mul_zero_right);
    transformations::apply(expr, transformations::join_empty_left);
    transformations::apply(expr, transformations::join_empty_right);
    transformations::apply(expr, transformations::bin_op);
    transformations::apply(expr, transformations::un_op);
    transformations::apply(expr, |expr| transformations::variable_field_access(expr, s));
    transformations::apply(expr, |expr| transformations::enum_field_access(expr, s));
    transformations::apply(expr, |expr| transformations::arg_field_access(expr, s));
    transformations::apply(expr, |expr| transformations::list_field_access(expr, s));
    transformations::apply(expr, |expr| {
        transformations::struct_literal_field_access(expr, d)
    });
}

fn visit_stmt_set_var(
    s: S,
    d: D,
    name: &Name,
    value: &Expr,
    _type: &Type,
    is_local: &bool,
    is_cloud: &bool,
) -> Option<Vec<Stmt>> {
    let expr = value;
    let struct_literal_fields = get_struct_literal_for_type(s, d, name, expr, |basename| {
        s.get_var(basename).map(|var| &var.type_)
    })?;
    Some(
        struct_literal_fields
            .iter()
            .map(|struct_literal_field| Stmt::SetVar {
                name: Name::DotName {
                    lhs: name.basename().clone(),
                    lhs_span: name.basespan().clone(),
                    rhs: struct_literal_field.name.clone(),
                    rhs_span: struct_literal_field.span.clone(),
                    is_generated: true,
                },
                value: struct_literal_field.value.clone(),
                type_: Type::Value,
                is_local: *is_local,
                is_cloud: *is_cloud,
            })
            .collect(),
    )
}

fn visit_stmt_list_set(s: S, d: D, name: &Name, index: &Expr, value: &Expr) -> Option<Vec<Stmt>> {
    let expr = value;
    let struct_literal_fields = get_struct_literal_for_type(s, d, name, expr, |basename| {
        s.get_list(basename).map(|list| &list.type_)
    })?;
    Some(
        struct_literal_fields
            .iter()
            .map(|struct_literal_field| Stmt::SetListIndex {
                name: Name::DotName {
                    lhs: name.basename().clone(),
                    lhs_span: name.basespan().clone(),
                    rhs: struct_literal_field.name.clone(),
                    rhs_span: struct_literal_field.span.clone(),
                    is_generated: true,
                },
                index: Box::new(index.clone()),
                value: struct_literal_field.value.clone(),
            })
            .collect(),
    )
}

fn visit_stmt_list_add(s: S, d: D, name: &Name, value: &Expr) -> Option<Vec<Stmt>> {
    let expr = value;
    let struct_literal_fields = get_struct_literal_for_type(s, d, name, expr, |basename| {
        s.get_list(basename).map(|list| &list.type_)
    })?;
    Some(
        struct_literal_fields
            .iter()
            .map(|struct_literal_field| Stmt::AddToList {
                name: Name::DotName {
                    lhs: name.basename().clone(),
                    lhs_span: name.basespan().clone(),
                    rhs: struct_literal_field.name.clone(),
                    rhs_span: struct_literal_field.span.clone(),
                    is_generated: true,
                },
                value: struct_literal_field.value.clone(),
            })
            .collect(),
    )
}

fn visit_stmt_delete_list(s: S, name: &Name) -> Option<Vec<Stmt>> {
    if name.fieldname().is_some() {
        return None;
    }
    let basename = name.basename();
    let type_ = &s.get_list(basename)?.type_;
    let (type_name, _) = type_.struct_()?;
    let struct_ = s.get_struct(type_name)?;
    Some(
        struct_
            .fields
            .iter()
            .map(|struct_field| {
                Stmt::DeleteList(Name::DotName {
                    lhs: name.basename().clone(),
                    lhs_span: name.basespan().clone(),
                    rhs: struct_field.name.clone(),
                    rhs_span: struct_field.span.clone(),
                    is_generated: true,
                })
            })
            .collect(),
    )
}

fn visit_stmt_insert_at_list(
    s: S,
    d: D,
    name: &Name,
    index: &Expr,
    value: &Expr,
) -> Option<Vec<Stmt>> {
    let expr = value;
    let struct_literal_fields = get_struct_literal_for_type(s, d, name, expr, |basename| {
        s.get_list(basename).map(|list| &list.type_)
    })?;
    Some(
        struct_literal_fields
            .iter()
            .map(|struct_literal_field| Stmt::InsertAtList {
                name: Name::DotName {
                    lhs: name.basename().clone(),
                    lhs_span: name.basespan().clone(),
                    rhs: struct_literal_field.name.clone(),
                    rhs_span: struct_literal_field.span.clone(),
                    is_generated: true,
                },
                index: Box::new(index.clone()),
                value: struct_literal_field.value.clone(),
            })
            .collect(),
    )
}

fn visit_stmt_delete_list_index(s: S, _d: D, name: &Name, index: &Expr) -> Option<Vec<Stmt>> {
    if name.fieldname().is_some() {
        return None;
    }
    let basename = name.basename();
    let type_ = &s.get_list(basename)?.type_;
    let (type_name, _) = type_.struct_()?;
    let struct_ = s.get_struct(type_name)?;
    Some(
        struct_
            .fields
            .iter()
            .map(|struct_field| Stmt::DeleteListIndex {
                name: Name::DotName {
                    lhs: name.basename().clone(),
                    lhs_span: name.basespan().clone(),
                    rhs: struct_field.name.clone(),
                    rhs_span: struct_field.span.clone(),
                    is_generated: true,
                },
                index: Box::new(index.clone()),
            })
            .collect(),
    )
}

fn get_struct_literal_for_type<'a, T>(
    s: S,
    d: D,
    name: &Name,
    expr: &'a Expr,
    get_type: T,
) -> Option<&'a [StructLiteralField]>
where
    T: FnOnce(&str) -> Option<&'a Type>,
{
    if name.fieldname().is_some() {
        return None;
    }
    let basename = name.basename();
    let basespan = name.basespan();
    let type_ = get_type(basename)?;
    let (type_name, type_span) = type_.struct_()?;
    let struct_ = s.get_struct(type_name)?;
    let Expr::StructLiteral {
        name: struct_literal_name,
        span: struct_literal_span,
        fields: struct_literal_fields,
    } = expr
    else {
        d.report(
            DiagnosticKind::TypeMismatch {
                expected: type_.clone(),
                given: Type::Value,
            },
            &basespan,
        );
        return None;
    };
    let Some(value_struct) = s.get_struct(struct_literal_name) else {
        d.report(
            DiagnosticKind::UnrecognizedStruct(struct_literal_name.clone()),
            struct_literal_span,
        );
        return None;
    };
    if struct_.name != value_struct.name {
        d.report(
            DiagnosticKind::TypeMismatch {
                expected: Type::Struct {
                    name: struct_.name.clone(),
                    span: type_span.clone(),
                },
                given: Type::Struct {
                    name: value_struct.name.clone(),
                    span: struct_literal_span.clone(),
                },
            },
            &basespan,
        );
        return None;
    }
    Some(struct_literal_fields)
}

fn visit_stmt_return(_value: &Expr) -> Option<Vec<Stmt>> {
    Some(vec![Stmt::Block {
        block: Block::StopThisScript,
        span: 0..0,
        args: vec![],
        kwargs: Default::default(),
    }])
}

fn struct_literal(s: S, d: D, name: &SmolStr, span: &Span, fields: &mut Vec<StructLiteralField>) {
    let Some(struct_) = s.get_struct(name) else {
        return;
    };
    let mut new_fields: Vec<StructLiteralField> = vec![];

    // First, add any provided fields in their original order
    let mut provided_field_names = std::collections::HashSet::new();
    for field in fields.iter() {
        provided_field_names.insert(&field.name);
        new_fields.push(field.clone());
    }

    // Then, add default values for missing fields
    for struct_field in &struct_.fields {
        if !provided_field_names.contains(&struct_field.name) {
            if let Some(default) = &struct_field.default {
                new_fields.push(StructLiteralField {
                    name: struct_field.name.clone(),
                    span: default.span(),
                    value: Box::new(default.clone().into()),
                });
            } else {
                d.report(
                    DiagnosticKind::MissingField {
                        struct_name: struct_.name.clone(),
                        field_name: struct_field.name.clone(),
                    },
                    span,
                );
            }
        }
    }

    *fields = new_fields;
}
