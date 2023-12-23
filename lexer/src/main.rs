mod lexer;
mod types;
mod utils;
fn main() {
    let text = r#"
    let formula = 20x^2 + 23y^2 == .212;

    fn func() {
        print(formula);
    }

    let k = 0;
    let str = "h\'i";
    loop {
        func();

        k += 1;

        if k == 10 {
            break;
        }
    }
    "#;

    let mut lexer = lexer::Lexer::new(text);

    let tokens = lexer.tokenize();

    utils::output_tokens_colored(text, &tokens);

    // println!("{:#?}", tokens);
}
