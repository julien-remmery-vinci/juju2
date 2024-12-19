use super::error::ParserError;

#[derive(PartialEq, Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Pow
}

impl Operation {
    pub fn new(
        str_op: &str
    ) -> Result<Operation, ParserError> {
        match str_op {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Sub),
            "*" => Ok(Operation::Mul),
            "/" => Ok(Operation::Div),
            "^^" => Ok(Operation::Pow),
            _ => Err(ParserError::BadOperation(str_op.to_string()))
        }
    }
}