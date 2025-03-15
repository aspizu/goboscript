use fxhash::{
    FxHashMap,
    FxHashSet,
};
use logos::Span;

use crate::{
    diagnostic::{
        Diagnostic,
        DiagnosticKind,
    },
    lexer::token::Token,
    misc::SmolStr,
};

type SpannedToken = (usize, Token, usize);

fn get_span(token: &SpannedToken) -> Span {
    token.0..token.2
}

fn get_token(token: &SpannedToken) -> &Token {
    &token.1
}

pub struct PreProcessor<'a, 'b> {
    tokens: &'a mut Vec<(usize, Token, usize)>,
    function_defines: FxHashMap<SmolStr, (Vec<Token>, Vec<Token>)>,
    simple_defines: FxHashMap<SmolStr, Vec<Token>>,
    i: &'b mut usize,
}

impl<'a, 'b> PreProcessor<'a, 'b> {
    pub fn apply(tokens: &'a mut Vec<SpannedToken>) -> Result<(), Diagnostic> {
        let length = tokens.len();
        let mut i = 0;
        let mut pre_processor = PreProcessor {
            tokens,
            i: &mut i,
            function_defines: Default::default(),
            simple_defines: Default::default(),
        };
        pre_processor.process(&mut (0..length), &Default::default())?;
        pre_processor.remove_marker_tokens();
        std::fs::write(
            "simple-defines.js",
            format!("({:?})", pre_processor.simple_defines).as_bytes(),
        )
        .unwrap();
        std::fs::write(
            "function-defines.js",
            format!("({:?})", pre_processor.function_defines).as_bytes(),
        )
        .unwrap();
        Ok(())
    }

    fn process(
        &mut self,
        span: &mut Span,
        suppress: &FxHashSet<SmolStr>,
    ) -> Result<(), Diagnostic> {
        let mut dirty = false;
        *self.i = span.start;
        while *self.i < span.end {
            if let Some(define_name) = self.parse_define_begin(span)? {
                if self.parse_function_define(span, define_name.clone())? {
                    continue;
                }
                self.parse_simple_define(span, define_name)?;
                continue;
            }
            if self.parse_undef(span)? {
                continue;
            }
            if self.substitute_simple_define(span, suppress)? {
                continue;
            }
            if self.substitute_function_define(span, suppress)? {
                continue;
            }
            if self.substitute_concat(span)? {
                dirty = true;
                continue;
            }
            *self.i += 1;
        }
        if dirty {
            self.process(span, suppress)?;
        }
        Ok(())
    }

    fn remove_marker_tokens(&mut self) {
        self.tokens.retain(|token| {
            !matches!(
                get_token(token),
                Token::Newline | Token::Define | Token::Undef | Token::Backslash
            )
        });
    }

    fn expect_no_eof(&self) -> Result<(), Diagnostic> {
        if *self.i >= self.tokens.len() {
            return Err(Diagnostic {
                kind: DiagnosticKind::UnrecognizedEof(vec![]),
                span: get_span(&self.tokens[*self.i - 1]),
            });
        }
        Ok(())
    }

    fn remove_token(&mut self, span: &mut Span) {
        self.tokens.remove(*self.i);
        span.end -= 1;
    }

    fn parse_define_begin(&mut self, span: &mut Span) -> Result<Option<Token>, Diagnostic> {
        if get_token(&self.tokens[*self.i]) != &Token::Define {
            return Ok(None);
        }
        *self.i += 1;
        self.expect_no_eof()?;
        let name = get_token(&self.tokens[*self.i]).clone();
        *self.i -= 1;
        self.remove_token(span);
        self.remove_token(span);
        Ok(Some(name))
    }

    fn parse_function_define(
        &mut self,
        span: &mut Span,
        define_name: Token,
    ) -> Result<bool, Diagnostic> {
        if !matches!(get_token(&self.tokens[*self.i]), Token::LParen) {
            return Ok(false);
        }
        *self.i += 1;
        self.expect_no_eof()?;
        let mut name = get_token(&self.tokens[*self.i]).clone();
        *self.i -= 1;
        if !matches!(name, Token::Name(_) | Token::LParen) {
            return Ok(false);
        }
        self.remove_token(span);
        let mut args = vec![];
        while name != Token::RParen {
            if name != Token::Comma {
                args.push(name);
            }
            self.remove_token(span);
            self.expect_no_eof()?;
            name = get_token(&self.tokens[*self.i]).clone();
        }
        self.remove_token(span);
        let body = self.parse_define_body(span)?;
        self.function_defines
            .insert(define_name.to_string().into(), (args, body));
        Ok(true)
    }

    fn parse_simple_define(
        &mut self,
        span: &mut Span,
        define_name: Token,
    ) -> Result<(), Diagnostic> {
        let define_name: SmolStr = define_name.to_string().into();
        let body = self.parse_define_body(span)?;
        self.simple_defines.insert(define_name.clone(), body);
        Ok(())
    }

    fn parse_define_body(&mut self, span: &mut Span) -> Result<Vec<Token>, Diagnostic> {
        let mut body = vec![];
        self.expect_no_eof()?;
        let mut token = get_token(&self.tokens[*self.i]);
        loop {
            if token == &Token::Backslash {
                self.remove_token(span);
                self.expect_no_eof()?;
                self.remove_token(span);
                self.expect_no_eof()?;
                token = get_token(&self.tokens[*self.i]);
            }
            if token == &Token::Newline {
                break;
            }
            body.push(token.clone());
            self.remove_token(span);
            self.expect_no_eof()?;
            token = get_token(&self.tokens[*self.i]);
        }
        self.remove_token(span);
        Ok(body)
    }

