use logos::Span;
use smol_str::SmolStr;

use super::{expr::Expr, type_::Type, Name};
use crate::{blocks::Block, misc::Rrc};

#[derive(Debug)]
pub enum Stmt {
    Repeat {
        times: Rrc<Expr>,
        body: Vec<Stmt>,
    },
    Forever {
        body: Vec<Stmt>,
        span: Span,
    },
    Branch {
        cond: Rrc<Expr>,
        if_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    Until {
        cond: Rrc<Expr>,
        body: Vec<Stmt>,
    },
    SetVar {
        name: Name,
        value: Rrc<Expr>,
        type_: Type,
        is_local: bool,
    },
    Show(Name),
    Hide(Name),
    AddToList {
        name: Name,
        value: Rrc<Expr>,
    },
    DeleteList(Name),
    DeleteListIndex {
        name: Name,
        index: Rrc<Expr>,
    },
    InsertAtList {
        name: Name,
        index: Rrc<Expr>,
        value: Rrc<Expr>,
    },
    SetListIndex {
        name: Name,
        index: Rrc<Expr>,
        value: Rrc<Expr>,
    },
    Block {
        block: Block,
        span: Span,
        args: Vec<Rrc<Expr>>,
    },
    ProcCall {
        name: SmolStr,
        span: Span,
        args: Vec<Rrc<Expr>>,
    },
}

impl Stmt {
    pub fn span(&self) -> &Span {
        todo!()
    }
}
