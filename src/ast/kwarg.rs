use logos::Span;
use smol_str::SmolStr;

use super::Expr;
use crate::misc::Rrc;

#[derive(Debug)]
pub struct Kwarg {
    pub value: Rrc<Expr>,
    pub name: Option<(SmolStr, Span)>,
}
