use symboscript_types::interpreter::{NativeFunction, Value};

use super::Interpreter;

pub mod io;

pub fn run_function(
    _interpreter: &mut Interpreter,
    native_function: &NativeFunction,
    args: Vec<Value>,
) -> Value {
    match native_function {
        NativeFunction::Println => io::println(&args),
        NativeFunction::Print => io::print(&args),
    }

    Value::None
}
