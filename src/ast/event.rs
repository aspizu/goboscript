use logos::Span;
use serde::Serialize;

use super::{
    event_kind::EventKind,
    stmt::Stmt,
    References,
};

#[derive(Debug, Serialize)]
pub struct Event {
    pub kind: EventKind,
    pub span: Span,
    pub body: Vec<Stmt>,
    pub references: References,
}
