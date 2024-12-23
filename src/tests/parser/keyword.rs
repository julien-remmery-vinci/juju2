#[cfg(test)]
// Tests the parsing of keywords (return)
mod keyword_parsing {
    use crate::parser::{error::ParserError, keyword::Keyword};

    #[test]
    fn parse_return() -> Result<(), ParserError> {
        assert_eq!(Keyword::parse("return".to_string())?, Keyword::Return);
        Ok(())
    }

    #[test]
    fn parse_unknown() {
        let tested = "unknown_keyword".to_string();
        let result = Keyword::parse(tested.clone());

        match result {
            Err(ParserError::NotAKeyword(str)) => {
                assert_eq!(str, tested);
            },
            _ => panic!("Expected NotAKeyword error, but got {:?}", result),
        }
    }
}