use std::fmt::{
    self,
    Display,
    Formatter,
    Write,
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

const CHARSET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
impl Display for NodeID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut n = self.value;
        write!(f, "\"")?;
        if n == 0 {
            f.write_char(CHARSET[0] as char)?;
        } else {
            while n > 0 {
                f.write_char(CHARSET[n % CHARSET.len()] as char)?;
                n /= CHARSET.len();
            }
        }
        write!(f, "\"")
    }
}
