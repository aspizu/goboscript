use grammar::SpriteParser;
use lalrpop_util::lalrpop_mod;

use crate::{ast::Sprite, diagnostic::Diagnostic, lexer::adaptor::Lexer};

lalrpop_mod!(grammar, "/parser/grammar.rs");

pub fn parse(src: &str) -> Result<Sprite, Diagnostic> {
    let tokens = Lexer::new(src).flatten();
    let parser = SpriteParser::new();
    let mut sprite = Sprite::default();
    parser.parse(&mut sprite, tokens)?;
    Ok(sprite)
}
