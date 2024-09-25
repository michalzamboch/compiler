#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Expresion {
    Unknown(String),
    Integer(i32),
    Binary(Box<Expresion>, &'static str, Box<Expresion>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Statement {
    Unknown(String),
    ExpresionStatement(Box<Expresion>),
}
