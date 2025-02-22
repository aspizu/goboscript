/// Implements the binary operators for constant folding optimization.
use super::Value;
use crate::blocks::BinOp;

impl Value {
    pub fn binop(&self, op: BinOp, rhs: &Value) -> Option<Value> {
        match op {
            BinOp::Add => self.add(rhs),
            BinOp::Sub => self.sub(rhs),
            BinOp::Mul => self.mul(rhs),
            BinOp::Div => self.div(rhs),
            BinOp::Mod => self.mod_(rhs),
            BinOp::Lt => self.lt(rhs),
            BinOp::Gt => self.gt(rhs),
            BinOp::Eq => self.eq(rhs),
            BinOp::And => self.and(rhs),
            BinOp::Or => self.or(rhs),
            BinOp::Join => self.join(rhs),
            BinOp::In => self.in_(rhs),
            BinOp::Of => self.of(rhs),
            BinOp::Le => self.le(rhs),
            BinOp::Ge => self.ge(rhs),
            BinOp::Ne => self.ne(rhs),
            BinOp::FloorDiv => self.floor_div(rhs),
        }
    }

    fn add(&self, rhs: &Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Int(a + b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) => Some(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Int(b)) => Some(Value::Float(a + *b as f64)),
            (Value::Int(a), Value::String(b)) => {
                Some(Value::String(b.clone()).map_unless_infinity(|_| Value::Int(*a)))
            }
            (Value::String(a), Value::Int(b)) => {
                Some(Value::String(a.clone()).map_unless_infinity(|_| Value::Int(*b)))
            }
            (Value::Float(a), Value::String(b)) => {
                Some(Value::String(b.clone()).map_unless_infinity(|_| Value::Float(*a)))
            }
            (Value::String(a), Value::Float(b)) => {
                Some(Value::String(a.clone()).map_unless_infinity(|_| Value::Float(*b)))
            }
            _ => None,
        }
    }

    fn sub(&self, rhs: &Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a - b)),
            (Value::Int(a), Value::Float(b)) => Some(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Int(b)) => Some(Value::Float(a - *b as f64)),
            _ => None,
        }
    }

    fn mul(&self, rhs: &Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) => Some(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Int(b)) => Some(Value::Float(a * *b as f64)),
            _ => None,
        }
    }

    fn div(&self, rhs: &Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    None
                } else {
                    Some(Value::Int(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => Some(Value::Float(a / b)),
            (Value::Int(a), Value::Float(b)) => Some(Value::Float(*a as f64 / b)),
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    None
                } else {
                    Some(Value::Float(a / *b as f64))
                }
            }
            _ => None,
        }
    }

    fn mod_(&self, rhs: &Value) -> Option<Value> {
        None
    }

    fn lt(&self, rhs: &Value) -> Option<Value> {
        None
    }

    fn gt(&self, rhs: &Value) -> Option<Value> {
        None
    }

    fn eq(&self, rhs: &Value) -> Option<Value> {
        None
    }

    fn and(&self, rhs: &Value) -> Option<Value> {
        None
    }

    fn or(&self, rhs: &Value) -> Option<Value> {
        None
    }

    fn join(&self, rhs: &Value) -> Option<Value> {
        match (self, rhs) {
            (Value::String(a), Value::String(b)) => Some(format!("{a}{b}").into()),
            _ => None,
        }
    }

    fn in_(&self, rhs: &Value) -> Option<Value> {
        None
    }

    fn of(&self, rhs: &Value) -> Option<Value> {
        None
    }

    fn le(&self, rhs: &Value) -> Option<Value> {
        None
    }

    fn ge(&self, rhs: &Value) -> Option<Value> {
        None
    }

    fn ne(&self, rhs: &Value) -> Option<Value> {
        None
    }

    fn floor_div(&self, rhs: &Value) -> Option<Value> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Value;

    #[test]
    fn test_add() {
        assert_eq!(Value::from(1).add(&Value::from(2)), Some(Value::from(3)));
        assert_eq!(
            Value::from(1.0).add(&Value::from(2.0)),
            Some(Value::from(3.0))
        );
        assert_eq!(
            Value::from(1).add(&Value::from(2.0)),
            Some(Value::from(3.0))
        );
        assert_eq!(
            Value::from(1.0).add(&Value::from(2)),
            Some(Value::from(3.0))
        );
        assert_eq!(Value::from("a").add(&Value::from(1)), Some(Value::from(1)));
        assert_eq!(
            Value::from("Infinity").add(&Value::from(1)),
            Some(Value::from("Infinity"))
        );
        assert_eq!(
            Value::from("-Infinity").add(&Value::from(1)),
            Some(Value::from("-Infinity"))
        );
    }
}
