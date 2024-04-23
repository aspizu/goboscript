use logos::{Logos, SpannedIter};

use super::token::Token;
use crate::diagnostic::{Diagnostic, DiagnosticKind};

pub struct Lexer<'source> {
    token_stream: SpannedIter<'source, Token>,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        Self { token_stream: Token::lexer(source).spanned() }
    }
}

impl<'source> From<&'source str> for Lexer<'source> {
    fn from(source: &'source str) -> Self {
        Lexer::new(source)
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<(usize, Token, usize), Diagnostic>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| {
            token
                .map(|token| (span.start, token, span.end))
                .map_err(|_| DiagnosticKind::InvalidToken.to_diagnostic(span))
        })
    }
}
