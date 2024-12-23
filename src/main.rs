use std::{fs::File, io};

use error::error::JError;
use lexer::lexer::lex;

pub mod error;
pub mod lexer;
pub mod parser;
pub mod tests;

fn main() -> Result<(), JError> {
    let file_path = "test.ju";
    let file = File::open(file_path).map_err(JError::FileOpenError)?;
    let mut reader= io::BufReader::new(file);

    let tokens: Vec<lexer::token::Token> = lex(&mut reader).map_err(JError::LexerError)?;
    Ok(())
}
