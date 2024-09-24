#![allow(dead_code)]

use std::collections::HashMap;

use crate::types::token_type::TokenType;

use super::token::Token;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Scanner {
    source: String,
    reg: Regex,
}

#[derive(Debug, Clone)]
struct TempState {
    current_line: i32,
    ignore_next: bool,
    pair_elements: HashMap<&'static str, &'static str>,
}

impl TempState {
    fn new() -> Self {
        let mut pairs = HashMap::new();
        pairs.insert("/*", "*/");
        pairs.insert("//", "\n");

        Self {
            current_line: 0,
            ignore_next: false,
            pair_elements: pairs,
        }
    }
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
        let mut state = TempState::new();

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
        let literal = element.to_string();
        let line = state.current_line;

        let token = match element {
            "=" => Token::new(TokenType::Equal, literal, line),
            "==" => Token::new(TokenType::EqualEqual, literal, line),
            ">" => Token::new(TokenType::Greater, literal, line),
            ">=" => Token::new(TokenType::GreaterEqual, literal, line),
            "<=" => Token::new(TokenType::LessEqual, literal, line),
            "<" => Token::new(TokenType::Less, literal, line),
            "!=" => Token::new(TokenType::BangEqual, literal, line),
            "!" => Token::new(TokenType::Bang, literal, line),
            ";" => Token::new(TokenType::Semicolon, literal, line),
            ")" => Token::new(TokenType::RightParen, literal, line),
            "(" => Token::new(TokenType::LeftParen, literal, line),
            "}" => Token::new(TokenType::RightBrace, literal, line),
            "{" => Token::new(TokenType::LeftBrace, literal, line),
            "]" => Token::new(TokenType::RightSquareBracket, literal, line),
            "[" => Token::new(TokenType::LeftSquareBracket, literal, line),
            "+" => Token::new(TokenType::Plus, literal, line),
            "-" => Token::new(TokenType::Minus, literal, line),
            "/" => Token::new(TokenType::Slash, literal, line),
            "*" => Token::new(TokenType::Star, literal, line),
            "." => Token::new(TokenType::Dot, literal, line),
            "," => Token::new(TokenType::Comma, literal, line),
            "||" => Token::new(TokenType::Or, literal, line),
            "&&" => Token::new(TokenType::And, literal, line),
            "true" => Token::new(TokenType::True, literal, line),
            "false" => Token::new(TokenType::False, literal, line),
            "int" => Token::new(TokenType::Int, literal, line),
            "float" => Token::new(TokenType::Float, literal, line),
            "string" => Token::new(TokenType::String, literal, line),
            "bool" => Token::new(TokenType::Boolean, literal, line),
            "if" => Token::new(TokenType::If, literal, line),
            "else" => Token::new(TokenType::Else, literal, line),
            "for" => Token::new(TokenType::For, literal, line),
            "while" => Token::new(TokenType::While, literal, line),
            "fun" => Token::new(TokenType::Fun, literal, line),
            "return" => Token::new(TokenType::Return, literal, line),
            "print" => Token::new(TokenType::Print, literal, line),
            "nil" => Token::new(TokenType::Nil, literal, line),
            _ => self.create_variable_token(element, line),
        };
        Some(token)
    }

    fn create_variable_token(&self, element: &str, line: i32) -> Token {
        let literal = element.to_string();

        if self.is_string_literal(element) {
            Token::new(TokenType::Str, literal, line)
        } else if self.is_number(element) {
            if element.contains(".") {
                Token::new(TokenType::Real, literal, line)
            } else {
                Token::new(TokenType::Interger, literal, line)
            }
        } else if self.is_idetifier(element) {
            Token::new(TokenType::Identifier, literal, line)
        } else {
            Token::new(TokenType::Unknown, literal, line)
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
