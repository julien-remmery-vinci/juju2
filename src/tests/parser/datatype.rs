#[cfg(test)]
// Tests the parsing of data types (int, double, float, char, string)
mod datatype_parsing {
    use crate::parser::{datatype::DataType, error::ParserError};

    #[test]
    fn parse_int() -> Result<(), ParserError> {
        assert_eq!(DataType::parse("int".to_string())?, DataType::Integer);
        Ok(())
    }

    #[test]
    fn parse_double() -> Result<(), ParserError> {
        assert_eq!(DataType::parse("double".to_string())?, DataType::Double);
        Ok(())
    }

    #[test]
    fn parse_float() -> Result<(), ParserError> {
        assert_eq!(DataType::parse("float".to_string())?, DataType::Float);
        Ok(())
    }

    #[test]
    fn parse_char() -> Result<(), ParserError> {
        assert_eq!(DataType::parse("char".to_string())?, DataType::Char);
        Ok(())
    }

    #[test]
    fn parse_string() -> Result<(), ParserError> {
        assert_eq!(DataType::parse("string".to_string())?, DataType::String);
        Ok(())
    }

    #[test]
    fn parse_unknown() {
        let tested = "unknown_type".to_string();
        let result = DataType::parse(tested.clone());

        match result {
            Err(ParserError::NotADataType(str)) => {
                assert_eq!(str, tested);
            },
            _ => panic!("Expected NotADataType error, but got {:?}", result),
        }
    }
}