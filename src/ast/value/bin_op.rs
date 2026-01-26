use std::ops::*;

use super::Value;
use crate::{
    blocks::BinOp,
    misc::SmolStr,
};

impl Value {
    pub fn bin_op(op: BinOp, lhs: &Value, rhs: &Value) -> Value {
        match op {
            BinOp::Add => lhs.to_number().add(rhs.to_number()).into(),
            BinOp::Sub => lhs.to_number().sub(rhs.to_number()).into(),
            BinOp::Mul => lhs.to_number().mul(rhs.to_number()).into(),
            BinOp::Div => {
                // Special handling for division by zero to preserve NaN/Infinity expressions
                let lhs_num = lhs.to_number();
                let rhs_num = rhs.to_number();
                if rhs_num == 0.0 {
                    // Division by zero - return NaN/Infinity as appropriate
                    if lhs_num == 0.0 {
                        Value::Number(f64::NAN)
                    } else if lhs_num > 0.0 {
                        Value::Number(f64::INFINITY)
                    } else if lhs_num < 0.0 {
                        Value::Number(f64::NEG_INFINITY)
                    } else {
                        Value::Number(lhs_num) // Shouldn't happen but handle anyway
                    }
                } else {
                    lhs_num.div(rhs_num).into()
                }
            }
            BinOp::Mod => modulo(lhs.to_number(), rhs.to_number()).into(),
            BinOp::Lt => (Value::compare(lhs, rhs) < 0.0).into(),
            BinOp::Gt => (Value::compare(lhs, rhs) > 0.0).into(),
            BinOp::Eq => (Value::compare(lhs, rhs) == 0.0).into(),
            BinOp::And => (lhs.to_boolean() && rhs.to_boolean()).into(),
            BinOp::Or => (lhs.to_boolean() || rhs.to_boolean()).into(),
            BinOp::Join => SmolStr::from(format!("{}{}", lhs.to_string(), rhs.to_string())).into(),
            BinOp::In => contains(lhs, rhs),
            BinOp::Of => letter_of(lhs, rhs),
            BinOp::Le => unreachable!(),
            BinOp::Ge => unreachable!(),
            BinOp::Ne => unreachable!(),
            BinOp::FloorDiv => unreachable!(),
        }
    }
}

fn modulo(lhs: f64, rhs: f64) -> f64 {
    let result = lhs % rhs;
    if result / rhs < 0.0 {
        result + rhs
    } else {
        result
    }
}

fn contains(lhs: &Value, rhs: &Value) -> Value {
    lhs.to_string()
        .to_lowercase()
        .contains(&rhs.to_string().to_lowercase())
        .into()
}

fn letter_of(lhs: &Value, rhs: &Value) -> Value {
    let index = rhs.to_number() - 1.0;
    let str = lhs.to_string();
    if index < 0.0 || index >= str.len() as f64 {
        return arcstr::literal!("").into();
    }
    SmolStr::from(str.chars().nth(index as usize).unwrap().to_string()).into()
}
