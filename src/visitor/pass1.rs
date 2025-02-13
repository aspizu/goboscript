use fxhash::{
    FxHashMap,
    FxHashSet,
};

use crate::{
    ast::*,
    misc::SmolStr,
};

struct S<'a> {
    references: &'a mut References,
    vars: &'a mut FxHashMap<SmolStr, Var>,
    callsites: &'a mut usize,
    funcs: &'a FxHashMap<SmolStr, Func>,
    func: Option<&'a Func>,
    proc: Option<&'a Proc>,
    used_args: Option<&'a mut FxHashSet<SmolStr>>,
}

pub fn visit_project(project: &mut Project) {
    let mut callsites = 0;
    visit_sprite(&mut project.stage, &mut callsites);
    for sprite in project.sprites.values_mut() {
        visit_sprite(sprite, &mut callsites);
    }
}

fn visit_sprite(sprite: &mut Sprite, callsites: &mut usize) {
    let old_callsites = *callsites;
    for proc in sprite.procs.values_mut() {
        let proc_definition = sprite.proc_definitions.get_mut(&proc.name).unwrap();
        let proc_references = sprite.proc_references.get_mut(&proc.name).unwrap();
        let used_args = sprite.proc_used_args.get_mut(&proc.name).unwrap();
        visit_stmts(
            proc_definition,
            &mut S {
                references: proc_references,
                vars: &mut sprite.vars,
                callsites,
                funcs: &sprite.funcs,
                func: None,
                proc: Some(proc),
                used_args: Some(used_args),
            },
        );
    }
    for func in sprite.funcs.values() {
        let func_definition = sprite.func_definitions.get_mut(&func.name).unwrap();
        let func_references = sprite.func_references.get_mut(&func.name).unwrap();
        let used_args = sprite.func_used_args.get_mut(&func.name).unwrap();
        visit_stmts(
            func_definition,
            &mut S {
                references: func_references,
                vars: &mut sprite.vars,
                callsites,
                funcs: &sprite.funcs,
                func: Some(func),
                proc: None,
                used_args: Some(used_args),
            },
        );
    }
    for event in &mut sprite.events {
        visit_stmts(
            &mut event.body,
            &mut S {
                references: &mut event.references,
                vars: &mut sprite.vars,
                callsites,
                funcs: &sprite.funcs,
                func: None,
                proc: None,
                used_args: None,
            },
        );
    }
    if *callsites != old_callsites {
        visit_sprite(sprite, callsites);
    }
}

fn visit_stmts(stmts: &mut Vec<Stmt>, s: &mut S) {
    let mut i = 0;
    while i < stmts.len() {
        let before = visit_stmt(&mut stmts[i], s);
        for stmt in before {
            stmts.insert(i, stmt);
            i += 1;
        }
        i += 1;
    }
}

