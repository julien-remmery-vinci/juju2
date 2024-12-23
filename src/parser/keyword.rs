use super::error::ParserError;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Return,
}

impl Keyword {
    pub fn parse(
        str: String
    ) -> Result<Keyword, ParserError> {
        match str.as_str() {
            "return" => Ok(Keyword::Return),
            _ => Err(ParserError::NotAKeyword(str))
        } 
    }
}