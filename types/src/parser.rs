use crate::lexer::{Token, TokenKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ast {
    pub program: Program,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum Statement {
    ExpressionStatement(Expression),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Expression {
    BinaryExpression(Box<BinaryExpression>),
    NumberLiteral(Token),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub node: Node,
    pub left: Expression,
    pub operator: TokenKind,
    pub right: Expression,
}
