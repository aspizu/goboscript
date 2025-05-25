use logos::Span;
use serde::{
    Deserialize,
    Serialize,
};

use super::{
    event_kind::EventKind,
    stmt::Stmt,
    References,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub kind: EventKind,
    pub span: Span,
    pub body: Vec<Stmt>,
    pub references: References,
}
