use fxhash::FxHashMap;
use log::info;
use logos::Span;
use smol_str::SmolStr;

use crate::{
    ast::*,
    blocks::{BinOp, UnOp},
    codegen::sb3::D,
    diagnostic::{DiagnosticKind, SpriteDiagnostics},
    misc::Rrc,
};

#[derive(Copy, Clone)]
struct S<'a> {
    args: Option<&'a Vec<Arg>>,
    local_vars: Option<&'a FxHashMap<SmolStr, Var>>,
    vars: &'a FxHashMap<SmolStr, Var>,
    lists: &'a FxHashMap<SmolStr, List>,
    enums: &'a FxHashMap<SmolStr, Enum>,
    structs: &'a FxHashMap<SmolStr, Struct>,
    global_vars: Option<&'a FxHashMap<SmolStr, Var>>,
    global_lists: Option<&'a FxHashMap<SmolStr, List>>,
    global_enums: Option<&'a FxHashMap<SmolStr, Enum>>,
    global_structs: Option<&'a FxHashMap<SmolStr, Struct>>,
}

impl<'a> S<'a> {
    fn get_var(&self, name: &str) -> Option<&Var> {
        self.local_vars
            .and_then(|local_vars| local_vars.get(name))
            .or_else(|| self.vars.get(name))
            .or_else(|| {
                self.global_vars
                    .and_then(|global_vars| global_vars.get(name))
            })
    }

    fn get_list(&self, name: &str) -> Option<&List> {
        self.lists.get(name).or_else(|| {
            self.global_lists
                .and_then(|global_lists| global_lists.get(name))
        })
    }

    fn get_struct(&self, name: &str) -> Option<&Struct> {
        self.structs.get(name).or_else(|| {
            self.global_structs
                .and_then(|global_structs| global_structs.get(name))
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
    for proc in sprite.procs.values_mut() {
        visit_stmts(
            &mut proc.body,
            S {
                args: Some(&proc.args),
                local_vars: Some(&proc.locals),
                vars: &sprite.vars,
                lists: &sprite.lists,
                enums: &sprite.enums,
                structs: &sprite.structs,
                global_vars: stage.map(|stage| &stage.vars),
                global_lists: stage.map(|stage| &stage.lists),
                global_enums: stage.map(|stage| &stage.enums),
                global_structs: stage.map(|stage| &stage.structs),
            },
            d,
        );
    }
    for event in &mut sprite.events {
        visit_stmts(
            &mut event.body,
            S {
                args: None,
                local_vars: None,
                vars: &sprite.vars,
                lists: &sprite.lists,
                enums: &sprite.enums,
                structs: &sprite.structs,
                global_vars: stage.map(|stage| &stage.vars),
                global_lists: stage.map(|stage| &stage.lists),
                global_enums: stage.map(|stage| &stage.enums),
                global_structs: stage.map(|stage| &stage.structs),
            },
            d,
        );
    }
}

fn visit_stmts(stmts: &mut Vec<Stmt>, s: S, d: D) {
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
            } => visit_stmt_set_var(s, d, name, value, type_, is_local),
            Stmt::SetListIndex { name, index, value } => {
                visit_stmt_list_set(s, d, name, index, value)
            }
            Stmt::AddToList { name, value } => visit_stmt_list_add(s, d, name, value),
            Stmt::DeleteList(name) => visit_stmt_delete_list(s, d, name),
            Stmt::DeleteListIndex { name, index } => {
                visit_stmt_delete_list_index(s, d, name, index)
            }
            Stmt::InsertAtList { name, index, value } => {
                visit_stmt_insert_at_list(s, d, name, index, value)
            }
            _ => None,
        };
        if let Some(replace) = replace {
            let len = replace.len();
            stmts.remove(i);
            for replace in replace.into_iter().rev() {
                stmts.insert(i, replace);
            }
            i += len - 1;
        }
        i += 1;
    }
}

fn visit_stmt(stmt: &mut Stmt, s: S, d: D) {
    match stmt {
        Stmt::Repeat { times, body } => {
            visit_expr(times, s, d);
            visit_stmts(body, s, d);
        }
        Stmt::Forever { body, span: _ } => {
            visit_stmts(body, s, d);
        }
        Stmt::Branch {
            cond,
            if_body,
            else_body,
        } => {
            visit_expr(cond, s, d);
            visit_stmts(if_body, s, d);
            visit_stmts(else_body, s, d);
        }
        Stmt::Until { cond, body } => {
            visit_expr(cond, s, d);
            visit_stmts(body, s, d);
        }
        Stmt::SetVar {
            name: _,
            value,
            type_: _,
            is_local: _,
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
        } => {
            for arg in args {
                visit_expr(arg, s, d);
            }
        }
        Stmt::ProcCall {
            name: _,
            span: _,
            args,
        } => {
            for arg in args {
                visit_expr(arg, s, d);
            }
        }
    }
}

