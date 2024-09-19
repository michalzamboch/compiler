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

#[test]
fn test_math_tokens() {
    let input = r"5 + 4 - 7*10/2";
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    println!("{:?}", tokens);
    assert_eq!(tokens.len(), 9);
    assert_eq!(tokens[1].token_type, TokenType::Plus);
    assert_eq!(tokens[3].token_type, TokenType::Minus);
    assert_eq!(tokens[5].token_type, TokenType::Star);
    assert_eq!(tokens[7].token_type, TokenType::Slash);
}

#[test]
fn test_commas() {
    let input = r#""hello",5,5.45"#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    println!("{:?}", tokens);
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0].lexeme, r#""hello""#);
    assert_eq!(tokens[1].lexeme, ",");
    assert_eq!(tokens[2].lexeme, "5");
    assert_eq!(tokens[3].lexeme, ",");
    assert_eq!(tokens[4].lexeme, "5.45");
}

#[test]
fn test_dots() {
    let input = r#"CLASS_NAME.FUNCTION,5,5.45"#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    println!("{:?}", tokens);
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].lexeme, "CLASS_NAME");
    assert_eq!(tokens[1].lexeme, ".");
    assert_eq!(tokens[2].lexeme, "FUNCTION");
    assert_eq!(tokens[3].lexeme, ",");
    assert_eq!(tokens[4].lexeme, "5");
    assert_eq!(tokens[5].lexeme, ",");
    assert_eq!(tokens[6].lexeme, "5.45");
}

#[test]
fn test_equals() {
    let input = r#"==="#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    println!("{:?}", tokens);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].lexeme, "==");
    assert_eq!(tokens[1].lexeme, "=");
}

#[test]
fn test_and_or() {
    let input = r#" || && "#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    println!("{:?}", tokens);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Or);
    assert_eq!(tokens[1].token_type, TokenType::And);
}
