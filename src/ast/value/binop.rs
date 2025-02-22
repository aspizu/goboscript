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
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    None
                } else {
                    Some(Value::Float(a / *b as f64))
                }
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    None
                } else {
                    Some(Value::Float(*a as f64 / *b as f64))
                }
            }
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
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    None
                } else {
                    Some(Value::Int(a % b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    None
                } else {
                    Some(Value::Float(a % b))
                }
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    None
                } else {
                    Some(Value::Float(*a as f64 % b))
                }
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    None
                } else {
                    Some(Value::Float(a % *b as f64))
                }
            }
            _ => None,
        }
    }

    fn lt(&self, rhs: &Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Bool(a < b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a < b)),
            (Value::Int(a), Value::Float(b)) => Some(Value::Bool((*a as f64) < *b)),
            (Value::Float(a), Value::Int(b)) => Some(Value::Bool(*a < (*b as f64))),
            _ => None,
        }
    }

    fn gt(&self, rhs: &Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Bool(a > b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a > b)),
            (Value::Int(a), Value::Float(b)) => Some(Value::Bool((*a as f64) > *b)),
            (Value::Float(a), Value::Int(b)) => Some(Value::Bool(*a > (*b as f64))),
            _ => None,
        }
    }

    fn eq(&self, rhs: &Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Bool(a == b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a == b)),
            (Value::Int(a), Value::Float(b)) => Some(Value::Bool((*a as f64) == *b)),
            (Value::Float(a), Value::Int(b)) => Some(Value::Bool(*a == (*b as f64))),
            _ => None,
        }
    }

    fn and(&self, rhs: &Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Some(Value::Bool(*a && *b)),
            _ => None,
        }
    }

    fn or(&self, rhs: &Value) -> Option<Value> {
        match (self, rhs) {
            (Value::Bool(a), Value::Bool(b)) => Some(Value::Bool(*a || *b)),
            _ => None,
        }
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
        // Less than or equal to
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Bool(a <= b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a <= b)),
            (Value::Int(a), Value::Float(b)) => Some(Value::Bool((*a as f64) <= *b)),
            (Value::Float(a), Value::Int(b)) => Some(Value::Bool(*a <= (*b as f64))),
            _ => None,
        }
    }

    fn ge(&self, rhs: &Value) -> Option<Value> {
        // Greater than or equal to
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Bool(a >= b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a >= b)),
            (Value::Int(a), Value::Float(b)) => Some(Value::Bool((*a as f64) >= *b)),
            (Value::Float(a), Value::Int(b)) => Some(Value::Bool(*a >= (*b as f64))),
            _ => None,
        }
    }

    fn ne(&self, rhs: &Value) -> Option<Value> {
        // Not equal
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => Some(Value::Bool(a != b)),
            (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a != b)),
            (Value::Int(a), Value::Float(b)) => Some(Value::Bool((*a as f64) != *b)),
            (Value::Float(a), Value::Int(b)) => Some(Value::Bool(*a != (*b as f64))),
            (Value::Bool(a), Value::Bool(b)) => Some(Value::Bool(a != b)),
            (Value::String(a), Value::String(b)) => Some(Value::Bool(a != b)),
            _ => None,
        }
    }

    fn floor_div(&self, rhs: &Value) -> Option<Value> {
        // Floor division (integer division)
        match (self, rhs) {
            (Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    None // Division by zero
                } else {
                    Some(Value::Int(a / b))
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    None // Division by zero
                } else {
                    Some(Value::Int((a / b).floor() as i64))
                }
            }
            (Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    None // Division by zero
                } else {
                    Some(Value::Int(((*a as f64) / b).floor() as i64))
                }
            }
            (Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    None // Division by zero
                } else {
                    Some(Value::Int((a / (*b as f64)).floor() as i64))
                }
            }
            _ => None,
        }
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
    }

    #[test]
    fn test_sub() {
        assert_eq!(Value::from(1).sub(&Value::from(2)), Some(Value::from(-1)));
        assert_eq!(
            Value::from(1.0).sub(&Value::from(2.0)),
            Some(Value::from(-1.0))
        );
        assert_eq!(
            Value::from(1).sub(&Value::from(2.0)),
            Some(Value::from(-1.0))
        );
        assert_eq!(
            Value::from(1.0).sub(&Value::from(2)),
            Some(Value::from(-1.0))
        );
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_div() {
            assert_eq!(Value::from(10).div(&Value::from(2)), Some(Value::from(5)));
            assert_eq!(
                Value::from(10.0).div(&Value::from(2.0)),
                Some(Value::from(5.0))
            );
            assert_eq!(
                Value::from(10).div(&Value::from(2.0)),
                Some(Value::from(5.0))
            );
            assert_eq!(
                Value::from(10.0).div(&Value::from(2)),
                Some(Value::from(5.0))
            );
            assert_eq!(Value::from(10).div(&Value::from(0)), None); // Division by zero
            assert_eq!(Value::from(10.0).div(&Value::from(0.0)), None); // Division by zero
        }

        #[test]
        fn test_lt() {
            assert_eq!(Value::from(1).lt(&Value::from(2)), Some(Value::from(true)));
            assert_eq!(
                Value::from(1.0).lt(&Value::from(2.0)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(1).lt(&Value::from(2.0)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(1.0).lt(&Value::from(2)),
                Some(Value::from(true))
            );
            assert_eq!(Value::from(2).lt(&Value::from(1)), Some(Value::from(false)));
        }

        #[test]
        fn test_gt() {
            assert_eq!(Value::from(2).gt(&Value::from(1)), Some(Value::from(true)));
            assert_eq!(
                Value::from(2.0).gt(&Value::from(1.0)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(2).gt(&Value::from(1.0)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(2.0).gt(&Value::from(1)),
                Some(Value::from(true))
            );
            assert_eq!(Value::from(1).gt(&Value::from(2)), Some(Value::from(false)));
        }

        #[test]
        fn test_and() {
            assert_eq!(
                Value::from(true).and(&Value::from(true)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(true).and(&Value::from(false)),
                Some(Value::from(false))
            );
            assert_eq!(
                Value::from(false).and(&Value::from(true)),
                Some(Value::from(false))
            );
            assert_eq!(
                Value::from(false).and(&Value::from(false)),
                Some(Value::from(false))
            );
            assert_eq!(Value::from(1).and(&Value::from(true)), None); // Invalid types
        }

        #[test]
        fn test_or() {
            assert_eq!(
                Value::from(true).or(&Value::from(true)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(true).or(&Value::from(false)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(false).or(&Value::from(true)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(false).or(&Value::from(false)),
                Some(Value::from(false))
            );
            assert_eq!(Value::from(1).or(&Value::from(true)), None); // Invalid types
        }

        #[test]
        fn test_le() {
            assert_eq!(Value::from(1).le(&Value::from(2)), Some(Value::from(true)));
            assert_eq!(
                Value::from(1.0).le(&Value::from(2.0)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(1).le(&Value::from(2.0)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(1.0).le(&Value::from(2)),
                Some(Value::from(true))
            );
            assert_eq!(Value::from(2).le(&Value::from(1)), Some(Value::from(false)));
        }

        #[test]
        fn test_ge() {
            assert_eq!(Value::from(2).ge(&Value::from(1)), Some(Value::from(true)));
            assert_eq!(
                Value::from(2.0).ge(&Value::from(1.0)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(2).ge(&Value::from(1.0)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(2.0).ge(&Value::from(1)),
                Some(Value::from(true))
            );
            assert_eq!(Value::from(1).ge(&Value::from(2)), Some(Value::from(false)));
        }

        #[test]
        fn test_ne() {
            assert_eq!(Value::from(1).ne(&Value::from(2)), Some(Value::from(true)));
            assert_eq!(
                Value::from(1.0).ne(&Value::from(2.0)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(1).ne(&Value::from(2.0)),
                Some(Value::from(true))
            );
            assert_eq!(
                Value::from(1.0).ne(&Value::from(2)),
                Some(Value::from(true))
            );
            assert_eq!(Value::from(1).ne(&Value::from(1)), Some(Value::from(false)));
        }

        #[test]
        fn test_floor_div() {
            assert_eq!(
                Value::from(10).floor_div(&Value::from(3)),
                Some(Value::from(3))
            );
            assert_eq!(
                Value::from(10.0).floor_div(&Value::from(3.0)),
                Some(Value::from(3))
            );
            assert_eq!(
                Value::from(10).floor_div(&Value::from(3.0)),
                Some(Value::from(3))
            );
            assert_eq!(
                Value::from(10.0).floor_div(&Value::from(3)),
                Some(Value::from(3))
            );
            assert_eq!(Value::from(10).floor_div(&Value::from(0)), None); // Division by zero
            assert_eq!(Value::from(10.0).floor_div(&Value::from(0.0)), None); // Division by zero
        }
    }
}
