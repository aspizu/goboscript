use annotate_snippets::Level;
use serde::{
    Deserialize,
    Serialize,
};
use tsify::Tsify;

use crate::{
    ast::{
        Sprite,
        Type,
    },
    blocks::{
        Block,
        Repr,
    },
    lexer::token::Token,
    misc::SmolStr,
};

#[derive(Debug, Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum DiagnosticKind {
    // Errors
    InvalidToken,
    UnrecognizedEof(Vec<String>),
    UnrecognizedToken(Token, Vec<String>),
    ExtraToken(Token),
    IOError(SmolStr),
    UnrecognizedReporter(SmolStr),
    UnrecognizedBlock(SmolStr),
    UnrecognizedVariable(SmolStr),
    UnrecognizedList(SmolStr),
    UnrecognizedEnum(SmolStr),
    UnrecognizedStruct(SmolStr),
    UnrecognizedProcedure(SmolStr),
    UnrecognizedFunction(SmolStr),
    UnrecognizedArgument(SmolStr),
    UnrecognizedStructField(SmolStr),
    UnrecognizedEnumVariant(SmolStr),
    UnrecognizedStandardLibraryHeader,
    NoCostumes,
    InvalidCostumeName(SmolStr),
    InvalidBackdropName(SmolStr),
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
    FuncArgsCountMismatch {
        func: SmolStr,
        given: usize,
    },
    MacroArgsCountMismatch {
        expected: usize,
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
    MissingField {
        struct_name: SmolStr,
        field_name: SmolStr,
    },
    EmptyStruct(SmolStr),
    // Warnings
    FollowedByUnreachableCode,
    UnrecognizedKey(SmolStr),
    UnusedVariable(SmolStr),
    UnusedList(SmolStr),
    UnusedEnum(SmolStr),
    UnusedStruct(SmolStr),
    UnusedProc(SmolStr),
    UnusedFunc(SmolStr),
    UnusedArg(SmolStr),
    UnusedStructField(SmolStr),
    UnusedEnumVariant(SmolStr),
}

