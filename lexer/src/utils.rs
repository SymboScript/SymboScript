use crate::types::{Kind, Token};
use colored::Colorize;

pub fn output_tokens_colored(text: &str, tokens: Vec<Token>) {
    for token in tokens {
        // match tokens by kind type
        let s = format!("{}", text[token.start..token.end].to_string());
        match token.kind {
            Kind::Identifier => print!("{}", s.yellow()),

            _ => print!("{}", s.red()),
        }
    }
}
