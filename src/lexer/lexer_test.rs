#![allow(unused_imports)]

use crate::{
    lexer::scanner::Scanner,
    types::{token_type::*, token_value::TokenValue},
};

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

    assert_eq!(tokens[0].token_type, TokenType::Int);

    assert_eq!(tokens[0].line, 1);
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
    assert_eq!(tokens[0].literal, r#""hello""#);
    assert_eq!(tokens[1].literal, ",");
    assert_eq!(tokens[2].literal, "5");
    assert_eq!(tokens[3].literal, ",");
    assert_eq!(tokens[4].literal, "5.45");
}

#[test]
fn test_dots() {
    let input = r#"CLASS_NAME.FUNCTION,5,5.45"#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    println!("{:?}", tokens);
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].literal, "CLASS_NAME");
    assert_eq!(tokens[1].literal, ".");
    assert_eq!(tokens[2].literal, "FUNCTION");
    assert_eq!(tokens[3].literal, ",");
    assert_eq!(tokens[4].literal, "5");
    assert_eq!(tokens[5].literal, ",");
    assert_eq!(tokens[6].literal, "5.45");
}

#[test]
fn test_equals() {
    let input = r#"==="#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].literal, "==");
    assert_eq!(tokens[1].literal, "=");
}

#[test]
fn test_and_or() {
    let input = r#" || && "#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Or);
    assert_eq!(tokens[1].token_type, TokenType::And);
}

#[test]
fn test_true_false() {
    let input = r#" true  false "#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].value, TokenValue::Bool(true));
    assert_eq!(tokens[1].value, TokenValue::Bool(false));
}

#[test]
fn test_string() {
    let input = r#" "Hello".,"World" "#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].value, TokenValue::String("Hello".to_owned()));
    assert_eq!(tokens[1].token_type, TokenType::Dot);
    assert_eq!(tokens[2].token_type, TokenType::Comma);
    assert_eq!(tokens[3].value, TokenValue::String("World".to_owned()));
}

#[test]
fn test_int() {
    let input = r#" 123.456Hello 123 Hello123"#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    assert_eq!(tokens[0].literal, "123.456");
    assert_eq!(tokens[0].token_type, TokenType::Real);
    assert_eq!(tokens[0].value, TokenValue::Float(123.456));

    assert_eq!(tokens[1].literal, "Hello");

    assert_eq!(tokens[2].literal, "123");
    assert_eq!(tokens[2].token_type, TokenType::Interger);
    assert_eq!(tokens[2].value, TokenValue::Int(123));

    assert_eq!(tokens[3].literal, "Hello123");
    assert_eq!(tokens[3].token_type, TokenType::Identifier);
    assert_eq!(tokens[3].value, TokenValue::None);
}

#[test]
fn test_name() {
    let input = r#" variable variable1 5000 variable2variable "#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].literal, "variable");
    assert_eq!(tokens[0].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].literal, "variable1");
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].literal, "5000");
    assert_eq!(tokens[2].token_type, TokenType::Interger);
    assert_eq!(tokens[3].literal, "variable2variable");
    assert_eq!(tokens[3].token_type, TokenType::Identifier);
}

#[test]
fn test_while() {
    let input = r#" 
    bool x = true;
    while(x==true)
    {
    }
    "#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    assert_eq!(tokens[0].token_type, TokenType::Boolean);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Equal);
    assert_eq!(tokens[3].token_type, TokenType::True);
    assert_eq!(tokens[4].token_type, TokenType::Semicolon);
    assert_eq!(tokens[5].token_type, TokenType::While);
    assert_eq!(tokens[6].token_type, TokenType::LeftParen);
    assert_eq!(tokens[7].token_type, TokenType::Identifier);
    assert_eq!(tokens[8].token_type, TokenType::EqualEqual);
    assert_eq!(tokens[9].token_type, TokenType::True);
    assert_eq!(tokens[10].token_type, TokenType::RightParen);
    assert_eq!(tokens[11].token_type, TokenType::LeftBrace);
    assert_eq!(tokens[12].token_type, TokenType::RightBrace);
}

#[test]
fn test_error_standard_strings() {
    let input = r#" 
    "
    hello
    "
    "#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    assert_eq!(tokens[0].token_type, TokenType::Unknown);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Unknown);
}

#[test]
fn test_assign() {
    let input = r#"int i=10+20;"#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    assert_eq!(tokens[0].token_type, TokenType::Int);
    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[2].token_type, TokenType::Equal);
    assert_eq!(tokens[3].token_type, TokenType::Interger);
    assert_eq!(tokens[4].token_type, TokenType::Plus);
    assert_eq!(tokens[5].token_type, TokenType::Interger);
}

#[test]
fn test_brackets() {
    let input = r#"{[()]}"#;
    let scanner = Scanner::new(input);
    let tokens = scanner.get_tokens();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type, TokenType::LeftBrace);
    assert_eq!(tokens[1].token_type, TokenType::LeftSquareBracket);
    assert_eq!(tokens[2].token_type, TokenType::LeftParen);
    assert_eq!(tokens[3].token_type, TokenType::RightParen);
    assert_eq!(tokens[4].token_type, TokenType::RightSquareBracket);
    assert_eq!(tokens[5].token_type, TokenType::RightBrace);
}
