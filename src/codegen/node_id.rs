use std::fmt::{self, Display, Formatter};

const BLOCK_ID_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Default, Copy, Clone)]
pub struct NodeID {
    value: usize,
}

impl Display for NodeID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.value == 0 {
            return write!(f, r#""{}""#, BLOCK_ID_CHARS.chars().next().unwrap());
        }
        let mut n = self.value;
        let len = BLOCK_ID_CHARS.chars().count();
        let mut chars = Vec::new();
        while n > 0 {
            chars.push(BLOCK_ID_CHARS.chars().nth(n % len).unwrap());
            n /= len;
        }
        write!(f, "\"")?;
        for ch in chars.iter().rev() {
            write!(f, "{}", ch)?;
        }
        write!(f, "\"")?;
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct NodeIDFactory {
    state: NodeID,
}

impl NodeIDFactory {
    pub fn reset(&mut self) {
        self.state.value = 0;
    }

    pub fn new_id(&mut self) -> NodeID {
        let id = self.state;
        self.state.value += 1;
        id
    }
}

impl Iterator for NodeIDFactory {
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.new_id())
    }
}
