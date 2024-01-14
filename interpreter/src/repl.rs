use crate::parser::Parser;
use crate::Interpreter;

use rustyline::error::ReadlineError;
use rustyline::Result;

use rustyline::validate::{
    MatchingBracketValidator, ValidationContext, ValidationResult, Validator,
};
use rustyline::Editor;
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};

#[derive(Completer, Helper, Highlighter, Hinter)]
struct InputValidator {
    brackets: MatchingBracketValidator,
}

impl Validator for InputValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult> {
        self.brackets.validate(ctx)
    }
}

pub fn start() -> Result<()> {
    let mut interpreter = Interpreter::new("repl//", "", true);

    interpreter.initialize();

    let mut k = 0;

    let h = InputValidator {
        brackets: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::new()?;
    rl.set_helper(Some(h));

    println!("Symboscript v{} REPL.\n", env!("CARGO_PKG_VERSION"));
    loop {
        let readline = rl.readline("");

        match readline {
            Ok(mut line) => {
                k += 1;

                if line.trim() == "" {
                    continue;
                } else if !line.ends_with(";") && !line.ends_with("}") {
                    line += ";";
                }

                let curr_src = format!("repl/{k}/");
                let ast = Parser::new(&curr_src, &line).parse();

                interpreter.append_to_current_source(line);

                interpreter.eval_ast(ast);
            }

            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }

            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
