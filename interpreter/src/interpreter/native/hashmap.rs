use symboscript_types::{
    interpreter::{NativeFunction, Scope, Value},
    parser::{CallExpression, FunctionDeclarator},
};

use crate::expect_args;

use super::Interpreter;

pub fn set(interpreter: &mut Interpreter, call_expr: &CallExpression, args: &Vec<Value>) {
    expect_args!(3, interpreter, call_expr, args);

    let scope_ref = args[0].clone();
    let key = args[1].clone();
    let value = args[2].clone();

    let scope = match scope_ref {
        Value::ScopeRef(ref_name) => ref_name,
        got => {
            interpreter.report(
                format!("{} is not a scope reference", got).as_str(),
                call_expr.node.start,
                call_expr.node.end,
            );
            unreachable!("Report ends proccess");
        }
    };

    interpreter
        .vault
        .get_mut(&scope)
        .unwrap()
        .values
        .insert(key.to_string(), value);
}

pub fn new(interpreter: &mut Interpreter, call_expr: &CallExpression, args: &Vec<Value>) -> Value {
    expect_args!(0, interpreter, call_expr, args);

    let scope = interpreter.start_declaration_of_id_scope();
    interpreter.declare_variable(&"this".to_owned(), Value::ScopeRef(scope.clone()));
    interpreter.eval_ast(interpreter.std_lang.hashmap.clone());
    interpreter.end_declaration_of_named_scope(&scope);

    Value::ScopeRef(scope)
}

pub fn inject(scope: &mut Scope) {
    scope.insert(
        "set".to_owned(),
        Value::NativeFunction(NativeFunction::HMSet),
    );

    scope.insert(
        "new".to_owned(),
        Value::NativeFunction(NativeFunction::HMNew),
    );
}
