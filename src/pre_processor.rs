use fxhash::FxHashMap;
use logos::Span;

use crate::{
    diagnostic::{
        Diagnostic,
        DiagnosticKind,
    },
    lexer::token::Token,
    misc::SmolStr,
};

fn span(token: &(usize, Token, usize)) -> Span {
    token.0..token.2
}

type SpannedToken = (usize, Token, usize);
type Define = Vec<SpannedToken>;
type Function = (Vec<SmolStr>, Vec<SpannedToken>);

pub fn pre_processor(tokens: &mut Vec<SpannedToken>) -> Result<(), Diagnostic> {
    let mut defines: FxHashMap<SmolStr, Define> = Default::default();
    let mut functions: FxHashMap<SmolStr, Function> = Default::default();
    let mut i = 0;
    while i < tokens.len() {
        if matches!(tokens[i].1, Token::Define) {
            tokens.remove(i);
            if i >= tokens.len() {
                return Err(Diagnostic {
                    kind: DiagnosticKind::UnrecognizedEof(vec!["NAME".to_owned()]),
                    span: span(&tokens[i - 1]),
                });
            }
            let (begin, token, end) = tokens.remove(i);
            let Token::Name(name) = token else {
                return Err(Diagnostic {
                    kind: DiagnosticKind::UnrecognizedToken(token, vec!["NAME".to_owned()]),
                    span: begin..end,
                });
            };
            let args = if matches!(tokens[i].1, Token::LParen) {
                let mut args = vec![];
                tokens.remove(i);
                while i < tokens.len() {
                    if matches!(tokens[i].1, Token::RParen) {
                        tokens.remove(i);
                        break;
                    }
                    if matches!(tokens[i].1, Token::Comma) {
                        tokens.remove(i);
                    }
                    if let Token::Name(name) = &tokens[i].1 {
                        args.push(name.clone());
                    }
                    tokens.remove(i);
                }
                Some(args)
            } else {
                None
            };
            let mut definition = vec![];
            while i < tokens.len() {
                if matches!(tokens[i].1, Token::Backslash) {
                    tokens.remove(i);
                    if matches!(tokens[i].1, Token::Newline) {
                        tokens.remove(i);
                    }
                    continue;
                }
                if matches!(tokens[i].1, Token::Newline) {
                    tokens.remove(i);
                    break;
                }
                definition.push(tokens.remove(i));
            }
            if let Some(args) = args {
                functions.insert(name.clone(), (args, definition));
            } else {
                defines.insert(name.clone(), definition);
            }
        } else if matches!(tokens[i].1, Token::Undef) {
            tokens.remove(i);
            if i >= tokens.len() {
                return Err(Diagnostic {
                    kind: DiagnosticKind::UnrecognizedEof(vec!["NAME".to_owned()]),
                    span: span(&tokens[i - 1]),
                });
            }
            let (begin, token, end) = tokens.remove(i);
            let Token::Name(name) = token else {
                return Err(Diagnostic {
                    kind: DiagnosticKind::UnrecognizedToken(token, vec!["NAME".to_owned()]),
                    span: begin..end,
                });
            };
            defines.remove(&name);
            functions.remove(&name);
        } else if matches!(tokens[i].1, Token::Name(_)) {
            let Token::Name(name) = tokens[i].1.clone() else {
                unreachable!()
            };
            if let Some(definition) = defines.get(&name) {
                tokens.splice(i..(i + 1), definition.iter().cloned());
            } else if let Some((args, definition)) = functions.get(&name) {
                tokens.remove(i);
                if matches!(tokens[i].1, Token::LParen) {
                    tokens.remove(i);
                }
                let mut parens = 0;
                let mut brackets = 0;
                let mut braces = 0;
                let mut parameters = vec![];
                let mut parameter = vec![];
                while i < tokens.len() {
                    if matches!(tokens[i].1, Token::RParen) {
                        if parens == 0 {
                            tokens.remove(i);
                            if !parameter.is_empty() {
                                parameters.push(parameter);
                            }
                            break;
                        }
                        parens -= 1;
                    } else if matches!(tokens[i].1, Token::LParen) {
                        parens += 1;
                    } else if matches!(tokens[i].1, Token::LBracket) {
                        brackets += 1;
                    } else if matches!(tokens[i].1, Token::RBracket) {
                        brackets -= 1;
                    } else if matches!(tokens[i].1, Token::LBrace) {
                        braces += 1;
                    } else if matches!(tokens[i].1, Token::RBrace) {
                        braces -= 1;
                    } else if matches!(tokens[i].1, Token::Comma)
                        && parens == 0
                        && brackets == 0
                        && braces == 0
                    {
                        parameters.push(parameter);
                        parameter = vec![];
                        tokens.remove(i);
                        continue;
                    }
                    parameter.push(tokens.remove(i));
                }
                let begin = i;
                for token in definition {
                    if let Token::Name(name) = &token.1 {
                        if let Some(index) = args.iter().position(|arg| arg == name) {
                            tokens.splice(i..i, parameters[index].iter().cloned());
                            i += parameters[index].len();
                            continue;
                        }
                    }
                    tokens.insert(i, token.clone());
                    i += 1;
                }
                i = begin;
            } else if name == "__xxCONCAT__" {
                tokens.remove(i);
                if i >= tokens.len() {
                    return Err(Diagnostic {
                        kind: DiagnosticKind::UnrecognizedEof(vec!["LHS".to_owned()]),
                        span: span(&tokens[i - 1]),
                    });
                }
                let (lbegin, ltoken, lend) = tokens.remove(i);
                if i >= tokens.len() {
                    return Err(Diagnostic {
                        kind: DiagnosticKind::UnrecognizedEof(vec!["RHS".to_owned()]),
                        span: span(&tokens[i - 1]),
                    });
                }
                let (rbegin, rtoken, rend) = tokens.remove(i);
                match (&ltoken, &rtoken) {
                    (Token::Name(lname), Token::Name(rname)) => {
                        tokens.insert(
                            i,
                            (lbegin, Token::Name(format!("{lname}{rname}").into()), lend),
                        );
                    }
                    (Token::Str(lstr), Token::Str(rstr)) => {
                        tokens.insert(
                            i,
                            (lbegin, Token::Str(format!("{lstr}{rstr}").into()), lend),
                        );
                    }
                    _ => {
                        return Err(Diagnostic {
                            kind: DiagnosticKind::UnrecognizedToken(
                                ltoken,
                                vec!["Concatenable".to_owned()],
                            ),
                            span: lbegin..lend,
                        })
                    }
                }
                i += 1;
            } else {
                i += 1;
            }
        } else if matches!(tokens[i].1, Token::Newline | Token::Backslash) {
            tokens.remove(i);
        } else {
            i += 1;
        }
    }
    Ok(())
}
