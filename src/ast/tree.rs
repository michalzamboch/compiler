#![allow(dead_code, unused_variables)]

use std::error::Error;

use crate::lexer::token::Token;
use super::tree_types::*;

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
        
        Ok(())
    }

    pub fn get(&self) -> &[Statement] {
        &self.statements
    }
}