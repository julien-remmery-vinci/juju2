#[derive(Debug)]
pub enum LexerError {
    FileReadError(std::io::Error)
}