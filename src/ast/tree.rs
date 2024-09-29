#![allow(dead_code, unused_variables)]

use std::error::Error;
use std::iter::*;

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
struct TempState<'a> {
    current_token: usize,
    statements: Vec<Expresion>,
    iterator: std::slice::Iter<'a, Token>,
}

impl<'a> TempState<'a> {
    fn new(iterator: std::slice::Iter<'a, Token>) -> TempState<'a> {
        Self {
            current_token: 0,
            statements: vec![],
            iterator,
        }
    }
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }

    pub fn accept_tokens(&mut self, tokens: &[Token]) -> Result<(), Box<dyn Error>> {
        let mut state = TempState::new(tokens.iter());

        while state.current_token < tokens.len() {
            self.match_tokens(&tokens[state.current_token], &mut state)
        }

        self.statements.clear();
        self.statements = state.statements;
        Ok(())
    }

    /*
    fn new_match_tokens(&self, state: &mut TempState) {
        let token = state.iterator.next();
        match token {
            Some(value) => {
                self.match_token(value, state);
            }
            None => (),
        }
    }

    fn match_token(&self, value: &Token, state: &mut TempState<'_>) {
        match value.token_type {
            TokenType::Unknown => panic!("Unknown token type."),
            TokenType::Integer => {
                match value.value {
                    TokenValue::Int(value) => {
                        let value = Expresion::Integer(value);
                        state.statements.push(value);
                    }
                    _ => panic!("Impossible int value."),
                };
            }
            _ => (),
        }
    }
    */

    fn match_tokens(&self, token: &Token, state: &mut TempState) {
        match token.token_type {
            TokenType::Unknown => panic!("Unknown token type."),
            TokenType::Integer => {
                match token.value {
                    TokenValue::Int(value) => {
                        let value = Expresion::Integer(value);
                        state.current_token += 1;
                        state.statements.push(value);
                    }
                    _ => panic!("Impossible int value."),
                };
            }
            _ => {
                state.current_token += 1;
            }
        };
    }

    pub fn get(&self) -> &[Expresion] {
        &self.statements
    }
}
