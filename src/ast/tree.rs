#![allow(dead_code, unused_variables)]

use std::error::Error;

use super::tree_types::*;
use crate::{
    lexer::token::Token,
    types::{token_type::*, token_value::TokenValue},
};

#[derive(Debug, Clone)]
pub struct AbstractSyntaxTree {
    statements: Vec<Statement>,
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }

    pub fn accept_tokens(&mut self, tokens: &[Token]) -> Result<(), Box<dyn Error>> {
        self.statements.clear();

        for token in tokens {
            self.match_tokens(token);
        }

        Ok(())
    }

    fn match_tokens(&mut self, token: &Token) {
        match token.token_type {
            TokenType::Unknown => panic!("Unknown token type."),
            TokenType::LeftParen => (),
            TokenType::RightParen => (),
            TokenType::LeftBrace => (),
            TokenType::RightBrace => (),
            TokenType::LeftSquareBracket => (),
            TokenType::RightSquareBracket => (),
            TokenType::Comma => (),
            TokenType::Dot => (),
            TokenType::Minus => (),
            TokenType::Plus => (),
            TokenType::Semicolon => (),
            TokenType::Slash => (),
            TokenType::Star => (),
            TokenType::Bang => (),
            TokenType::BangEqual => (),
            TokenType::Equal => (),
            TokenType::EqualEqual => (),
            TokenType::Greater => (),
            TokenType::GreaterEqual => (),
            TokenType::Less => (),
            TokenType::LessEqual => (),
            TokenType::Identifier => (),
            TokenType::Int => (),
            TokenType::Float => (),
            TokenType::String => (),
            TokenType::Str => (),
            TokenType::Integer => {
                match token.value {
                    TokenValue::Int(value) => {
                        let value =
                            Statement::ExpresionStatement(Box::new(Expresion::Integer(value)));
                        self.statements.push(value);
                    }
                    _ => panic!("Impossible int value."),
                };
            }
            TokenType::Real => (),
            TokenType::Boolean => (),
            TokenType::And => (),
            TokenType::Else => (),
            TokenType::False => (),
            TokenType::Fun => (),
            TokenType::For => (),
            TokenType::If => (),
            TokenType::Nil => (),
            TokenType::Or => (),
            TokenType::Print => (),
            TokenType::Return => (),
            TokenType::True => (),
            TokenType::While => (),
        };
    }

    pub fn get(&self) -> &[Statement] {
        &self.statements
    }
}
