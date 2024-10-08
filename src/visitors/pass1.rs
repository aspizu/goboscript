use fxhash::FxHashMap;
use smol_str::SmolStr;

use super::pass0;
use crate::{
    ast::{
        Enum, Event, Expr, List, OnMessage, Proc, Project, References, Rrc, Sprite,
        Stmt, Var,
    },
    blocks::{BinOp, Block, UnOp},
};

struct V<'a> {
    references: &'a mut References,
    used_args: Option<&'a mut FxHashMap<SmolStr, bool>>,
}

struct S<'a> {
    vars: &'a FxHashMap<SmolStr, Var>,
    lists: &'a FxHashMap<SmolStr, List>,
    enums: &'a FxHashMap<SmolStr, Enum>,
    global_vars: Option<&'a FxHashMap<SmolStr, Var>>,
    global_lists: Option<&'a FxHashMap<SmolStr, List>>,
}

pub fn visit_project(project: &mut Project) {
    visit_sprite(&mut project.stage, None);
    for sprite in project.sprites.values_mut() {
        visit_sprite(sprite, Some(&project.stage));
    }
}

fn visit_sprite(sprite: &mut Sprite, stage: Option<&Sprite>) {
    let s = &mut S {
        vars: &sprite.vars,
        lists: &sprite.lists,
        enums: &sprite.enums,
        global_vars: stage.map(|s| &s.vars),
        global_lists: stage.map(|s| &s.lists),
    };
    for event in &mut sprite.events {
        visit_event(event, s);
    }
    for on_message in sprite.on_messages.values_mut() {
        visit_on_message(on_message, s);
    }
    for proc in sprite.procs.values_mut() {
        visit_proc(proc, s);
    }
}

fn visit_proc(proc: &mut Proc, s: &mut S<'_>) {
    pass0::visit_proc(proc);
    visit_stmts(
        &mut proc.body,
        &mut V {
            references: &mut proc.references,
            used_args: Some(&mut proc.used_args),
        },
        s,
    );
}

fn visit_event(event: &mut Event, s: &mut S<'_>) {
    visit_stmts(
        &mut event.body,
        &mut V { references: &mut event.references, used_args: None },
        s,
    );
}

fn visit_on_message(on_message: &mut OnMessage, s: &mut S<'_>) {
    on_message.references.messages.insert(on_message.message.clone());
    visit_stmts(
        &mut on_message.body,
        &mut V { references: &mut on_message.references, used_args: None },
        s,
    );
}

fn visit_stmts(stmts: &mut Vec<Stmt>, v: &mut V<'_>, s: &mut S<'_>) {
    for stmt in stmts {
        visit_stmt(stmt, v, s);
    }
}

fn visit_stmt(stmt: &mut Stmt, v: &mut V<'_>, s: &mut S<'_>) {
    match stmt {
        Stmt::Repeat { times, body } => {
            visit_expr(times, v, s);
            for stmt in body {
                visit_stmt(stmt, v, s);
            }
        }
        Stmt::Forever { body, span: _ } => {
            for stmt in body {
                visit_stmt(stmt, v, s);
            }
        }
        Stmt::Branch { cond, if_body, else_body } => {
            visit_expr(cond, v, s);
            for stmt in if_body {
                visit_stmt(stmt, v, s);
            }
            for stmt in else_body {
                visit_stmt(stmt, v, s);
            }
        }
        Stmt::Until { cond, body } => {
            visit_expr(cond, v, s);
            for stmt in body {
                visit_stmt(stmt, v, s);
            }
        }
        Stmt::SetVar { value, .. } => {
            // v.references.vars.insert(name.clone());
            // should set variable count as a reference?
            visit_expr(value, v, s);
        }
        Stmt::ChangeVar { value, .. } => {
            // v.references.vars.insert(name.clone());
            // should change variable count as a reference?
            visit_expr(value, v, s);
        }
        Stmt::Show { name: _, span: _ } => {}
        Stmt::Hide { name: _, span: _ } => {}
        Stmt::ListAdd { name, span: _, value } => {
            v.references.lists.insert(name.clone());
            visit_expr(value, v, s);
        }
        Stmt::ListDelete { name, span: _, index } => {
            v.references.lists.insert(name.clone());
            visit_expr(index, v, s);
        }
        Stmt::ListDeleteAll { name, span: _ } => {
            v.references.lists.insert(name.clone());
        }
        Stmt::ListInsert { name, span: _, index, value } => {
            v.references.lists.insert(name.clone());
            visit_expr(index, v, s);
            visit_expr(value, v, s);
        }
        Stmt::ListSet { name, span: _, index, value } => {
            v.references.lists.insert(name.clone());
            visit_expr(index, v, s);
            visit_expr(value, v, s);
        }
        Stmt::ListChange { op: _, name, span: _, index, value } => {
            v.references.lists.insert(name.clone());
            visit_expr(index, v, s);
            visit_expr(value, v, s);
        }
        Stmt::Block { block, span: _, args } => {
            // reference the broadcast if this is a broadcast block.
            // if the broadcast argument is an expression, mark shadowed_message.
            // if shadowed_message is true,
            // then later down the line the broadcast block's shadowed input will be set to
            // an arbitrary broadcast, or a placeholder "message1" if no broadcasts exist in the project.
            if let Block::Broadcast | Block::BroadcastAndWait = block {
                if let Expr::Str(broadcast_name) = &mut *args[0].borrow_mut() {
                    v.references.messages.insert(broadcast_name.clone());
                }
            }

            for arg in args {
                visit_expr(arg, v, s);
            }
        }
        Stmt::ProcCall { name, span: _, args } => {
            v.references.procs.insert(name.clone());
            for arg in args {
                visit_expr(arg, v, s);
            }
        }
    }
}

