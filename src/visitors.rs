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
            Declr::OnFlag(body, _span) => {
                self.visit_stmts(body);
            }
            Declr::OnKey(_key, body, _span) => {
                self.visit_stmts(body);
            }
            Declr::OnClick(body, _span) => {
                self.visit_stmts(body);
            }
            Declr::OnBackdrop(_backdrop, body, _span) => {
                self.visit_stmts(body);
            }
            Declr::OnLoudnessGreaterThan(loudness, body, _span) => {
                self.visit_expr(loudness);
                self.visit_stmts(body);
            }
            Declr::OnTimerGreaterThan(timer, body, _span) => {
                self.visit_expr(timer);
                self.visit_stmts(body);
            }
            Declr::OnMessage(_message, body, _span) => {
                self.visit_stmts(body);
            }
            Declr::OnClone(body, _span) => {
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
            Stmt::Repeat(times, body, _span) => {
                self.visit_expr(times);
                self.visit_stmts(body);
            }
            Stmt::Forever(body, _span) => {
                self.visit_stmts(body);
            }
            Stmt::Branch(condition, if_body, else_body, _span) => {
                self.visit_expr(condition);
                self.visit_stmts(if_body);
                self.visit_stmts(else_body);
            }
            Stmt::Until(condition, body, _span) => {
                self.visit_expr(condition);
                self.visit_stmts(body);
            }
            Stmt::SetVariable(name, value, _span) => {
                self.variables.insert(name);
                self.visit_expr(value);
            }
            Stmt::ChangeVariable(_name, value, _span) => {
                self.visit_expr(value);
            }
            Stmt::Show(_name, _span) => todo!(),
            Stmt::Hide(_name, _span) => todo!(),
            Stmt::ListAdd(name, value, _span) => {
                self.lists.insert(name);
                self.visit_expr(value);
            }
            Stmt::ListDelete(name, index, _span) => {
                self.lists.insert(name);
                self.visit_expr(index);
            }
            Stmt::ListDeleteAll(name, _span) => {
                self.lists.insert(name);
            }
            Stmt::ListInsert(name, index, value, _span) => {
                self.lists.insert(name);
                self.visit_expr(index);
                self.visit_expr(value);
            }
            Stmt::ListReplace(name, index, value, _span) => {
                self.lists.insert(name);
                self.visit_expr(index);
                self.visit_expr(value);
            }
            Stmt::Block(_name, args, _span) => {
                for arg in args {
                    self.visit_expr(arg);
                }
            }
            Stmt::Call(_name, args, _span) => {
                for arg in args {
                    self.visit_expr(arg);
                }
            }
        }
    }

    fn visit_expr(&mut self, expr: &mut Rrc<Expr<'src>>) {
        let mut replace = None;
        match &mut *expr.borrow_mut() {
            Expr::Int(_value, _span) => {}
            Expr::Float(_value, _span) => {}
            Expr::String(_value, _span) => {}
            Expr::Name(_name, _span) => {}
            Expr::Arg(_name, _span) => {}
            Expr::Reporter(_reporter, args, _span) => {
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
            Expr::BinaryOp(op, left, right, _span) => {
                self.visit_expr(left);
                self.visit_expr(right);
                match op {
                    BinaryOp::Add => {
                        match (&mut *left.borrow_mut(), &mut *right.borrow_mut()) {
                            (Expr::Int(l, _), Expr::Int(r, _)) => {
                                *l += *r;
                                replace = Some(left.clone());
                            }
                            (
                                Expr::Int(l, _),
                                Expr::BinaryOp(
                                    BinaryOp::Add,
                                    right_left,
                                    right_right,
                                    _,
                                ),
                            ) => {
                                if let Expr::Int(rl, _) = &mut *right_left.borrow_mut()
                                {
                                    *rl += *l;
                                    replace = Some(right.clone());
                                } else if let Expr::Int(rr, _) =
                                    &mut *right_right.borrow_mut()
                                {
                                    *rr += *l;
                                    replace = Some(right.clone());
                                }
                            }
                            (Expr::Float(l, _), Expr::Float(r, _)) => {
                                *l += *r;
                                replace = Some(left.clone());
                            }
                            _ => {}
                        }
                    }
                    BinaryOp::Sub => {
                        match (&mut *left.borrow_mut(), &mut *right.borrow_mut()) {
                            (Expr::Int(l, _), Expr::Int(r, _)) => {
                                *l -= *r;
                                replace = Some(left.clone());
                            }
                            (Expr::Float(l, _), Expr::Float(r, _)) => {
                                *l -= *r;
                                replace = Some(left.clone());
                            }
                            _ => {}
                        }
                    }
                    BinaryOp::Mul => {
                        match (&mut *left.borrow_mut(), &mut *right.borrow_mut()) {
                            (Expr::Int(l, _), Expr::Int(r, _)) => {
                                *l *= *r;
                                replace = Some(left.clone());
                            }
                            (
                                Expr::Int(l, _),
                                Expr::BinaryOp(
                                    BinaryOp::Mul,
                                    right_left,
                                    right_right,
                                    _,
                                ),
                            ) => {
                                if let Expr::Int(rl, _) = &mut *right_left.borrow_mut()
                                {
                                    *rl *= *l;
                                    replace = Some(right.clone());
                                } else if let Expr::Int(rr, _) =
                                    &mut *right_right.borrow_mut()
                                {
                                    *rr *= *l;
                                    replace = Some(right.clone());
                                }
                            }
                            (Expr::Float(l, _), Expr::Float(r, _)) => {
                                *l *= *r;
                                replace = Some(left.clone());
                            }
                            _ => {}
                        }
                    }
                    BinaryOp::Div => {
                        match (&mut *left.borrow_mut(), &mut *right.borrow_mut()) {
                            (Expr::Int(l, _), Expr::Int(r, _)) => {
                                if *r != 0 {
                                    *l /= *r;
                                    replace = Some(left.clone());
                                }
                            }
                            (Expr::Float(l, _), Expr::Float(r, _)) => {
                                if *r != 0.0 {
                                    *l /= *r;
                                    replace = Some(left.clone());
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
        if let Some(replace) = replace {
            *expr = replace;
        }
    }
}
