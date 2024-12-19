use std::fs;

use error::error::JError;
use lexer::lexer::lex;

pub mod error;
pub mod lexer;
pub mod parser;
pub mod tests;

fn main() -> Result<(), JError> {
    let input_file = fs::OpenOptions::new()
        .read(true)
        .open("test.ju")
        .map_err(JError::FileOpenError)?;
    let tokens: Vec<lexer::token::Token> = lex(input_file).map_err(JError::LexerError)?;
    for token in tokens {
        println!("Token: {}", token.value);
    }
    Ok(())
}
