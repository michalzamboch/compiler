#![allow(dead_code)]

use std::fmt;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TokenValue {
    #[default]
    None,
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
