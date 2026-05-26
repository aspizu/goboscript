use logos::Lexer;

use super::token::Token;
use crate::misc::SmolStr;

pub fn name(lex: &mut Lexer<Token>) -> SmolStr {
    SmolStr::from(lex.slice())
}

pub fn string(lex: &mut Lexer<Token>) -> Option<SmolStr> {
    serde_json::from_str::<String>(lex.slice()).ok().map(SmolStr::from)
}

pub fn arg(lex: &mut Lexer<Token>) -> SmolStr {
    SmolStr::from(&lex.slice()[1..])
}

pub fn bin(lex: &mut Lexer<Token>) -> Option<i64> {
    i64::from_str_radix(&lex.slice()[2..].replace('_', ""), 2).ok()
}

pub fn oct(lex: &mut Lexer<Token>) -> Option<i64> {
    i64::from_str_radix(&lex.slice()[2..].replace('_', ""), 8).ok()
}

pub fn int(lex: &mut Lexer<Token>) -> Option<i64> {
    lex.slice().replace('_', "").parse().ok()
}

pub fn hex(lex: &mut Lexer<Token>) -> Option<i64> {
    i64::from_str_radix(&lex.slice()[2..].replace('_', ""), 16).ok()
}

pub fn float(lex: &mut Lexer<Token>) -> Option<f64> {
    lex.slice().replace('_', "").parse().ok()
}
