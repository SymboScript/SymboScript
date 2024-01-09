use colored::Colorize;
use symboscript_types::interpreter::{NativeFunction, Scope, Value};

pub fn println(s: &Vec<Value>) {
    print(s);
    println!();
}

pub fn print(s: &Vec<Value>) {
    for val in s {
        match val {
            Value::None => print!("{}", "None".blue().bold()),
            Value::Number(n) => print!("{}", n.to_string().green()),
            Value::Bool(b) => print!("{}", b.to_string().blue().bold()),
            Value::Str(str) => print!("{}", str),
            Value::Sequence(_) => todo!(),
            Value::Ast(v) => print!("{}", v),
            Value::ScopeRef(v) => print!("{}", v),
            Value::NativeFunction(_) => todo!(),
            Value::Function(v) => print!("{}", v),
            Value::Err(e) => print!("{}", e),
        }

        print!(" ")
    }
}

pub fn inject(scope: &mut Scope) {
    scope.insert(
        "print".to_owned(),
        Value::NativeFunction(NativeFunction::IOPrint),
    );

    scope.insert(
        "println".to_owned(),
        Value::NativeFunction(NativeFunction::IOPrintln),
    );
}
