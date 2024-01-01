use clap::Parser;
use std::fs::OpenOptions;

use symboscript_lexer as lexer;

mod formatter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the file
    path: String,

    /// Enable debug mode (prints formatted code)
    #[clap(short, long)]
    debug: bool,

    /// Dont write formatted code
    #[clap(long)]
    dry_run: bool,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let text = OpenOptions::new().read(true).open(&args.path).unwrap();
    let text = &std::io::read_to_string(text).unwrap();

    let lexer = lexer::Lexer::new(&args.path, text, true);

    let mut formatter = formatter::Formatter::new(&text, lexer);

    let fmt_str = formatter.format();

    if args.debug {
        println!("{}", fmt_str);
    }

    if !args.dry_run {
        let tmp_path = &format!("{}.tmp.{}", args.path, std::process::id());
        std::fs::write(tmp_path, fmt_str)?;
        std::fs::rename(tmp_path, args.path)?;
    }

    Ok(())
}
