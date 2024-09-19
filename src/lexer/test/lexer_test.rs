#![allow(unused_imports)]

use crate::{lexer::scanner::Scanner, types::token_type::*};

#[test]
fn test_length() {
    let input = "int i = 10;";
    let scanner = Scanner::new(input);

    let tokens = scanner.get_tokens();
    assert_eq!(tokens.len(), 5);
}

#[test]
fn test_int_expression() {
    let input = r"int i = 10;";
    let scanner = Scanner::new(input);

    let tokens = scanner.get_tokens();

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].token_type, TokenType::Int);
}

#[test]
fn test_new_line() {
    let input = r"
    int";
    let scanner = Scanner::new(input);

    let tokens = scanner.get_tokens();
    assert_eq!(tokens.len(), 2);

    assert_eq!(tokens[1].token_type, TokenType::Int);
    assert_eq!(tokens[0].token_type, TokenType::NewLine);

    assert_eq!(tokens[0].line, 0);
    assert_eq!(tokens[1].line, 1);
}

#[test]
fn test_emelent_count() {
    let input = r"
        int x = 10;
        float y = 3.14f;
        boolean z = true;
        class MyClass {
            void myMethod() {
                print('Hello, world!');
            }
        }
    ";
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    assert_eq!(
        tokens
            .iter()
            .filter(|e| e.token_type == TokenType::LeftBrace)
            .count(),
        2
    );

    assert_eq!(
        tokens
            .iter()
            .filter(|e| e.token_type == TokenType::NewLine)
            .count(),
        9
    );

    assert_eq!(
        tokens
            .iter()
            .any(|e| e.token_type == TokenType::LeftParen || e.token_type == TokenType::RightParen),
        true
    );
}

#[test]
fn test_element_lines() {
    let input = r"
        int x = 10;
        float y = 3.14f;
        boolean z = true;
        class MyClass {
            void myMethod() {
                print('Hello, world!');
            }
        }
    ";
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    let result = tokens
        .iter()
        .filter(|e| e.token_type == TokenType::Semicolon)
        .map(|e| e.line)
        .collect::<Vec<i32>>();
    assert_eq!(result, vec![1, 2, 3, 6]);
}

#[test]
fn test_eq_tokens() {
    let input = r"    == <= >= != =    ";
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    println!("{:?}", tokens);
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].token_type, TokenType::EqualEqual);
    assert_eq!(tokens[1].token_type, TokenType::LessEqual);
    assert_eq!(tokens[2].token_type, TokenType::GreaterEqual);
    assert_eq!(tokens[3].token_type, TokenType::BangEqual);
    assert_eq!(tokens[4].token_type, TokenType::Equal);
}
