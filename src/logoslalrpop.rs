use logos::*;

use crate::{lexer::*, reporting::*};

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'a> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'a, Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Spanned<Token<'a>, usize, ParserError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| {
            if let Ok(token) = token {
                Ok((span.start, token, span.end))
            } else {
                Err(ParserError::InvalidToken(span))
            }
        })
    }
}
