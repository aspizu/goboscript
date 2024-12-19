use logos::Span;

use super::{event_kind::EventKind, stmt::Stmt};

#[derive(Debug)]
pub struct Event {
    pub kind: EventKind,
    pub span: Span,
    pub body: Vec<Stmt>,
}
