use std::fmt;

use crate::lexer::{Token, TokenKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ast {
    pub program: Statement,
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.program)
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub node: Node,
    pub body: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.body {
            write!(f, " {}\n", statement)?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Statement {
    ExpressionStatement(Expression),
    Program(Program),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::ExpressionStatement(expr) => write!(f, "{}", expr),
            Statement::Program(program) => write!(f, "{}", program),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expression {
    BinaryExpression(Box<BinaryExpression>),
    UnaryExpression(Box<UnaryExpression>),
    NumberLiteral(Token),
    Identifier(Token),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::BinaryExpression(expr) => write!(f, "({})", expr),
            Expression::NumberLiteral(token) | Expression::Identifier(token) => {
                write!(f, "{}", token)
            }
            Expression::UnaryExpression(expr) => write!(f, "({})", expr),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub node: Node,
    pub left: Expression,
    pub operator: TokenKind,
    pub right: Expression,
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.left, self.operator, self.right)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub node: Node,
    pub operator: TokenKind,
    pub right: Expression,
}

impl fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.right)
    }
}
