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

pub fn inject(interpreter: &mut Interpreter) {
    interpreter.declare_named_scope("io");
    io::inject(interpreter.get_curr_scope_values());
    interpreter.exit_named_scope();
}
