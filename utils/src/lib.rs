use std::cmp::max;

use colored::Colorize;
use symboscript_types::lexer::{Token, TokenKind::*};

pub fn output_tokens_colored(text: &str, tokens: &Vec<Token>, show_tokens: Option<bool>) {
    let show_tokens = show_tokens.unwrap_or(false);
    let mut last_start;
    let mut last_end = 0;

    for token in tokens {
        last_start = token.start;
        print!("{}", text[last_end..last_start].to_string());
        last_end = token.end;

        let s = if show_tokens {
            format!("<{}>", text[token.start..token.end].to_string())
        } else {
            format!("{}", text[token.start..token.end].to_string())
        };

        match token.kind {
            Identifier => print!("{}", s.yellow()),

            Plus | Minus | Multiply | Divide | Power | Assign | Equal | Range | FormulaAssign
            | And | Or | Xor | Not | BitAnd | BitOr | BitNot | BitXor | BitLeftShift
            | BitRightShift => {
                print!("{}", s.green())
            }

            Number => print!("{}", s.blue()),

            LParen | RParen | LBrace | RBrace => print!("{}", s.cyan()),

            If | Else | While | For | Loop | Let | Return | Break | Continue | Function | True
            | False | In => print!("{}", s.magenta()),

            Str => print!("{}", s.truecolor(206, 145, 120)),

            DocComment => print!("{}", s.green()),

            _ => print!("{}", s),
        }
    }

    print!("{}", text[last_end..].to_string());

    println!();
}

pub fn report_error(path: &str, source: &str, error: &str, start: usize, end: usize) {
    let line = max(source[..start].lines().count(), 1);
    let line_end = line - 1 + source[start..end].lines().count();

    let column = start + 1 - source[..start].rfind('\n').unwrap_or(0);
    let column_end = end - source[..start].rfind('\n').unwrap_or(0);

    let near_text = source.lines().nth(line - 1).unwrap_or(&"").trim_end();

    let line_n = format!("{line} |");

    let error_pointer = (" ".repeat(column + line_n.len())
        + "^".repeat(max(end - start, 1)).as_str())
    .red()
    .bold();
    let error_pointer_text = (&error).red().bold();

    println!(
        "{}\n{} {near_text}\n{error_pointer} {error_pointer_text}",
        format!(
            "--> {}:{}:{}-{}:{} ({start} - {end})",
            path, line, column, line_end, column_end
        )
        .blue()
        .bold(),
        line_n.to_string().blue().bold(),
    );

    std::process::exit(1);
}