fn visit_expr(expr: &mut Rrc<Expr>, s: S, d: D) {
    let replace: Option<Rrc<Expr>> = match &mut *expr.borrow_mut() {
        Expr::Value { value: _, span: _ } => None,
        Expr::Name(name) => visit_expr_name(s, name),
        Expr::Arg(name) => visit_expr_arg(s, name),
        Expr::Dot { lhs, rhs, rhs_span } => {
            visit_expr(lhs, s, d);
            visit_expr_dot(d, lhs, rhs, rhs_span)
        }
        Expr::Repr {
            repr: _,
            span: _,
            args,
        } => {
            for arg in args {
                visit_expr(arg, s, d);
            }
            None
        }
        Expr::UnOp {
            op: _,
            span: _,
            opr,
        } => {
            visit_expr(opr, s, d);
            None
        }
        Expr::BinOp { op, span, lhs, rhs } => {
            visit_expr(lhs, s, d);
            visit_expr(rhs, s, d);
            match op {
                BinOp::Add => None,
                BinOp::Sub => None,
                BinOp::Mul => None,
                BinOp::Div => None,
                BinOp::Mod => None,
                BinOp::Lt => None,
                BinOp::Gt => None,
                BinOp::Eq => None,
                BinOp::And => None,
                BinOp::Or => None,
                BinOp::Join => None,
                BinOp::In => None,
                BinOp::Of => visit_expr_bin_op_of(s, span, lhs, rhs),
                BinOp::Le => Some(
                    UnOp::Not
                        .to_expr(
                            span.clone(),
                            BinOp::Gt
                                .to_expr(span.clone(), lhs.clone(), rhs.clone())
                                .into(),
                        )
                        .into(),
                ),
                BinOp::Ge => Some(
                    UnOp::Not
                        .to_expr(
                            span.clone(),
                            BinOp::Lt
                                .to_expr(span.clone(), lhs.clone(), rhs.clone())
                                .into(),
                        )
                        .into(),
                ),
                BinOp::Ne => Some(
                    UnOp::Not
                        .to_expr(
                            span.clone(),
                            BinOp::Eq
                                .to_expr(span.clone(), lhs.clone(), rhs.clone())
                                .into(),
                        )
                        .into(),
                ),
                BinOp::FloorDiv => Some(
                    UnOp::Floor
                        .to_expr(
                            span.clone(),
                            BinOp::Div
                                .to_expr(span.clone(), lhs.clone(), rhs.clone())
                                .into(),
                        )
                        .into(),
                ),
            }
        }
        Expr::StructLiteral {
            name: _,
            span: _,
            fields,
        } => {
            for field in fields {
                visit_expr(&mut field.value, s, d);
            }
            None
        }
    };
    if let Some(replace) = replace {
        *expr = replace;
    }
}

fn visit_expr_name(s: S, name: &Name) -> Option<Rrc<Expr>> {
    info!(target: "pass1", "visit_expr_name {name:#?}");
    if name.fieldname().is_some() {
        return None;
    }
    let basename = name.basename();
    let span = name.span();
    let var = &s.get_var(basename)?;
    info!(target: "pass1", "var {var:#?}");
    let (type_name, type_span) = var.type_.struct_()?;
    info!(target: "pass1", "type_name {type_name:#?}");
    let struct_ = s.get_struct(type_name)?;
    info!(target: "pass1", "struct_ {struct_:#?}");
    Some(
        Expr::StructLiteral {
            name: type_name.clone(),
            span: type_span.clone(),
            fields: struct_
                .fields
                .iter()
                .map(|field| StructLiteralField {
                    name: field.name.clone(),
                    span: field.span.clone(),
                    value: Expr::Name(Name::DotName {
                        lhs: var.name.clone(),
                        lhs_span: span.clone(),
                        rhs: field.name.clone(),
                        rhs_span: field.span.clone(),
                    })
                    .into(),
                })
                .collect(),
        }
        .into(),
    )
}

