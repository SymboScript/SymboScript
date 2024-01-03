use crate::parser::*;
use std::{collections::HashMap, ops};

pub type Vault = HashMap<String, ScopeValue>;

#[derive(Clone, Debug)]
pub enum Value {
    None,
    Number(f64),
    Str(String),
    Sequence(Vec<Value>),
}

#[derive(Clone, Debug)]
pub struct ScopeValue {
    pub values: HashMap<String, ScopeValues>,
    pub named_scope_refs: Vec<String>,
}

impl ScopeValue {
    pub fn new() -> ScopeValue {
        ScopeValue {
            values: HashMap::new(),
            named_scope_refs: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ScopeValues {
    Variable(Value),
    FormulaVariable(Expression),

    Function(FunctionDeclarator),
    NativeFunction(NativeFunction),

    ScopeRef(String),
}

#[derive(Clone, Debug)]
pub enum NativeFunction {
    Print,
    Println,
}

// ----------------- Math -----------------

impl ops::Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 + n2),
            (Value::Number(n), Value::Str(str)) => Value::Str(format!("{}{}", n, str)),
            (Value::Str(str), Value::Number(n)) => Value::Str(format!("{}{}", str, n)),
            (Value::Str(str1), Value::Str(str2)) => Value::Str(str1 + &str2),

            _ => Value::None,
        }
    }
}

impl ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 - n2),

            _ => Value::None,
        }
    }
}

impl ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 * n2),
            (Value::Str(str), Value::Number(n)) => Value::Str(str.repeat(n as usize)),
            (Value::Number(n), Value::Str(str)) => Value::Str(str.repeat(n as usize)),

            _ => Value::None,
        }
    }
}

impl ops::Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 / n2),
            _ => Value::None,
        }
    }
}

impl ops::Rem for Value {
    type Output = Value;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 % n2),
            _ => Value::None,
        }
    }
}
