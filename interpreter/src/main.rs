use clap::Parser;
use std::fs::OpenOptions;

use symboscript_optimizer as optimizer;
use symboscript_parser as parser;

mod interpreter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the file
    path: String,

    /// Enable debug mode (prints idk what it does)
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let text = OpenOptions::new().read(true).open(&args.path).unwrap();
    let text = &std::io::read_to_string(text).unwrap();

    let mut parser = parser::Parser::new(&args.path, text);

    let ast = {
        let ast = parser.parse();

        optimizer::optimize(&ast)
    };

    let mut interpreter = interpreter::Interpreter::new(&args.path, text, &ast);

    interpreter.run();
}
