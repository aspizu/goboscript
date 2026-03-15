mod bin_op;
mod js;
mod un_op;

use serde::{
    Deserialize,
    Serialize,
};

use crate::misc::SmolStr;

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    String(SmolStr),
}

impl Value {
    pub fn boolean(&self) -> Option<bool> {
        if let Value::Boolean(boolean) = self {
            Some(*boolean)
        } else {
            None
        }
    }

    pub fn number(&self) -> Option<f64> {
        if let Value::Number(number) = self {
            Some(*number)
        } else {
            None
        }
    }

    pub fn string(&self) -> Option<SmolStr> {
        if let Value::String(string) = self {
            Some(string.clone())
        } else {
            None
        }
    }

    pub fn to_number(&self) -> f64 {
        match self {
            Value::Boolean(false) => 0.0,
            Value::Boolean(true) => 1.0,
            Value::Number(number) if number.is_nan() => 0.0,
            Value::Number(number) => *number,
            Value::String(string) => match js::parse_float(string) {
                number if number.is_nan() => 0.0,
                number => number,
            },
        }
    }

    pub fn to_js_number(&self) -> f64 {
        match self {
            Value::Boolean(false) => 0.0,
            Value::Boolean(true) => 1.0,
            Value::Number(number) => *number,
            Value::String(string) => js::parse_float(string),
        }
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            Value::Boolean(boolean) => *boolean,
            Value::Number(number) if number.is_nan() => false,
            Value::Number(0.0) => false,
            Value::Number(_) => true,
            Value::String(string) if string.is_empty() => false,
            Value::String(string) if string == "0" => false,
            Value::String(string) if string.to_lowercase() == "false" => false,
            Value::String(_) => true,
        }
    }

    pub fn to_string(&self) -> SmolStr {
        match self {
            Value::Boolean(true) => arcstr::literal!("true"),
            Value::Boolean(false) => arcstr::literal!("false"),
            Value::Number(number) if number.is_nan() => arcstr::literal!("NaN"),
            Value::Number(number) if number.is_infinite() && number.is_sign_positive() => {
                arcstr::literal!("Infinity")
            }
            Value::Number(number) if number.is_infinite() && number.is_sign_negative() => {
                arcstr::literal!("-Infinity")
            }
            Value::Number(number) if number.fract() == 0.0 => (*number as i64).to_string().into(),
            Value::Number(number) => serde_json::to_string(number).unwrap().into(),
            Value::String(string) => string.clone(),
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            Value::Boolean(_) => true,
            Value::Number(number) if number.is_nan() => true,
            Value::Number(number) => number.fract() == 0.0,
            Value::String(string) => !string.contains('.'),
        }
    }

    pub fn is_whitespace(&self) -> bool {
        self.string().is_some_and(|string| string.trim().is_empty())
    }

    pub fn compare(v1: &Value, v2: &Value) -> f64 {
        let mut n1 = v1.to_js_number();
        let mut n2 = v2.to_js_number();
        if n1 == 0.0 && v1.is_whitespace() {
            n1 = f64::NAN;
        } else if n2 == 0.0 && v2.is_whitespace() {
            n2 = f64::NAN;
        }
        if n1.is_nan() || n2.is_nan() {
            let s1 = v1.to_string().to_lowercase();
            let s2 = v2.to_string().to_lowercase();
            if s1 < s2 {
                return -1.0;
            } else if s1 > s2 {
                return 1.0;
            }
            return 0.0;
        }
        if n1.is_infinite() && n2.is_infinite() && n1.signum() == n2.signum() {
            return 0.0;
        }
        n1 - n2
    }

    pub fn to_list_index(&self, length: usize) -> Option<ListIndex> {
        if let Value::String(string) = self {
            if string == "all" {
                return Some(ListIndex::All);
            }
            if string == "last" {
                return Some(ListIndex::Index(length - 1));
            }
        }
        let index = self.to_number().floor();
        if index < 1.0 || index > length as f64 {
            return None;
        }
        Some(ListIndex::Index((index - 1.0) as usize))
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

impl From<SmolStr> for Value {
    fn from(value: SmolStr) -> Self {
        Value::String(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(SmolStr::from(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(SmolStr::from(value))
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

pub enum ListIndex {
    Index(usize),
    All,
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match self {
            Value::Boolean(boolean) => serializer.serialize_bool(*boolean),
            Value::Number(number) if number.fract() == 0.0 => {
                serializer.serialize_i64(*number as i64)
            }
            Value::Number(number) => serializer.serialize_f64(*number),
            Value::String(string) => serializer.serialize_str(string),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        struct ValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a number, string, or boolean")
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where E: serde::de::Error {
                Ok(Value::Boolean(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
            where E: serde::de::Error {
                Ok(Value::Number(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: serde::de::Error {
                Ok(Value::String(value.into()))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}
