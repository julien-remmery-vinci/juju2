use super::error::ParserError;

#[derive(PartialEq, Debug, Clone)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Pow
}

impl Operation {
    pub fn parse(
        str: String
    ) -> Result<Operation, ParserError> {
        match str.as_str() {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Sub),
            "*" => Ok(Operation::Mul),
            "/" => Ok(Operation::Div),
            "**" => Ok(Operation::Pow),
            _ => Err(ParserError::NotAnOperation(str))
        }
    }
}