use std::io::{
    self,
    Seek,
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
    diagnostic::DiagnosticKind,
    misc::write_comma_io,
};

pub fn is_expr_boolean(expr: &Expr) -> bool {
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

pub fn coerce_condition(expr: &Expr) -> Expr {
    if is_expr_boolean(expr) {
        return expr.clone();
    }
    BinOp::Eq.to_expr(0..0, expr.clone(), Value::from(true).to_expr(0..0))
}

impl<T> Sb3<T>
where T: Write + Seek
{
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
        write_comma_io(&mut self.zip, &mut self.inputs_comma)?;
        write!(self, r#""{input_name}":"#)?;
        match expr {
            Expr::Value { value, span: _ } => return self.value_input(input_name, value),
            Expr::Name(name) => return self.name_input(s, d, input_name, name, shadow_id),
            Expr::Dot { lhs, rhs, rhs_span } => {
                if let Expr::Name(lhs_name) = &**lhs {
                    if let Some(enum_) = s.get_enum(lhs_name.basename()) {
                        if let Some(variant) =
                            enum_.variants.iter().find(|variant| &variant.name == rhs)
                        {
                            return self
                                .value_input(input_name, &variant.value.as_ref().unwrap().0);
                        } else {
                            d.report(
                                DiagnosticKind::UnrecognizedEnumVariant(
                                    lhs_name.basename().clone(),
                                ),
                                rhs_span,
                            );
                        }
                    }
                }
            }
            _ => {}
        }
        self.node_input(input_name, this_id, shadow_id, no_empty_shadow)
    }

    fn value_input(&mut self, name: &str, value: &Value) -> io::Result<()> {
        match value {
            Value::Boolean(boolean) => {
                write!(self, "[1,[4,{}]]", json!(*boolean as i64))
            }
            Value::Number(number) if number.fract() == 0.0 => {
                write!(self, "[1,[4,{}]]", json!(number))
            }
            Value::Number(number) => {
                write!(self, "[1,[4,{}]]", json!(number))
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
                    write!(self, "[1,[11,{},{}]]", json!(**string), json!(**string))
                } else if let Some(color) = color {
                    write!(self, "[1,[9,{}]]", json!(color.to_hex_string()))
                } else {
                    write!(self, "[1,[10,{}]]", json!(**string))
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
        match s.qualify_name(d, name) {
            Some(QualifiedName::Var(name, _)) => {
                write!(self, "[3,[12,{},{}],", json!(*name), json!(*name))?;
            }
            Some(QualifiedName::List(name, _)) => {
                write!(self, "[3,[13,{},{}],", json!(*name), json!(*name))?;
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
            return write!(self, "[2,{node_id}]");
        }
        write!(self, "[3,{node_id},")?;
        self.shadow_input(input_name, shadow_id)
    }

    fn shadow_input(&mut self, input_name: &str, shadow_id: Option<NodeID>) -> io::Result<()> {
        if let Some(shadow_id) = shadow_id {
            write!(self, "{shadow_id}]")
        } else if input_name == "BROADCAST_INPUT" {
            let broadcast_name = json!("message1");
            write!(self, "[11,{},{}]]", broadcast_name, broadcast_name)
        } else {
            write!(self, r#"[10, ""]]"#)
        }
    }
}
