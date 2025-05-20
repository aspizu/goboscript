mod diagnostic_kind;
pub mod keys;
mod project_diagnostics;
mod sprite_diagnostics;

pub use diagnostic_kind::*;
use lalrpop_util::ParseError;
use logos::Span;
pub use project_diagnostics::*;
use serde::{
    Deserialize,
    Serialize,
};
pub use sprite_diagnostics::*;
use tsify::Tsify;

use crate::lexer::token::Token;

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub span: Span,
}

impl From<ParseError<usize, Token, Diagnostic>> for Diagnostic {
    fn from(value: ParseError<usize, Token, Diagnostic>) -> Self {
        match value {
            ParseError::InvalidToken { location } => Self {
                kind: DiagnosticKind::InvalidToken,
                span: location..location + 1,
            },
            ParseError::UnrecognizedEof { location, expected } => Self {
                kind: DiagnosticKind::UnrecognizedEof(expected),
                span: location..location + 1,
            },
            ParseError::UnrecognizedToken {
                token: (left, token, right),
                expected,
            } => Self {
                kind: DiagnosticKind::UnrecognizedToken(token, expected),
                span: left..right,
            },
            ParseError::ExtraToken {
                token: (left, token, right),
            } => Self {
                kind: DiagnosticKind::ExtraToken(token),
                span: left..right,
            },
            ParseError::User { error } => error,
        }
    }
}
