use symboscript_types::{
    interpreter::{NativeFunction, Value},
    parser::CallExpression,
};

use super::Interpreter;

pub mod conversions;
pub mod io;

pub fn run_function(
    interpreter: &mut Interpreter,
    call_expr: &CallExpression,
    native_function: &NativeFunction,
    args: &Vec<Value>,
) -> Value {
    match native_function {
        NativeFunction::Println => io::println(&args),
        NativeFunction::Print => io::print(&args),
        NativeFunction::ToString => return conversions::to_string(interpreter, call_expr, args),
        NativeFunction::IsError => return conversions::is_err(interpreter, call_expr, args),
    }
    Value::None
}

pub fn inject(interpreter: &mut Interpreter) {
    let scope = interpreter.start_declaration_of_named_scope("io");
    io::inject(interpreter.get_curr_scope_values_mut());
    interpreter.end_declaration_of_named_scope(&scope);

    // ----------------- Std conversions --------------------------------

    for name in ["&number", "&bool", "&str", "&sequence", "&ast", "&err"] {
        let scope = interpreter.start_declaration_of_named_scope(name);
        conversions::inject_methods(interpreter.get_curr_scope_values_mut());
        interpreter.end_declaration_of_named_scope(&scope);
    }
}
