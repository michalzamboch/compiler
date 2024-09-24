#![allow(dead_code)]

use crate::types::token_type::TokenType;

use super::token::Token;
use regex::Regex;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    reg: Regex,
}

#[derive(Debug, Default)]
struct TempState {
    current_line: i32,
    ignore_next: bool,
}

impl Scanner {
    pub fn new(src: &str) -> Self {
        let expresion = r#"(?:[0-9]*\.?[0-9]+|"(.*?)"|\w+|\(|\)|\{|\}|\[|\]|\-|\+|\*|//|\/|==|<=|>=|\n|\n\r|<|>|!=|!|=|;|\,|\.|\|\||&&|\S)"#;
        let re = Regex::new(expresion);

        Self {
            source: src.to_owned(),
            reg: re.unwrap(),
        }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        let mut state = TempState::default();

        let tokens: Vec<Token> = self
            .reg
            .captures_iter(&self.source)
            .filter_map(|element| self.get_token(element, &mut state))
            .collect();

        tokens
    }

    fn get_token(&self, element: regex::Captures<'_>, state: &mut TempState) -> Option<Token> {
        let extracted = &element[0];
        match extracted {
            "\n\r" | "\n" => {
                state.current_line += 1;
                state.ignore_next = false;
                None
            }
            "//" => {
                state.ignore_next = true;
                None
            }
            _ => {
                if !state.ignore_next {
                    self.create_correct_token(extracted, state)
                } else {
                    None
                }
            }
        }
    }

    fn create_correct_token(&self, element: &str, state: &TempState) -> Option<Token> {
        let token = match element {
            "=" => Token::new(TokenType::Equal, element.to_string(), state.current_line),
            "==" => Token::new(
                TokenType::EqualEqual,
                element.to_string(),
                state.current_line,
            ),
            ">" => Token::new(TokenType::Greater, element.to_string(), state.current_line),
            ">=" => Token::new(
                TokenType::GreaterEqual,
                element.to_string(),
                state.current_line,
            ),
            "<=" => Token::new(
                TokenType::LessEqual,
                element.to_string(),
                state.current_line,
            ),
            "<" => Token::new(TokenType::Less, element.to_string(), state.current_line),
            "!=" => Token::new(
                TokenType::BangEqual,
                element.to_string(),
                state.current_line,
            ),
            "!" => Token::new(TokenType::Bang, element.to_string(), state.current_line),
            ";" => Token::new(
                TokenType::Semicolon,
                element.to_string(),
                state.current_line,
            ),
            ")" => Token::new(
                TokenType::RightParen,
                element.to_string(),
                state.current_line,
            ),
            "(" => Token::new(
                TokenType::LeftParen,
                element.to_string(),
                state.current_line,
            ),
            "}" => Token::new(
                TokenType::RightBrace,
                element.to_string(),
                state.current_line,
            ),
            "{" => Token::new(
                TokenType::LeftBrace,
                element.to_string(),
                state.current_line,
            ),
            "]" => Token::new(
                TokenType::RightSquareBracket,
                element.to_string(),
                state.current_line,
            ),
            "[" => Token::new(
                TokenType::LeftSquareBracket,
                element.into(),
                state.current_line,
            ),
            "+" => Token::new(TokenType::Plus, element.to_string(), state.current_line),
            "-" => Token::new(TokenType::Minus, element.to_string(), state.current_line),
            "/" => Token::new(TokenType::Slash, element.to_string(), state.current_line),
            "*" => Token::new(TokenType::Star, element.to_string(), state.current_line),
            "." => Token::new(TokenType::Dot, element.to_string(), state.current_line),
            "," => Token::new(TokenType::Comma, element.to_string(), state.current_line),
            "||" => Token::new(TokenType::Or, element.to_string(), state.current_line),
            "&&" => Token::new(TokenType::And, element.to_string(), state.current_line),
            "true" => Token::new(TokenType::True, element.to_string(), state.current_line),
            "false" => Token::new(TokenType::False, element.to_string(), state.current_line),
            "int" => Token::new(TokenType::Int, element.to_string(), state.current_line),
            "float" => Token::new(TokenType::Float, element.to_string(), state.current_line),
            "string" => Token::new(TokenType::String, element.to_string(), state.current_line),
            "bool" => Token::new(TokenType::Boolean, element.to_string(), state.current_line),
            "if" => Token::new(TokenType::If, element.to_string(), state.current_line),
            "else" => Token::new(TokenType::Else, element.to_string(), state.current_line),
            "for" => Token::new(TokenType::For, element.to_string(), state.current_line),
            "while" => Token::new(TokenType::While, element.to_string(), state.current_line),
            "fun" => Token::new(TokenType::Fun, element.to_string(), state.current_line),
            "return" => Token::new(TokenType::Return, element.to_string(), state.current_line),
            "print" => Token::new(TokenType::Print, element.to_string(), state.current_line),
            "nil" => Token::new(TokenType::Nil, element.to_string(), state.current_line),
            _ => self.create_variable_token(element, state.current_line),
        };
        Some(token)
    }

    fn create_variable_token(&self, element: &str, line: i32) -> Token {
        if self.is_string_literal(element) {
            Token::new(TokenType::Str, element.to_string(), line)
        } else if self.is_number(element) {
            if element.contains(".") {
                Token::new(TokenType::Real, element.to_string(), line)
            } else {
                Token::new(TokenType::Interger, element.to_string(), line)
            }
        } else if self.is_idetifier(element) {
            Token::new(TokenType::Identifier, element.to_string(), line)
        } else {
            Token::new(TokenType::Unknown, element.to_string(), line)
        }
    }

    fn is_string_literal(&self, element: &str) -> bool {
        element.starts_with("\"") && element.ends_with("\"") && element.len() > 1
    }

    fn is_number(&self, element: &str) -> bool {
        element.chars().next().is_some_and(|c| c.is_ascii_digit())
    }

    fn is_idetifier(&self, element: &str) -> bool {
        element
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic())
    }
}
