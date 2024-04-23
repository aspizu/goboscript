use lalrpop_util::{lalrpop_mod, ParseError};

use crate::{lexer::adaptor::Lexer, parser::grammar::SpriteParser};

lalrpop_mod!(grammar, "/parser/grammar.rs");

use crate::{
    ast::Sprite,
    diagnostic::{Diagnostic, DiagnosticKind},
    preproc,
};

pub fn parse(src: &str) -> Result<Sprite, Diagnostic> {
    let tokens = preproc::process(
        Lexer::new(src).flatten().map(|(left, token, right)| (token, left..right)),
        &mut Default::default(),
    );
    let parser = SpriteParser::new();
    let mut sprite = Sprite::default();
    parser
        .parse(
            &mut sprite,
            tokens.into_iter().map(|(token, span)| (span.start, token, span.end)),
        )
        .map(|_| sprite)
        .map_err(|err| match err {
            ParseError::InvalidToken { location } => {
                DiagnosticKind::InvalidToken.to_diagnostic(location..location + 1)
            }
            ParseError::UnrecognizedEof { location, expected } => {
                DiagnosticKind::UnrecognizedEof(expected)
                    .to_diagnostic(location..location + 1)
            }
            ParseError::UnrecognizedToken { token: (left, token, right), expected } => {
                DiagnosticKind::UnrecognizedToken(token, expected)
                    .to_diagnostic(left..right)
            }
            ParseError::ExtraToken { token: (left, token, right) } => {
                DiagnosticKind::ExtraToken(token).to_diagnostic(left..right)
            }
            ParseError::User { error } => error,
        })
}
