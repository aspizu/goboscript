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
            Value::Number(number) if number.is_nan() => "NaN".into(),
            Value::Number(number) if number.is_infinite() => {
                if number.is_sign_positive() {
                    "Infinity".into()
                } else {
                    "-Infinity".into()
                }
            }
            Value::Number(number) => serde_json::to_string(&number).unwrap().into(),
            Value::String(string) => string,
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
}
