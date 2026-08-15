use std::io::{
    self,
    Write,
};

use serde_json::json;

use super::{
    node_id::NodeID,
    sb3::{
        QualifiedName,
        Sb3,
        D,
        S,
    },
};
use crate::{
    ast::{
        Expr,
        Name,
        Value,
    },
    blocks::{
        BinOp,
        Repr,
        UnOp,
    },
};

pub fn is_expr_boolean(expr: &Expr, s: S) -> bool {
    if let Expr::BinOp {
        op: BinOp::Of, lhs, ..
    } = expr
    {
        if let Expr::Name(name) = &**lhs {
            if let Some(QualifiedName::List(..)) = s.qualify_name(None, name) {
                return true;
            }
        }
    }
    matches!(
        expr,
        Expr::UnOp { op: UnOp::Not, .. }
            | Expr::BinOp {
                op: BinOp::Eq
                    | BinOp::Ne
                    | BinOp::Lt
                    | BinOp::Le
                    | BinOp::Gt
                    | BinOp::Ge
                    | BinOp::And
                    | BinOp::Or
                    | BinOp::In,
                ..
            }
            | Expr::Repr {
                repr: Repr::ColorIsTouchingColor
                    | Repr::KeyPressed
                    | Repr::MouseDown
                    | Repr::Touching
                    | Repr::TouchingColor
                    | Repr::TouchingEdge
                    | Repr::TouchingMousePointer
                    | Repr::Contains,
                ..
            }
    )
}

pub fn coerce_condition(expr: &Expr, s: S) -> Expr {
    if is_expr_boolean(expr, s) {
        return expr.clone();
    }
    BinOp::Eq.to_expr(0..0, expr.clone(), Value::from(true).to_expr(0..0))
}

impl Sb3 {
    pub fn input(
        &mut self,
        s: S,
        d: D,
        name: &str,
        expr: &Expr,
        this_id: NodeID,
        no_empty_shadow: bool,
    ) -> io::Result<()> {
        self._input(s, d, name, expr, this_id, None, no_empty_shadow)
    }

    pub fn input_with_shadow(
        &mut self,
        s: S,
        d: D,
        name: &str,
        expr: &Expr,
        this_id: NodeID,
        shadow_id: NodeID,
    ) -> io::Result<()> {
        self._input(s, d, name, expr, this_id, Some(shadow_id), false)
    }

    fn _input(
        &mut self,
        s: S,
        d: D,
        input_name: &str,
        expr: &Expr,
        this_id: NodeID,
        shadow_id: Option<NodeID>,
        no_empty_shadow: bool,
    ) -> io::Result<()> {
        if std::mem::replace(&mut self.inputs_comma, true) {
            self.json.write_all(b",")?;
        }
        write!(self.json, r#""{input_name}":"#)?;
        match expr {
            Expr::Value { value, span: _ } => return self.value_input(input_name, value),
            Expr::Name(name) => return self.name_input(s, d, input_name, name, shadow_id),
            _ => {}
        }
        self.node_input(input_name, this_id, shadow_id, no_empty_shadow)
    }

    fn value_input(&mut self, name: &str, value: &Value) -> io::Result<()> {
        match value {
            Value::Boolean(boolean) => {
                write!(self.json, "[1,[4,{}]]", json!(*boolean as i64))
            }
            Value::Number(number) if number.is_infinite() || number.is_nan() => match number {
                n if n.is_infinite() && *n > 0.0 => {
                    write!(self.json, "[1,[4,\"Infinity\"]]")
                }
                n if n.is_infinite() && *n < 0.0 => {
                    write!(self.json, "[1,[4,\"-Infinity\"]]")
                }
                _ => {
                    write!(self.json, "[1,[4,\"NaN\"]]")
                }
            },
            Value::Number(number) if number.fract() == 0.0 => {
                write!(self.json, "[1,[4,{}]]", json!(*number as i64))
            }
            Value::Number(number) => {
                write!(self.json, "[1,[4,{}]]", json!(number))
            }
            Value::String(string) => {
                let color = ["COLOR", "COLOR2"]
                    .contains(&name)
                    .then(|| {
                        csscolorparser::parse(string)
                            .ok()
                            .filter(|color| color.a == 1.0)
                    })
                    .flatten();
                if name == "BROADCAST_INPUT" {
                    write!(
                        self.json,
                        "[1,[11,{},{}]]",
                        json!(**string),
                        json!(**string)
                    )
                } else if let Some(color) = color {
                    write!(self.json, "[1,[9,{}]]", json!(color.to_css_hex()))
                } else {
                    write!(self.json, "[1,[10,{}]]", json!(**string))
                }
            }
        }
    }

    fn name_input(
        &mut self,
        s: S,
        d: D,
        input_name: &str,
        name: &Name,
        shadow_id: Option<NodeID>,
    ) -> io::Result<()> {
        match s.qualify_name(Some(d), name) {
            Some(QualifiedName::Var(name, _)) => {
                self.block_count += 1;
                write!(self.json, "[3,[12,{},{}],", json!(*name), json!(*name))?;
            }
            Some(QualifiedName::List(name, _)) => {
                self.block_count += 1;
                write!(self.json, "[3,[13,{},{}],", json!(*name), json!(*name))?;
            }
            None => {}
        }
        self.shadow_input(input_name, shadow_id)
    }

    fn node_input(
        &mut self,
        input_name: &str,
        node_id: NodeID,
        shadow_id: Option<NodeID>,
        no_empty_shadow: bool,
    ) -> io::Result<()> {
        if no_empty_shadow {
            return write!(self.json, "[2,{node_id}]");
        }
        write!(self.json, "[3,{node_id},")?;
        self.shadow_input(input_name, shadow_id)
    }

    fn shadow_input(&mut self, input_name: &str, shadow_id: Option<NodeID>) -> io::Result<()> {
        if let Some(shadow_id) = shadow_id {
            write!(self.json, "{shadow_id}]")
        } else if input_name == "BROADCAST_INPUT" {
            let broadcast_name = json!("message1");
            write!(self.json, "[11,{},{}]]", broadcast_name, broadcast_name)
        } else {
            write!(self.json, r#"[10, ""]]"#)
        }
    }
}
