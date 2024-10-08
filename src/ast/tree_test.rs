#![allow(unused_imports, unused_variables, dead_code)]

use super::tree::AbstractSyntaxTree;
use crate::{ast::tree_types::*, lexer::scanner::Scanner};

#[test]
fn test_basic_int() {
    let input = " 5 ";
    let tokens = Scanner::new(input).get_tokens();

    let mut ast = AbstractSyntaxTree::new();
    let result = ast.accept_tokens(&tokens);

    assert!(result.is_ok());

    let statements = ast.get();

    let value = Expresion::Integer(5);
    assert_eq!(value, statements[0]);

    for i in statements.iter() {
        println!("{:?}", i);
    }
}

//#[test]
fn basic_plus_expression() {
    let input = " 5 + 5";
    let tokens = Scanner::new(input).get_tokens();

    let mut ast = AbstractSyntaxTree::new();
    let result = ast.accept_tokens(&tokens);

    assert!(result.is_ok());

    let statements = ast.get();
    let value = Expresion::Binary(
        Box::new(Expresion::Integer(5)),
        "+",
        Box::new(Expresion::Integer(5)),
    );
}
