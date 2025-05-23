use std::fmt::{
    self,
    Display,
};

#[derive(Debug, Copy, Clone)]
pub struct NodeID {
    value: usize,
}

impl NodeID {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

impl Display for NodeID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
