use crate::ast::{
    Expr,
    Project,
    Stmt,
};

pub fn visit_project(project: &mut Project) {
    visit_sprite(&mut project.stage);
    for sprite in project.sprites.values_mut() {
        visit_sprite(sprite);
    }
}

fn visit_sprite(sprite: &mut crate::ast::Sprite) {
    for body in sprite.proc_definitions.values_mut() {
        extract_ternary_from_stmts(body);
    }
    for body in sprite.func_definitions.values_mut() {
        extract_ternary_from_stmts(body);
    }
    for event in &mut sprite.events {
        extract_ternary_from_stmts(&mut event.body);
    }
}

pub fn extract_ternary_from_stmts(stmts: &mut [Stmt]) {
    for stmt in stmts {
        extract_ternary_from_stmt(stmt);
    }
}

fn extract_ternary_from_stmt(stmt: &mut Stmt) {
    while let Some(condition) = stmt_find_closest_ternary(stmt) {
        let condition = condition.clone();
        let mut tbranch = stmt.clone();
        let mut fbranch = stmt.clone();
        assert!(stmt_split_closest_ternary(&mut tbranch, true));
        assert!(stmt_split_closest_ternary(&mut fbranch, false));
        *stmt = Stmt::Branch {
            cond: Box::new(condition),
            if_body: vec![tbranch],
            else_body: vec![fbranch],
        }
    }
    match stmt {
        Stmt::Repeat { body, .. } => extract_ternary_from_stmts(body),
        Stmt::Forever { body, .. } => extract_ternary_from_stmts(body),
        Stmt::Branch {
            if_body, else_body, ..
        } => {
            extract_ternary_from_stmts(if_body);
            extract_ternary_from_stmts(else_body);
        }
        Stmt::Until { body, .. } => extract_ternary_from_stmts(body),
        _ => {}
    }
}

fn stmt_find_closest_ternary(stmt: &Stmt) -> Option<&Expr> {
    match stmt {
        Stmt::Forever { .. } => None,
        Stmt::Show(..) => None,
        Stmt::Hide(..) => None,
        Stmt::DeleteList(..) => None,
        Stmt::Repeat { times, .. } => expr_find_closest_ternary(times),
        Stmt::Branch { cond, .. } => expr_find_closest_ternary(cond),
        Stmt::Until { cond, .. } => expr_find_closest_ternary(cond),
        Stmt::SetVar { value, .. } => expr_find_closest_ternary(value),
        Stmt::ChangeVar { value, .. } => expr_find_closest_ternary(value),
        Stmt::AddToList { value, .. } => expr_find_closest_ternary(value),
        Stmt::DeleteListIndex { index, .. } => expr_find_closest_ternary(index),
        Stmt::InsertAtList { index, value, .. } => {
            expr_find_closest_ternary(index).or_else(|| expr_find_closest_ternary(value))
        }
        Stmt::SetListIndex { index, value, .. } => {
            expr_find_closest_ternary(index).or_else(|| expr_find_closest_ternary(value))
        }
        Stmt::Block { args, kwargs, .. } => {
            for arg in args {
                if let Some(found) = expr_find_closest_ternary(arg) {
                    return Some(found);
                }
            }
            for (_, kwarg) in kwargs.values() {
                if let Some(found) = expr_find_closest_ternary(kwarg) {
                    return Some(found);
                }
            }
            return None;
        }
        Stmt::ProcCall { args, kwargs, .. } => {
            for arg in args {
                if let Some(found) = expr_find_closest_ternary(arg) {
                    return Some(found);
                }
            }
            for (_, kwarg) in kwargs.values() {
                if let Some(found) = expr_find_closest_ternary(kwarg) {
                    return Some(found);
                }
            }
            return None;
        }
        Stmt::FuncCall { args, kwargs, .. } => {
            for arg in args {
                if let Some(found) = expr_find_closest_ternary(arg) {
                    return Some(found);
                }
            }
            for (_, kwarg) in kwargs.values() {
                if let Some(found) = expr_find_closest_ternary(kwarg) {
                    return Some(found);
                }
            }
            return None;
        }
        Stmt::Return { value, .. } => expr_find_closest_ternary(value),
    }
}

