use logos::Span;

use super::{event_kind::EventKind, stmt::Stmt, References};

#[derive(Debug)]
pub struct Event {
    pub kind: EventKind,
    pub span: Span,
    pub body: Vec<Stmt>,
    pub references: References,
}
