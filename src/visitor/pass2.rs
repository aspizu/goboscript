use crate::{ast::*, misc::Rrc};

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
    visit_stmts(&mut proc.body, callsites);
}

fn visit_func(func: &mut Func, callsites: &mut usize) {
    visit_stmts(&mut func.body, callsites);
}

fn visit_event(event: &mut Event, callsites: &mut usize) {
    visit_stmts(&mut event.body, callsites);
}

fn visit_stmts(stmts: &mut Vec<Stmt>, callsites: &mut usize) {
    let mut i = 0;
    while i < stmts.len() {
        let before = visit_stmt(&mut stmts[i], callsites);
        for stmt in before {
            stmts.insert(i, stmt);
            i += 1;
        }
        i += 1;
    }
}

fn visit_stmt(stmt: &mut Stmt, callsites: &mut usize) -> Vec<Stmt> {
    let mut before = vec![];
    match stmt {
        Stmt::Repeat { times, body } => {
            visit_expr(times, &mut before, callsites);
            visit_stmts(body, callsites);
        }
        Stmt::Forever { body, span: _ } => visit_stmts(body, callsites),
        Stmt::Branch {
            cond,
            if_body,
            else_body,
        } => {
            visit_expr(cond, &mut before, callsites);
            visit_stmts(if_body, callsites);
            visit_stmts(else_body, callsites);
        }
        Stmt::Until { cond, body } => {
            visit_expr(cond, &mut before, callsites);
            visit_stmts(body, callsites);
        }
        Stmt::SetVar {
            name: _,
            value,
            type_: _,
            is_local: _,
            is_cloud: _,
        } => {
            visit_expr(value, &mut before, callsites);
        }
        Stmt::SetCallSite { id: _, func: _ } => {}
        Stmt::ChangeVar { name: _, value } => {
            visit_expr(value, &mut before, callsites);
        }
        Stmt::Show(_name) => {}
        Stmt::Hide(_name) => {}
        Stmt::AddToList { name: _, value } => {
            visit_expr(value, &mut before, callsites);
        }
        Stmt::DeleteList(_name) => {}
        Stmt::DeleteListIndex { name: _, index } => {
            visit_expr(index, &mut before, callsites);
        }
        Stmt::InsertAtList {
            name: _,
            index,
            value,
        } => {
            visit_expr(index, &mut before, callsites);
            visit_expr(value, &mut before, callsites);
        }
        Stmt::SetListIndex {
            name: _,
            index,
            value,
        } => {
            visit_expr(index, &mut before, callsites);
            visit_expr(value, &mut before, callsites);
        }
        Stmt::Block {
            block: _,
            span: _,
            args,
        } => {
            for kwarg in args {
                visit_expr(&mut kwarg.value, &mut before, callsites);
            }
        }
        Stmt::ProcCall {
            name: _,
            span: _,
            args,
        } => {
            for kwarg in args {
                visit_expr(&mut kwarg.value, &mut before, callsites);
            }
        }
        Stmt::FuncCall {
            name: _,
            span: _,
            args,
        } => {
            for arg in args {
                visit_expr(arg, &mut before, callsites);
            }
        }
        Stmt::Return { value } => {
            visit_expr(value, &mut before, callsites);
        }
    }
    before
}

fn visit_expr(expr: &mut Rrc<Expr>, before: &mut Vec<Stmt>, callsites: &mut usize) {
    let replace: Option<Rrc<Expr>> = match &mut *expr.borrow_mut() {
        Expr::CallSite { .. } => None,
        Expr::Value { value: _, span: _ } => None,
        Expr::Name(_name) => None,
        Expr::Dot {
            lhs,
            rhs: _,
            rhs_span: _,
        } => {
            visit_expr(lhs, before, callsites);
            None
        }
        Expr::Arg(_name) => None,
        Expr::Repr {
            repr: _,
            span: _,
            args,
        } => {
            for arg in args {
                visit_expr(arg, before, callsites);
            }
            None
        }
        Expr::FuncCall { name, span, args } => {
            *callsites += 1;
            before.push(Stmt::FuncCall {
                name: name.clone(),
                span: span.clone(),
                args: args.clone(),
            });
            before.push(Stmt::SetCallSite {
                id: *callsites,
                func: name.clone(),
            });
            Some(Expr::CallSite { id: *callsites }.into())
        }
        Expr::UnOp {
            op: _,
            span: _,
            opr,
        } => {
            visit_expr(opr, before, callsites);
            None
        }
        Expr::BinOp {
            op: _,
            span: _,
            lhs,
            rhs,
        } => {
            visit_expr(lhs, before, callsites);
            visit_expr(rhs, before, callsites);
            None
        }
        Expr::StructLiteral {
            name: _,
            span: _,
            fields,
        } => {
            for field in fields {
                visit_expr(&mut field.value, before, callsites);
            }
            None
        }
    };
    if let Some(replace) = replace {
        *expr = replace;
    }
}
