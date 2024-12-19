use annotate_snippets::Level;
use smol_str::SmolStr;

use super::SpriteDiagnostics;
use crate::{
    ast::{Project, Type},
    blocks::{Block, Repr},
    lexer::token::Token,
};

#[derive(Debug)]
pub enum DiagnosticKind {
    // Errors
    InvalidToken,
    UnrecognizedEof(Vec<String>),
    UnrecognizedToken(Token, Vec<String>),
    ExtraToken(Token),
    FileNotFound(SmolStr),
    UnrecognizedReporter(SmolStr),
    UnrecognizedBlock(SmolStr),
    UnrecognizedVariable(SmolStr),
    UnrecognizedList(SmolStr),
    UnrecognizedEnum(SmolStr),
    UnrecognizedStruct(SmolStr),
    UnrecognizedProcedure(SmolStr),
    UnrecognizedArgument(SmolStr),
    UnrecognizedStructField(SmolStr),
    UnrecognizedEnumVariant(SmolStr),
    UnrecognizedKey(SmolStr),
    NoCostumes,
    BlockArgsCountMismatch {
        block: Block,
        given: usize,
    },
    ReprArgsCountMismatch {
        repr: Repr,
        given: usize,
    },
    ProcArgsCountMismatch {
        proc: SmolStr,
        given: usize,
    },
    CommandFailed {
        stderr: Vec<u8>,
    },
    TypeMismatch {
        expected: Type,
        given: Type,
    },
    NotStruct,
    StructDoesNotHaveField {
        type_name: SmolStr,
        field_name: SmolStr,
    },
    // Warnings
    FollowedByUnreachableCode,
    UnusedVariable(SmolStr),
    UnusedList(SmolStr),
    UnusedEnum(SmolStr),
    UnusedStruct(SmolStr),
    UnusedProcedure(SmolStr),
    UnusedArgument(SmolStr),
    UnusedStructField(SmolStr),
    UnusedEnumVariant(SmolStr),
}

impl DiagnosticKind {
    pub fn to_string(&self, project: &Project, sprite_diagnostics: &SpriteDiagnostics) -> String {
        match self {
            DiagnosticKind::InvalidToken => "invalid token".to_string(),
            DiagnosticKind::UnrecognizedEof(vec) => {
                format!("unrecognized eof, expected one of {:?}", vec)
            }
            DiagnosticKind::UnrecognizedToken(token, vec) => {
                format!("unrecognized token {:?}, expected one of {:?}", token, vec)
            }
            DiagnosticKind::ExtraToken(token) => format!("extra token {:?}", token),
            DiagnosticKind::FileNotFound(smol_str) => format!("file not found: {:?}", smol_str),
            DiagnosticKind::UnrecognizedReporter(name) => format!("unrecognized reporter `{name}`"),
            DiagnosticKind::UnrecognizedBlock(name) => format!("unrecognized block `{name}`"),
            DiagnosticKind::UnrecognizedVariable(name) => format!("unrecognized variable `{name}`"),
            DiagnosticKind::UnrecognizedList(name) => format!("unrecognized list {name}"),
            DiagnosticKind::UnrecognizedEnum(name) => format!("unrecognized enum {name}"),
            DiagnosticKind::UnrecognizedStruct(name) => format!("unrecognized struct {name}"),
            DiagnosticKind::UnrecognizedProcedure(name) => format!("unrecognized procedure {name}"),
            DiagnosticKind::UnrecognizedArgument(name) => format!("unrecognized argument {name}"),
            DiagnosticKind::UnrecognizedStructField(name) => {
                format!("unrecognized struct field {name}")
            }
            DiagnosticKind::UnrecognizedEnumVariant(name) => {
                format!("unrecognized enum variant {name}")
            }
            DiagnosticKind::UnrecognizedKey(name) => format!("unrecognized key {name}"),
            DiagnosticKind::NoCostumes => "no costumes".to_string(),
            DiagnosticKind::BlockArgsCountMismatch { block, given } => {
                format!(
                    "block {:?} expects {} arguments, but {} were given",
                    block,
                    block.args().len(),
                    given
                )
            }
            DiagnosticKind::ReprArgsCountMismatch { repr, given } => {
                format!(
                    "repr {:?} expects {} arguments, but {} were given",
                    repr,
                    repr.args().len(),
                    given
                )
            }
            DiagnosticKind::ProcArgsCountMismatch { proc, given } => {
                format!(
                    "proc {:?} expects unknown arguments, but {} were given",
                    proc, given
                )
            }
            DiagnosticKind::CommandFailed { .. } => "command failed".to_string(),
            DiagnosticKind::TypeMismatch { expected, given } => {
                format!("type mismatch: expected {}, but got {}", expected, given)
            }
            DiagnosticKind::FollowedByUnreachableCode => "followed by unreachable code".to_string(),
            DiagnosticKind::UnusedVariable(name) => format!("unused variable {name}"),
            DiagnosticKind::UnusedList(name) => format!("unused list {name}"),
            DiagnosticKind::UnusedEnum(name) => format!("unused enum {name}"),
            DiagnosticKind::UnusedStruct(name) => format!("unused struct {name}"),
            DiagnosticKind::UnusedProcedure(name) => format!("unused procedure {name}"),
            DiagnosticKind::UnusedArgument(name) => format!("unused argument {name}"),
            DiagnosticKind::UnusedStructField(name) => format!("unused struct field {name}"),
            DiagnosticKind::UnusedEnumVariant(name) => format!("unused enum variant {name}"),
            DiagnosticKind::NotStruct => "not a struct".to_string(),
            DiagnosticKind::StructDoesNotHaveField {
                type_name,
                field_name,
            } => {
                format!("struct {type_name} does not have field {field_name}")
            }
        }
    }
}

