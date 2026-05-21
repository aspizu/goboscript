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
            BinOp::Div => lhs.to_number().div(rhs.to_number()).into(),
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

    // We cannot check for out-of-bounds `index > str.len()`, since we only know
    // the length of `str` in bytes rather than characters. Unicode characters
    // can consist of multiple bytes.
    if index < 0.0 {
        return arcstr::literal!("").into();
    }

    SmolStr::from(
        str.chars()
            .nth(index as usize)
            .map(|c| c.to_string())
            .unwrap_or("".into()),
    )
    .into()
}

#[cfg(test)]
mod test {
    use super::letter_of;
    use crate::ast::Value;

    #[test]
    fn test_letter_of_ascii_out_of_bounds() {
        let foo = arcstr::literal!("foo").into();
        let index = Value::Number(7.0);

        assert_eq!(letter_of(&foo, &index), arcstr::literal!("").into());
    }

    #[test]
    fn test_letter_of_unicode_in_bounds() {
        let foo = arcstr::literal!("яблоко").into();
        let index = Value::Number(6.0);

        assert_eq!(letter_of(&foo, &index), arcstr::literal!("о").into());
    }

    #[test]
    fn test_letter_of_unicode_out_of_bounds() {
        let foo = arcstr::literal!("яблоко").into();
        let index = Value::Number(7.0);

        assert_eq!(letter_of(&foo, &index), arcstr::literal!("").into());
    }
}
