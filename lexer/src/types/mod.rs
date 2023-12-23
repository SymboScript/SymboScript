use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Token {
    /// Token Type
    pub kind: Kind,

    /// Start offset in source
    pub start: usize,

    /// End offset in source
    pub end: usize,

    pub value: TokenValue,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Eof, // end of file
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
    Equate,
    Range,

    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
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
    True,
    False,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenValue {
    None,
    Number(f64),
    String(String),
    Identifier(String),
}
