use symboscript_types::{
    interpreter::{NativeFunction, Scope, Value},
    parser::CallExpression,
};

use crate::{expect_args, mut_values_hm};

use super::Interpreter;

pub fn set(interpreter: &mut Interpreter, call_expr: &CallExpression, args: &Vec<Value>) {
    expect_args!(3, interpreter, call_expr, args);

    let scope_ref = args[0].clone();
    let key = args[1].clone();
    let value = args[2].clone();

    let scope = match_scope(&scope_ref, interpreter, call_expr);

    interpreter
        .vault
        .get_mut(&scope)
        .unwrap()
        .values
        .insert(key.to_string(), value);
}

pub fn del(interpreter: &mut Interpreter, call_expr: &CallExpression, args: &Vec<Value>) {
    let values = mut_values_hm!(2, interpreter, call_expr, args);

    values.remove(&args[1].to_string());
}

pub fn has(interpreter: &mut Interpreter, call_expr: &CallExpression, args: &Vec<Value>) -> Value {
    let values = mut_values_hm!(2, interpreter, call_expr, args);

    Value::Bool(values.contains_key(&args[1].to_string()))
}

pub fn len(interpreter: &mut Interpreter, call_expr: &CallExpression, args: &Vec<Value>) -> Value {
    let values = mut_values_hm!(1, interpreter, call_expr, args);

    Value::Number(values.len() as f64)
}

pub fn keys(interpreter: &mut Interpreter, call_expr: &CallExpression, args: &Vec<Value>) -> Value {
    let values = mut_values_hm!(1, interpreter, call_expr, args);

    Value::Sequence(
        values
            .keys()
            .filter(|v| match v.as_str() {
                // TODO: Remove this when arrays will be implemented in language and add this to native/lang/hashmap.syms(.rs)
                "this" | "set" | "get" | "del" | "has" | "len" | "keys" | "values" | "clear" => {
                    false
                }
                _ => true,
            })
            .map(|k| Value::Str(k.to_string()))
            .collect(),
    )
}

pub fn values(
    interpreter: &mut Interpreter,
    call_expr: &CallExpression,
    args: &Vec<Value>,
) -> Value {
    let values = mut_values_hm!(1, interpreter, call_expr, args);

    Value::Sequence(
        values
            .values()
            .cloned()
            .filter(|v| match v {
                Value::Function(_) | Value::ScopeRef(_) => false,
                _ => true,
            })
            .collect(),
    )
}

pub fn clear(interpreter: &mut Interpreter, call_expr: &CallExpression, args: &Vec<Value>) {
    let values = mut_values_hm!(1, interpreter, call_expr, args);

    for key in values.clone().keys() {
        // TODO: remove this when arrays will be implemented in language and add this to native/lang/hashmap.syms(.rs)
        if [
            "this", "set", "get", "del", "has", "len", "keys", "values", "clear",
        ]
        .contains(&key.as_str())
        {
            continue;
        }

        values.remove(key);
    }
}

pub fn get(interpreter: &mut Interpreter, call_expr: &CallExpression, args: &Vec<Value>) -> Value {
    let values = mut_values_hm!(2, interpreter, call_expr, args);
    let val = values.get(&args[1].to_string());

    match val {
        Some(v) => v.clone(),
        None => Value::None,
    }
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

    scope.insert(
        "get".to_owned(),
        Value::NativeFunction(NativeFunction::HMGet),
    );

    scope.insert(
        "del".to_owned(),
        Value::NativeFunction(NativeFunction::HMDelete),
    );

    scope.insert(
        "has".to_owned(),
        Value::NativeFunction(NativeFunction::HMHas),
    );

    scope.insert(
        "len".to_owned(),
        Value::NativeFunction(NativeFunction::HMLen),
    );

    scope.insert(
        "keys".to_owned(),
        Value::NativeFunction(NativeFunction::HMKeys),
    );

    scope.insert(
        "values".to_owned(),
        Value::NativeFunction(NativeFunction::HMValues),
    );

    scope.insert(
        "clear".to_owned(),
        Value::NativeFunction(NativeFunction::HMClear),
    );
}

fn match_scope(
    scope_ref: &Value,
    interpreter: &mut Interpreter,
    call_expr: &CallExpression,
) -> String {
    match scope_ref {
        Value::ScopeRef(ref_name) => ref_name.to_string(),
        got => {
            interpreter.report(
                format!("{} is not a scope reference", got).as_str(),
                call_expr.node.start,
                call_expr.node.end,
            );
            unreachable!("Report ends proccess");
        }
    }
}
