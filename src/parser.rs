use grammar::SpriteParser;
use lalrpop_util::lalrpop_mod;

use crate::{ast::Sprite, diagnostic::Diagnostic, lexer::preproc, preproc::PreProc};

lalrpop_mod!(grammar, "/parser/grammar.rs");

pub fn parse(preproc_: &PreProc) -> Result<Sprite, Diagnostic> {
    let tokens = preproc::preproc(preproc_);
    let parser = SpriteParser::new();
    let mut sprite = Sprite::default();
    parser.parse(&mut sprite, tokens)?;
    Ok(sprite)
}
