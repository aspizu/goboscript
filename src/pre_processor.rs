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

#[derive(Debug)]
enum MacroDefinition {
    Simple(Define),
    Function(Vec<SmolStr>, Vec<SpannedToken>),
}

struct PreProcessor {
    defines: FxHashMap<SmolStr, MacroDefinition>,
}

impl PreProcessor {
    fn new() -> Self {
        Self {
            defines: Default::default(),
        }
    }

    fn process_define(
        &mut self,
        tokens: &mut Vec<SpannedToken>,
        i: &mut usize,
    ) -> Result<(), Diagnostic> {
        tokens.remove(*i);
        let name = self.extract_name(tokens, i)?;

        if matches!(tokens[*i].1, Token::LParen) {
            let (args, definition) = self.parse_function_define(tokens, i)?;
            self.defines
                .insert(name, MacroDefinition::Function(args, definition));
        } else {
            let definition = self.parse_simple_define(tokens, i)?;
            self.defines
                .insert(name, MacroDefinition::Simple(definition));
        }
        Ok(())
    }

    fn extract_name(
        &self,
        tokens: &mut Vec<SpannedToken>,
        i: &mut usize,
    ) -> Result<SmolStr, Diagnostic> {
        if *i >= tokens.len() {
            return Err(Diagnostic {
                kind: DiagnosticKind::UnrecognizedEof(vec!["NAME".to_owned()]),
                span: span(&tokens[*i - 1]),
            });
        }

        let (begin, token, end) = tokens.remove(*i);
        match token {
            Token::Name(name) => Ok(name),
            _ => Err(Diagnostic {
                kind: DiagnosticKind::UnrecognizedToken(token, vec!["NAME".to_owned()]),
                span: begin..end,
            }),
        }
    }

    fn parse_function_define(
        &self,
        tokens: &mut Vec<SpannedToken>,
        i: &mut usize,
    ) -> Result<(Vec<SmolStr>, Vec<SpannedToken>), Diagnostic> {
        let mut args = vec![];
        tokens.remove(*i); // Remove LParen

        while *i < tokens.len() {
            match &tokens[*i].1 {
                Token::RParen => {
                    tokens.remove(*i);
                    break;
                }
                Token::Comma => {
                    tokens.remove(*i);
                }
                Token::Name(name) => {
                    args.push(name.clone());
                    tokens.remove(*i);
                }
                _ => {
                    tokens.remove(*i);
                }
            }
        }

        let definition = self.parse_definition(tokens, i)?;
        Ok((args, definition))
    }

    fn parse_simple_define(
        &self,
        tokens: &mut Vec<SpannedToken>,
        i: &mut usize,
    ) -> Result<Vec<SpannedToken>, Diagnostic> {
        self.parse_definition(tokens, i)
    }

    fn parse_definition(
        &self,
        tokens: &mut Vec<SpannedToken>,
        i: &mut usize,
    ) -> Result<Vec<SpannedToken>, Diagnostic> {
        let mut definition = vec![];
        while *i < tokens.len() {
            match tokens[*i].1 {
                Token::Backslash => {
                    tokens.remove(*i);
                    if matches!(tokens[*i].1, Token::Newline) {
                        tokens.remove(*i);
                    }
                }
                Token::Newline => {
                    tokens.remove(*i);
                    break;
                }
                _ => definition.push(tokens.remove(*i)),
            }
        }
        Ok(definition)
    }

    fn process_concat(
        &self,
        tokens: &mut Vec<SpannedToken>,
        i: &mut usize,
    ) -> Result<(), Diagnostic> {
        tokens.remove(*i);
        let (lbegin, ltoken, lend) = self.expect_token(tokens, i, "LHS")?;
        let (_, rtoken, _) = self.expect_token(tokens, i, "RHS")?;

        let new_token = match (&ltoken, &rtoken) {
            (Token::Name(lname), Token::Name(rname)) => {
                Token::Name(format!("{lname}{rname}").into())
            }
            (Token::Str(lstr), Token::Str(rstr)) => Token::Str(format!("{lstr}{rstr}").into()),
            _ => {
                return Err(Diagnostic {
                    kind: DiagnosticKind::UnrecognizedToken(
                        ltoken,
                        vec!["Concatenable".to_owned()],
                    ),
                    span: lbegin..lend,
                })
            }
        };

        tokens.insert(*i, (lbegin, new_token, lend));
        *i += 1;
        Ok(())
    }

    fn expect_token(
        &self,
        tokens: &mut Vec<SpannedToken>,
        i: &mut usize,
        expected: &str,
    ) -> Result<SpannedToken, Diagnostic> {
        if *i >= tokens.len() {
            return Err(Diagnostic {
                kind: DiagnosticKind::UnrecognizedEof(vec![expected.to_owned()]),
                span: span(&tokens[*i - 1]),
            });
        }
        Ok(tokens.remove(*i))
    }
}

pub fn pre_processor(tokens: &mut Vec<SpannedToken>) -> Result<(), Diagnostic> {
    let mut processor = PreProcessor::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i].1 {
            Token::Define => processor.process_define(tokens, &mut i)?,
            Token::Undef => {
                tokens.remove(i);
                let name = processor.extract_name(tokens, &mut i)?;
                processor.defines.remove(&name);
            }
            Token::Name(name) if name == "__xxCONCAT__" => {
                processor.process_concat(tokens, &mut i)?;
            }
            Token::Name(name) => match processor.defines.get(name) {
                Some(MacroDefinition::Simple(definition)) => {
                    tokens.splice(i..(i + 1), definition.iter().cloned());
                }
                Some(MacroDefinition::Function(args, definition)) => {
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
                }
                None => i += 1,
            },
            Token::Newline | Token::Backslash => {
                tokens.remove(i);
            }
            _ => i += 1,
        }
    }
    Ok(())
}
