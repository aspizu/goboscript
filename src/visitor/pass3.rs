use crate::ast::*;

struct S<'a> {
    references: &'a mut References,
    proc: Option<&'a Proc>,
    func: Option<&'a Func>,
}

pub fn visit_project(project: &mut Project) {
    visit_sprite(&mut project.stage);
    for sprite in project.sprites.values_mut() {
        visit_sprite(sprite);
    }
}

fn visit_sprite(sprite: &mut Sprite) {
    for proc in sprite.procs.values_mut() {
        let proc_definition = sprite.proc_definitions.get_mut(&proc.name).unwrap();
        let proc_references = sprite.proc_references.get_mut(&proc.name).unwrap();
        visit_stmts(
            proc_definition,
            &mut S {
                references: proc_references,
                proc: Some(proc),
                func: None,
            },
        );
    }
    for func in sprite.funcs.values() {
        let func_definition = sprite.func_definitions.get_mut(&func.name).unwrap();
        let func_references = sprite.func_references.get_mut(&func.name).unwrap();
        visit_stmts(
            func_definition,
            &mut S {
                references: func_references,
                proc: None,
                func: Some(func),
            },
        );
    }
    for event in &mut sprite.events {
        visit_stmts(
            &event.body,
            &mut S {
                references: &mut event.references,
                proc: None,
                func: None,
            },
        );
    }
}

fn visit_stmts(stmts: &[Stmt], s: &mut S) {
    for stmt in stmts {
        visit_stmt(stmt, s);
    }
}

fn visit_stmt(stmt: &Stmt, s: &mut S) {
    match stmt {
        Stmt::Repeat { times, body } => {
            visit_expr(times, s);
            visit_stmts(body, s);
        }
        Stmt::Forever { body, span: _ } => visit_stmts(body, s),
        Stmt::Branch {
            cond,
            if_body,
            else_body,
        } => {
            visit_expr(cond, s);
            visit_stmts(if_body, s);
            visit_stmts(else_body, s);
        }
        Stmt::Until { cond, body } => {
            visit_expr(cond, s);
            visit_stmts(body, s);
        }
        Stmt::SetVar {
            name: _,
            value,
            type_: _,
            is_local: _,
            is_cloud: _,
        } => {
            visit_expr(value, s);
        }
        Stmt::ChangeVar { name: _, value } => {
            visit_expr(value, s);
        }
        Stmt::Show(_name) => {}
        Stmt::Hide(_name) => {}
        Stmt::AddToList { name: _, value } => {
            visit_expr(value, s);
        }
        Stmt::DeleteList(_name) => {}
        Stmt::DeleteListIndex { name: _, index } => {
            visit_expr(index, s);
        }
        Stmt::InsertAtList {
            name: _,
            index,
            value,
        } => {
            visit_expr(index, s);
            visit_expr(value, s);
        }
        Stmt::SetListIndex {
            name: _,
            index,
            value,
        } => {
            visit_expr(index, s);
            visit_expr(value, s);
        }
        Stmt::Block {
            block: _,
            span: _,
            args,
            kwargs,
        } => {
            for arg in args {
                visit_expr(arg, s);
            }
            for (_, arg) in kwargs.values() {
                visit_expr(arg, s);
            }
        }
        Stmt::ProcCall {
            name,
            span: _,
            args,
            kwargs,
        } => {
            s.references.procs.insert(name.clone());
            for arg in args {
                visit_expr(arg, s);
            }
            for (_, arg) in kwargs.values() {
                visit_expr(arg, s);
            }
        }
        Stmt::FuncCall {
            name,
            span: _,
            args,
            kwargs,
        } => {
            s.references.funcs.insert(name.clone());
            for arg in args {
                visit_expr(arg, s);
            }
            for (_, arg) in kwargs.values() {
                visit_expr(arg, s);
            }
        }
        Stmt::Return {
            value: _,
            visited: _,
        } => {}
    }
}

fn visit_expr(expr: &Expr, s: &mut S) {
    match expr {
        Expr::Value { value: _, span: _ } => {}
        Expr::Name(name) => {
            let references = if name.is_generated() {
                &mut s.references.generated_names
            } else {
                &mut s.references.names
            };
            references.insert(NameReference {
                name: name.basename().clone(),
                field: name.fieldname().cloned(),
                proc: s.proc.map(|p| p.name.clone()),
                func: s.func.map(|f| f.name.clone()),
            });
        }
        Expr::Dot {
            lhs,
            rhs: _,
            rhs_span: _,
        } => visit_expr(lhs, s),
        Expr::Arg(name) => {
            s.references.args.insert(NameReference {
                name: name.basename().clone(),
                field: name.fieldname().cloned(),
                proc: s.proc.map(|p| p.name.clone()),
                func: s.func.map(|f| f.name.clone()),
            });
        }
        Expr::Repr {
            repr: _,
            span: _,
            args,
        } => {
            for arg in args {
                visit_expr(arg, s);
            }
        }
        Expr::FuncCall {
            name: _,
            span: _,
            args: _,
            kwargs: _,
        } => {}
        Expr::UnOp {
            op: _,
            span: _,
            opr,
        } => {
            visit_expr(opr, s);
        }
        Expr::BinOp {
            op: _,
            span: _,
            lhs,
            rhs,
        } => {
            visit_expr(lhs, s);
            visit_expr(rhs, s);
        }
        Expr::StructLiteral {
            name,
            span: _,
            fields,
        } => {
            s.references.structs.insert(name.clone());
            for field in fields {
                visit_expr(&field.value, s);
            }
        }
    }
}
