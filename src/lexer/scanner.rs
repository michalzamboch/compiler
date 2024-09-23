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
            .filter_map(|element| self.get_token(element, &mut current_line))
            .collect();

        tokens
    }

    fn get_token(&self, element: regex::Captures<'_>, current_line: &mut i32) -> Option<Token> {
        let extracted = &element[0];
        match extracted {
            "\n\r" | "\n" => {
                *current_line += 1;
                None
            }
            _ => self.create_correct_token(extracted, *current_line),
        }
    }

    fn create_correct_token(&self, element: &str, line: i32) -> Option<Token> {
        match element {
            "=" => Token::new(TokenType::Equal, element.to_string(), line).into(),
            "==" => Token::new(TokenType::EqualEqual, element.to_string(), line).into(),
            ">" => Token::new(TokenType::Greater, element.to_string(), line).into(),
            ">=" => Token::new(TokenType::GreaterEqual, element.to_string(), line).into(),
            "<=" => Token::new(TokenType::LessEqual, element.to_string(), line).into(),
            "<" => Token::new(TokenType::Less, element.to_string(), line).into(),
            "!=" => Token::new(TokenType::BangEqual, element.to_string(), line).into(),
            "!" => Token::new(TokenType::Bang, element.to_string(), line).into(),
            ";" => Token::new(TokenType::Semicolon, element.to_string(), line).into(),
            ")" => Token::new(TokenType::RightParen, element.to_string(), line).into(),
            "(" => Token::new(TokenType::LeftParen, element.to_string(), line).into(),
            "}" => Token::new(TokenType::RightBrace, element.to_string(), line).into(),
            "{" => Token::new(TokenType::LeftBrace, element.to_string(), line).into(),
            "]" => Token::new(TokenType::RightSquareBracket, element.to_string(), line).into(),
            "[" => Token::new(TokenType::LeftSquareBracket, element.to_string(), line).into(),
            "+" => Token::new(TokenType::Plus, element.to_string(), line).into(),
            "-" => Token::new(TokenType::Minus, element.to_string(), line).into(),
            "/" => Token::new(TokenType::Slash, element.to_string(), line).into(),
            "*" => Token::new(TokenType::Star, element.to_string(), line).into(),
            "." => Token::new(TokenType::Dot, element.to_string(), line).into(),
            "," => Token::new(TokenType::Comma, element.to_string(), line).into(),
            "||" => Token::new(TokenType::Or, element.to_string(), line).into(),
            "&&" => Token::new(TokenType::And, element.to_string(), line).into(),
            "true" => Token::new(TokenType::True, element.to_string(), line).into(),
            "false" => Token::new(TokenType::False, element.to_string(), line).into(),
            "int" => Token::new(TokenType::Int, element.to_string(), line).into(),
            "float" => Token::new(TokenType::Float, element.to_string(), line).into(),
            "string" => Token::new(TokenType::String, element.to_string(), line).into(),
            "bool" => Token::new(TokenType::Boolean, element.to_string(), line).into(),
            "if" => Token::new(TokenType::If, element.to_string(), line).into(),
            "else" => Token::new(TokenType::Else, element.to_string(), line).into(),
            "for" => Token::new(TokenType::For, element.to_string(), line).into(),
            "while" => Token::new(TokenType::While, element.to_string(), line).into(),
            "fun" => Token::new(TokenType::Fun, element.to_string(), line).into(),
            "return" => Token::new(TokenType::Return, element.to_string(), line).into(),
            "print" => Token::new(TokenType::Print, element.to_string(), line).into(),
            "nil" => Token::new(TokenType::Nil, element.to_string(), line).into(),
            _ => self.create_variable_token(element, line),
        }
    }

    fn create_variable_token(&self, element: &str, line: i32) -> Option<Token> {
        if element.starts_with("\"") && element.ends_with("\"") && element.len() > 1 {
            Token::new(TokenType::Str, element.to_string(), line).into()
        } else if element.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            if element.contains(".") {
                Token::new(TokenType::Real, element.to_string(), line).into()
            } else {
                Token::new(TokenType::Interger, element.to_string(), line).into()
            }
        } else if element
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic())
        {
            Token::new(TokenType::Identifier, element.to_string(), line).into()
        } else {
            Token::new(TokenType::Unknown, element.to_string(), line).into()
        }
    }
}
