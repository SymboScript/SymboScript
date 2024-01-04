use crate::parser::*;
use std::{collections::HashMap, ops};

pub type Vault = HashMap<String, ScopeValue>;

#[derive(Clone, Debug)]
pub enum Value {
    None,
    Number(f64),
    Bool(bool),
    Str(String),
    Sequence(Vec<Value>),

    Ast(Expression),

    Function(FunctionDeclarator),
    NativeFunction(NativeFunction),

    Scope(ScopeValue),
}

pub type ScopeValue = HashMap<String, ScopeValues>;

// #[derive(Clone, Debug)]
// pub struct ScopeValue {
//     pub values: HashMap<String, ScopeValues>,
//     pub named_scope_refs: Vec<String>,
// }

// impl ScopeValue {
//     pub fn new() -> ScopeValue {
//         ScopeValue {
//             values: HashMap::new(),
//             named_scope_refs: Vec::new(),
//         }
//     }
// }

#[derive(Clone, Debug)]
pub enum ScopeValues {
    Variable(Value),

    Function(FunctionDeclarator),
    NativeFunction(NativeFunction),

    Scope(ScopeValue),
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
            (Value::Bool(b1), Value::Bool(b2)) => Value::Bool(b1 || b2),

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
            (Value::Bool(b1), Value::Bool(b2)) => Value::Bool(b1 && b2),

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

impl ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => Value::Number(-n),
            _ => Value::None,
        }
    }
}

impl ops::Not for Value {
    type Output = Value;

    fn not(self) -> Self::Output {
        match self {
            Value::None => Value::None,
            Value::Number(n) => Value::Bool(if n == 0.0 { true } else { false }),
            Value::Bool(b) => Value::Bool(!b),
            _ => Value::Bool(false),
        }
    }
}

impl ops::Shl for Value {
    type Output = Value;

    fn shl(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => {
                Value::Number(((n1 as usize) << n2 as usize) as f64)
            }
            _ => Value::None,
        }
    }
}

impl ops::Shr for Value {
    type Output = Value;

    fn shr(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => {
                Value::Number(((n1 as usize) >> n2 as usize) as f64)
            }
            _ => Value::None,
        }
    }
}

impl ops::AddAssign for Value {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl ops::SubAssign for Value {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs
    }
}

impl ops::MulAssign for Value {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs
    }
}

impl ops::DivAssign for Value {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs
    }
}

impl ops::RemAssign for Value {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.clone() % rhs
    }
}

impl ops::ShlAssign for Value {
    fn shl_assign(&mut self, rhs: Self) {
        *self = self.clone() << rhs
    }
}

impl ops::ShrAssign for Value {
    fn shr_assign(&mut self, rhs: Self) {
        *self = self.clone() >> rhs
    }
}
