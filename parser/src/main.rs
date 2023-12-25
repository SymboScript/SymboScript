use clap::Parser;
use std::fs::OpenOptions;

mod parser;

use symboscript_lexer as lexer;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the file
    path: String,

    /// Enable debug mode (prints all tokens)
    #[clap(short, long)]
    debug: bool,

    /// Show tokens <token>
    #[clap(short, long)]
    show_tokens: bool,
}

fn main() {
    let args = Args::parse();

    let text = OpenOptions::new().read(true).open(&args.path).unwrap();
    let text = &std::io::read_to_string(text).unwrap();
    let lexer = lexer::Lexer::new(&args.path, text);

    let mut parser = parser::Parser::new(&args.path, &text, lexer);

    let ast = parser.parse();

    println!("{}", ast);
}