fn visit_expr(expr: &mut Rrc<Expr>, v: &mut V<'_>, s: &mut S<'_>) {
    let mut replace: Option<Rrc<Expr>> = None;
    match &mut *expr.borrow_mut() {
        Expr::Int(_) => {}
        Expr::Float(_) => {}
        Expr::Str(_) => {}
        Expr::EnumVariant { enum_name, variant_name, .. } => {
            if s.enums.contains_key(enum_name) {
                v.references
                    .enum_variants
                    .insert((enum_name.clone(), variant_name.clone()));
            }
        }
        Expr::Name { name, .. } => {
            if s.vars.contains_key(name)
                || s.global_vars.is_some_and(|it| it.contains_key(name))
            {
                v.references.vars.insert(name.clone());
            } else {
                v.references.lists.insert(name.clone());
            }
        }
        Expr::Arg { name, .. } => {
            if let Some(used_args) = &mut v.used_args {
                if let Some(arg) = used_args.get_mut(name) {
                    *arg = true;
                }
            }
        }
        Expr::Repr { args, .. } => {
            for arg in args {
                visit_expr(arg, v, s);
            }
        }
        Expr::UnOp { op, val } => {
            visit_expr(val, v, s);
            match op {
                UnOp::Minus => match &mut *val.borrow_mut() {
                    Expr::Int(value) => {
                        *value = -*value;
                        replace = Some(val.clone());
                    }
                    Expr::Float(value) => {
                        *value = -*value;
                        replace = Some(val.clone());
                    }
                    Expr::BinOp { op: BinOp::Sub, lhs, rhs }
                        if lhs.borrow().is_zero() =>
                    {
                        replace = Some(rhs.clone());
                    }
                    _ => {
                        replace = Some(
                            BinOp::Sub.to_expr(Expr::Int(0).into(), val.clone()).into(),
                        );
                    }
                },
                UnOp::Not => {
                    if let Expr::UnOp { op: UnOp::Not, val } = &mut *val.borrow_mut() {
                        replace = Some(val.clone());
                    }
                }
                _ => {}
            }
        }
        Expr::BinOp { op, lhs, rhs } => {
            visit_expr(lhs, v, s);
            visit_expr(rhs, v, s);
            match op {
                BinOp::Of => {
                    if let Expr::Name { name, .. } = &*lhs.borrow() {
                        if s.lists.contains_key(name)
                            || s.global_lists.is_some_and(|it| it.contains_key(name))
                        {
                            v.references.lists.insert(name.clone());
                        }
                    }
                }
                BinOp::Add => match (&mut *lhs.borrow_mut(), &mut *rhs.borrow_mut()) {
                    (Expr::Int(lval), Expr::Int(rval)) => {
                        *lval += *rval;
                        replace = Some(lhs.clone());
                    }
                    (Expr::Int(lval), Expr::Float(rval)) => {
                        *rval += *lval as f64;
                        replace = Some(rhs.clone());
                    }
                    (Expr::Float(lval), Expr::Float(rval)) => {
                        *lval += *rval;
                        replace = Some(lhs.clone());
                    }
                    (Expr::Float(lval), Expr::Int(rval)) => {
                        *lval += *rval as f64;
                        replace = Some(lhs.clone());
                    }
                    _ => {}
                },
                BinOp::Sub => match (&mut *lhs.borrow_mut(), &mut *rhs.borrow_mut()) {
                    (Expr::Int(lval), Expr::Int(rval)) => {
                        *lval -= *rval;
                        replace = Some(lhs.clone());
                    }
                    (Expr::Int(lval), Expr::Float(rval)) => {
                        *rval = *lval as f64 - *rval;
                        replace = Some(rhs.clone());
                    }
                    (Expr::Float(lval), Expr::Float(rval)) => {
                        *lval -= *rval;
                        replace = Some(lhs.clone());
                    }
                    (Expr::Float(lval), Expr::Int(rval)) => {
                        *lval -= *rval as f64;
                        replace = Some(lhs.clone());
                    }
                    _ => {}
                },
                BinOp::Mul => match (&mut *lhs.borrow_mut(), &mut *rhs.borrow_mut()) {
                    (Expr::Int(lval), Expr::Int(rval)) => {
                        *lval *= *rval;
                        replace = Some(lhs.clone());
                    }
                    (Expr::Int(lval), Expr::Float(rval)) => {
                        *rval *= *lval as f64;
                        replace = Some(rhs.clone());
                    }
                    (Expr::Float(lval), Expr::Float(rval)) => {
                        *lval *= *rval;
                        replace = Some(lhs.clone());
                    }
                    (Expr::Float(lval), Expr::Int(rval)) => {
                        *lval *= *rval as f64;
                        replace = Some(lhs.clone());
                    }
                    _ => {}
                },
                BinOp::Le => {
                    replace = Some(
                        UnOp::Not
                            .to_expr(BinOp::Lt.to_expr(rhs.clone(), lhs.clone()).into())
                            .into(),
                    )
                }
                BinOp::Ge => {
                    replace = Some(
                        UnOp::Not
                            .to_expr(BinOp::Gt.to_expr(rhs.clone(), lhs.clone()).into())
                            .into(),
                    )
                }
                BinOp::Ne => {
                    replace = Some(
                        UnOp::Not
                            .to_expr(BinOp::Eq.to_expr(lhs.clone(), rhs.clone()).into())
                            .into(),
                    )
                }
                BinOp::FloorDiv => {
                    replace = Some(
                        UnOp::Floor
                            .to_expr(
                                BinOp::Div.to_expr(lhs.clone(), rhs.clone()).into(),
                            )
                            .into(),
                    )
                }
                _ => {}
            }
        }
    }
    if let Some(replace) = replace {
        *expr = replace;
    }
}
