use crate::lexer::error::LexerError;

#[derive(Debug)]
pub enum JError {
    FileOpenError(std::io::Error),
    LexerError(LexerError)
}