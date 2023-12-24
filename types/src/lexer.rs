use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Token {
    /// Token Type
    pub kind: TokenKind,

    /// Start offset in source
    pub start: usize,

    /// End offset in source
    pub end: usize,

    pub value: TokenValue,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Eof, // end of file
    Comment,
    Unexpected,

    Semicolon,
    Comma,
    Colon,
    Dot,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Power,
    Range,
    Modulo,

    // Logic operators
    And,
    Or,
    Not,

    /// Assignments operators (+=, -=, *=, /=...)
    Assign,
    FormulaAssign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    PowerAssign,
    ModuloAssign,

    // Comparison operators
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Brackets
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Identifiers
    Identifier,

    // Literals
    Number,
    String,
    True,
    False,

    // Keywords
    If,
    Else,
    While,
    For,
    Loop,
    Let,
    Return,
    Break,
    Continue,
    Function,
    In,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenValue {
    None,
    Number(f64),
    String(String),
}