fn visit_expr_arg(s: S, name: &Name) -> Option<Rrc<Expr>> {
    if name.fieldname().is_some() {
        return None;
    }
    let basename = name.basename();
    let span = name.span();
    let arg = s.args?.iter().find(|arg| &arg.name == basename)?;
    let (type_name, type_span) = arg.type_.struct_()?;
    let struct_ = s.get_struct(type_name)?;
    Some(
        Expr::StructLiteral {
            name: type_name.clone(),
            span: type_span.clone(),
            fields: struct_
                .fields
                .iter()
                .map(|field| StructLiteralField {
                    name: field.name.clone(),
                    span: field.span.clone(),
                    value: Expr::Arg(Name::DotName {
                        lhs: arg.name.clone(),
                        lhs_span: span.clone(),
                        rhs: field.name.clone(),
                        rhs_span: field.span.clone(),
                    })
                    .into(),
                })
                .collect(),
        }
        .into(),
    )
}

fn visit_expr_bin_op_of(s: S, span: &Span, lhs: &Rrc<Expr>, rhs: &Rrc<Expr>) -> Option<Rrc<Expr>> {
    let Expr::Name(Name::Name { name, span }) = &*lhs.borrow() else {
        return None;
    };
    let list = s.get_list(name)?;
    let (type_name, type_span) = list.type_.struct_()?;
    let struct_ = s.get_struct(type_name)?;
    Some(
        Expr::StructLiteral {
            name: type_name.clone(),
            span: type_span.clone(),
            fields: struct_
                .fields
                .iter()
                .map(|field| StructLiteralField {
                    name: field.name.clone(),
                    span: field.span.clone(),
                    value: BinOp::Of
                        .to_expr(
                            span.clone(),
                            Expr::Name(Name::DotName {
                                lhs: name.clone(),
                                lhs_span: span.clone(),
                                rhs: field.name.clone(),
                                rhs_span: field.span.clone(),
                            })
                            .into(),
                            rhs.clone(),
                        )
                        .into(),
                })
                .collect(),
        }
        .into(),
    )
}

fn visit_expr_dot(d: D, lhs: &Rrc<Expr>, rhs: &SmolStr, rhs_span: &Span) -> Option<Rrc<Expr>> {
    let Expr::StructLiteral {
        name: lhs_name,
        span: _,
        fields,
    } = &*lhs.borrow()
    else {
        return None;
    };
    let Some(field) = fields.iter().find(|field| &field.name == rhs) else {
        d.report(
            DiagnosticKind::StructDoesNotHaveField {
                type_name: lhs_name.clone(),
                field_name: rhs.clone(),
            },
            rhs_span,
        );
        return None;
    };
    Some(field.value.clone())
}

fn visit_stmt_set_var(
    s: S,
    d: D,
    name: &Name,
    value: &Rrc<Expr>,
    _type: &Type,
    _is_local: &bool,
) -> Option<Vec<Stmt>> {
    let expr = &*value.borrow();
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
                },
                value: struct_literal_field.value.clone(),
                type_: Type::Value,
                is_local: false,
            })
            .collect(),
    )
}

fn visit_stmt_list_set(
    s: S,
    d: D,
    name: &Name,
    index: &Rrc<Expr>,
    value: &Rrc<Expr>,
) -> Option<Vec<Stmt>> {
    let expr = &*value.borrow();
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
                },
                index: index.clone(),
                value: struct_literal_field.value.clone(),
            })
            .collect(),
    )
}

fn visit_stmt_list_add(s: S, d: D, name: &Name, value: &Rrc<Expr>) -> Option<Vec<Stmt>> {
    let expr = &*value.borrow();
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
                },
                value: struct_literal_field.value.clone(),
            })
            .collect(),
    )
}

fn visit_stmt_delete_list(s: S, d: D, name: &Name) -> Option<Vec<Stmt>> {
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
                })
            })
            .collect(),
    )
}

fn visit_stmt_insert_at_list(
    s: S,
    d: D,
    name: &Name,
    index: &Rrc<Expr>,
    value: &Rrc<Expr>,
) -> Option<Vec<Stmt>> {
    let expr = &*value.borrow();
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
                },
                index: index.clone(),
                value: struct_literal_field.value.clone(),
            })
            .collect(),
    )
}

fn visit_stmt_delete_list_index(s: S, _d: D, name: &Name, index: &Rrc<Expr>) -> Option<Vec<Stmt>> {
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
                },
                index: index.clone(),
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