impl From<&DiagnosticKind> for Level {
    fn from(val: &DiagnosticKind) -> Self {
        match val {
            | DiagnosticKind::InvalidToken
            | DiagnosticKind::UnrecognizedEof(_)
            | DiagnosticKind::UnrecognizedToken(_, _)
            | DiagnosticKind::ExtraToken(_)
            | DiagnosticKind::FileNotFound(_)
            | DiagnosticKind::UnrecognizedReporter(_)
            | DiagnosticKind::UnrecognizedBlock(_)
            | DiagnosticKind::UnrecognizedVariable(_)
            | DiagnosticKind::UnrecognizedList(_)
            | DiagnosticKind::UnrecognizedEnum(_)
            | DiagnosticKind::UnrecognizedStruct(_)
            | DiagnosticKind::UnrecognizedProcedure(_)
            | DiagnosticKind::UnrecognizedArgument(_)
            | DiagnosticKind::UnrecognizedStructField(_)
            | DiagnosticKind::UnrecognizedEnumVariant(_)
            | DiagnosticKind::UnrecognizedKey(_)
            | DiagnosticKind::NoCostumes
            | DiagnosticKind::BlockArgsCountMismatch { .. }
            | DiagnosticKind::ReprArgsCountMismatch { .. }
            | DiagnosticKind::ProcArgsCountMismatch { .. }
            | DiagnosticKind::CommandFailed { .. }
            | DiagnosticKind::TypeMismatch { .. }
            | DiagnosticKind::NotStruct
            | DiagnosticKind::StructDoesNotHaveField { .. } => Level::Error,

            | DiagnosticKind::FollowedByUnreachableCode
            | DiagnosticKind::UnusedVariable(_)
            | DiagnosticKind::UnusedList(_)
            | DiagnosticKind::UnusedEnum(_)
            | DiagnosticKind::UnusedStruct(_)
            | DiagnosticKind::UnusedProcedure(_)
            | DiagnosticKind::UnusedArgument(_)
            | DiagnosticKind::UnusedStructField(_)
            | DiagnosticKind::UnusedEnumVariant(_) => Level::Warning,
        }
    }
}
