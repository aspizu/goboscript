use super::Value;
use crate::blocks::UnOp;

impl Value {
    pub fn unop(&self, op: UnOp) -> Option<Value> {
        match op {
            UnOp::Not => None,
            UnOp::Length => self.length(),
            UnOp::Round => self.round(),
            UnOp::Abs => todo!(),
            UnOp::Floor => todo!(),
            UnOp::Ceil => todo!(),
            UnOp::Sqrt => todo!(),
            UnOp::Sin => todo!(),
            UnOp::Cos => todo!(),
            UnOp::Tan => todo!(),
            UnOp::Asin => todo!(),
            UnOp::Acos => todo!(),
            UnOp::Atan => todo!(),
            UnOp::Ln => todo!(),
            UnOp::Log => todo!(),
            UnOp::AntiLn => todo!(),
            UnOp::AntiLog => todo!(),
            UnOp::Minus => todo!(),
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
