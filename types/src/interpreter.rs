use crate::lexer::*;
use crate::parser::*;
use std::collections::HashMap;

pub type Scope = HashMap<String, ScopeValue>;

pub enum ScopeValue {
    Variable(Expression),
    Function(Vec<String>, BlockStatement),
    ScopeName(String),
}
