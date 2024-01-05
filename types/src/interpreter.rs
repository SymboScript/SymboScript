use crate::parser::*;
use std::{collections::HashMap, ops};

pub type Vault = HashMap<String, ScopeValue>;

pub type Scope = HashMap<String, Value>;
#[derive(Clone, Debug)]
pub enum Value {
    None,
    Number(f64),
    Bool(bool),
    Str(String),
    Sequence(Vec<Value>),

    Ast(Expression),
    ScopeRef(String),

    NativeFunction(NativeFunction),
    Function(FunctionDeclarator),
}

#[derive(Clone, Debug)]
pub enum ControlFlow {
    Continue,
    Break,
    Return(Value),
    Yield(Value),
    None,
}

#[derive(Clone, Debug)]
pub struct ScopeValue {
    pub values: Scope,
    pub named_scope_refs: Vec<String>,
}

impl ScopeValue {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            named_scope_refs: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub enum NativeFunction {
    Print,
    Println,
    ToString,
}

// ----------------- Math -----------------

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::None => false,
            Value::Number(n) => *n != 0.0,
            Value::Bool(b) => *b,
            _ => true,
        }
    }

    pub fn and(&self, other: &Value) -> Value {
        Value::Bool(match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => *b1 && *b2,
            _ => false,
        })
    }

    pub fn or(&self, other: &Value) -> Value {
        Value::Bool(match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => *b1 || *b2,
            _ => false,
        })
    }

    pub fn xor(&self, other: &Value) -> Value {
        Value::Bool(match (self, other) {
            (Value::Bool(b1), Value::Bool(b2)) => *b1 ^ *b2,
            _ => false,
        })
    }

    pub fn bit_and(&self, other: &Value) -> Value {
        Value::Number(match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => {
                if n1.fract() != 0.0 || n2.fract() != 0.0 {
                    return Value::None;
                } else {
                    ((*n1 as usize) & (*n2 as usize)) as f64
                }
            }
            _ => 0.0,
        })
    }

    pub fn bit_or(&self, other: &Value) -> Value {
        Value::Number(match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => {
                if n1.fract() != 0.0 || n2.fract() != 0.0 {
                    return Value::None;
                } else {
                    ((*n1 as usize) | (*n2 as usize)) as f64
                }
            }
            _ => 0.0,
        })
    }

    pub fn bit_xor(&self, other: &Value) -> Value {
        Value::Number(match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => {
                if n1.fract() != 0.0 || n2.fract() != 0.0 {
                    return Value::None;
                } else {
                    ((*n1 as usize) ^ (*n2 as usize)) as f64
                }
            }
            _ => 0.0,
        })
    }

    pub fn pow(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left.powf(*right)),
            _ => Value::None,
        }
    }

    pub fn range(&self, other: &Value) -> Value {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => {
                let left = left.round() as usize;
                let right = right.round() as usize;

                let val = (left..=right).collect::<Vec<usize>>();
                Value::Sequence(val.into_iter().map(|p| Value::Number(p as f64)).collect())
            }
            _ => Value::None,
        }
    }

    pub fn equal(&self, other: &Value) -> Value {
        Value::Bool(match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Str(s1), Value::Str(s2)) => s1 == s2,
            // (Value::Ast(e1), Value::Ast(e2)) => e1 == e2,
            (Value::ScopeRef(s1), Value::ScopeRef(s2)) => s1 == s2,
            // (Value::Function(f1), Value::Function(f2)) => f1 == f2,
            // (Value::NativeFunction(n1), Value::NativeFunction(n2)) => n1 == n2,
            (Value::None, Value::None) => true,
            // (Value::Sequence(s1), Value::Sequence(s2)) => s1 == s2,
            _ => false,
        })
    }

    pub fn not_equal(&self, other: &Value) -> Value {
        return !self.equal(other);
    }

    pub fn greater(&self, other: &Value) -> Value {
        Value::Bool(match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => n1 > n2,
            _ => false,
        })
    }

    pub fn greater_equal(&self, other: &Value) -> Value {
        Value::Bool(match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => n1 >= n2,
            _ => false,
        })
    }

    pub fn less(&self, other: &Value) -> Value {
        Value::Bool(match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => n1 > n2,
            _ => false,
        })
    }

    pub fn less_equal(&self, other: &Value) -> Value {
        Value::Bool(match (self, other) {
            (Value::Number(n1), Value::Number(n2)) => n1 >= n2,
            _ => false,
        })
    }
}

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
