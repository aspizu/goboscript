use std::rc::Rc;

use colored::*;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use lalrpop_util::ParseError;
use logos::Span;

use crate::{
    ast::{Block, Reporter},
    build::FunctionPrototype,
    codegen::KEYS,
    lexer::Token,
};

pub enum ParserError<'src> {
    InvalidToken(Span),
    UnknownReporter(&'src str, Span),
}

pub enum ReportLevel {
    Warning,
    Error,
}

pub enum Report<'src> {
    ParserError(ParseError<usize, Token<'src>, ParserError<'src>>),
    FileNotFound(Rc<str>, Span),
    UnknownKey(Rc<str>, Span),
    TooFewArgsForReporter {
        reporter: Reporter,
        given: usize,
        span: Span,
    },
    TooManyArgsForReporter {
        reporter: Reporter,
        given: usize,
        span: Span,
    },
    TooFewArgsForBlock {
        block: Block,
        given: usize,
        span: Span,
    },
    TooManyArgsForBlock {
        block: Block,
        given: usize,
        span: Span,
    },
    TooFewArgsForFunction {
        function: FunctionPrototype<'src>,
        given: usize,
        span: Span,
    },
    TooManyArgsForFunction {
        function: FunctionPrototype<'src>,
        given: usize,
        span: Span,
    },
    UndefinedVariable(&'src str, Span),
    UndefinedArg(&'src str, Span),
    UndefinedList(&'src str, Span),
    UndefinedBlock(&'src str, Span),
    UnreachableCode(Span),
}

impl<'src> Report<'src> {
    #[rustfmt::skip]
    pub fn level(&self) -> ReportLevel {
        match self {
            | Report::ParserError(..)
            | Report::FileNotFound(..)
            | Report::TooFewArgsForReporter { .. }
            | Report::TooManyArgsForReporter { .. }
            | Report::TooFewArgsForBlock { .. }
            | Report::TooManyArgsForBlock { .. }
            | Report::TooFewArgsForFunction { .. }
            | Report::TooManyArgsForFunction { .. }
            | Report::UndefinedVariable(..)
            | Report::UndefinedArg(..)
            | Report::UndefinedList(..)
            | Report::UndefinedBlock(..)
            => ReportLevel::Error,

            | Report::UnknownKey(..)
            | Report::UnreachableCode(..)
            => ReportLevel::Warning
        }
    }

    pub fn print(&self, path: &str, src: &str) {
        match self {
            Report::ParserError(error) => match error {
                ParseError::InvalidToken { location } => display_error(
                    path,
                    src,
                    self.level(),
                    "Invalid token.",
                    *location..location + 1,
                    "",
                ),
                ParseError::ExtraToken { token: (l, _, r) } => {
                    display_error(path, src, self.level(), "Extra token.", *l..*r, "")
                }
                ParseError::UnrecognizedToken { token: (l, _, r), expected } => {
                    let hint = format!("Expected one of: {}", expected.join(", "));
                    display_error(
                        path,
                        src,
                        self.level(),
                        "Unrecognized token.",
                        *l..*r,
                        if expected.is_empty() {
                            "Expected nothing."
                        } else {
                            hint.as_str()
                        },
                    )
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    let hint = format!("Expected one of: {}", expected.join(", "));
                    display_error(
                        path,
                        src,
                        self.level(),
                        "Unrecognized end of file.",
                        *location..location + 1,
                        if expected.is_empty() {
                            "Expected nothing."
                        } else {
                            hint.as_str()
                        },
                    )
                }
                ParseError::User { error } => match error {
                    ParserError::InvalidToken(span) => display_error(
                        path,
                        src,
                        self.level(),
                        "Invalid token.",
                        span.clone(),
                        "",
                    ),
                    ParserError::UnknownReporter(name, span) => display_error(
                        path,
                        src,
                        self.level(),
                        "Unknown reporter.",
                        span.clone(),
                        name,
                    ),
                },
            },
            Report::FileNotFound(file, span) => display_error(
                path,
                src,
                self.level(),
                "File not found",
                span.clone(),
                file,
            ),
            Report::UnknownKey(key, span) => {
                let matcher = SkimMatcherV2::default().ignore_case();
                let matched = KEYS
                    .iter()
                    .filter_map(|valid_key| {
                        matcher
                            .fuzzy_match(valid_key, key)
                            .map(|score| (*valid_key, score))
                    })
                    .max_by_key(|item| item.1)
                    .map(|item| item.0);
                if let Some(matched) = matched {
                    display_error(
                        path,
                        src,
                        self.level(),
                        "Not a key",
                        span.clone(),
                        format!("Did you mean `{}`?", matched).as_str(),
                    )
                } else {
                    display_error(
                        path,
                        src,
                        self.level(),
                        "Not a key",
                        span.clone(),
                        "",
                    )
                }
            }
            Report::TooFewArgsForReporter { reporter: _, given: _, span } => {
                display_error(
                    path,
                    src,
                    self.level(),
                    "Too few arguments for reporter",
                    span.clone(),
                    "",
                )
            }
            Report::TooManyArgsForReporter { reporter: _, given: _, span } => {
                display_error(
                    path,
                    src,
                    self.level(),
                    "Too many arguments for reporter",
                    span.clone(),
                    "",
                )
            }
            Report::TooFewArgsForBlock { block: _, given: _, span } => display_error(
                path,
                src,
                self.level(),
                "Too few arguments for block",
                span.clone(),
                "",
            ),
            Report::TooManyArgsForBlock { block: _, given: _, span } => display_error(
                path,
                src,
                self.level(),
                "Too many arguments for block",
                span.clone(),
                "",
            ),
            Report::TooFewArgsForFunction { function: _, given: _, span } => {
                display_error(
                    path,
                    src,
                    self.level(),
                    "Too few arguments for function",
                    span.clone(),
                    "",
                )
            }
            Report::TooManyArgsForFunction { function: _, given: _, span } => {
                display_error(
                    path,
                    src,
                    self.level(),
                    "Too many arguments for function",
                    span.clone(),
                    "",
                )
            }
            Report::UndefinedVariable(name, span) => display_error(
                path,
                src,
                self.level(),
                "Undefined variable",
                span.clone(),
                name,
            ),
            Report::UndefinedArg(name, span) => display_error(
                path,
                src,
                self.level(),
                "Undefined argument",
                span.clone(),
                name,
            ),
            Report::UndefinedList(name, span) => display_error(
                path,
                src,
                self.level(),
                "Undefined list",
                span.clone(),
                name,
            ),
            Report::UndefinedBlock(name, span) => display_error(
                path,
                src,
                self.level(),
                "Undefined block",
                span.clone(),
                name,
            ),
            Report::UnreachableCode(span) => display_error(
                path,
                src,
                self.level(),
                "Unreachable code",
                span.clone(),
                "Code after this is removed",
            ),
        }
    }
}

pub fn display_error(
    path: &str,
    src: &str,
    level: ReportLevel,
    code: &str,
    span: Span,
    hint: &str,
) {
    let mut line = 0;
    let mut column = 0;
    let mut i = 0;
    for (n, ln) in src.lines().enumerate() {
        line = n;
        column = span.start - i;
        i += ln.len() + 1;
        if i >= span.start {
            break;
        }
    }
    eprintln!(
        "{}: {}",
        match level {
            ReportLevel::Warning => "Warning".yellow().bold(),
            ReportLevel::Error => "Error".red().bold(),
        },
        code.bold()
    );
    eprintln!(" {} {}:{}:{}", "-->".blue().bold(), path.bold(), 1 + line, 1 + column,);
    let mut i = 0;
    for (n, line) in src.lines().enumerate() {
        if i <= span.start && span.start <= i + line.len() {
            eprintln!("{}{line}", format!("{: >4} | ", n + 1).blue().bold());
            eprintln!(
                "     {} {}{} {}",
                "|".blue().bold(),
                " ".repeat(span.start - i),
                "^".repeat(span.end - span.start).blue().bold(),
                hint.blue().bold(),
            );
            break;
        }
        i += line.len() + 1;
    }
}

pub struct Summary {
    pub warnings: usize,
    pub errors: usize,
}

impl Summary {
    pub fn new() -> Self {
        Self { warnings: 0, errors: 0 }
    }

    pub fn summarize(&mut self, reports: &Vec<Report>) {
        for report in reports {
            match report.level() {
                ReportLevel::Warning => self.warnings += 1,
                ReportLevel::Error => self.errors += 1,
            }
        }
    }
}
