use std::io::{self, Seek, Write};

use serde_json::json;

use super::{
    node_id::NodeID,
    sb3::{QualifiedName, Sb3, D, S},
};
use crate::{
    ast::{Expr, Name, Value},
    misc::write_comma_io,
};

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
    ) -> io::Result<()> {
        self._input(s, d, name, expr, this_id, None)
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
        self._input(s, d, name, expr, this_id, Some(shadow_id))
    }

    fn _input(
        &mut self,
        s: S,
        d: D,
        input_name: &str,
        expr: &Expr,
        this_id: NodeID,
        shadow_id: Option<NodeID>,
    ) -> io::Result<()> {
        write_comma_io(&mut self.zip, &mut self.inputs_comma)?;
        write!(self, r#""{input_name}":"#)?;
        match expr {
            Expr::Value { value, span: _ } => self.value_input(input_name, value),
            Expr::Name(name) => self.name_input(s, d, input_name, name, shadow_id),
            _ => self.node_input(input_name, this_id, shadow_id),
        }
    }

    fn value_input(&mut self, name: &str, value: &Value) -> io::Result<()> {
        match value {
            Value::Int(int_value) => {
                write!(self, "[1,[4,{}]]", json!(int_value))
            }
            Value::Float(float_value) => {
                write!(self, "[1,[4,{}]]", json!(float_value))
            }
            Value::String(string_value) => {
                let color = ["COLOR", "COLOR2"]
                    .contains(&name)
                    .then(|| {
                        csscolorparser::parse(string_value)
                            .ok()
                            .filter(|color| color.a == 1.0)
                    })
                    .flatten();
                if name == "BROADCAST_INPUT" {
                    write!(
                        self,
                        "[1,[11,{},{}]]",
                        json!(**string_value),
                        json!(**string_value)
                    )
                } else if let Some(color) = color {
                    write!(self, "[1,[9,{}]]", json!(color.to_hex_string()))
                } else {
                    write!(self, "[1,[10,{}]]", json!(**string_value))
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
    ) -> io::Result<()> {
        if ["CONDITION", "CONDITION2"].contains(&input_name) {
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