    fn substitute_simple_define(
        &mut self,
        span: &mut Span,
        suppress: &FxHashSet<SmolStr>,
    ) -> Result<bool, Diagnostic> {
        let macro_name_token = get_token(&self.tokens[*self.i]);
        let macro_name_span = get_span(&self.tokens[*self.i]);
        let macro_name: SmolStr = macro_name_token.to_string().into();
        let Some(simple_define_body) = self.simple_defines.get(&*macro_name) else {
            return Ok(false);
        };
        if suppress.contains(&*macro_name) {
            return Ok(false);
        }
        self.tokens.splice(
            *self.i..(*self.i + 1),
            simple_define_body
                .iter()
                .map(|token| (macro_name_span.start, token.clone(), macro_name_span.end)),
        );
        let mut suppress = suppress.clone();
        suppress.insert(macro_name);
        span.end += simple_define_body.len() - 1;
        let subspan_end = *self.i + simple_define_body.len();
        let mut subspan = *self.i..subspan_end;
        self.process(&mut subspan, &suppress)?;
        span.end =
            ((span.end as isize) + ((subspan.end as isize) - (subspan_end as isize))) as usize;
        Ok(true)
    }

    fn substitute_function_define(
        &mut self,
        span: &mut Span,
        suppress: &FxHashSet<SmolStr>,
    ) -> Result<bool, Diagnostic> {
        let macro_name_token = get_token(&self.tokens[*self.i]);
        let macro_name_span = get_span(&self.tokens[*self.i]);
        let macro_name: SmolStr = macro_name_token.to_string().into();
        let Some((function_define_params, function_define_body)) =
            self.function_defines.get(&*macro_name).cloned()
        else {
            return Ok(false);
        };
        if suppress.contains(&*macro_name) {
            return Ok(false);
        }
        if get_token(&self.tokens[*self.i + 1]) != &Token::LParen {
            return Ok(false);
        }
        self.remove_token(span);
        self.remove_token(span);
        self.expect_no_eof()?;
        let mut token = get_token(&self.tokens[*self.i]).clone();
        let mut args = vec![];
        let mut arg = vec![];
        let mut parens = 0;
        while parens >= 0 {
            match token {
                Token::LParen => {
                    parens += 1;
                    arg.push(token);
                }
                Token::RParen => {
                    parens -= 1;
                    if parens < 0 {
                        args.push(arg);
                        arg = vec![];
                    } else {
                        arg.push(token);
                    }
                }
                Token::Comma if parens == 0 => {
                    args.push(arg);
                    arg = vec![];
                }
                _ => {
                    arg.push(token);
                }
            }
            self.remove_token(span);
            token = get_token(&self.tokens[*self.i]).clone();
        }
        if args.len() != function_define_params.len() {
            return Err(Diagnostic {
                kind: DiagnosticKind::MacroArgsCountMismatch {
                    expected: function_define_params.len(),
                    given: args.len(),
                },
                span: macro_name_span,
            });
        }
        let mut i = *self.i;
        for token in function_define_body {
            if let Some(position) = function_define_params
                .iter()
                .position(|param| param == &token)
            {
                for token in &args[position] {
                    self.tokens.insert(
                        i,
                        (macro_name_span.start, token.clone(), macro_name_span.end),
                    );
                    i += 1;
                    span.end += 1;
                }
            } else {
                self.tokens
                    .insert(i, (macro_name_span.start, token, macro_name_span.end));
                i += 1;
                span.end += 1;
            }
        }
        let mut suppress = suppress.clone();
        suppress.insert(macro_name);
        let mut subspan = *self.i..i;
        self.process(&mut subspan, &suppress)?;
        span.end = ((span.end as isize) + ((subspan.end as isize) - (i as isize))) as usize;
        Ok(true)
    }

    fn parse_undef(&mut self, span: &mut Span) -> Result<bool, Diagnostic> {
        if get_token(&self.tokens[*self.i]) != &Token::Undef {
            return Ok(false);
        }
        self.remove_token(span);
        self.expect_no_eof()?;
        let name = get_token(&self.tokens[*self.i]).clone();
        self.simple_defines.remove(name.to_string().as_str());
        self.function_defines.remove(name.to_string().as_str());
        self.remove_token(span);
        Ok(true)
    }

    fn substitute_concat(&mut self, span: &mut Span) -> Result<bool, Diagnostic> {
        let Token::Name(macro_name) = get_token(&self.tokens[*self.i]) else {
            return Ok(false);
        };
        let macro_name_span = get_span(&self.tokens[*self.i]);
        if macro_name != "CONCAT" {
            return Ok(false);
        }
        if self
            .tokens
            .get(*self.i + 1)
            .is_none_or(|token| get_token(token) != &Token::LParen)
        {
            return Ok(false);
        }
        if self
            .tokens
            .get(*self.i + 3)
            .is_none_or(|token| get_token(token) != &Token::Comma)
        {
            return Ok(false);
        }
        if self
            .tokens
            .get(*self.i + 5)
            .is_none_or(|token| get_token(token) != &Token::RParen)
        {
            return Ok(false);
        }
        let Some(Token::Name(left)) = self.tokens.get(*self.i + 2).map(get_token).cloned() else {
            return Ok(false);
        };
        let Some(Token::Name(right)) = self.tokens.get(*self.i + 4).map(get_token).cloned() else {
            return Ok(false);
        };
        self.remove_token(span);
        self.remove_token(span);
        self.remove_token(span);
        self.remove_token(span);
        self.remove_token(span);
        self.remove_token(span);
        self.tokens.insert(
            *self.i,
            (
                macro_name_span.start,
                Token::Name(format!("{}{}", left, right).into()),
                macro_name_span.end,
            ),
        );
        span.end += 1;
        Ok(true)
    }
}
