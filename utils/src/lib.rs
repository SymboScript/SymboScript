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

        print!("{}", {
            match token.kind {
                Identifier => s.yellow(),

                Plus | Minus | Star | Slash | Caret | Assign | Equal | Range | FormulaAssign
                | AmpersandAmpersand | PipePipe | Xor | ExclamationMark | Ampersand | Pipe
                | Tilde | BitXor | BitLeftShift | BitRightShift => s.green(),

                Number => s.blue(),

                LParen | RParen | LAngle | RAngle => s.cyan(),

                If | Else | While | For | Loop | Let | Return | Break | Continue | Function
                | True | False | In => s.magenta(),

                Str => s.truecolor(206, 145, 120),

                DocComment | Comment => s.green(),

                _ => s.into(),
            }
        });
    }

    print!("{}", text[last_end..].to_string());

    println!();
}

pub fn report_error(path: &str, source: &str, error: &str, start: usize, end: usize) {
    let line_start = max(source[..start].lines().count(), 1);
    let line_end = max(source[..end].lines().count(), 1);

    let mut column_start = start - source[..start].rfind('\n').unwrap_or(0);
    let mut column_end = end - source[..end].rfind('\n').unwrap_or(0);

    if line_start == 1 || line_end == 1 {
        column_end += 1;
        column_start += 1;
    }

    if column_end < column_start {
        column_end = source[..end].rfind('\n').unwrap_or(0);
    }

    let near_text = source.lines().nth(line_end - 1).unwrap_or("").trim_end();

    let line_n = format!("{line_end} |");

    let error_pointer = (" ".repeat(column_start + line_n.len())
        + "^".repeat(column_end - column_start).as_str())
    .red()
    .bold();

    let error_pointer_text = error
        .replace(
            "\n",
            &format!(
                "\n{} ",
                " ".repeat(column_end - column_start) + &" ".repeat(column_start + line_n.len())
            ),
        )
        .red()
        .bold();

    let file_src = format!(
        "--> {}:{}:{}-{}:{} ({start} - {end})",
        path, line_start, column_start, line_end, column_end
    );

    println!("{}", file_src.blue().bold());

    for i in line_start..line_end {
        println!(
            "{} {}",
            format!("{} |", i).blue().bold(),
            source.lines().nth(i - 1).unwrap_or("")
        );
    }

    println!(
        "{} {near_text}\n{error_pointer} {error_pointer_text}",
        line_n.to_string().blue().bold(),
    );

    std::process::exit(1);
}
