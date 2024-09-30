#![allow(dead_code, unused_variables)]

use std::error::Error;

use super::tree_types::*;
use crate::{
    lexer::token::Token,
    types::{token_type::*, token_value::TokenValue},
};

#[derive(Debug, Clone)]
pub struct AbstractSyntaxTree {
    statements: Vec<Expresion>,
}

#[derive(Debug, Clone)]
struct TempState {
    current: usize,
    statements: Vec<Expresion>,
    tokens: Vec<Token>,
}

impl TempState {
    fn new(tokens: &[Token]) -> Self {
        Self {
            current: 0,
            statements: vec![],
            tokens: tokens.into(),
        }
    }
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }

    pub fn accept_tokens(&mut self, tokens: &[Token]) -> Result<(), Box<dyn Error>> {
        let mut state = TempState::new(tokens);

        while state.current < tokens.len() {
            self.match_tokens(&tokens[state.current], &mut state)
        }

        self.statements.clear();
        self.statements = state.statements;
        Ok(())
    }
    

    fn match_tokens(&self, token: &Token, state: &mut TempState) {
        match token.token_type {
            TokenType::Unknown => panic!("Unknown token type."),
            TokenType::Integer => {
                match token.value {
                    TokenValue::Int(value) => {
                        let value = Expresion::Integer(value);
                        state.current += 1;
                        state.statements.push(value);
                    }
                    _ => panic!("Impossible int value."),
                };
            }
            _ => {
                state.current += 1;
            }
        };
    }

    pub fn get(&self) -> &[Expresion] {
        &self.statements
    }
}
