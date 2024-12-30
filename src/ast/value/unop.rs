use super::Value;
use crate::blocks::UnOp;

impl Value {
    pub fn unop(&self, op: UnOp) -> Option<Value> {
        match op {
            UnOp::Not => None,
            UnOp::Length => self.length(),
            UnOp::Round => self.round(),
            UnOp::Abs => None,
            UnOp::Floor => None,
            UnOp::Ceil => None,
            UnOp::Sqrt => None,
            UnOp::Sin => None,
            UnOp::Cos => None,
            UnOp::Tan => None,
            UnOp::Asin => None,
            UnOp::Acos => None,
            UnOp::Atan => None,
            UnOp::Ln => None,
            UnOp::Log => None,
            UnOp::AntiLn => None,
            UnOp::AntiLog => None,
            UnOp::Minus => None,
        }
    }

    fn length(&self) -> Option<Value> {
        match self {
            Self::Int(integer) => Some(integer.to_string().len().into()),
            Self::Float(float) => Some(float.to_string().len().into()),
            Self::String(string) => Some(string.len().into()),
        }
    }

    fn round(&self) -> Option<Value> {
        match self {
            Self::Int(_) => None,
            Self::Float(float) => {
                if float.is_nan() {
                    Some(0_i64.into())
                } else {
                    Some(float.round().into())
                }
            }
            _ => None,
        }
    }
}
