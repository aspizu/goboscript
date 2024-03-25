pub mod keys;

use std::cmp::Ordering;

use colored::Colorize;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use logos::Span;
use smol_str::SmolStr;

use self::keys::all_keys;
use crate::{
    ast::Sprite,
    blocks::{Block, Repr},
    lexer::token::Token,
};

#[derive(Debug)]
pub struct Diagnostic {
    pub detail: DiagnosticDetail,
    pub span: Span,
}

#[derive(Debug)]
pub enum DiagnosticDetail {
    InvalidToken,
    UnrecognizedEof(Vec<String>),
    UnrecognizedToken(Token, Vec<String>),
    ExtraToken(Token),
    FileNotFound(SmolStr),
    FollowedByUnreachableCode,
    UnrecognizedReporter(SmolStr),
    UnrecognizedVariable(SmolStr),
    UnrecognizedProcedure(SmolStr),
    UnrecognizedList(SmolStr),
    UnrecognizedKey(SmolStr),
    UnrecognizedArgument { name: SmolStr, proc: Option<SmolStr> },
    UnrecognizedEnum { enum_name: SmolStr, variant_name: SmolStr },
    UnrecognizedEnumVariant { enum_name: SmolStr, variant_name: SmolStr },
    UnusedVariable(SmolStr),
    UnusedProcedure(SmolStr),
    UnusedList(SmolStr),
    UnusedArgument(SmolStr),
    UnusedEnumVariant { enum_name: SmolStr, variant_name: SmolStr },
    BlockArgsCountMismatch { block: Block, given: usize },
    ReprArgsCountMismatch { repr: Repr, given: usize },
    ProcArgsCountMismatch { proc: SmolStr, given: usize },
    NoCostumes,
}

impl DiagnosticDetail {
    pub fn to_diagnostic(self, span: Span) -> Diagnostic {
        Diagnostic { detail: self, span }
    }

