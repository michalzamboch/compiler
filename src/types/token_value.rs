#![allow(dead_code)]

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TokenValue {
    #[default]
    None,
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
}
