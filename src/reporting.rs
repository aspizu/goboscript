use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use colored::*;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use lalrpop_util::ParseError;
use logos::Span;
use strum::VariantArray;

use crate::{
    ast::{Block, Reporter},
    codegen::KEYS,
    details::{block_details, reporter_details},
    lexer::Token,
    visitors::ProcedurePrototype,
};

pub enum ParserError<'src> {
    InvalidToken(Span),
    UnknownReporter(&'src str, Span),
}

pub enum ReportLevel {
    Warning,
    Error,
}

impl ReportLevel {
    fn colored(&self) -> ColoredString {
        match self {
            ReportLevel::Warning => "Warning".yellow().bold(),
            ReportLevel::Error => "Error".red().bold(),
        }
    }
}

pub enum Report<'src> {
    ParserError(ParseError<usize, Token<'src>, ParserError<'src>>),
    StageNotFound,
    NoCostumes,
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
    TooFewArgsForProcedure {
        procedure: ProcedurePrototype<'src>,
        given: usize,
        span: Span,
    },
    TooManyArgsForProcedure {
        procedure: ProcedurePrototype<'src>,
        given: usize,
        span: Span,
    },
    UndefinedVariable(&'src str, Span),
    UndefinedArg {
        procedure: ProcedurePrototype<'src>,
        name: &'src str,
        span: Span,
    },
    UndefinedList(&'src str, Span),
    UndefinedBlock(&'src str, Span),
    UnreachableCode(Span),
}

pub struct ReportScope<'a, 'b> {
    pub variables: Option<&'a HashSet<&'b str>>,
    pub global_variables: Option<&'a HashSet<&'b str>>,
    pub lists: Option<&'a HashSet<&'b str>>,
    pub global_lists: Option<&'a HashSet<&'b str>>,
    pub procedures: Option<&'b HashMap<&'b str, ProcedurePrototype<'b>>>,
}

impl<'src> Report<'src> {
    pub fn level(&self) -> ReportLevel {
        match self {
            Self::UnknownKey(..) | Self::UnreachableCode(..) => ReportLevel::Warning,
            _ => ReportLevel::Error,
        }
    }

    fn description(&self) -> &str {
        match self {
            Self::ParserError(ParseError::InvalidToken { .. }) => "Syntax error.",
            Self::ParserError(ParseError::ExtraToken { .. }) => "Syntax error.",
            Self::ParserError(ParseError::UnrecognizedToken { .. }) => "Syntax error.",
            Self::ParserError(ParseError::UnrecognizedEof { .. }) => "Syntax error.",
            Self::ParserError(ParseError::User { error }) => match error {
                ParserError::InvalidToken(..) => "Syntax error.",
                ParserError::UnknownReporter(..) => "Unknown reporter.",
            },
            Self::StageNotFound => "Stage not found.",
            Self::NoCostumes => "No costumes defined.",
            Self::FileNotFound(..) => "File not found.",
            Self::UnknownKey(..) => "Unknown key.",
            Self::TooFewArgsForReporter { .. } => "Missing arguments for reporter.",
            Self::TooManyArgsForReporter { .. } => "Too many arguments for reporter.",
            Self::TooFewArgsForBlock { .. } => "Missing arguments for block.",
            Self::TooManyArgsForBlock { .. } => "Too many arguments for block.",
            Self::TooFewArgsForProcedure { .. } => "Missing arguments for procedure.",
            Self::TooManyArgsForProcedure { .. } => "Too many arguments for procedure.",
            Self::UndefinedVariable(..) => "Undefined variable.",
            Self::UndefinedArg { .. } => "Undefined argument.",
            Self::UndefinedList(..) => "Undefined list.",
            Self::UndefinedBlock(..) => "Unknown block or undefined procedure.",
            Self::UnreachableCode(..) => "Unreachable code.",
        }
    }

    pub fn eprint(&self, path: &str, src: &str, scope: ReportScope) {
        self.eprint_header();
        match self {
            Report::ParserError(err) => match err {
                ParseError::InvalidToken { location } => {
                    eprint_span(*location..*location + 1, path, src, "");
                }
                ParseError::UnrecognizedEof { location, expected } => {
                    eprint_span(
                        *location..*location + 1,
                        path,
                        src,
                        &expected.join(", "),
                    );
                }
                ParseError::UnrecognizedToken {
                    token: (left, token, right),
                    expected,
                } => {
                    eprint_span(
                        *left..*right,
                        path,
                        src,
                        &format!("Expected {}", expected.join(", ")),
                    );
                }
                ParseError::ExtraToken { token: (left, token, right) } => {
                    eprint_span(*left..*right, path, src, "");
                }
                ParseError::User { error } => match error {
                    ParserError::InvalidToken(span) => {
                        eprint_span(span.clone(), path, src, "");
                    }
                    ParserError::UnknownReporter(token, span) => {
                        eprint_span(span.clone(), path, src, "");
                    }
                },
            },
            Report::StageNotFound => {
                eprintln!("All projects must have a `stage.gs` file with atleast one costume defined.");
            }
            Report::NoCostumes => {
                eprint_path(path);
                eprintln!(
                    "All sprites and the stage must have atleast one costume defined."
                );
            }
            Report::FileNotFound(file, span) => {
                eprint_span(span.clone(), path, src, "");
            }
            Report::UnknownKey(key, span) => {
                let mut matcher = SkimMatcherV2::default();
                let mut matches = KEYS
                    .iter()
                    .filter_map(|choice| {
                        matcher.fuzzy_match(choice, key).map(|score| (choice, score))
                    })
                    .collect::<Vec<_>>();
                matches.sort_by_key(|(_, score)| *score);
                if let Some((&m, _)) = matches.last() {
                    eprint_span(
                        span.clone(),
                        path,
                        src,
                        &format!("Did you mean `{m}`?"),
                    );
                } else {
                    eprint_span(span.clone(), path, src, "");
                }
            }
            Report::TooFewArgsForReporter { reporter, given, span } => {
                let (_, args, _) = reporter_details(*reporter);
                eprint_span(
                    span.clone(),
                    path,
                    src,
                    &format!("Missing {}", args[*given..].join(", ")),
                );
            }
            Report::TooManyArgsForReporter { reporter, given, span } => {
                let (_, args, _) = reporter_details(*reporter);
                eprint_span(
                    span.clone(),
                    path,
                    src,
                    &format!("Takes {}", args.join(", ")),
                );
            }
            Report::TooFewArgsForBlock { block, given, span } => {
                let (_, args, _) = block_details(*block);
                eprint_span(
                    span.clone(),
                    path,
                    src,
                    &format!("Missing {}", args[*given..].join(", ")),
                );
            }
            Report::TooManyArgsForBlock { block, given, span } => {
                let (_, args, _) = block_details(*block);
                eprint_span(
                    span.clone(),
                    path,
                    src,
                    &format!("Takes {}", args.join(", ")),
                );
            }
            Report::TooFewArgsForProcedure { procedure, given, span } => {
                let missing = procedure.args[*given..]
                    .iter()
                    .map(|&(arg, _)| arg)
                    .collect::<Vec<_>>()
                    .join(", ");
                eprint_span(span.clone(), path, src, &format!("Missing {missing}"));
            }
            Report::TooManyArgsForProcedure { procedure, given, span } => {
                let args = procedure
                    .args
                    .iter()
                    .map(|&(arg, _)| arg)
                    .collect::<Vec<_>>()
                    .join(", ");
                eprint_span(span.clone(), path, src, &format!("Takes {args}"));
            }
            Report::UndefinedArg { procedure, name, span } => {
                let mut matcher = SkimMatcherV2::default();
                let mut matches = procedure
                    .args
                    .iter()
                    .filter_map(|(arg, _)| {
                        matcher.fuzzy_match(arg, name).map(|score| (arg, score))
                    })
                    .collect::<Vec<_>>();
                matches.sort_by_key(|(_, score)| *score);
                if let Some((&m, _)) = matches.last() {
                    eprint_span(
                        span.clone(),
                        path,
                        src,
                        &format!("Did you mean `{m}`?"),
                    );
                } else {
                    eprint_span(
                        span.clone(),
                        path,
                        src,
                        &format!("Undefined argument `{name}`"),
                    );
                }
            }
            | Report::UndefinedVariable(name, span)
            | Report::UndefinedList(name, span) => {
                let empty = HashSet::new();
                let matcher = SkimMatcherV2::default();
                let mut matches = scope
                    .variables
                    .unwrap_or(&empty)
                    .iter()
                    .chain(scope.global_variables.unwrap_or(&empty))
                    .chain(scope.lists.unwrap_or(&empty))
                    .chain(scope.global_lists.unwrap_or(&empty))
                    .filter_map(|choice| {
                        matcher.fuzzy_match(choice, name).map(|score| (choice, score))
                    })
                    .collect::<Vec<_>>();
                matches.sort_by_key(|(_, score)| *score);
                if let Some((&m, _)) = matches.last() {
                    eprint_span(
                        span.clone(),
                        path,
                        src,
                        &format!("Did you mean `{m}`?"),
                    );
                } else {
                    eprint_span(span.clone(), path, src, "");
                }
            }
            Report::UndefinedBlock(name, span) => {
                let matcher = SkimMatcherV2::default();
                let mut matches = Vec::new();
                if let Some(procedures) = scope.procedures {
                    for procedure in procedures.keys() {
                        if let Some(score) = matcher.fuzzy_match(procedure, name) {
                            matches.push((*procedure, score));
                        }
                    }
                }
                for block in Block::VARIANTS {
                    let block = block.as_ref();
                    if let Some(score) = matcher.fuzzy_match(block, name) {
                        matches.push((block, score));
                    }
                }
                matches.sort_by_key(|(_, score)| *score);
                if let Some((m, _)) = matches.last() {
                    eprint_span(
                        span.clone(),
                        path,
                        src,
                        &format!("Did you mean `{m}`?"),
                    );
                } else {
                    eprint_span(span.clone(), path, src, "");
                }
            }
            Report::UnreachableCode(span) => {
                eprint_span(
                    span.clone(),
                    path,
                    src,
                    "Code after this statement was removed.",
                );
            }
        }
    }

    fn eprint_header(&self) {
        eprintln!("{}: {}", self.level().colored(), self.description().bold());
    }
}

fn eprint_path(path: &str) {
    eprintln!("{} {}", "-->".blue().bold(), path.bold());
}

fn eprint_span(span: Span, path: &str, src: &str, hint: &str) {
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