    fn message(&self, sprite: &Sprite) -> &'static str {
        match self {
            Self::InvalidToken => "invalid token",
            Self::UnrecognizedEof(_) => "unrecognized end of file",
            Self::UnrecognizedToken(_, _) => "unrecognized token",
            Self::ExtraToken(_) => "extra token",
            Self::FileNotFound(_) => "file not found",
            Self::FollowedByUnreachableCode => "this is followed by unreachable code",
            Self::UnrecognizedReporter(_) => "unrecognized reporter",
            Self::UnrecognizedVariable(_) => "unrecognized variable",
            Self::UnrecognizedProcedure(_) => "unrecognized block or procedure",
            Self::UnrecognizedList(_) => "unrecognized list",
            Self::UnrecognizedKey(_) => "unrecognized key",
            Self::UnrecognizedArgument { .. } => "unrecognized argument",
            Self::UnrecognizedEnum { .. } => "unrecognized enum",
            Self::UnrecognizedEnumVariant { .. } => "unrecognized enum variant",
            Self::UnusedVariable(_) => "unused variable",
            Self::UnusedProcedure(_) => "unused procedure",
            Self::UnusedList(_) => "unused list",
            Self::UnusedArgument(_) => "unused argument",
            Self::UnusedEnumVariant { .. } => "unused enum variant",
            Self::BlockArgsCountMismatch { block, given } => {
                match given.cmp(&block.args().len()) {
                    Ordering::Less => "too few arguments for block",
                    Ordering::Greater => "too many arguments for block",
                    Ordering::Equal => unreachable!(),
                }
            }
            Self::ReprArgsCountMismatch { repr, given } => {
                match given.cmp(&repr.args().len()) {
                    Ordering::Less => "too few arguments for reporter",
                    Ordering::Greater => "too many arguments for reporter",
                    Ordering::Equal => unreachable!(),
                }
            }
            Self::ProcArgsCountMismatch { proc, given } => {
                match given.cmp(&sprite.procs[proc].args.len()) {
                    Ordering::Less => "too few arguments for procedure",
                    Ordering::Greater => "too many arguments for procedure",
                    Ordering::Equal => unreachable!(),
                }
            }
            Self::NoCostumes => "no costumes declared",
        }
    }

    fn help(&self, sprite: &Sprite) -> Option<String> {
        match self {
            Self::BlockArgsCountMismatch { block, given: _ } => {
                let overloads = Block::overloads(block.name());
                if !overloads.is_empty() {
                    return Some(format!(
                        "this block takes:\n - {}",
                        overloads
                            .iter()
                            .map(|block| block.args().join(", "))
                            .collect::<Vec<_>>()
                            .join("\n - ")
                    ));
                }
                if block.args().is_empty() {
                    return Some("this block takes no arguments".to_string());
                }
                Some(format!("this block takes {}", block.args().join(", ")))
            }
            Self::ReprArgsCountMismatch { repr, given: _ } => {
                let overloads = Repr::overloads(repr.name());
                if !overloads.is_empty() {
                    return Some(format!(
                        "this reporter takes:\n - {}",
                        overloads
                            .iter()
                            .map(|repr| repr.args().join(", "))
                            .collect::<Vec<_>>()
                            .join("\n - ")
                    ));
                }
                if repr.args().is_empty() {
                    return Some("this reporter takes no arguments".to_string());
                }
                Some(format!("this reporter takes {}", repr.args().join(", ")))
            }
            Self::ProcArgsCountMismatch { proc, given: _ } => Some(format!(
                "this procedure takes {}",
                sprite.procs[proc]
                    .args
                    .iter()
                    .map(|(name, _)| name)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            Self::UnrecognizedReporter(name) => {
                get_closest_match(name, Repr::all_names().iter().copied())
            }
            Self::UnrecognizedProcedure(name) => get_closest_match(
                name,
                Block::all_names()
                    .iter()
                    .copied()
                    .chain(sprite.procs.keys().map(SmolStr::as_str)),
            ),
            Self::UnrecognizedArgument { name, proc } => {
                let proc = sprite.procs.get(proc.as_ref()?)?;
                get_closest_match(name, proc.args.iter().map(|(arg, _)| arg.as_str()))
            }
            Self::UnrecognizedVariable(name) => {
                get_closest_match(name, sprite.vars.keys().map(SmolStr::as_str))
            }
            Self::UnrecognizedList(name) => {
                get_closest_match(name, sprite.lists.keys().map(SmolStr::as_str))
            }
            Self::UnrecognizedKey(name) => get_closest_match(name, all_keys()),
            Self::UnrecognizedEnumVariant { enum_name, variant_name } => {
                let enum_ = sprite.enums.get(enum_name)?;
                get_closest_match(
                    variant_name,
                    enum_.variants.iter().map(|(variant, _)| variant.as_str()),
                )
            }
            _ => None,
        }
    }

    fn info(&self) -> Option<String> {
        None
    }
}

impl Diagnostic {
    pub fn eprint(&self, path: &str, src: &str, sprite: &Sprite) {
        let mut line_no = 0;
        let mut col_no = 0;
        let mut i = 0;
        for (line_no1, line) in src.lines().enumerate() {
            if i <= self.span.start && self.span.end <= (i + line.len()) {
                line_no = line_no1;
                col_no = self.span.start - i;
                break;
            }
            i += line.len() + 1;
        }
        eprintln!(
            "{}{} {}",
            "error".red().bold(),
            ":".bold(),
            self.detail.message(sprite).bold(),
        );
        if self.span == (0..0) {
            eprintln!(
                "      {} {}:{}:{}",
                "─→".bold(),
                path.blue(),
                line_no + 1,
                col_no + 1
            );
            return;
        }
        let mut help = self.detail.help(sprite);
        if let DiagnosticDetail::UnrecognizedToken(token, expected) = &self.detail {
            if expected.iter().any(|s| s == "\";\"") {
                line_no -= 1;
                let line = src.lines().nth(line_no).unwrap();
                col_no = line.trim_end().len();
                help = Some("missing semicolon here?".to_string());
            }
        }
        let line = src.lines().nth(line_no).unwrap();
        eprintln!(
            "      {} {}:{}:{}",
            "╭→".bold(),
            path.blue(),
            line_no + 1,
            col_no + 1
        );
        eprintln!("{}", "      │".bold());
        eprintln!("{} {}", format!(" {:4} │", line_no + 1).bold(), line);
        let pad = " ".repeat(col_no);
        let padn = " ".repeat(self.span.len());
        eprintln!(
            "{} {}{} {}",
            "      │".bold(),
            pad,
            "─".repeat(self.span.len()).bold().red(),
            help.unwrap_or_default()
                .replace('\n', &format!("\n         {pad}{padn}"))
                .bold()
                .green(),
        );
        if let Some(info) = self.detail.info() {
            eprintln!("{}", info.magenta());
        }
    }
}

fn get_closest_match<'a, T>(pattern: &str, choices: T) -> Option<String>
where T: Iterator<Item = &'a str> {
    let matcher = SkimMatcherV2::default();
    let mut matches: Vec<_> = choices
        .filter_map(|choice| {
            matcher.fuzzy_match(choice, pattern).map(|score| (choice, score))
        })
        .collect();
    matches.sort_by_key(|(_, score)| *score);
    matches.last().map(|(choice, _)| format!("did you mean `{choice}`?"))
}
