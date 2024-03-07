use std::collections::{HashMap, HashSet};

use crate::{
    ast::{rrc, BinaryOp, Declr, Declrs, Expr, Rrc, Stmt, Stmts, UnaryOp},
    build::FunctionPrototype,
    reporting::Report,
};

pub struct Visitor<'src, 'b> {
    pub variables: &'b mut HashSet<&'src str>,
    pub lists: &'b mut HashSet<&'src str>,
    pub functions: &'b mut HashMap<&'src str, FunctionPrototype<'src>>,
    pub reports: &'b mut Vec<Report<'src>>,
}

impl<'src, 'b> Visitor<'src, 'b> {
    pub fn visit_declrs(&mut self, declrs: &mut Declrs<'src>) {
        for declr in declrs.iter_mut() {
            self.visit_declr(declr);
        }
    }

    fn visit_declr(&mut self, declr: &mut Rrc<Declr<'src>>) {
        match &mut *declr.borrow_mut() {
            Declr::Costumes(_, _) => {}
            Declr::Sounds(_, _) => {}
            Declr::Def(function) => {
                self.visit_stmts(&mut function.body);
                self.functions.insert(
                    function.name,
                    FunctionPrototype {
                        args: function.args.clone(),
                        args_set: function.args.iter().map(|(arg, _)| *arg).collect(),
                        warp: function.warp,
                        span: function.span.clone(),
                    },
                );
            }
            Declr::OnFlag(body, span) => {
                self.visit_stmts(body);
            }
            Declr::OnKey(key, body, span) => {
                self.visit_stmts(body);
            }
            Declr::OnClick(body, span) => {
                self.visit_stmts(body);
            }
            Declr::OnBackdrop(backdrop, body, span) => {
                self.visit_stmts(body);
            }
            Declr::OnLoudnessGreaterThan(loudness, body, span) => {
                self.visit_expr(loudness);
                self.visit_stmts(body);
            }
            Declr::OnTimerGreaterThan(timer, body, span) => {
                self.visit_expr(timer);
                self.visit_stmts(body);
            }
            Declr::OnMessage(message, body, span) => {
                self.visit_stmts(body);
            }
            Declr::OnClone(body, span) => {
                self.visit_stmts(body);
            }
        }
    }

    fn visit_stmts(&mut self, stmts: &mut Stmts<'src>) {
        for stmt in stmts.iter_mut() {
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Rrc<Stmt<'src>>) {
        match &mut *stmt.borrow_mut() {
            Stmt::Repeat(times, body, span) => {
                self.visit_expr(times);
                self.visit_stmts(body);
            }
            Stmt::Forever(body, span) => {
                self.visit_stmts(body);
            }
            Stmt::Branch(condition, if_body, else_body, span) => {
                self.visit_expr(condition);
                self.visit_stmts(if_body);
                self.visit_stmts(else_body);
            }
            Stmt::Until(condition, body, span) => {
                self.visit_expr(condition);
                self.visit_stmts(body);
            }
            Stmt::SetVariable(name, value, span) => {
                self.variables.insert(name);
                self.visit_expr(value);
            }
            Stmt::ChangeVariable(name, value, span) => {
                self.visit_expr(value);
            }
            Stmt::Show(name, span) => todo!(),
            Stmt::Hide(name, span) => todo!(),
            Stmt::ListAdd(name, value, span) => {
                self.lists.insert(name);
                self.visit_expr(value);
            }
            Stmt::ListDelete(name, index, span) => {
                self.lists.insert(name);
                self.visit_expr(index);
            }
            Stmt::ListDeleteAll(name, span) => {
                self.lists.insert(name);
            }
            Stmt::ListInsert(name, index, value, span) => {
                self.lists.insert(name);
                self.visit_expr(index);
                self.visit_expr(value);
            }
            Stmt::ListReplace(name, index, value, span) => {
                self.lists.insert(name);
                self.visit_expr(index);
                self.visit_expr(value);
            }
            Stmt::Block(name, args, span) => {
                for arg in args {
                    self.visit_expr(arg);
                }
            }
            Stmt::Call(name, args, span) => {
                for arg in args {
                    self.visit_expr(arg);
                }
            }
        }
    }

    fn visit_expr(&mut self, expr: &mut Rrc<Expr<'src>>) {
        let mut replace = None;
        match &mut *expr.borrow_mut() {
            Expr::Int(value, span) => {}
            Expr::Float(value, span) => {}
            Expr::String(value, span) => {}
            Expr::Name(name, span) => {}
            Expr::Arg(name, span) => {}
            Expr::Reporter(reporter, args, span) => {
                for arg in args {
                    self.visit_expr(arg);
                }
            }
            Expr::UnaryOp(op, operand, span) => {
                self.visit_expr(operand);
                match op {
                    UnaryOp::Minus => match &mut *operand.borrow_mut() {
                        Expr::Int(value, _) => {
                            *value = -*value;
                            replace = Some(operand.clone());
                        }
                        Expr::Float(value, _) => {
                            *value = -*value;
                            replace = Some(operand.clone());
                        }
                        _ => {
                            replace = Some(rrc(Expr::BinaryOp(
                                BinaryOp::Sub,
                                rrc(Expr::Int(0, span.clone())),
                                operand.clone(),
                                span.clone(),
                            )))
                        }
                    },
                    _ => {}
                }
            }
            Expr::BinaryOp(op, left, right, span) => {
                self.visit_expr(left);
                self.visit_expr(right);
            }
        }
        if let Some(replace) = replace {
            *expr = replace;
        }
    }
}
