use std::fmt::{
    self,
    Display,
};

use logos::Logos;
use serde::{
    Deserialize,
    Serialize,
};

use super::literal::*;
use crate::misc::SmolStr;

#[derive(Debug, Logos, Clone, PartialEq, Serialize, Deserialize)]
#[logos(skip r"[ \r\t\f]+")]
#[logos(skip r"#[^\n]*\n")]
pub enum Token {
    #[token("%define")]
    Define,
    #[token("%undef")]
    Undef,
    #[token("\n")]
    Newline,
    #[token("\\")]
    Backslash,
    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*", name)]
    Name(SmolStr),
    #[regex(r"\$[_a-zA-Z0-9]+", arg)]
    Arg(SmolStr),
    #[regex(r"0b[0-1][_0-1]*", bin)]
    Bin(i64),
    #[regex(r"0o[0-7][_0-7]*", oct)]
    Oct(i64),
    #[regex(r"[0-9][_0-9]*", priority=2, callback=int)]
    Int(i64),
    #[regex(r"0x[0-9a-fA-F][_0-9a-fA-F]*", hex)]
    Hex(i64),
    #[regex(r"(0|[1-9][0-9]*)(\.[0-9]+)?([Ee][\-+][0-9]+)?", priority=1, callback=float)]
    Float(f64),
    #[regex(r#""([^"\\]|\\["\\/bfnrt]|\\u[0-9a-zA-Z]{4})*""#, string)]
    Str(SmolStr),
    #[regex(r#"```([^`]|\n)*```"#, cmd)]
    Cmd(SmolStr),
    #[token("costumes")]
    Costumes,
    #[token("sounds")]
    Sounds,
    #[token("local")]
    Local,
    #[token("proc")]
    Proc,
    #[token("func")]
    Func,
    #[token("return")]
    Return,
    #[token("nowarp")]
    NoWarp,
    #[token("on")]
    On,
    #[token("onflag")]
    OnFlag,
    #[token("onkey")]
    OnKey,
    #[token("onclick")]
    OnClick,
    #[token("onbackdrop")]
    OnBackdrop,
    #[token("onloudness")]
    OnLoudness,
    #[token("ontimer")]
    OnTimer,
    #[token("onclone")]
    OnClone,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("elif")]
    Elif,
    #[token("until")]
    Until,
    #[token("forever")]
    Forever,
    #[token("repeat")]
    Repeat,
    #[token(",")]
    Comma,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,
    #[token("+=")]
    AssignAdd,
    #[token("-=")]
    AssignSubtract,
    #[token("*=")]
    AssignMultiply,
    #[token("/=")]
    AssignDivide,
    #[token("//=")]
    AssignFloorDiv,
    #[token("%=")]
    AssignModulo,
    #[token("&=")]
    AssignJoin,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(".")]
    Dot,
    #[token("!=")]
    Ne,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    Le,
    #[token(">=")]
    Ge,
    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("in")]
    In,
    #[token("&")]
    Amp,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("//")]
    FloorDiv,
    #[token("%")]
    Percent,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("length")]
    Length,
    #[token("round")]
    Round,
    #[token("abs")]
    Abs,
    #[token("floor")]
    Floor,
    #[token("ceil")]
    Ceil,
    #[token("sqrt")]
    Sqrt,
    #[token("sin")]
    Sin,
    #[token("cos")]
    Cos,
    #[token("tan")]
    Tan,
    #[token("asin")]
    Asin,
    #[token("acos")]
    Acos,
    #[token("atan")]
    Atan,
    #[token("ln")]
    Ln,
    #[token("log")]
    Log,
    #[token("antiln")]
    Antiln,
    #[token("antilog")]
    Antilog,
    #[token("show")]
    Show,
    #[token("hide")]
    Hide,
    #[token("add")]
    Add,
    #[token("to")]
    To,
    #[token("delete")]
    Delete,
    #[token("insert")]
    Insert,
    #[token("at")]
    At,
    #[token("of")]
    Of,
    #[token("as")]
    As,
    #[token("enum")]
    Enum,
    #[token("struct")]
    Struct,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("list")]
    List,
    #[token("cloud")]
    Cloud,
    #[token("|>")]
    Pipe,
    #[token("set_x")]
    SetX,
    #[token("set_y")]
    SetY,
    #[token("set_size")]
    SetSize,
    #[token("point_in_direction")]
    PointInDirection,
    #[token("set_volume")]
    SetVolume,
    #[token("set_rotation_style_left_right")]
    SetRotationStyleLeftRight,
    #[token("set_rotation_style_all_around")]
    SetRotationStyleAllAround,
    #[token("set_rotation_style_do_not_rotate")]
    SetRotationStyleDoNotRotate,
    #[token("set_layer_order")]
    SetLayerOrder,
    #[token("var")]
    Var,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Name(name) => write!(f, "name{}", name),
            Token::Define => write!(f, "%define"),
            Token::Undef => write!(f, "%undef"),
            Token::Newline => writeln!(f),
            Token::Backslash => write!(f, "\\"),
            Token::Arg(name) => write!(f, "${}", name),
            Token::Bin(value) => write!(f, "bin{}", value),
            Token::Oct(value) => write!(f, "oct{}", value),
            Token::Int(value) => write!(f, "int{}", value),
            Token::Hex(value) => write!(f, "hex{}", value),
            Token::Float(value) => write!(f, "float{}", value),
            Token::Str(value) => write!(f, "str{}", value),
            Token::Cmd(value) => write!(f, "cmd{}", value),
            Token::Costumes => write!(f, "costumes"),
            Token::Sounds => write!(f, "sounds"),
            Token::Local => write!(f, "local"),
            Token::Proc => write!(f, "proc"),
            Token::Func => write!(f, "func"),
            Token::Return => write!(f, "return"),
            Token::NoWarp => write!(f, "nowarp"),
            Token::On => write!(f, "on"),
            Token::OnFlag => write!(f, "onflag"),
            Token::OnKey => write!(f, "onkey"),
            Token::OnClick => write!(f, "onclick"),
            Token::OnBackdrop => write!(f, "onbackdrop"),
            Token::OnLoudness => write!(f, "onloudness"),
            Token::OnTimer => write!(f, "ontimer"),
            Token::OnClone => write!(f, "onclone"),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Elif => write!(f, "elif"),
            Token::Until => write!(f, "until"),
            Token::Forever => write!(f, "forever"),
            Token::Repeat => write!(f, "repeat"),
            Token::Comma => write!(f, ","),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Assign => write!(f, "="),
            Token::Eq => write!(f, "=="),
            Token::Increment => write!(f, "++"),
            Token::Decrement => write!(f, "--"),
            Token::AssignAdd => write!(f, "+="),
            Token::AssignSubtract => write!(f, "-="),
            Token::AssignMultiply => write!(f, "*="),
            Token::AssignDivide => write!(f, "/="),
            Token::AssignFloorDiv => write!(f, "//="),
            Token::AssignModulo => write!(f, "%="),
            Token::AssignJoin => write!(f, "&="),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::Dot => write!(f, "."),
            Token::Ne => write!(f, "!="),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::Le => write!(f, "<="),
            Token::Ge => write!(f, ">="),
            Token::Not => write!(f, "not"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::In => write!(f, "in"),
            Token::Amp => write!(f, "&"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::FloorDiv => write!(f, "//"),
            Token::Percent => write!(f, "%"),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::Length => write!(f, "length"),
            Token::Round => write!(f, "round"),
            Token::Abs => write!(f, "abs"),
            Token::Floor => write!(f, "floor"),
            Token::Ceil => write!(f, "ceil"),
            Token::Sqrt => write!(f, "sqrt"),
            Token::Sin => write!(f, "sin"),
            Token::Cos => write!(f, "cos"),
            Token::Tan => write!(f, "tan"),
            Token::Asin => write!(f, "asin"),
            Token::Acos => write!(f, "acos"),
            Token::Atan => write!(f, "atan"),
            Token::Ln => write!(f, "ln"),
            Token::Log => write!(f, "log"),
            Token::Antiln => write!(f, "antiln"),
            Token::Antilog => write!(f, "antilog"),
            Token::Show => write!(f, "show"),
            Token::Hide => write!(f, "hide"),
            Token::Add => write!(f, "add"),
            Token::To => write!(f, "to"),
            Token::Delete => write!(f, "delete"),
            Token::Insert => write!(f, "insert"),
            Token::At => write!(f, "at"),
            Token::Of => write!(f, "of"),
            Token::As => write!(f, "as"),
            Token::Enum => write!(f, "enum"),
            Token::Struct => write!(f, "struct"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::List => write!(f, "list"),
            Token::Cloud => write!(f, "cloud"),
            Token::Pipe => write!(f, "|>"),
            Token::SetX => write!(f, "set_x"),
            Token::SetY => write!(f, "set_y"),
            Token::SetSize => write!(f, "set_size"),
            Token::PointInDirection => write!(f, "point_in_direction"),
            Token::SetVolume => write!(f, "set_volume"),
            Token::SetRotationStyleLeftRight => write!(f, "set_rotation_style_left_right"),
            Token::SetRotationStyleAllAround => write!(f, "set_rotation_style_all_around"),
            Token::SetRotationStyleDoNotRotate => write!(f, "set_rotation_style_do_not_rotate"),
            Token::SetLayerOrder => write!(f, "set_layer_order"),
            Token::Var => write!(f, "var"),
        }
    }
}
