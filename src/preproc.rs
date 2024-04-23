use fxhash::FxHashMap;
use logos::Span;
use smol_str::SmolStr;

use crate::lexer::token::Token;

#[derive(Default)]
enum State {
    #[default]
    None,
    Mac {
        span: Span,
        mac: SmolStr,
        args: Vec<Vec<(Token, Span)>>,
        depth: usize,
    },
    Rules0,
    Rules1 {
        name: SmolStr,
        span: Span,
        args: Vec<SmolStr>,
    },
    Rules2 {
        name: SmolStr,
        span: Span,
        args: Vec<SmolStr>,
        body: Vec<(Token, Span)>,
        depth: usize,
    },
}

macro_rules! token_stream {
    () => {
        impl Iterator<Item = (Token, Span)>
    };
}

fn join(span: Span, args: &[Vec<(Token, Span)>]) -> Vec<(Token, Span)> {
    let (left, _) = args.first().unwrap().first().unwrap();
    let (right, _) = args.last().unwrap().last().unwrap();
    let token = match (left, right) {
        (Token::Name(left), Token::Name(right)) => {
            Token::Name(SmolStr::from(format!("{}{}", left, right)))
        }
        (Token::Str(left), Token::Str(right)) => {
            Token::Str(SmolStr::from(format!("{}{}", left, right)))
        }
        _ => panic!(),
    };
    vec![(token.clone(), span)]
}

fn costumes_ascii(span: Span, args: &[Vec<(Token, Span)>]) -> Vec<(Token, Span)> {
    let path = args.first().unwrap().first().unwrap();
    let mut tokens = Vec::new();
    for i in 32..127 {
        tokens.push(path.clone());
        tokens.push((Token::As, span.clone()));
        tokens.push((
            Token::Str(SmolStr::from(std::str::from_utf8(&[b'_', i]).unwrap())),
            span.clone(),
        ));
        if i != 126 {
            tokens.push((Token::Comma, span.clone()));
        }
    }
    tokens
}

#[derive(Debug)]
pub struct Rule {
    name: SmolStr,
    args: Vec<SmolStr>,
    body: Vec<(Token, Span)>,
}

pub fn process(
    stream: token_stream!(),
    rules: &mut FxHashMap<SmolStr, Rule>,
) -> Vec<(Token, Span)> {
    let mut tokens = Vec::new();
    let mut state = State::default();
    for (token, span) in stream {
        match &mut state {
            State::None => match &token {
                Token::Mac(mac) if mac == "macro" => {
                    state = State::Rules0;
                }
                Token::Mac(mac) => {
                    state = State::Mac {
                        span,
                        mac: mac.clone(),
                        args: Default::default(),
                        depth: 0,
                    };
                }
                _ => {
                    tokens.push((token, span));
                }
            },
            State::Mac { span, mac, args, depth } => match &token {
                Token::LBracket | Token::LBrace => {
                    *depth += 1;
                    args.last_mut().unwrap().push((token.clone(), span.clone()));
                }
                Token::RBracket | Token::RBrace => {
                    *depth -= 1;
                    args.last_mut().unwrap().push((token.clone(), span.clone()));
                }
                Token::LParen => {
                    *depth += 1;
                    if *depth == 1 {
                        args.push(Vec::new());
                    } else {
                        let args = args.last_mut().unwrap();
                        args.push((token.clone(), span.clone()));
                    }
                }
                Token::RParen => {
                    *depth -= 1;
                    if *depth == 0 {
                        for arg in args.iter_mut() {
                            let a = process(arg.drain(..), rules);
                            arg.extend(a);
                        }
                        let processed = if mac == "join" {
                            join(span.clone(), args)
                        } else if mac == "costumes_ascii" {
                            costumes_ascii(span.clone(), args)
                        } else if let Some(rule) = rules.get(mac) {
                            let map: FxHashMap<_, _> =
                                rule.args.iter().zip(args).collect();
                            for (token, span) in &rule.body {
                                if let Token::Mac(m) = token {
                                    if let Some(a) = map.get(&m) {
                                        tokens.extend(a.iter().cloned());
                                    } else {
                                        tokens.push((token.clone(), span.clone()));
                                    }
                                } else {
                                    tokens.push((token.clone(), span.clone()));
                                }
                            }
                            state = State::None;
                            continue;
                        } else {
                            panic!()
                        };
                        tokens.extend(processed);
                        state = State::None;
                    } else {
                        args.last_mut().unwrap().push((token.clone(), span.clone()));
                    }
                }
                Token::Comma => {
                    if *depth == 1 {
                        args.push(Vec::new());
                    } else {
                        args.last_mut().unwrap().push((token.clone(), span.clone()));
                    }
                }
                _ => {
                    let args = args.last_mut().unwrap();
                    args.push((token.clone(), span.clone()));
                }
            },
            State::Rules0 => {
                state = State::Rules1 {
                    name: match &token {
                        Token::Mac(name) => name.clone(),
                        _ => panic!(),
                    },
                    span,
                    args: Default::default(),
                };
            }
            State::Rules1 { name, span, args } => match &token {
                Token::Comma => {}
                Token::Name(name) => {
                    args.push(name.clone());
                }
                Token::LBrace => {
                    state = State::Rules2 {
                        name: name.clone(),
                        span: span.clone(),
                        args: args.clone(),
                        body: Vec::new(),
                        depth: 0,
                    };
                }
                _ => panic!(),
            },
            State::Rules2 { name, span, args, body, depth } => match &token {
                Token::LBrace => {
                    *depth += 1;
                    body.push((token.clone(), span.clone()));
                }
                Token::RBrace => {
                    if *depth == 0 {
                        rules.insert(
                            name.clone(),
                            Rule {
                                name: name.clone(),
                                args: args.clone(),
                                body: body.clone(),
                            },
                        );
                        state = State::None;
                    } else {
                        *depth -= 1;
                        body.push((token.clone(), span.clone()));
                    }
                }
                _ => {
                    body.push((token.clone(), span.clone()));
                }
            },
        }
    }
    let has_mac = tokens.iter().any(|(token, _)| matches!(token, Token::Mac(..)));
    if has_mac {
        process(tokens.into_iter(), rules)
    } else {
        tokens
    }
}
