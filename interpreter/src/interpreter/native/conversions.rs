use symboscript_types::{
    interpreter::{NativeFunction, Scope, Value},
    parser::CallExpression,
};

use super::Interpreter;

pub fn to_string(
    interpreter: &mut Interpreter,
    call_expr: &CallExpression,
    args: &Vec<Value>,
) -> Value {
    if args.len() != 0 {
        interpreter.report(
            "Wrong number of arguments (expected 0)",
            call_expr.node.start,
            call_expr.node.end,
        );
    }

    let value = interpreter.get_cur_value(&"$value".to_owned());

    match value.clone() {
        Value::Str(_) => return value,
        Value::None => return Value::Str("None".to_owned()),
        Value::Number(n) => return Value::Str(n.to_string()),
        Value::Bool(b) => return Value::Str(b.to_string()),
        Value::Sequence(_) => todo!(),
        Value::Ast(_) => todo!(),
        Value::ScopeRef(sref) => return Value::Str(sref),
        Value::NativeFunction(_) => todo!(),
        Value::Function(_) => todo!(),
        Value::Err(_) => todo!(),
    }
}

pub fn is_err(
    interpreter: &mut Interpreter,
    call_expr: &CallExpression,
    args: &Vec<Value>,
) -> Value {
    if args.len() != 0 {
        interpreter.report(
            "Wrong number of arguments (expected 0)",
            call_expr.node.start,
            call_expr.node.end,
        );
    }

    let value = interpreter.get_cur_value(&"$value".to_owned());

    match value {
        Value::Err(_) => Value::Bool(true),
        _ => Value::Bool(false),
    }
}

pub fn inject_methods(scope: &mut Scope) {
    scope.insert(
        "to_string".to_owned(),
        Value::NativeFunction(NativeFunction::ToString),
    );

    scope.insert(
        "is_err".to_owned(),
        Value::NativeFunction(NativeFunction::IsError),
    );
}
