#[cfg(test)]
mod operation_tests {
    use crate::parser::{error::ParserError, operation::Operation};
    #[test]
    fn parse_add() -> Result<(), ParserError> {
        assert_eq!(Operation::new("+")?, Operation::Add);
        Ok(())
    }
    #[test]
    fn parse_sub() -> Result<(), ParserError> {
        assert_eq!(Operation::new("-")?, Operation::Sub);
        Ok(())
    }
    #[test]
    fn parse_mul() -> Result<(), ParserError> {
        assert_eq!(Operation::new("*")?, Operation::Mul);
        Ok(())
    }
    #[test]
    fn parse_div() -> Result<(), ParserError> {
        assert_eq!(Operation::new("/")?, Operation::Div);
        Ok(())
    }
    #[test]
    fn parse_pow() -> Result<(), ParserError> {
        assert_eq!(Operation::new("^^")?, Operation::Pow);
        Ok(())
    }
}