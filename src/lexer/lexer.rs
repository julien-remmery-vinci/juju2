use std::{fs::File, io::{self, Read}};

use super::{error::LexerError, token::Token};

/**
 * Takes in an input file.
 * Returns an array of tokens present in the file.
 */
pub fn lex(reader: &mut io::BufReader<File>) -> Result<Vec<Token>, LexerError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf = String::new();
    reader.read_to_string(&mut buf).map_err(LexerError::FileReadError)?;
    
    let mut token: String = String::from("");
    let mut char_seq: String = String::from("");

    // Offset to track the position of a char or line in the file
    let mut offset = 0;
    let mut line_counter = 0;

    for char in buf.chars() {
        if char == '\n' || char == '\t' || char.is_whitespace() {
            push_token(&mut token, &mut tokens, offset, line_counter);
            push_token(&mut char_seq, &mut tokens, offset, line_counter);
            if char == '\n' {
                offset += 1;
                line_counter = offset;
                continue;
            }
        }
        else if "()[]{}+-*/,;.:=!?&|\"'#^%~".contains(char) {
            push_token(&mut token, &mut tokens, offset, line_counter);
            if !char_seq.is_empty() && char_seq.contains(char) && !"{}".contains(char){
                char_seq.push(char);
            }
            else {
                push_token(&mut char_seq, &mut tokens, offset, line_counter);
                char_seq.push(char);
            }
        }
        else if char.is_alphanumeric() {
            push_token(&mut char_seq, &mut tokens, offset, line_counter);
            token.push(char);
        }

        // Used to keep track of the char's position in the file
        offset += 1;
    }

    push_token(&mut token, &mut tokens, offset, line_counter);
    push_token(&mut char_seq, &mut tokens, offset, line_counter);

    Ok(tokens)
}

fn push_token(token: &mut String, tokens: &mut Vec<Token>, offset: u64, line: u64) {
    if !token.is_empty() {
        tokens.push(Token::new(token.clone(), offset, line));
        token.clear();
    }
}