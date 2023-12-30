use std::fmt::{self};

use crate::lexer::{Token, TokenKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ast {
    pub program: Program,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Program {
    pub node: Node,
    pub body: Vec<Statement>,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Node {
    /// Start offset in source
    pub start: usize,

    /// End offset in source
    pub end: usize,
}

impl Node {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Property {
    pub node: Node,
    pub key: Expression,
    pub value: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Statement {
    ExpressionStatement(Expression),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Expression {
    BinaryExpression(Box<BinaryExpression>),
    UnaryExpression(Box<UnaryExpression>),
    ConditionalExpression(Box<ConditionalExpression>),
    AssignmentExpression(Box<AssignmentExpression>),
    CallExpression(Box<CallExpression>),
    MemberExpression(Box<MemberExpression>),
    SequenceExpression(Box<SequenceExpression>),
    WordExpression(Box<WordExpression>),
    MapExpression(Box<MapExpression>),
    Literal(Token),
    Identifier(Token),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BinaryExpression {
    pub node: Node,
    pub left: Expression,
    pub operator: TokenKind,
    pub right: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnaryExpression {
    pub node: Node,
    pub operator: TokenKind,
    pub right: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConditionalExpression {
    pub node: Node,
    pub test: Expression,
    pub consequent: Expression,
    pub alternate: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CallExpression {
    pub node: Node,
    pub callee: Expression,
    pub arguments: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemberExpression {
    pub node: Node,
    pub object: Expression,
    pub property: Expression,
    pub is_expr: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssignmentExpression {
    pub node: Node,
    pub assignment: TokenKind,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SequenceExpression {
    pub node: Node,
    pub expressions: Vec<Expression>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapExpression {
    pub node: Node,
    pub properties: Vec<Property>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WordExpression {
    pub node: Node,
    pub argument: Expression,
    pub operator: TokenKind,
}

//----------Display------------

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.program)
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.body {
            writeln!(f, "{}", statement)?;
        }

        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::ExpressionStatement(expr) => write!(f, "{};", expr),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::BinaryExpression(expr) => write!(f, "({})", expr),
            Expression::Literal(token) | Expression::Identifier(token) => {
                write!(f, "{}", token)
            }
            Expression::UnaryExpression(expr) => write!(f, "({})", expr),
            Expression::ConditionalExpression(expr) => write!(f, "({})", expr),
            Expression::CallExpression(expr) => write!(f, "({})", expr),
            Expression::MemberExpression(expr) => write!(f, "({})", expr),
            Expression::WordExpression(expr) => write!(f, "({})", expr),
            Expression::AssignmentExpression(expr) => write!(f, "({})", expr),
            Expression::SequenceExpression(expr) => {
                let len = expr.expressions.len();
                let mut k = 0;
                write!(f, "[")?;
                for expr in &expr.expressions {
                    k += 1;
                    write!(f, "{}", expr)?;
                    if k < len {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            Expression::MapExpression(expr) => {
                write!(f, "{{")?;
                for property in &expr.properties {
                    write!(f, " {}: {}; ", property.key, property.value)?;
                }

                write!(f, "}}")
            }
        }
    }
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.left, self.operator, self.right)
    }
}

impl fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.right)
    }
}

impl fmt::Display for ConditionalExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ? {} : {}",
            self.test, self.consequent, self.alternate
        )
    }
}

impl fmt::Display for CallExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.callee, self.arguments)
    }
}

impl fmt::Display for MemberExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_expr {
            write!(f, "{}.{}", self.object, self.property)
        } else {
            write!(f, "{}.[{}]", self.object, self.property)
        }
    }
}

impl fmt::Display for AssignmentExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.left, self.right)
    }
}

impl fmt::Display for SequenceExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for expr in &self.expressions {
            write!(f, "{},", expr)?;
        }
        write!(f, "")
    }
}

impl fmt::Display for WordExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.operator, self.argument)
    }
}
