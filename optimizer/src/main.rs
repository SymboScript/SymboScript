use clap::Parser;
use serde_json::json;
use std::fs::OpenOptions;
use symboscript_parser as parser;

mod optimizer;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the file
    path: String,

    /// Enable debug mode (prints the AST)
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let text = OpenOptions::new().read(true).open(&args.path).unwrap();
    let text = &std::io::read_to_string(text).unwrap();

    let mut parser = parser::Parser::new(&args.path, text);

    let ast = parser.parse();

    let optimized_ast = optimizer::optimize(&ast);

    let ast_str = {
        let ast_str = format!("{}", ast);
        ast_str.trim().to_owned()
    };
    let optimized_ast_str = {
        let ast_str = format!("{}", optimized_ast);
        ast_str.trim().to_owned()
    };

    println!("{} -> {}", ast_str, optimized_ast_str);

    if args.debug {
        println!("{}", serde_yaml::to_string(&json!(optimized_ast)).unwrap())
    };
}
