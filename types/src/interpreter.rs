use crate::{lexer::TokenValue, parser::*};
use std::collections::HashMap;

pub type Vault = HashMap<String, ScopeValue>;

pub type VariableValue = TokenValue;

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
    Variable(VariableValue),
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
