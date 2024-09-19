#![allow(dead_code)]

use super::token::Token;
use regex::Regex;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    re: Regex,
}

// https://regex101.com/r/qQCUrf/1

// ^[a-zA-Z0-9\^_=\!#\$%&\(\)\*\+\-\.:'/\?@ ]*$
// (?:[-+]?[0-9]*\.?[0-9]+|\d+|\w+)
// (?:[0-9]*\.?[0-9]+|\d+|\w+|\;|\(|\)|\{|\}|\[|\]|\-|\+|\*|\/|\=)
// (?:if|else|for|while|return|break|continue|switch|case|default|try|catch|throw|new|class|public|private|protected|static|void|int|float|double|char|boolean|true|false)

impl Scanner {
    pub fn new(src: &str) -> Self {
        let expresion = r"(?:[0-9]*\.?[0-9]+|\d+|\w+|\;|\(|\)|\{|\}|\[|\]|\-|\+|\*|\/|\=)";
        let re = Regex::new(&expresion);

        Self {
            source: src.to_owned(),
            re: re.unwrap(),
        }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        for element in self.re.captures_iter(&self.source) {
            println!("{:?}", element);
        }

        vec![]
    }
}