fn expr_find_closest_ternary(expr: &Expr) -> Option<&Expr> {
    match expr {
        Expr::Value { .. } => None,
        Expr::Name(..) => None,
        Expr::Arg(..) => None,
        Expr::Dot { lhs, .. } => expr_find_closest_ternary(lhs),
        Expr::Repr { args, .. } => {
            for arg in args {
                if let Some(found) = expr_find_closest_ternary(arg) {
                    return Some(found);
                }
            }
            return None;
        }
        Expr::FuncCall { args, kwargs, .. } => {
            for arg in args {
                if let Some(found) = expr_find_closest_ternary(arg) {
                    return Some(found);
                }
            }
            for (_, kwarg) in kwargs.values() {
                if let Some(found) = expr_find_closest_ternary(kwarg) {
                    return Some(found);
                }
            }
            return None;
        }
        Expr::UnOp { opr, .. } => expr_find_closest_ternary(opr),
        Expr::BinOp { lhs, rhs, .. } => {
            expr_find_closest_ternary(lhs).or_else(|| expr_find_closest_ternary(rhs))
        }
        Expr::StructLiteral { fields, .. } => {
            for field in fields {
                if let Some(found) = expr_find_closest_ternary(&field.value) {
                    return Some(found);
                }
            }
            return None;
        }
        Expr::Property { object, .. } => expr_find_closest_ternary(object),
        Expr::Ternary { condition, .. } => Some(condition),
    }
}

fn stmt_split_closest_ternary(stmt: &mut Stmt, condition: bool) -> bool {
    match stmt {
        Stmt::Forever { .. } => false,
        Stmt::Show(..) => false,
        Stmt::Hide(..) => false,
        Stmt::DeleteList(..) => false,
        Stmt::Repeat { times, .. } => expr_split_closest_ternary(times, condition),
        Stmt::Branch { cond, .. } => expr_split_closest_ternary(cond, condition),
        Stmt::Until { cond, .. } => expr_split_closest_ternary(cond, condition),
        Stmt::SetVar { value, .. } => expr_split_closest_ternary(value, condition),
        Stmt::ChangeVar { value, .. } => expr_split_closest_ternary(value, condition),
        Stmt::AddToList { value, .. } => expr_split_closest_ternary(value, condition),
        Stmt::DeleteListIndex { index, .. } => expr_split_closest_ternary(index, condition),
        Stmt::InsertAtList { index, value, .. } => {
            expr_split_closest_ternary(index, condition)
                || expr_split_closest_ternary(value, condition)
        }
        Stmt::SetListIndex { index, value, .. } => {
            expr_split_closest_ternary(index, condition)
                || expr_split_closest_ternary(value, condition)
        }
        Stmt::Block { args, kwargs, .. } => {
            for arg in args {
                if expr_split_closest_ternary(arg, condition) {
                    return true;
                }
            }
            for (_, kwarg) in kwargs.values_mut() {
                if expr_split_closest_ternary(kwarg, condition) {
                    return true;
                }
            }
            false
        }
        Stmt::ProcCall { args, kwargs, .. } => {
            for arg in args {
                if expr_split_closest_ternary(arg, condition) {
                    return true;
                }
            }
            for (_, kwarg) in kwargs.values_mut() {
                if expr_split_closest_ternary(kwarg, condition) {
                    return true;
                }
            }
            false
        }
        Stmt::FuncCall { args, kwargs, .. } => {
            for arg in args {
                if expr_split_closest_ternary(arg, condition) {
                    return true;
                }
            }
            for (_, kwarg) in kwargs.values_mut() {
                if expr_split_closest_ternary(kwarg, condition) {
                    return true;
                }
            }
            false
        }
        Stmt::Return { value, .. } => expr_split_closest_ternary(value, condition),
    }
}

fn expr_split_closest_ternary(expr: &mut Expr, condition: bool) -> bool {
    let mut replace: Option<Expr> = None;
    match expr {
        Expr::Value { .. } => {}
        Expr::Name(..) => {}
        Expr::Arg(..) => {}
        Expr::Dot { lhs, .. } => return expr_split_closest_ternary(lhs, condition),
        Expr::Repr { args, .. } => {
            for arg in args {
                if expr_split_closest_ternary(arg, condition) {
                    return true;
                }
            }
            return false;
        }
        Expr::FuncCall { args, kwargs, .. } => {
            for arg in args {
                if expr_split_closest_ternary(arg, condition) {
                    return true;
                }
            }
            for (_, kwarg) in kwargs.values_mut() {
                if expr_split_closest_ternary(kwarg, condition) {
                    return true;
                }
            }
            return false;
        }
        Expr::UnOp { opr, .. } => return expr_split_closest_ternary(opr, condition),
        Expr::BinOp { lhs, rhs, .. } => {
            return expr_split_closest_ternary(lhs, condition)
                || expr_split_closest_ternary(rhs, condition)
        }
        Expr::StructLiteral { fields, .. } => {
            for field in fields {
                if expr_split_closest_ternary(&mut field.value, condition) {
                    return true;
                }
            }
            return false;
        }
        Expr::Property { object, .. } => return expr_split_closest_ternary(object, condition),
        Expr::Ternary { tvalue, fvalue, .. } => {
            if condition {
                replace = Some(*tvalue.clone());
            } else {
                replace = Some(*fvalue.clone());
            }
        }
    };
    let result = replace.is_some();
    if let Some(replace) = replace {
        *expr = replace;
    }
    result
}
