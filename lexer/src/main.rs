use clap::Parser;
use std::fs::OpenOptions;

mod lexer;

use symboscript_utils as utils;

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
    let mut lexer = lexer::Lexer::new(&args.path, text, true);
    let tokens = lexer.tokenize();

    if args.debug {
        println!("{:#?}", tokens);
    }

    utils::output_tokens_colored(text, &tokens, Some(args.show_tokens));
}
