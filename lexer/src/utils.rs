use crate::types::{Kind::*, Token};
use colored::Colorize;

pub fn output_tokens_colored(text: &str, tokens: Vec<Token>) {
    for token in tokens {
        // match tokens by kind type
        let s = format!("{}", text[token.start..token.end].to_string());
        match token.kind {
            Identifier => print!("{}", s.yellow()),

            Plus | Minus | Star | Slash | Power | Equation | Equal => print!("{}", s.green()),

            Number => print!("{}", s.blue()),

            LParen | RParen | LBrace | RBrace => print!("{}", s.cyan()),

            If | Else | While | For | Loop | Let | Return | Break | Continue | Function | True
            | False => print!("{}", s.magenta()),

            _ => print!("{}", s.red()),
        }
    }
}
