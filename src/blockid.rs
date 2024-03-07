use std::fmt::{self, Display, Formatter};

const BLOCK_ID_CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

#[derive(Debug, Copy, Clone)]
pub struct BlockID {
    value: usize,
}

impl Display for BlockID {
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

#[derive(Debug)]
pub struct BlockIDFactory {
    state: BlockID,
}

impl BlockIDFactory {
    pub fn reset(&mut self) {
        self.state.value = 0;
    }

    pub fn create_id(&mut self) -> BlockID {
        let id = self.state;
        self.state.value += 1;
        id
    }
}

impl Default for BlockIDFactory {
    fn default() -> Self {
        Self {
            state: BlockID { value: 0 },
        }
    }
}
