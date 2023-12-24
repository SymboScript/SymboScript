use colored::Colorize;
use symboscript_types::lexer::{Token, TokenKind::*};

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

            Plus | Minus | Star | Slash | Power | Assign | Equal | Range | FormulaAssign | And
            | Or | Xor | Not | BitAnd | BitOr | BitNot | BitXor | BitLeftShift | BitRightShift => {
                print!("{}", s.green())
            }

            Number => print!("{}", s.blue()),

            LParen | RParen | LBrace | RBrace => print!("{}", s.cyan()),

            If | Else | While | For | Loop | Let | Return | Break | Continue | Function | True
            | False | In => print!("{}", s.magenta()),

            String => print!("{}", s.truecolor(206, 145, 120)),

            Comment => print!("{}", s.green()),

            _ => print!("{}", s),
        }
    }

    print!("{}", text[last_end..].to_string());

    println!();
}

pub fn report_error(path: &str, source: &str, error: &str, start: usize, end: usize) {
    let line = source[..start + 1].lines().count();
    let line_end = line - 1 + source[start..end].lines().count();

    let column = start - source[..start].rfind('\n').unwrap_or(0);
    let column_end = end - source[..start].rfind('\n').unwrap_or(0);

    let near_text = source.lines().nth(line - 1).unwrap_or(&"").trim_end();

    let line_n = format!("{line} |");

    let error_pointer = (" ".repeat(column + line_n.len()) + "^".repeat(end - start).as_str())
        .red()
        .bold();
    let error_pointer_text = (&error).red().bold();

    println!(
        "{}\n{} {near_text}\n{error_pointer} {error_pointer_text}",
        format!(
            "--> {}:{}:{}-{}:{}",
            path, line, column, line_end, column_end
        )
        .blue()
        .bold(),
        line_n.to_string().blue().bold(),
    );

    std::process::exit(1);
}