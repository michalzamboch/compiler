#![allow(dead_code)]
use std::fmt;

use crate::types::{token_type::TokenType, token_value::TokenValue};

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub value: TokenValue,
    pub line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, line: i32) -> Self {
        Self {
            token_type,
            literal: literal.clone(),
            value: Self::get_token_value(token_type, &literal),
            line,
        }
    }

    fn get_token_value(token_type: TokenType, literal: &str) -> TokenValue {
        if token_type == TokenType::True && literal == "true" {
            TokenValue::Bool(true)
        } else if token_type == TokenType::False && literal == "false" {
            TokenValue::Bool(false)
        } else if token_type == TokenType::Str
            && literal.starts_with("\"")
            && literal.ends_with("\"")
        {
            TokenValue::String(literal.trim_matches('\"').to_owned())
        } else if token_type == TokenType::Real {
            TokenValue::Float(literal.parse().unwrap_or_default())
        } else if token_type == TokenType::Integer {
            TokenValue::Int(literal.parse().unwrap_or_default())
        } else {
            TokenValue::None
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.line, self.token_type, self.literal, self.value
        )
    }
}
