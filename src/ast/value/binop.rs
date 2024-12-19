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
        todo!()
    }

    fn sub(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn mul(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn div(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn mod_(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn lt(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn gt(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn eq(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn and(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn or(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn join(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn in_(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn of(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn le(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn ge(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn ne(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }

    fn floor_div(&self, rhs: &Value) -> Option<Value> {
        todo!()
    }
}
