use super::node_id::NodeID;

#[derive(Debug)]
pub struct NodeIDFactory {
    value: usize,
}

impl NodeIDFactory {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }

    pub fn new_id(&mut self) -> NodeID {
        let value = self.value;
        self.value += 1;
        NodeID::new(value)
    }
}

impl Iterator for NodeIDFactory {
    type Item = NodeID;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.new_id())
    }
}
