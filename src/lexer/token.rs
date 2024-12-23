#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub position: u64,
    pub line: u64,
}

impl Token {
    pub fn new(
        value: String,
        position: u64,
        line: u64
    ) -> Token {
        Token {
            value,
            position,
            line
        }
    }
}