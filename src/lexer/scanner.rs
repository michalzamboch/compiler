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
        let expresion = r#"(?:[0-9]*\.?[0-9]+|"(.*?)"|\w+|\(|\)|\{|\}|\[|\]|\-|\+|\*|\/|==|<=|>=|\n|\n\r|<|>|!=|!|=|;|\,|\.|\|\||&&|\S)"#;
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
            .filter(|token| token.token_type != TokenType::NewLine)
            .collect();

        tokens
    }

    fn get_token(&self, element: regex::Captures<'_>, current_line: &mut i32) -> Token {
        let extracted = &element[0];
        let token = self.create_correct_token(extracted, *current_line);

        if token.token_type == TokenType::NewLine {
            *current_line += 1;
        }
        token
    }

    fn create_correct_token(&self, element: &str, line: i32) -> Token {
        match element {
            "=" => Token::new(TokenType::Equal, element.to_string(), line),
            "==" => Token::new(TokenType::EqualEqual, element.to_string(), line),
            ">" => Token::new(TokenType::Greater, element.to_string(), line),
            ">=" => Token::new(TokenType::GreaterEqual, element.to_string(), line),
            "<=" => Token::new(TokenType::LessEqual, element.to_string(), line),
            "<" => Token::new(TokenType::Less, element.to_string(), line),
            "!=" => Token::new(TokenType::BangEqual, element.to_string(), line),
            "!" => Token::new(TokenType::Bang, element.to_string(), line),
            ";" => Token::new(TokenType::Semicolon, element.to_string(), line),
            ")" => Token::new(TokenType::RightParen, element.to_string(), line),
            "(" => Token::new(TokenType::LeftParen, element.to_string(), line),
            "}" => Token::new(TokenType::RightBrace, element.to_string(), line),
            "{" => Token::new(TokenType::LeftBrace, element.to_string(), line),
            "]" => Token::new(TokenType::RightSquareBracket, element.to_string(), line),
            "[" => Token::new(TokenType::LeftSquareBracket, element.to_string(), line),
            "+" => Token::new(TokenType::Plus, element.to_string(), line),
            "-" => Token::new(TokenType::Minus, element.to_string(), line),
            "/" => Token::new(TokenType::Slash, element.to_string(), line),
            "*" => Token::new(TokenType::Star, element.to_string(), line),
            "." => Token::new(TokenType::Dot, element.to_string(), line),
            "," => Token::new(TokenType::Comma, element.to_string(), line),
            "||" => Token::new(TokenType::Or, element.to_string(), line),
            "&&" => Token::new(TokenType::And, element.to_string(), line),
            "\n\r" | "\n" => Token::new(TokenType::NewLine, element.to_string(), line),
            "true" => Token::new(TokenType::True, element.to_string(), line),
            "false" => Token::new(TokenType::False, element.to_string(), line),
            "int" => Token::new(TokenType::Int, element.to_string(), line),
            "float" => Token::new(TokenType::Float, element.to_string(), line),
            "string" => Token::new(TokenType::String, element.to_string(), line),
            "bool" => Token::new(TokenType::Boolean, element.to_string(), line),
            "if" => Token::new(TokenType::If, element.to_string(), line),
            "else" => Token::new(TokenType::Else, element.to_string(), line),
            "for" => Token::new(TokenType::For, element.to_string(), line),
            "while" => Token::new(TokenType::While, element.to_string(), line),
            "fun" => Token::new(TokenType::Fun, element.to_string(), line),
            "return" => Token::new(TokenType::Return, element.to_string(), line),
            "print" => Token::new(TokenType::Print, element.to_string(), line),
            "nil" => Token::new(TokenType::Nil, element.to_string(), line),
            _ => self.create_variable_token(element, line),
        }
    }

    fn create_variable_token(&self, element: &str, line: i32) -> Token {
        if element.starts_with("\"") && element.ends_with("\"") && element.len() > 1 {
            Token::new(TokenType::Str, element.to_string(), line)
        } else if element.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            if element.contains(".") {
                Token::new(TokenType::Real, element.to_string(), line)
            } else {
                Token::new(TokenType::Interger, element.to_string(), line)
            }
        } else if element
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic())
        {
            Token::new(TokenType::Identifier, element.to_string(), line)
        } else {
            Token::new(TokenType::Unknown, element.to_string(), line)
        }
    }
}
