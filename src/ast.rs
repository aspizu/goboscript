use std::{cell::RefCell, rc::Rc};

use logos::Span;
use strum_macros::EnumString;

pub type Rrc<T> = Rc<RefCell<T>>;

pub fn rrc<T>(t: T) -> Rrc<T> {
    Rc::new(RefCell::new(t))
}

#[derive(Debug, EnumString, Copy, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Block {
    // Motion
    Move,
    TurnLeft,
    TurnRight,
    GotoRandomPosition,
    GotoMousePointer,
    GotoSprite,
    Goto,
    GlideToRandomPosition,
    GlideToMousePointer,
    GlideToSprite,
    Glide,
    PointInDirection,
    PointTowardsMousePointer,
    PointTowardsRandomDirection,
    PointTowards,
    ChangeX,
    SetX,
    ChangeY,
    SetY,
    IfOnEdgeBounce,
    SetRotationStyleLeftRight,
    SetRotationStyleDoNotRotate,
    SetRotationStyleAllAround,
    // Looks
    SayForSeconds,
    Say,
    ThinkForSeconds,
    Think,
    SwitchCostume,
    NextCostume,
    SwitchBackdrop,
    NextBackdrop,
    ChangeSize,
    SetSize,
    ChangeColorEffect,
    ChangeFisheyeEffect,
    ChangeWhirlEffect,
    ChangePixelateEffect,
    ChangeMosaicEffect,
    ChangeBrightnessEffect,
    ChangeGhostEffect,
    SetColorEffect,
    SetFisheyeEffect,
    SetWhirlEffect,
    SetPixelateEffect,
    SetMosaicEffect,
    SetBrightnessEffect,
    SetGhostEffect,
    ClearGraphicEffects,
    Show,
    Hide,
    GotoFront,
    GotoBack,
    GoForward,
    GoBackward,
    // Sound
    PlaySoundUntilDone,
    StartSound,
    StopAllSounds,
    ChangePitchEffect,
    ChangePanEffect,
    SetPitchEffect,
    SetPanEffect,
    ClearSoundEffects,
    ChangeVolume,
    SetVolume,
    // Events
    Broadcast,
    BroadcastAndWait,
    // Control
    Wait,
    WaitUntil,
    StopAll,
    StopThisScript,
    StopOtherScripts,
    CloneSprite,
    Clone,
    DeleteThisClone,
    // Sensing
    Ask,
    SetDraggable,
    SetNotDraggable,
    ResetTimer,
    // Pen
    EraseAll,
    Stamp,
    PenDown,
    PenUp,
    SetPenColor,
    ChangePenHue,
    ChangePenSaturation,
    ChangePenBrightness,
    ChangePenTransparency,
    SetPenHue,
    SetPenSaturation,
    SetPenBrightness,
    SetPenTransparency,
    ChangePenSize,
    SetPenSize,
    // Music
    PlayDrum,
    Rest,
    PlayNote,
    SetInstrument,
    SetTempo,
    ChangeTempo,
    // Scratch Addons
    Breakpoint,
    Log,
    Warn,
    Error,
}

#[derive(Debug, EnumString, Copy, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Reporter {
    // Motion
    XPosition,
    YPosition,
    Direction,
    // Looks
    CostumeNumber,
    CostumeName,
    BackdropNumber,
    BackdropName,
    Size,
    // Sound
    Volume,
    // Sensing
    TouchingMousePointer,
    TouchingEdge,
    TouchingSprite,
    TouchingColor,
    ColorIsTouchingColor,
    DistanceToMousePointer,
    DistanceToSprite,
    Answer,
    MouseDown,
    MouseX,
    MouseY,
    Loudness,
    Timer,
    CurrentYear,
    CurrentMonth,
    CurrentDate,
    CurrentDayOfWeek,
    CurrentHour,
    CurrentMinute,
    CurrentSecond,
    #[strum(serialize = "days_since_2000")]
    DaysSince2000,
    Username,
    // Operators
    Random,

    // Music
    Tempo,
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    // Operators
    Minus,
    Not,
    Length,
    Round,
    Abs,
    Floor,
    Ceil,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Ln,
    Log,
    AntiLn,
    AntiLog,
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOp {
    // Operators
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    And,
    Or,
    Join,
    Of,
    In,
}

pub type Declrs<'src> = Vec<Rrc<Declr<'src>>>;

#[derive(Debug)]
pub struct Function<'src> {
    pub name: &'src str,
    pub args: Names<'src>,
    pub body: Stmts<'src>,
    pub warp: bool,
    pub span: Span,
}

pub type Names<'src> = Vec<(&'src str, Span)>;
pub type Strings = Vec<(Rc<str>, Span)>;

#[derive(Debug)]
pub enum Declr<'src> {
    Costumes(Strings, Span),
    Sounds(Strings, Span),
    Def(Function<'src>),
    OnFlag(Stmts<'src>, Span),
    OnKey(Rc<str>, Stmts<'src>, Span),
    OnClick(Stmts<'src>, Span),
    OnBackdrop(Rc<str>, Stmts<'src>, Span),
    OnLoudnessGreaterThan(Rrc<Expr<'src>>, Stmts<'src>, Span),
    OnTimerGreaterThan(Rrc<Expr<'src>>, Stmts<'src>, Span),
    OnMessage(Rc<str>, Stmts<'src>, Span),
    OnClone(Stmts<'src>, Span),
}

pub type Stmts<'src> = Vec<Rrc<Stmt<'src>>>;

#[derive(Debug)]
pub enum Stmt<'src> {
    Repeat(Rrc<Expr<'src>>, Stmts<'src>, Span),
    Forever(Stmts<'src>, Span),
    Branch(Rrc<Expr<'src>>, Stmts<'src>, Stmts<'src>, Span),
    Until(Rrc<Expr<'src>>, Stmts<'src>, Span),
    SetVariable(&'src str, Rrc<Expr<'src>>, Span),
    ChangeVariable(&'src str, Rrc<Expr<'src>>, Span),
    Show(&'src str, Span),
    Hide(&'src str, Span),
    ListAdd(&'src str, Rrc<Expr<'src>>, Span),
    ListDelete(&'src str, Rrc<Expr<'src>>, Span),
    ListDeleteAll(&'src str, Span),
    ListInsert(&'src str, Rrc<Expr<'src>>, Rrc<Expr<'src>>, Span),
    ListReplace(&'src str, Rrc<Expr<'src>>, Rrc<Expr<'src>>, Span),
    Block(Block, Exprs<'src>, Span),
    Call(&'src str, Exprs<'src>, Span),
}

pub type Exprs<'src> = Vec<Rrc<Expr<'src>>>;

#[derive(Debug)]
pub enum Expr<'src> {
    Int(i64, Span),
    Float(f64, Span),
    String(Rc<str>, Span),
    Name(&'src str, Span),
    Arg(&'src str, Span),
    Reporter(Reporter, Exprs<'src>, Span),
    UnaryOp(UnaryOp, Rrc<Expr<'src>>, Span),
    BinaryOp(BinaryOp, Rrc<Expr<'src>>, Rrc<Expr<'src>>, Span),
}
