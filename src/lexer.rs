use logos::Logos;

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"[ \t\n\f]+")]
#[logos(skip r"#[^\n]*\n")]
#[logos(skip r"/\*([^*]|\*[^/])*\*/")]
pub enum Token<'a> {
    Comment(&'a str),
    #[regex(r"///[^\n]*\n")]
    DocComment(&'a str),
    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*")]
    Name(&'a str),
    #[regex(r"\$[a-zA-Z_0-9]+")]
    ArgumentName(&'a str),
    #[regex(r"[a-zA-Z_0-9]+!")]
    MacroName(&'a str),
    #[regex(r"[0-9][_0-9]*")]
    Int(&'a str),
    #[regex(r"0x[_0-9a-fA-F]+")]
    // We don't include minus sign because the optimizer will deal with it.
    Hexadecimal(&'a str),
    #[regex(r"0b[_0-1]+")]
    Binary(&'a str),
    #[regex(r"0o[_0-7]+")]
    Octal(&'a str),
    #[regex(r"([0-9][0-9]*)?\.[0-9][_0-9]*")] // TODO: support for ex "10."
    Float(&'a str),
    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#)]
    String(&'a str),
    #[token("costumes")]
    Costumes,
    #[token("sounds")]
    Sounds,
    #[token("global")]
    Global,
    #[token("variables")]
    Variables,
    #[token("lists")]
    Lists,
    #[token("def")]
    Def,
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
    #[token("macro")]
    Macro,
    #[token("local")]
    Local,
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
    #[token("->")]
    Arrow,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("=")]
    Assign,
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
    Join,
    #[token("+")]
    Plus,
    #[token("-")]
    Subtract,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("%")]
    Modulo,
    #[token(";")]
    Semicolon,
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
}
