use crate::ast::*;

pub fn is_truthy(value: Value) -> bool {
    match value {
        Value::Int(int) => int != 0,
        Value::Float(float) => float != 0.,
        Value::String(string) => match string.to_lowercase().as_str() {
            "" | "0" | "false" => false,
            _ => true,
        },
    }
}

pub fn to_float(value: Value) -> Option<f64> {
    match value {
        Value::Int(i) => Some(i as f64),
        Value::Float(f) => Some(f),
        Value::String(s) => s.parse().ok(),
    }
}