impl DiagnosticKind {
    pub fn to_string(&self, sprite: &Sprite) -> String {
        match self {
            DiagnosticKind::InvalidToken => "invalid token".to_string(),
            DiagnosticKind::UnrecognizedEof(expected) => {
                format!(
                    "unrecognized end of file, expected one of {}",
                    expected
                        .iter()
                        .map(|expected| format!("`{}`", expected.replace("\"", "")))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            DiagnosticKind::UnrecognizedToken(_, expected) => {
                format!(
                    "unrecognized token, expected one of {}",
                    expected
                        .iter()
                        .map(|expected| format!("`{}`", expected.replace("\"", "")))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            DiagnosticKind::ExtraToken(_) => "extra token".to_string(),
            DiagnosticKind::IOError(error) => format!("{error}"),
            DiagnosticKind::UnrecognizedReporter(_) => "unrecognized reporter".to_string(),
            DiagnosticKind::UnrecognizedBlock(_) => "unrecognized block".to_string(),
            DiagnosticKind::UnrecognizedVariable(_) => "unrecognized variable".to_string(),
            DiagnosticKind::UnrecognizedList(_) => "unrecognized list".to_string(),
            DiagnosticKind::UnrecognizedEnum(_) => "unrecognized enum".to_string(),
            DiagnosticKind::UnrecognizedStruct(_) => "unrecognized struct".to_string(),
            DiagnosticKind::UnrecognizedProcedure(_) => "unrecognized procedure".to_string(),
            DiagnosticKind::UnrecognizedFunction(_) => "unrecognized function".to_string(),
            DiagnosticKind::UnrecognizedArgument(_) => "unrecognized argument".to_string(),
            DiagnosticKind::UnrecognizedStructField(_) => "unrecognized struct field".to_string(),
            DiagnosticKind::UnrecognizedEnumVariant(_) => "unrecognized enum variant".to_string(),
            DiagnosticKind::UnrecognizedKey(_) => "unrecognized key".to_string(),
            DiagnosticKind::UnrecognizedStandardLibraryHeader => {
                "unrecognized standard library header".to_string()
            }
            DiagnosticKind::NoCostumes => "no costumes".to_string(),
            DiagnosticKind::InvalidCostumeName(name) => {
                format!("costume '{}' does not exist", name)
            }
            DiagnosticKind::InvalidBackdropName(name) => {
                format!("backdrop '{}' does not exist", name)
            }
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
                    "procedure expects {} arguments, but {} were given",
                    sprite.proc_args[proc].len(),
                    given
                )
            }
            DiagnosticKind::FuncArgsCountMismatch { func, given } => {
                format!(
                    "function expects {} arguments, but {} were given",
                    sprite.func_args[func].len(),
                    given
                )
            }
            DiagnosticKind::MacroArgsCountMismatch { expected, given } => {
                format!(
                    "macro expects {} arguments, but {} were given",
                    expected, given
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
            DiagnosticKind::UnusedProc(name) => format!("unused procedure {name}"),
            DiagnosticKind::UnusedFunc(name) => format!("unused function {name}"),
            DiagnosticKind::UnusedArg(name) => format!("unused argument {name}"),
            DiagnosticKind::UnusedStructField(name) => {
                format!("unused struct field {name} (never read)")
            }
            DiagnosticKind::UnusedEnumVariant(name) => format!("unused enum variant {name}"),
            DiagnosticKind::NotStruct => "not a struct".to_string(),
            DiagnosticKind::StructDoesNotHaveField {
                type_name,
                field_name,
            } => {
                format!("struct {type_name} does not have field {field_name}")
            }
            DiagnosticKind::MissingField {
                struct_name,
                field_name,
            } => {
                format!("struct {struct_name} is missing field {field_name}")
            }
            DiagnosticKind::EmptyStruct(name) => format!("struct {name} is empty"),
        }
    }

    pub fn help(&self) -> Option<String> {
        match self {
            DiagnosticKind::NoCostumes => {
                Some("if this is a header, move it inside a directory such as `lib/`".to_string())
            }
            DiagnosticKind::InvalidCostumeName(name) => {
                if name.contains('.') {
                    Some("costume names should not include file extensions - they are automatically derived from the file name without extension".to_string())
                } else {
                    None
                }
            }
            DiagnosticKind::InvalidBackdropName(name) => {
                if name.contains('.') {
                    Some("backdrop names should not include file extensions - they are automatically derived from the file name without extension".to_string())
                } else {
                    None
                }
            }
            DiagnosticKind::UnrecognizedToken(token, expected) => match token {
                Token::FloorDiv => Some("Use # for comments".to_owned()),
                Token::Var => Some("var should only be used at top-level.".to_owned()),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn should_be_suppressed(&self) -> bool {
        match self {
            DiagnosticKind::UnrecognizedArgument(name) => name.starts_with('_'),
            DiagnosticKind::UnusedArg(name) => name.starts_with('_'),
            DiagnosticKind::UnusedEnum(name) => name.starts_with('_'),
            DiagnosticKind::UnusedEnumVariant(name) => name.starts_with('_'),
            DiagnosticKind::UnusedList(name) => name.starts_with('_'),
            DiagnosticKind::UnusedProc(name) => name.starts_with('_'),
            DiagnosticKind::UnusedFunc(name) => name.starts_with('_'),
            DiagnosticKind::UnusedStruct(name) => name.starts_with('_'),
            DiagnosticKind::UnusedVariable(name) => name.starts_with('_'),
            DiagnosticKind::UnusedStructField(name) => name.starts_with('_'),
            _ => false,
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
            | DiagnosticKind::IOError(_)
            | DiagnosticKind::UnrecognizedReporter(_)
            | DiagnosticKind::UnrecognizedBlock(_)
            | DiagnosticKind::UnrecognizedVariable(_)
            | DiagnosticKind::UnrecognizedList(_)
            | DiagnosticKind::UnrecognizedEnum(_)
            | DiagnosticKind::UnrecognizedStruct(_)
            | DiagnosticKind::UnrecognizedProcedure(_)
            | DiagnosticKind::UnrecognizedFunction(_)
            | DiagnosticKind::UnrecognizedArgument(_)
            | DiagnosticKind::UnrecognizedStructField(_)
            | DiagnosticKind::UnrecognizedEnumVariant(_)
            | DiagnosticKind::UnrecognizedStandardLibraryHeader
            | DiagnosticKind::NoCostumes
            | DiagnosticKind::BlockArgsCountMismatch { .. }
            | DiagnosticKind::ReprArgsCountMismatch { .. }
            | DiagnosticKind::ProcArgsCountMismatch { .. }
            | DiagnosticKind::FuncArgsCountMismatch { .. }
            | DiagnosticKind::MacroArgsCountMismatch { .. }
            | DiagnosticKind::CommandFailed { .. }
            | DiagnosticKind::TypeMismatch { .. }
            | DiagnosticKind::NotStruct
            | DiagnosticKind::MissingField { .. }
            | DiagnosticKind::StructDoesNotHaveField { .. }
            | DiagnosticKind::EmptyStruct(_)
            | DiagnosticKind::InvalidCostumeName(_)
            | DiagnosticKind::InvalidBackdropName(_) => Level::Error,

            | DiagnosticKind::FollowedByUnreachableCode
            | DiagnosticKind::UnrecognizedKey(_)
            | DiagnosticKind::UnusedVariable(_)
            | DiagnosticKind::UnusedList(_)
            | DiagnosticKind::UnusedEnum(_)
            | DiagnosticKind::UnusedStruct(_)
            | DiagnosticKind::UnusedProc(_)
            | DiagnosticKind::UnusedFunc(_)
            | DiagnosticKind::UnusedArg(_)
            | DiagnosticKind::UnusedStructField(_)
            | DiagnosticKind::UnusedEnumVariant(_) => Level::Warning,
        }
    }
}
