use super::{adaptor::Lexer, token::Token};
use crate::preproc::PreProc;

pub fn preproc(preproc: &PreProc) -> Vec<(usize, Token, usize)> {
    let lex = Lexer::new(preproc.get_translation_unit()).flatten();
    _preproc(preproc, lex)
}

fn _preproc<T>(preproc: &PreProc, mut lex: T) -> Vec<(usize, Token, usize)>
where T: Iterator<Item = (usize, Token, usize)> {
    let mut dirty = false;
    let mut tokens = vec![];
    while let Some((start, token, end)) = lex.next() {
        match &token {
            Token::Name(name) => {
                if let Some(value) = preproc.defines.get(&**name) {
                    tokens.extend(Lexer::new(value).flatten());
                    dirty = true;
                } else if let Some(mac) = preproc.macros.get(&**name) {
                    dirty = true;
                    if let Some((start, token, end)) = lex.next() {
                        if let Token::LParen = &token {
                        } else {
                            tokens.push((start, token, end));
                            continue;
                        }
                    } else {
                        continue;
                    }
                    let mut parens = 1;
                    let mut braces = 0;
                    let mut brackets = 0;

                    let mut args = vec![];
                    'args: loop {
                        let mut arg = vec![];
                        for (start, token, end) in lex.by_ref() {
                            match &token {
                                Token::LParen => parens += 1,
                                Token::RParen => {
                                    parens -= 1;
                                    if parens == 0 && braces == 0 && brackets == 0 {
                                        args.push(arg);
                                        break 'args;
                                    }
                                }
                                Token::LBrace => braces += 1,
                                Token::RBrace => braces -= 1,
                                Token::LBracket => brackets += 1,
                                Token::RBracket => brackets -= 1,
                                Token::Comma => {
                                    if parens == 1 && braces == 0 && brackets == 0 {
                                        break;
                                    }
                                }
                                _ => {}
                            }
                            arg.push((start, token, end));
                        }
                        args.push(arg);
                    }
                    for (start, token, end) in Lexer::new(&mac.substitution).flatten() {
                        match &token {
                            Token::Name(name) => {
                                if let Some(value) = preproc.defines.get(&**name) {
                                    tokens.extend(Lexer::new(value).flatten());
                                } else if let Some(index) =
                                    mac.args.iter().position(|arg| **arg == **name)
                                {
                                    tokens.extend(args[index].iter().cloned());
                                } else {
                                    tokens.push((start, Token::Name(name.clone()), end));
                                }
                            }
                            _ => {
                                tokens.push((start, token, end));
                            }
                        }
                    }
                } else {
                    tokens.push((start, token, end));
                }
            }
            _ => {
                tokens.push((start, token, end));
            }
        }
    }
    if dirty {
        _preproc(preproc, tokens.into_iter())
    } else {
        tokens
    }
}
