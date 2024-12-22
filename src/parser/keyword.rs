use super::error::ParserError;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Return,
}

impl Keyword {
    pub fn parse(
        str: &'static str
    ) -> Result<Keyword, ParserError> {
        match str {
            "return" => Ok(Keyword::Return),
            _ => Err(ParserError::NotAKeyword(str))
        } 
    }
}