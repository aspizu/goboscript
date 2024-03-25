use logos::Lexer;
use smol_str::SmolStr;

use super::token::Token;

pub fn name(lex: &mut Lexer<Token>) -> SmolStr {
    SmolStr::from(lex.slice())
}

pub fn string(lex: &mut Lexer<Token>) -> SmolStr {
    SmolStr::from(serde_json::from_str::<'_, String>(lex.slice()).unwrap())
}

pub fn arg(lex: &mut Lexer<Token>) -> SmolStr {
    SmolStr::from(&lex.slice()[1..])
}

pub fn mac(lex: &mut Lexer<Token>) -> SmolStr {
    SmolStr::from(&lex.slice()[..lex.slice().len() - 1])
}

pub fn bin(lex: &mut Lexer<Token>) -> i64 {
    let mut neg = false;
    let mut value = 0;
    for &c in lex.slice().as_bytes().iter() {
        if c == b'-' {
            neg = true;
            continue;
        }
        if c == b'_' {
            continue;
        }
        value *= 2;
        value += (c - b'0') as i64;
    }
    if neg {
        -value
    } else {
        value
    }
}

pub fn oct(lex: &mut Lexer<Token>) -> i64 {
    let mut neg = false;
    let mut value = 0;
    for &c in lex.slice().as_bytes().iter() {
        if c == b'-' {
            neg = true;
            continue;
        }
        if c == b'_' {
            continue;
        }
        value *= 8;
        value += (c - b'0') as i64;
    }
    if neg {
        -value
    } else {
        value
    }
}

pub fn int(lex: &mut Lexer<Token>) -> i64 {
    let mut neg = false;
    let mut value = 0;
    for &c in lex.slice().as_bytes().iter() {
        if c == b'-' {
            neg = true;
            continue;
        }
        if c == b'_' {
            continue;
        }
        value *= 10;
        value += (c - b'0') as i64;
    }
    if neg {
        -value
    } else {
        value
    }
}

pub fn hex(lex: &mut Lexer<Token>) -> i64 {
    let mut neg = false;
    let mut value = 0;
    for &c in lex.slice().as_bytes().iter() {
        if c == b'-' {
            neg = true;
            continue;
        }
        if c == b'_' {
            continue;
        }
        value *= 16;
        value += match c {
            0..=b'9' => (c - b'0') as i64,
            b'a'..=b'f' => (c - b'a' + 10) as i64,
            _ => (c - b'A' + 10) as i64,
        };
    }
    if neg {
        -value
    } else {
        value
    }
}

pub fn float(lex: &mut Lexer<Token>) -> f64 {
    serde_json::from_str(lex.slice()).unwrap()
}
