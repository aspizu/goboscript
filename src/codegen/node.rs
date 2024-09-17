use std::io::{self, Seek, Write};

use super::{node_id::NodeID, Sb3};

#[derive(Default, Copy, Clone)]
pub struct Node {
    opcode: &'static str,
    this_id: NodeID,
    next_id: Option<NodeID>,
    parent_id: Option<NodeID>,
    top_level: bool,
    shadow: bool,
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
        }
    }

    pub fn next_id(mut self, next_id: NodeID) -> Self {
        self.next_id = Some(next_id);
        self
    }

    pub fn some_next_id(mut self, next_id: Option<NodeID>) -> Self {
        self.next_id = next_id;
        self
    }

    pub fn parent_id(mut self, parent_id: NodeID) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn some_parent_id(mut self, parent_id: Option<NodeID>) -> Self {
        self.parent_id = parent_id;
        self
    }

    pub fn top_level(mut self, top_level: bool) -> Self {
        self.top_level = top_level;
        self
    }

    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }
}

impl<T> Sb3<T>
where T: Write + Seek
{
    pub fn node(&mut self, node: Node) -> io::Result<()> {
        if self.blocks_comma {
            self.write_all(b",")?;
        }
        self.blocks_comma = true;
        write!(self, r#"{}:{{"opcode":"{}""#, node.this_id, node.opcode)?;
        if let Some(next_id) = node.next_id {
            write!(self, r#","next":{next_id}"#)?;
        } else {
            write!(self, r#","next":null"#)?;
        }
        if let Some(parent_id) = node.parent_id {
            write!(self, r#","parent":{parent_id}"#)?;
        } else {
            write!(self, r#","parent":null"#)?;
        }
        if node.top_level {
            self.write_all(br#","topLevel":true"#)?;
        } else {
            self.write_all(br#","topLevel":false"#)?;
        }
        if node.shadow {
            self.write_all(br#","shadow":true"#)?;
        } else {
            self.write_all(br#","shadow":false"#)?;
        }
        Ok(())
    }
}
