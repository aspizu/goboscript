use std::fmt::{
    self,
    Display,
};

use super::node_id::NodeID;

#[derive(Debug, Copy, Clone)]
pub struct Node {
    opcode: &'static str,
    this_id: NodeID,
    next_id: Option<NodeID>,
    parent_id: Option<NodeID>,
    top_level: bool,
    shadow: bool,
    x: Option<i32>,
    y: Option<i32>,
}

impl Node {
    pub fn new(opcode: &'static str, this_id: NodeID) -> Self {
        Self {
            opcode,
            this_id,
            next_id: None,
            parent_id: None,
            top_level: false,
            shadow: false,
            x: None,
            y: None,
        }
    }

    pub fn parent_id(self, parent_id: NodeID) -> Self {
        Self {
            parent_id: Some(parent_id),
            ..self
        }
    }

    pub fn top_level(self, top_level: bool) -> Self {
        Self { top_level, ..self }
    }

    pub fn shadow(self, shadow: bool) -> Self {
        Self { shadow, ..self }
    }

    pub fn some_next_id(self, next_id: Option<NodeID>) -> Self {
        Self { next_id, ..self }
    }

    pub fn x(self, x: i32) -> Self {
        Self { x: Some(x), ..self }
    }

    pub fn y(self, y: i32) -> Self {
        Self { y: Some(y), ..self }
    }

    pub fn xy(self, x: i32, y: i32) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
            ..self
        }
    }

    pub fn some_parent_id(self, parent_id: Option<NodeID>) -> Self {
        Self { parent_id, ..self }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{{", self.this_id)?;
        write!(f, "\"opcode\":\"{}\"", self.opcode)?;
        if let Some(next_id) = self.next_id {
            write!(f, ",\"next\":{next_id}")?;
        } else {
            write!(f, ",\"next\":null")?;
        }
        if let Some(parent_id) = self.parent_id {
            write!(f, ",\"parent\":{parent_id}")?;
        } else {
            write!(f, ",\"parent\":null")?;
        }
        if self.top_level {
            write!(f, ",\"topLevel\":true")?;
        } else {
            write!(f, ",\"topLevel\":false")?;
        }
        if self.shadow {
            write!(f, ",\"shadow\":true")?;
        } else {
            write!(f, ",\"shadow\":false")?;
        }
        if let Some(x) = self.x {
            write!(f, ",\"x\":{x}")?;
        }
        if let Some(y) = self.y {
            write!(f, ",\"y\":{y}")?;
        }
        Ok(())
    }
}
