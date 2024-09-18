#![allow(dead_code)]
use std::fmt;

use crate::types::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: i32,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: String, line: i32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.line,
            self.token_type.to_string(),
            self.lexeme,
            self.literal
        )
    }
}
