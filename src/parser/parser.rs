use crate::lexer::token::Token;

use super::{error::ParserError, program::Program};

pub fn parse(
    _tokens: Vec<Token>
) -> Result<Program, ParserError> {
    let program: Program = Program {
        expressions: Vec::new()
    };
    Ok(program)
}