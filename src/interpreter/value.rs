use std::fmt::{
    self,
    Display,
};

use serde_json::json;

use crate::{
    ast,
    misc::SmolStr,
};

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    String(SmolStr),
}

impl From<ast::Value> for Value {
    fn from(value: ast::Value) -> Self {
        match value {
            ast::Value::Int(int) => Value::Number(int as f64),
            ast::Value::Float(float) => Value::Number(float),
            ast::Value::String(string) => Value::String(string),
        }
    }
}

impl Value {
    pub fn to_number(self) -> f64 {
        match self {
            Value::Boolean(false) => 0.0,
            Value::Boolean(true) => 1.0,
            Value::String(string) => {
                let trimmed = string.trim();
                if trimmed == "Infinity" {
                    return f64::INFINITY;
                }
                if trimmed == "-Infinity" {
                    return f64::NEG_INFINITY;
                }
                if trimmed == "NaN" {
                    return f64::NAN;
                }
                if let Some(number) = serde_json::from_str::<f64>(trimmed).ok() {
                    number
                } else {
                    f64::NAN
                }
            }
            Value::Number(number) => number,
        }
    }

    pub fn to_boolean(self) -> bool {
        match self {
            Value::Boolean(boolean) => boolean,
            Value::Number(number) => match number {
                number if number.is_nan() => false,
                number if number == 0.0 => false,
                _ => true,
            },
            Value::String(string) => match string.as_str() {
                "" => false,
                "0" => false,
                string if string.to_lowercase() == "false" => false,
                _ => true,
            },
        }
    }

    pub fn to_string(self) -> SmolStr {
        match self {
            Value::Boolean(true) => "true".into(),
            Value::Boolean(false) => "false".into(),
            Value::Number(n) if n.is_nan() => "NaN".into(),
            Value::Number(n) if n.is_infinite() => if n.is_sign_positive() {
                "Infinity"
            } else {
                "-Infinity"
            }
            .into(),
            Value::Number(n) if n.fract() == 0.0 => SmolStr::from((n as i64).to_string()),
            Value::Number(n) => n.to_string().into(),
            Value::String(s) => s,
        }
    }

    pub fn compare(self, other: Value) -> f64 {
        let n1 = self.clone().to_number();
        let n2 = other.clone().to_number();

        if n1.is_nan() || n2.is_nan() {
            let s1 = self.to_string().to_lowercase();
            let s2 = other.to_string().to_lowercase();
            if s1 < s2 {
                return -1.0;
            } else if s1 > s2 {
                return 1.0;
            }
            return 0.0;
        }

        if (n1.is_infinite() && n1 > 0.0 && n2.is_infinite() && n2 > 0.0)
            || (n1.is_infinite() && n1 < 0.0 && n2.is_infinite() && n2 < 0.0)
        {
            return 0.0;
        }
        n1 - n2
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            Value::Number(number) if number.is_nan() => true,
            Value::Number(number) if number.round() == *number => true,
            Value::String(string) => !string.contains('.'),
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Number(n) if n.is_nan() => write!(f, "NaN"),
            Value::Number(n) if n.is_infinite() => write!(
                f,
                "{}",
                if n.is_sign_positive() {
                    "Infinity"
                } else {
                    "-Infinity"
                }
            ),
            Value::Number(n) if n.fract() == 0.0 => write!(f, "{}", *n as i64),
            Value::Number(n) => write!(f, "{}", json!(n)),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Number(value as f64)
    }
}

impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value::Number(value as f64)
    }
}

impl From<SmolStr> for Value {
    fn from(value: SmolStr) -> Self {
        Value::String(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value.into())
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.into())
    }
}
