use clap::Parser;
use std::fs::OpenOptions;

// use symboscript_optimizer as optimizer;
use symboscript_parser as parser;

mod interpreter;
mod repl;

use interpreter::Interpreter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the file
    path: Option<String>,

    /// Enable debug mode
    /// TODO: implement
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    match args.path {
        Some(path) => {
            let text = OpenOptions::new().read(true).open(&path).unwrap();
            let text = &std::io::read_to_string(text).unwrap();

            let mut parser = parser::Parser::new(&path, text);

            let ast = parser.parse();
            // let ast = optimizer::optimize(&ast);

            let mut interpreter = Interpreter::new(&path, text, false);

            interpreter.run(ast);
        }

        None => {
            let _ = repl::start();
        }
    }
}
