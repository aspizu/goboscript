use crate::ast::*;

struct S<'a> {
    references: &'a mut References,
    args: Option<&'a mut Vec<Arg>>,
    callsites: &'a mut usize,
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
        visit_proc(proc, callsites);
    }
    for func in sprite.funcs.values_mut() {
        visit_func(func, callsites);
    }
    for event in &mut sprite.events {
        visit_event(event, callsites);
    }
    if *callsites != old_callsites {
        visit_sprite(sprite, callsites);
    }
}

fn visit_proc(proc: &mut Proc, callsites: &mut usize) {
    visit_stmts(
        &mut proc.body,
        &mut S {
            references: &mut proc.references,
            args: Some(&mut proc.args),
            callsites,
        },
    );
}

fn visit_func(func: &mut Func, callsites: &mut usize) {
    visit_stmts(
        &mut func.body,
        &mut S {
            references: &mut func.references,
            args: Some(&mut func.args),
            callsites,
        },
    );
}

fn visit_event(event: &mut Event, callsites: &mut usize) {
    visit_stmts(
        &mut event.body,
        &mut S {
            references: &mut event.references,
            args: None,
            callsites,
        },
    );
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
        Stmt::SetCallSite { id: _, func: _ } => {}
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
            for arg in args {
                visit_expr(arg, &mut before, s);
            }
        }
        Stmt::ProcCall {
            name,
            span: _,
            args,
        } => {
            s.references.procs.insert(name.clone());
            for arg in args {
                visit_expr(arg, &mut before, s);
            }
        }
        Stmt::FuncCall {
            name,
            span: _,
            args,
        } => {
            s.references.funcs.insert(name.clone());
            for arg in args {
                visit_expr(arg, &mut before, s);
            }
        }
        Stmt::Return { value } => {
            visit_expr(value, &mut before, s);
        }
    }
    before
}

fn visit_expr(expr: &mut Expr, before: &mut Vec<Stmt>, s: &mut S) {
    let replace: Option<Expr> = match expr {
        Expr::CallSite { id: _ } => None,
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
            if let Some(args) = &mut s.args {
                if let Some(arg) = args.iter_mut().find(|arg| &arg.name == name.basename()) {
                    arg.is_used = true;
                }
            }
            None
        }
        Expr::Repr {
            repr: _,
            span: _,
            args,
        } => {
            for arg in args {
                visit_expr(arg, before, s);
            }
            None
        }
        Expr::FuncCall { name, span, args } => {
            *s.callsites += 1;
            before.push(Stmt::FuncCall {
                name: name.clone(),
                span: span.clone(),
                args: args.clone(),
            });
            before.push(Stmt::SetCallSite {
                id: *s.callsites,
                func: name.clone(),
            });
            Some(Expr::CallSite { id: *s.callsites })
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
