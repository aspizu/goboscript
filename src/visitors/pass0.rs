use fxhash::FxHashMap;
use smol_str::SmolStr;

use crate::ast::{Proc, Stmt, Var};

pub fn visit_proc(proc: &mut Proc) {
    visit_stmts(&mut proc.body, &mut proc.locals);
}

fn visit_stmts(stmts: &mut Vec<Stmt>, locals: &mut FxHashMap<SmolStr, Var>) {
    for stmt in stmts {
        visit_stmt(stmt, locals);
    }
}

fn visit_stmt(stmt: &mut Stmt, locals: &mut FxHashMap<SmolStr, Var>) {
    match stmt {
        Stmt::SetVar { name, span, value: _, is_local: true } => {
            locals.insert(name.clone(), Var::new(name.clone(), span.clone(), None));
        }
        Stmt::Until { body, .. }
        | Stmt::Forever { body, .. }
        | Stmt::Repeat { body, .. } => {
            visit_stmts(body, locals);
        }
        Stmt::Branch { if_body, else_body, .. } => {
            visit_stmts(if_body, locals);
            visit_stmts(else_body, locals);
        }
        _ => {}
    }
}
