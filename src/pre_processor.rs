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
type FunctionDefines = FxHashMap<SmolStr, FxHashMap<usize, (Vec<Token>, Vec<Token>)>>;

fn get_span(token: &SpannedToken) -> Span {
    token.0..token.2
}

fn get_token(token: &SpannedToken) -> &Token {
    &token.1
}

pub struct PreProcessor<'a, 'b> {
    tokens: &'a mut Vec<(usize, Token, usize)>,
    function_defines: FunctionDefines,
    simple_defines: FxHashMap<SmolStr, Vec<Token>>,
    i: &'b mut usize,
}

impl<'a> PreProcessor<'a, '_> {
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
            if self.substitute_concat(span, suppress)? {
                dirty = true;
                continue;
            }
            if self.substitute_fstring(span)? {
                dirty = true;
                continue;
            }
            if self.substitute_stringify(span)? {
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
            let span = if *self.i > 0 {
                get_span(&self.tokens[*self.i - 1])
            } else {
                0..0
            };
            return Err(Diagnostic {
                kind: DiagnosticKind::UnrecognizedEof(vec![]),
                span,
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
        if !matches!(name, Token::Name(_) | Token::RParen) {
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
        let arity = args.len();
        let body = self.parse_define_body(span)?;
        let define_name: SmolStr = define_name.to_string().into();
        self.function_defines
            .entry(define_name)
            .or_default()
            .insert(arity, (args, body));
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
        if simple_define_body.is_empty() {
            self.tokens.remove(*self.i);
            span.end = span.end.saturating_sub(1);
            return Ok(true);
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

        let Some(overloads) = self.function_defines.get(&macro_name).cloned() else {
            return Ok(false);
        };
        if suppress.contains(&*macro_name) {
            return Ok(false);
        }
        if self
            .tokens
            .get(*self.i + 1)
            .is_none_or(|token| get_token(token) != &Token::LParen)
        {
            return Ok(false);
        }

        let (_, args) = self.parse_macro_call_args(span)?;

        let arity = args.len();
        let Some((function_define_params, function_define_body)) = overloads.get(&arity).cloned()
        else {
            return Err(Diagnostic {
                kind: DiagnosticKind::MacroArgsCountMismatch {
                    expected: overloads.keys().next().copied().unwrap_or(0),
                    given: arity,
                },
                span: macro_name_span,
            });
        };

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
        let name: SmolStr = get_token(&self.tokens[*self.i]).to_string().into();
        // Remove all overloads for this name.
        self.function_defines.remove(&name);
        self.simple_defines.remove(&*name);
        self.remove_token(span);
        Ok(true)
    }

    fn substitute_stringify(&mut self, span: &mut Span) -> Result<bool, Diagnostic> {
        let Token::Name(macro_name) = get_token(&self.tokens[*self.i]) else {
            return Ok(false);
        };
        let macro_name_span = get_span(&self.tokens[*self.i]);
        if macro_name != "STRINGIFY" {
            return Ok(false);
        }
        if self
            .tokens
            .get(*self.i + 1)
            .is_none_or(|token| get_token(token) != &Token::LParen)
        {
            return Ok(false);
        }
        // Remove STRINGIFY and '('
        self.remove_token(span);
        self.remove_token(span);
        self.expect_no_eof()?;
        // Collect all tokens until the matching ')'
        let mut parts: Vec<String> = vec![];
        let mut parens: i32 = 0;
        loop {
            let token = get_token(&self.tokens[*self.i]).clone();
            if token == Token::RParen && parens == 0 {
                self.remove_token(span);
                break;
            }
            match token {
                Token::LParen => parens += 1,
                Token::RParen => parens -= 1,
                _ => {}
            }
            parts.push(token.to_string());
            self.remove_token(span);
            self.expect_no_eof()?;
        }
        let stringified: SmolStr = parts.join(" ").into();
        self.tokens.insert(
            *self.i,
            (
                macro_name_span.start,
                Token::Str(stringified),
                macro_name_span.end,
            ),
        );
        span.end += 1;
        Ok(true)
    }

    fn parse_macro_call_args(
        &mut self,
        span: &mut Span,
    ) -> Result<(Span, Vec<Vec<Token>>), Diagnostic> {
        self.remove_token(span);

        if get_token(&self.tokens[*self.i]) != &Token::LParen {
            return Err(Diagnostic {
                kind: DiagnosticKind::UnrecognizedToken(
                    get_token(&self.tokens[*self.i]).clone(),
                    vec!["(".to_string()],
                ),
                span: get_span(&self.tokens[*self.i]),
            });
        }

        self.remove_token(span);
        self.expect_no_eof()?;

        let args_start = *self.i;

        let mut token = get_token(&self.tokens[*self.i]).clone();

        let mut args: Vec<Vec<Token>> = vec![];
        let mut arg: Vec<Token> = vec![];

        if token != Token::RParen {
            let mut parens = 0;

            while parens >= 0 {
                match token {
                    Token::LParen => {
                        parens += 1;
                        arg.push(token.clone());
                    }

                    Token::RParen => {
                        parens -= 1;

                        if parens < 0 {
                            args.push(arg);
                            arg = vec![];
                        } else {
                            arg.push(token.clone());
                        }
                    }

                    Token::Comma if parens == 0 => {
                        args.push(arg);
                        arg = vec![];
                    }

                    _ => {
                        arg.push(token.clone());
                    }
                }

                self.remove_token(span);
                if parens >= 0 {
                    self.expect_no_eof()?;
                    token = get_token(&self.tokens[*self.i]).clone();
                }
            }
        } else {
            self.remove_token(span);
        }

        let args_end = *self.i;

        Ok((args_start..args_end, args))
    }

    fn expand_token_list(
        &mut self,
        tokens: Vec<Token>,
        suppress: &FxHashSet<SmolStr>,
    ) -> Result<Vec<Token>, Diagnostic> {
        let mut spanned: Vec<SpannedToken> =
            tokens.into_iter().map(|token| (0, token, 0)).collect();

        let length = spanned.len();

        if length > 0 {
            let mut i = 0;

            let mut pre_processor = PreProcessor {
                tokens: &mut spanned,
                i: &mut i,
                function_defines: self.function_defines.clone(),
                simple_defines: self.simple_defines.clone(),
            };

            pre_processor.process(&mut (0..length), suppress)?;
            pre_processor.remove_marker_tokens();
        }

        Ok(spanned.into_iter().map(|(_, token, _)| token).collect())
    }

    fn peek_macro_call_args(&self) -> Result<Option<Vec<Vec<Token>>>, Diagnostic> {
        let mut i = *self.i + 1;
        if i >= self.tokens.len() || get_token(&self.tokens[i]) != &Token::LParen {
            return Ok(None);
        }
        i += 1;

        if i >= self.tokens.len() {
            return Err(Diagnostic {
                kind: DiagnosticKind::UnrecognizedEof(vec![]),
                span: get_span(&self.tokens[i - 1]),
            });
        }

        let mut args: Vec<Vec<Token>> = vec![];
        let mut arg: Vec<Token> = vec![];
        let mut token = get_token(&self.tokens[i]).clone();

        if token != Token::RParen {
            let mut parens = 0;

            while parens >= 0 {
                match token {
                    Token::LParen => {
                        parens += 1;
                        arg.push(token.clone());
                    }

                    Token::RParen => {
                        parens -= 1;

                        if parens < 0 {
                            args.push(arg);
                            arg = vec![];
                        } else {
                            arg.push(token.clone());
                        }
                    }

                    Token::Comma if parens == 0 => {
                        args.push(arg);
                        arg = vec![];
                    }

                    _ => {
                        arg.push(token.clone());
                    }
                }

                i += 1;
                if parens >= 0 {
                    if i >= self.tokens.len() {
                        return Err(Diagnostic {
                            kind: DiagnosticKind::UnrecognizedEof(vec![]),
                            span: get_span(&self.tokens[i - 1]),
                        });
                    }
                    token = get_token(&self.tokens[i]).clone();
                }
            }
        }

        Ok(Some(args))
    }

    fn concat_tokens(&self, left: &Token, right: &Token, span: Span) -> Result<Token, Diagnostic> {
        let pasted = format!("{left}{right}");
        let mut lexer = crate::lexer::adaptor::Lexer::new(&pasted);

        let Some(first) = lexer.next() else {
            return Err(Diagnostic {
                kind: DiagnosticKind::InvalidToken,
                span,
            });
        };

        let Ok((_, token, end)) = first else {
            return Err(Diagnostic {
                kind: DiagnosticKind::InvalidToken,
                span,
            });
        };

        if end != pasted.len() {
            return Err(Diagnostic {
                kind: DiagnosticKind::InvalidToken,
                span,
            });
        }

        match lexer.next() {
            None => Ok(token),
            Some(Ok(_)) => Err(Diagnostic {
                kind: DiagnosticKind::InvalidToken,
                span,
            }),
            Some(Err(_)) => Err(Diagnostic {
                kind: DiagnosticKind::InvalidToken,
                span,
            }),
        }
    }

    fn substitute_concat(
        &mut self,
        span: &mut Span,
        suppress: &FxHashSet<SmolStr>,
    ) -> Result<bool, Diagnostic> {
        let Token::Name(macro_name) = get_token(&self.tokens[*self.i]) else {
            return Ok(false);
        };

        if macro_name != "CONCAT" {
            return Ok(false);
        }

        let macro_name_span = get_span(&self.tokens[*self.i]);

        let Some(args) = self.peek_macro_call_args()? else {
            return Ok(false);
        };

        if args.len() != 2 {
            return Err(Diagnostic {
                kind: DiagnosticKind::MacroArgsCountMismatch {
                    expected: 2,
                    given: args.len(),
                },
                span: macro_name_span,
            });
        }

        let left = self.expand_token_list(args[0].clone(), suppress)?;
        let right = self.expand_token_list(args[1].clone(), suppress)?;

        let [left] = &left[..] else {
            return Err(Diagnostic {
                kind: DiagnosticKind::InvalidToken,
                span: macro_name_span,
            });
        };

        let [right] = &right[..] else {
            return Err(Diagnostic {
                kind: DiagnosticKind::InvalidToken,
                span: macro_name_span,
            });
        };

        let token = self.concat_tokens(left, right, macro_name_span.clone())?;

        let (_, _) = self.parse_macro_call_args(span)?;

        self.tokens
            .insert(*self.i, (macro_name_span.start, token, macro_name_span.end));

        span.end += 1;

        Ok(true)
    }

    fn substitute_fstring(&mut self, span: &mut Span) -> Result<bool, Diagnostic> {
        let Token::Name(prefix) = get_token(&self.tokens[*self.i]) else {
            return Ok(false);
        };
        if prefix != "f" {
            return Ok(false);
        }
        let Some(format_token) = self.tokens.get(*self.i + 1) else {
            return Ok(false);
        };
        let Token::Str(format) = get_token(format_token) else {
            return Ok(false);
        };
        let tokens = Self::tokenize_fmt(format, get_span(format_token))?;
        self.remove_token(span);
        self.remove_token(span);
        for (i, token) in (*self.i..).zip(tokens) {
            self.tokens.insert(i, token);
            span.end += 1;
        }
        Ok(true)
    }

    fn tokenize_fmt(format: &str, span: Span) -> Result<Vec<SpannedToken>, Diagnostic> {
        let mut tokens = vec![];
        let mut literal_start = 0;
        let mut literal = String::new();
        let content_start = span.start + 1;
        let mut chars = format.char_indices().peekable();
        while let Some((index, char)) = chars.next() {
            if char == '{' {
                if chars.peek().is_some_and(|(_, char)| char == &'{') {
                    chars.next();
                    if literal.is_empty() {
                        literal_start = index;
                    }
                    literal.push('{');
                    continue;
                }
                if !literal.is_empty() {
                    tokens.push((
                        content_start + literal_start,
                        Token::Str(literal.clone().into()),
                        content_start + index,
                    ));
                    literal.clear();
                }
                let expr_start = index + char.len_utf8();
                let mut expr = String::new();
                let mut closed = false;
                let mut expr_end = format.len();
                while let Some((index, char)) = chars.next() {
                    if char == '{' && chars.peek().is_some_and(|(_, char)| char == &'{') {
                        chars.next();
                        expr.push('{');
                        continue;
                    }
                    if char == '}' && chars.peek().is_some_and(|(_, char)| char == &'}') {
                        let mut lookahead = chars.clone();
                        lookahead.next();
                        if lookahead.peek().is_some_and(|(_, char)| char == &'}') {
                            closed = true;
                            expr_end = index;
                            break;
                        }
                        chars.next();
                        expr.push('}');
                        continue;
                    }
                    if char == '}' {
                        closed = true;
                        expr_end = index;
                        break;
                    }
                    expr.push(char);
                }
                if !closed {
                    return Err(Diagnostic {
                        kind: DiagnosticKind::UnrecognizedEof(vec![]),
                        span,
                    });
                }
                Self::append_join(&mut tokens);
                for result in crate::lexer::adaptor::Lexer::new(&expr) {
                    match result {
                        Ok((start, token, end)) => {
                            tokens.push((
                                content_start + expr_start + start,
                                token,
                                content_start + expr_start + end,
                            ));
                        }
                        Err(diagnostic) => {
                            return Err(Diagnostic {
                                kind: diagnostic.kind,
                                span: content_start + expr_start + diagnostic.span.start
                                    ..content_start + expr_start + diagnostic.span.end,
                            });
                        }
                    }
                }
                literal_start = expr_end + 1;
            } else if char == '}' {
                if chars.peek().is_some_and(|(_, char)| char == &'}') {
                    chars.next();
                    if literal.is_empty() {
                        literal_start = index;
                    }
                    literal.push('}');
                } else {
                    return Err(Diagnostic {
                        kind: DiagnosticKind::UnrecognizedToken(
                            Token::RBrace,
                            vec!["}}".to_string()],
                        ),
                        span: content_start + index..content_start + index + char.len_utf8(),
                    });
                }
            } else {
                if literal.is_empty() {
                    literal_start = index;
                }
                literal.push(char);
            }
        }
        if !literal.is_empty() {
            Self::append_join(&mut tokens);
            tokens.push((
                content_start + literal_start,
                Token::Str(literal.into()),
                span.end.saturating_sub(1),
            ));
        }
        if tokens.is_empty() {
            tokens.push((span.start, Token::Str("".into()), span.end));
        }
        Ok(tokens)
    }

    fn append_join(tokens: &mut Vec<SpannedToken>) {
        if !tokens.is_empty() {
            let span = tokens.last().map_or(0..0, get_span);
            tokens.push((span.end, Token::Amp, span.end));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::adaptor::Lexer;

    fn preprocess(source: &str) -> Result<Vec<Token>, Diagnostic> {
        let mut tokens: Vec<_> = Lexer::new(source).collect::<Result<_, _>>()?;
        PreProcessor::apply(&mut tokens)?;
        Ok(tokens.into_iter().map(|(_, token, _)| token).collect())
    }

    #[test]
    fn concat_expands_function_macro_arguments_before_pasting() {
        let tokens =
            preprocess("%define SUFFIX bar\n%define C(x) CONCAT(x, SUFFIX)\nC(foo)\n").unwrap();

        assert_eq!(tokens, vec![Token::Name("foobar".into())]);
    }

    #[test]
    fn function_macro_call_at_end_of_expansion_does_not_panic() {
        let tokens = preprocess("%define C(x) x\n%define WRAPPED C(foo)\nWRAPPED\n").unwrap();

        assert_eq!(tokens, vec![Token::Name("foo".into())]);
    }

    #[test]
    fn concat_reprocesses_the_pasted_token() {
        let tokens = preprocess("%define foobar 1\n%define C(x) CONCAT(x, bar)\nC(foo)\n").unwrap();

        assert_eq!(tokens, vec![Token::Int(1)]);
    }

    #[test]
    fn concat_errors_without_removing_tokens_for_invalid_pastes() {
        let mut tokens: Vec<_> = Lexer::new("%define TWO foo bar\nCONCAT(TWO, baz)\n")
            .collect::<Result<_, _>>()
            .unwrap();

        let result = PreProcessor::apply(&mut tokens);

        assert!(result.is_err());
        assert!(!tokens.is_empty());
    }

    #[test]
    fn fmt_interpolates_expression_between_literals() {
        let tokens = preprocess(r#"f"({x})""#).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Str("(".into()),
                Token::Amp,
                Token::Name("x".into()),
                Token::Amp,
                Token::Str(")".into())
            ]
        );
    }

    #[test]
    fn fmt_interpolates_adjacent_expressions() {
        let tokens = preprocess(r#"f"{x}{y}""#).unwrap();

        assert_eq!(
            tokens,
            vec![Token::Name("x".into()), Token::Amp, Token::Name("y".into())]
        );
    }

    #[test]
    fn fmt_tokenizes_expression_contents() {
        let tokens = preprocess(r#"f"x = {x + 1}""#).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Str("x = ".into()),
                Token::Amp,
                Token::Name("x".into()),
                Token::Plus,
                Token::Int(1)
            ]
        );
    }

    #[test]
    fn fmt_tokenizes_nested_fstrings() {
        let tokens = preprocess(r#"f"{1+f\"{{x}}\"}""#).unwrap();

        assert_eq!(
            tokens,
            vec![Token::Int(1), Token::Plus, Token::Name("x".into())]
        );
    }

    #[test]
    fn fmt_escapes_literal_braces() {
        let tokens = preprocess(r#"f"{{{x}}}""#).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Str("{".into()),
                Token::Amp,
                Token::Name("x".into()),
                Token::Amp,
                Token::Str("}".into())
            ]
        );
    }

    #[test]
    fn fmt_expands_empty_string() {
        let tokens = preprocess(r#"f"""#).unwrap();

        assert_eq!(tokens, vec![Token::Str("".into())]);
    }

    #[test]
    fn fmt_errors_for_unterminated_interpolation() {
        let result = preprocess(r#"f"{x""#);

        assert!(result.is_err());
    }
}
