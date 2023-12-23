mod lexer;
mod types;
mod utils;
fn main() {
    let text = r#"
    let f = 2x^2 + 2y^2 == 2
    "#;

    let mut lexer = lexer::Lexer::new(text);

    let tokens = lexer.tokenize();

    utils::output_tokens_colored(text, tokens);
}
