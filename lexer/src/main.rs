use std::fs::OpenOptions;

mod lexer;
mod types;
mod utils;
fn main() {
    let text = OpenOptions::new()
        .read(true)
        .open("./examples/test.syms")
        .unwrap();

    let text = &std::io::read_to_string(text).unwrap();

    let mut lexer = lexer::Lexer::new("./examples/test.syms", text);

    let tokens = lexer.tokenize();

    utils::output_tokens_colored(text, &tokens);

    // println!("{:#?}", tokens);
}
