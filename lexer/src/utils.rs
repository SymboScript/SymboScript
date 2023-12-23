use crate::types::{Kind::*, Token};
use colored::Colorize;

pub fn output_tokens_colored(text: &str, tokens: &Vec<Token>) {
    let mut last_start;
    let mut last_end = 0;

    for token in tokens {
        last_start = token.start;
        print!("{}", text[last_end..last_start].to_string());
        last_end = token.end;
        let s = format!("{}", text[token.start..token.end].to_string());
        match token.kind {
            Identifier => print!("{}", s.yellow()),

            Plus | Minus | Star | Slash | Power | Equate | Equal => print!("{}", s.green()),

            Number => print!("{}", s.blue()),

            LParen | RParen | LBrace | RBrace => print!("{}", s.cyan()),

            If | Else | While | For | Loop | Let | Return | Break | Continue | Function | True
            | False => print!("{}", s.magenta()),

            String => print!("{}", s.truecolor(206, 145, 120)),

            _ => print!("{}", s),
        }
    }
    println!();
}
