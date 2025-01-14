use grammar::SpriteParser;
use lalrpop_util::lalrpop_mod;

use crate::{
    ast::Sprite,
    diagnostic::Diagnostic,
    lexer::adaptor,
    pre_processor::pre_processor,
    translation_unit::TranslationUnit,
};

lalrpop_mod!(grammar, "/parser/grammar.rs");

pub fn parse(translation_unit: &TranslationUnit) -> Result<Sprite, Diagnostic> {
    let mut tokens = vec![];
    for token in adaptor::Lexer::new(translation_unit.get_text()) {
        let token = token?;
        tokens.push(token);
    }
    pre_processor(&mut tokens)?;
    let parser = SpriteParser::new();
    let mut sprite = Sprite::default();
    parser.parse(&mut sprite, tokens)?;
    Ok(sprite)
}
