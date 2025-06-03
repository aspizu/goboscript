use std::{
    f64::consts::PI,
    ops::Not,
};

use super::Value;
use crate::blocks::UnOp;

impl Value {
    pub fn un_op(op: UnOp, opr: &Value) -> Value {
        match op {
            UnOp::Not => opr.to_boolean().not().into(),
            UnOp::Length => opr.to_string().len().into(),
            UnOp::Round => opr.to_number().round().into(),
            UnOp::Abs => opr.to_number().abs().into(),
            UnOp::Floor => opr.to_number().floor().into(),
            UnOp::Ceil => opr.to_number().ceil().into(),
            UnOp::Sqrt => opr.to_number().sqrt().into(),
            UnOp::Sin => (opr.to_number() * PI / 180.0).sin().into(),
            UnOp::Cos => (opr.to_number() * PI / 180.0).cos().into(),
            UnOp::Tan => (opr.to_number() * PI / 180.0).tan().into(),
            UnOp::Asin => (opr.to_number().asin() * 180.0 / PI).into(),
            UnOp::Acos => (opr.to_number().acos() * 180.0 / PI).into(),
            UnOp::Atan => (opr.to_number().atan() * 180.0 / PI).into(),
            UnOp::Ln => opr.to_number().ln().into(),
            UnOp::Log => opr.to_number().log(10.0).into(),
            UnOp::AntiLn => (10.0_f64).powf(opr.to_number()).into(),
            UnOp::AntiLog => opr.to_number().exp().into(),
            UnOp::Minus => (-opr.to_number()).into(),
        }
    }
}
