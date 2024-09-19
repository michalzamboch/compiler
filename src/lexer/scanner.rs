#![allow(dead_code)]

use crate::types::token_type::TokenType;

use super::token::Token;
use regex::Regex;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    reg: Regex,
}

impl Scanner {
    pub fn new(src: &str) -> Self {
        let expresion = r"(?:[0-9]*\.?[0-9]+|\d+|\w+|\;|\(|\)|\{|\}|\[|\]|\-|\+|\*|\/|\=|\n|\n\r)";
        let re = Regex::new(expresion);

        Self {
            source: src.to_owned(),
            reg: re.unwrap(),
        }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        let mut current_line = 0;

        let tokens: Vec<Token> = self
            .reg
            .captures_iter(&self.source)
            .map(|element| self.get_token(element, &mut current_line))
            .collect();

        tokens
    }

    fn get_token(&self, element: regex::Captures<'_>, current_line: &mut i32) -> Token {
        let extracted = element[0].to_owned();
        let token = self.create_correct_token(&extracted, *current_line);

        if token.token_type == TokenType::NewLine {
            *current_line += 1;
        }
        token
    }

    fn create_correct_token(&self, element: &str, line: i32) -> Token {
        match element {
            ")" => Token::new(TokenType::RightParen, element.to_string(), line),
            "(" => Token::new(TokenType::LeftParen, element.to_string(), line),
            "}" => Token::new(TokenType::RightBrace, element.to_string(), line),
            "{" => Token::new(TokenType::LeftBrace, element.to_string(), line),
            "\n\r" | "\n" => Token::new(TokenType::NewLine, element.to_string(), line),
            "int" => Token::new(TokenType::Int, element.to_string(), line),
            _ => Token::new(TokenType::Identifier, element.to_string(), line),
        }
    }
}
