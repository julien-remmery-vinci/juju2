#[cfg(test)]
// Tests the parsing of operations (+, -, *, /, ^^)
mod operation_parsing {
    use crate::parser::{error::ParserError, operation::Operation};
    #[test]
    fn parse_add() -> Result<(), ParserError> {
        assert_eq!(Operation::parse("+")?, Operation::Add);
        Ok(())
    }
    #[test]
    fn parse_sub() -> Result<(), ParserError> {
        assert_eq!(Operation::parse("-")?, Operation::Sub);
        Ok(())
    }
    #[test]
    fn parse_mul() -> Result<(), ParserError> {
        assert_eq!(Operation::parse("*")?, Operation::Mul);
        Ok(())
    }
    #[test]
    fn parse_div() -> Result<(), ParserError> {
        assert_eq!(Operation::parse("/")?, Operation::Div);
        Ok(())
    }
    #[test]
    fn parse_pow() -> Result<(), ParserError> {
        assert_eq!(Operation::parse("^^")?, Operation::Pow);
        Ok(())
    }

    #[test]
    fn parse_unknown() {
        let tested = "unknown_operation";
        let result = Operation::parse(tested);

        match result {
            Err(ParserError::NotAnOperation(str)) => {
                assert_eq!(str, tested);
            },
            _ => panic!("Expected NotAnOperation error, but got {:?}", result),
        }
    }
}