use fxhash::FxHashMap;
use logos::Span;
use serde::{Serialize, Deserialize};

use super::{
    expr::Expr,
    type_::Type,
    Name,
    Value,
};
use crate::{
    blocks::{
        BinOp,
        Block,
    },
    misc::SmolStr,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Stmt {
    Repeat {
        times: Box<Expr>,
        body: Vec<Stmt>,
    },
    Forever {
        body: Vec<Stmt>,
        span: Span,
    },
    Branch {
        cond: Box<Expr>,
        if_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    Until {
        cond: Box<Expr>,
        body: Vec<Stmt>,
    },
    SetVar {
        name: Name,
        value: Box<Expr>,
        type_: Type,
        is_local: bool,
        is_cloud: bool,
    },
    ChangeVar {
        name: Name,
        value: Box<Expr>,
    },
    Show(Name),
    Hide(Name),
    AddToList {
        name: Name,
        value: Box<Expr>,
    },
    DeleteList(Name),
    DeleteListIndex {
        name: Name,
        index: Box<Expr>,
    },
    InsertAtList {
        name: Name,
        index: Box<Expr>,
        value: Box<Expr>,
    },
    SetListIndex {
        name: Name,
        index: Box<Expr>,
        value: Box<Expr>,
    },
    Block {
        block: Block,
        span: Span,
        args: Vec<Expr>,
        kwargs: FxHashMap<SmolStr, (Span, Expr)>,
    },
    ProcCall {
        name: SmolStr,
        span: Span,
        args: Vec<Expr>,
        kwargs: FxHashMap<SmolStr, (Span, Expr)>,
    },
    FuncCall {
        name: SmolStr,
        span: Span,
        args: Vec<Expr>,
        kwargs: FxHashMap<SmolStr, (Span, Expr)>,
    },
    Return {
        value: Box<Expr>,
        visited: bool,
    },
}

impl Stmt {
    pub fn span(&self) -> &Span {
        todo!()
    }

    pub fn increment(name: Name) -> Self {
        let span = name.span();
        Self::ChangeVar {
            name,
            value: Box::new(Value::from(1.0).to_expr(span)),
        }
    }

    pub fn decrement(name: Name) -> Self {
        let span = name.span();
        Self::ChangeVar {
            name,
            value: Box::new(Value::from(-1.0).to_expr(span)),
        }
    }

    pub fn augmented_assign(op: BinOp, name: SmolStr, span: Span, value: Expr) -> Self {
        let name = Name::Name {
            name,
            span: span.clone(),
        };
        Stmt::SetVar {
            name: name.clone(),
            value: Box::new(op.to_expr(span, Expr::Name(name), value)),
            type_: Type::Value,
            is_local: false,
            is_cloud: false,
        }
    }

    pub fn augmented_field_assign(
        op: BinOp,
        name: SmolStr,
        span: Span,
        field: SmolStr,
        field_span: Span,
        value: Expr,
    ) -> Self {
        let name = Name::DotName {
            lhs: name,
            lhs_span: span,
            rhs: field,
            rhs_span: field_span,
        };
        Stmt::SetVar {
            name: name.clone(),
            value: Box::new(op.to_expr(name.span().clone(), Expr::Name(name), value)),
            type_: Type::Value,
            is_local: false,
            is_cloud: false,
        }
    }

    pub fn increment_index(name: Name, index: Expr) -> Self {
        let span = name.span();
        Stmt::SetListIndex {
            name: name.clone(),
            index: Box::new(index.clone()),
            value: Box::new(BinOp::Add.to_expr(
                span.clone(),
                BinOp::Of.to_expr(span.clone(), Expr::Name(name), index),
                Value::from(1.0).to_expr(span),
            )),
        }
    }

    pub fn decrement_index(name: Name, index: Expr) -> Self {
        let span = name.span();
        Stmt::SetListIndex {
            name: name.clone(),
            index: Box::new(index.clone()),
            value: Box::new(BinOp::Add.to_expr(
                span.clone(),
                BinOp::Of.to_expr(span.clone(), Expr::Name(name), index),
                Value::from(1.0).to_expr(span),
            )),
        }
    }

    pub fn augmented_index_assign(op: BinOp, name: Name, index: Expr, value: Expr) -> Self {
        let span = name.span();
        Stmt::SetListIndex {
            name: name.clone(),
            index: Box::new(index.clone()),
            value: Box::new(op.to_expr(
                span.clone(),
                BinOp::Of.to_expr(span.clone(), Expr::Name(name), index),
                value,
            )),
        }
    }
}

pub fn split_args(
    mut args: Vec<(Option<(SmolStr, Span)>, Expr)>,
) -> (Vec<Expr>, FxHashMap<SmolStr, (Span, Expr)>) {
    let mut positional = Vec::new();
    let mut named = FxHashMap::default();

    // Drain the vector so that we consume the arguments.
    for (maybe_name, expr) in args.drain(..) {
        if let Some((name, span)) = maybe_name {
            // Insert into the named arguments map.
            named.insert(name, (span, expr));
        } else {
            // Otherwise, treat it as a positional argument.
            positional.push(expr);
        }
    }

    (positional, named)
}
