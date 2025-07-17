use grammar::SpriteParser;
use lalrpop_util::lalrpop_mod;

use crate::{
    ast::Sprite,
    diagnostic::Diagnostic,
    lexer::{
        adaptor,
        token::Token,
    },
    pre_processor::PreProcessor,
    translation_unit::TranslationUnit,
};

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    grammar,
    "/parser/grammar.rs"
);

type SpannedToken = (usize, Token, usize);

/// Tokenize the source code from a translation unit
fn tokenize(translation_unit: &TranslationUnit) -> (Vec<SpannedToken>, Vec<Diagnostic>) {
    let mut tokens = Vec::new();
    let mut diagnostics = Vec::new();

    adaptor::Lexer::new(translation_unit.get_text()).for_each(|result| match result {
        Ok(token) => tokens.push(token),
        Err(diagnostic) => diagnostics.push(diagnostic),
    });

    (tokens, diagnostics)
}

/// Apply preprocessing to the tokens
fn preprocess(mut tokens: Vec<SpannedToken>) -> (Vec<SpannedToken>, Option<Diagnostic>) {
    match PreProcessor::apply(&mut tokens) {
        Ok(()) => (tokens, None),
        Err(diagnostic) => (tokens, Some(diagnostic)),
    }
}

/// Parse the tokens into a sprite AST
fn parse_sprite(tokens: Vec<SpannedToken>) -> (Sprite, Vec<Diagnostic>) {
    let parser = SpriteParser::new();
    let mut sprite = Sprite::default();
    let mut diagnostics = Vec::new();

    if let Err(parse_error) = parser.parse(&mut sprite, &mut diagnostics, tokens) {
        diagnostics.push(parse_error.into());
    }

    (sprite, diagnostics)
}

/// Parse a translation unit into a sprite AST
///
/// This function performs the complete parsing pipeline:
/// 1. Tokenizes the source code
/// 2. Applies preprocessing transformations
/// 3. Parses the tokens into an AST
///
/// Returns the parsed sprite and any diagnostics encountered during parsing.
pub fn parse(translation_unit: &TranslationUnit) -> (Sprite, Vec<Diagnostic>) {
    let (tokens, tokenize_diagnostics) = tokenize(translation_unit);
    let (tokens, preprocess_diagnostic) = preprocess(tokens);
    let (sprite, parse_diagnostics) = parse_sprite(tokens);

    let all_diagnostics = tokenize_diagnostics
        .into_iter()
        .chain(preprocess_diagnostic.into_iter())
        .chain(parse_diagnostics.into_iter())
        .collect();

    (sprite, all_diagnostics)
}