fn visit_stmt(stmt: &mut Stmt, s: &mut S) -> Vec<Stmt> {
    let mut before = vec![];
    match stmt {
        Stmt::Repeat { times, body } => {
            visit_expr(times, &mut before, s);
            visit_stmts(body, s);
        }
        Stmt::ForEach { times, body, .. } => {
            visit_expr(times, &mut before, s);
            visit_stmts(body, s);
        }
        Stmt::For {
            name,
            value,
            type_,
            cond,
            incr,
            body,
        } => {
            before.push(Stmt::SetVar {
                name: name.clone(),
                value: value.clone(),
                type_: type_.clone(),
                is_local: false,
                is_cloud: false,
            });
            visit_expr(value, &mut before, s);
            visit_expr(cond, &mut before, s);
            visit_stmt(incr, s);
            visit_stmts(body, s);
        }
        Stmt::Forever { body, span: _ } => visit_stmts(body, s),
        Stmt::Branch {
            cond,
            if_body,
            else_body,
        } => {
            visit_expr(cond, &mut before, s);
            visit_stmts(if_body, s);
            visit_stmts(else_body, s);
        }
        Stmt::Until { cond, body } => {
            visit_expr(cond, &mut before, s);
            visit_stmts(body, s);
        }
        Stmt::SetVar {
            name: _,
            value,
            type_: _,
            is_local: _,
            is_cloud: _,
        } => {
            visit_expr(value, &mut before, s);
        }
        Stmt::ChangeVar { name: _, value } => {
            visit_expr(value, &mut before, s);
        }
        Stmt::Show(_name) => {}
        Stmt::Hide(_name) => {}
        Stmt::AddToList { name: _, value } => {
            visit_expr(value, &mut before, s);
        }
        Stmt::DeleteList(_name) => {}
        Stmt::DeleteListIndex { name: _, index } => {
            visit_expr(index, &mut before, s);
        }
        Stmt::InsertAtList {
            name: _,
            index,
            value,
        } => {
            visit_expr(index, &mut before, s);
            visit_expr(value, &mut before, s);
        }
        Stmt::SetListIndex {
            name: _,
            index,
            value,
        } => {
            visit_expr(index, &mut before, s);
            visit_expr(value, &mut before, s);
        }
        Stmt::Block {
            block: _,
            span: _,
            args,
        } => {
            for (_, arg) in args {
                visit_expr(arg, &mut before, s);
            }
        }
        Stmt::ProcCall {
            name,
            span: _,
            args,
        } => {
            s.references.procs.insert(name.clone());
            for (_, arg) in args {
                visit_expr(arg, &mut before, s);
            }
        }
        Stmt::FuncCall {
            name,
            span: _,
            args,
        } => {
            s.references.funcs.insert(name.clone());
            for (_, arg) in args {
                visit_expr(arg, &mut before, s);
            }
        }
        Stmt::Return { value, visited } => {
            if !*visited {
                *visited = true;
                if let Some(func) = s.func {
                    before.push(Stmt::SetVar {
                        name: Name::Name {
                            name: format!("{}:return", func.name).into(),
                            span: 0..0,
                        },
                        value: value.clone(),
                        type_: Type::Value,
                        is_local: false,
                        is_cloud: false,
                    })
                }
            }
            visit_expr(value, &mut before, s);
        }
    }
    before
}

fn visit_expr(expr: &mut Expr, before: &mut Vec<Stmt>, s: &mut S) {
    let replace: Option<Expr> = match expr {
        Expr::Value { value: _, span: _ } => None,
        Expr::Name(name) => {
            s.references.names.insert(name.basename().clone());
            None
        }
        Expr::Dot {
            lhs,
            rhs: _,
            rhs_span: _,
        } => {
            visit_expr(lhs, before, s);
            None
        }
        Expr::Arg(name) => {
            if let Some(used_args) = &mut s.used_args {
                used_args.insert(name.basename().clone());
            }
            None
        }
        Expr::Repr {
            repr: _,
            span: _,
            args,
        } => {
            for (_, arg) in args {
                visit_expr(arg, before, s);
            }
            None
        }
        Expr::FuncCall { name, span, args } => {
            if let Some(func) = s.funcs.get(name) {
                *s.callsites += 1;
                before.push(Stmt::FuncCall {
                    name: name.clone(),
                    span: span.clone(),
                    args: args.clone(),
                });
                let callsite = Name::Name {
                    name: format!("@{}", *s.callsites).into(),
                    span: span.clone(),
                };
                s.vars.insert(
                    callsite.basename().clone(),
                    Var {
                        name: callsite.basename().clone(),
                        span: callsite.basespan().clone(),
                        type_: func.type_.clone(),
                        is_cloud: false,
                        is_used: true,
                    },
                );
                before.push(Stmt::SetVar {
                    name: callsite.clone(),
                    value: Box::new(Expr::Name(Name::Name {
                        name: format!("{}:return", name).into(),
                        span: span.clone(),
                    })),
                    type_: Type::Value,
                    is_local: false,
                    is_cloud: false,
                });
                Some(Expr::Name(callsite))
            } else {
                None
            }
        }
        Expr::UnOp {
            op: _,
            span: _,
            opr,
        } => {
            visit_expr(opr, before, s);
            None
        }
        Expr::BinOp {
            op: _,
            span: _,
            lhs,
            rhs,
        } => {
            visit_expr(lhs, before, s);
            visit_expr(rhs, before, s);
            None
        }
        Expr::StructLiteral {
            name,
            span: _,
            fields,
        } => {
            s.references.structs.insert(name.clone());
            for field in fields {
                s.references
                    .struct_fields
                    .insert((name.clone(), field.name.clone()));
                visit_expr(&mut field.value, before, s);
            }
            None
        }
    };
    if let Some(replace) = replace {
        *expr = replace;
    }
}
