use std::{fs::File, io::Read};

use super::{error::LexerError, token::Token};

/**
 * Takes in an input file.
 * Returns an array of tokens present in the file.
 */
pub fn lex(mut input_file: File) -> Result<Vec<Token>, LexerError> {
    let mut tokens: Vec<Token> = Vec::new();
    let buf= &mut String::default();
    input_file.read_to_string(buf)
        .map_err(LexerError::FileReadError)?;
    let buf = buf.to_string();
    let mut token: String = String::from("");
    let mut char_seq: String = String::from("");

    for char in buf.chars() {
        if " \n\t".contains(char) && !token.eq("") {
            tokens.push(Token {
                value: token.clone(),
            });
            token.clear();
        }
        if "()[]{}+-*/,;.:=!?&|\"'#^%~".contains(char) {
            if !token.is_empty() {
                tokens.push(Token {
                    value: token.clone(),
                });
                token.clear();
            }
            if !char_seq.is_empty() {
                if char_seq.contains(char) {
                    char_seq.push(char);
                }
                else {
                    tokens.push(Token {
                        value: char_seq.clone(),
                    });
                    char_seq.clear();
                    char_seq.push(char);
                }
            }
            else {
                char_seq.push(char);
            }
        }
        else if char.is_alphanumeric() {
            if !char_seq.is_empty() {
                tokens.push(Token {
                    value: char_seq.clone(),
                });
                char_seq.clear();
            }
            token.push(char);
        }
    }

    Ok(tokens)
}