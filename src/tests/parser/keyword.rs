#[cfg(test)]
// Tests the parsing of keywords (return)
mod keyword_parsing {
    use crate::parser::{error::ParserError, keyword::Keyword};

    #[test]
    fn parse_return() -> Result<(), ParserError> {
        assert_eq!(Keyword::parse("return")?, Keyword::Return);
        Ok(())
    }

    #[test]
    fn parse_unknown() {
        let tested = "unknown_keyword";
        let result = Keyword::parse(tested);

        match result {
            Err(ParserError::NotAKeyword(str)) => {
                assert_eq!(str, tested);
            },
            _ => panic!("Expected NotAKeyword error, but got {:?}", result),
        }
    }
}