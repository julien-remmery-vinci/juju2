#[derive(Debug)]
pub enum ParserError {
    NotAnOperation(&'static str),
    NotADataType(&'static str),
    NotAKeyword(&'static str),
}