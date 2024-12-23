#[derive(Debug)]
pub enum ParserError {
    NotAnOperation(String),
    NotADataType(String),
    NotAKeyword(String),
    SyntaxError(String),
}