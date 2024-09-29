#![allow(dead_code)]

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum Expresion {
    Unknown(String),
    Integer(i32),
    Binary(Box<Expresion>, &'static str, Box<Expresion>),
    Unary(&'static str, Box<Expresion>),
}

impl Display for Expresion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = serde_json::to_string_pretty(self);

        write!(f, "{:?}", result.unwrap_or("Error".to_string()))
    }
}
