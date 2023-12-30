use std::fmt;

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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            TokenValue::None => write!(f, "{}", self.kind),
            _ => write!(f, "{}", self.value),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Eof, // end of file
    DocComment,
    Unexpected,
    Skip,
    Start,

    Semicolon,
    Comma,
    Colon,
    Dot,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Range,
    Modulo,

    // Bitwise operators (Keyword2Operator)
    BitAnd,
    BitOr,
    BitNot,
    BitXor,
    BitLeftShift,
    BitRightShift,

    // Unary operators
    PlusPlus,
    MinusMinus,

    // Ternary operators
    Question,

    // Logic operators (Keyword2Operator)
    And,
    Or,
    Xor,

    // Unary logic operators
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
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    // Identifiers
    Identifier,

    // Literals
    Number,
    Str,

    // --- Keywords ---

    // Keyword literals
    True,
    False,
    None,

    // Keywords
    If,
    Else,
    While,
    For,
    Loop,
    Let,
    Return,
    Yield,
    Break,
    Continue,
    Function,
    In,
    Of,
    Delete,
    New,

    // Import/Export
    Import,
    Export,
    As,

    // Async/Await
    Async,
    Await,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::DocComment => write!(f, "DocComment"),
            TokenKind::Unexpected => write!(f, "Unexpected"),
            TokenKind::Skip => write!(f, "Skip"),
            TokenKind::Start => write!(f, "Start"),

            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Dot => write!(f, "."),

            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Multiply => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::Power => write!(f, "^"),
            TokenKind::Range => write!(f, ".."),
            TokenKind::Modulo => write!(f, "%"),

            TokenKind::BitAnd => write!(f, "&"),
            TokenKind::BitOr => write!(f, "|"),
            TokenKind::BitNot => write!(f, "~"),
            TokenKind::BitXor => write!(f, "^"),
            TokenKind::BitLeftShift => write!(f, "<<"),
            TokenKind::BitRightShift => write!(f, ">>"),

            TokenKind::PlusPlus => write!(f, "++"),
            TokenKind::MinusMinus => write!(f, "--"),
            TokenKind::Question => write!(f, "?"),

            TokenKind::And => write!(f, "&&"),
            TokenKind::Or => write!(f, "||"),
            TokenKind::Xor => write!(f, "xor"),
            TokenKind::Not => write!(f, "!"),

            TokenKind::Assign => write!(f, "="),
            TokenKind::FormulaAssign => write!(f, ":="),
            TokenKind::PlusAssign => write!(f, "+="),
            TokenKind::MinusAssign => write!(f, "-="),
            TokenKind::MultiplyAssign => write!(f, "*="),
            TokenKind::DivideAssign => write!(f, "/="),
            TokenKind::PowerAssign => write!(f, "^="),
            TokenKind::ModuloAssign => write!(f, "%="),

            TokenKind::Equal => write!(f, "=="),
            TokenKind::NotEqual => write!(f, "!="),
            TokenKind::Less => write!(f, "<"),
            TokenKind::LessEqual => write!(f, "<="),
            TokenKind::Greater => write!(f, ">"),
            TokenKind::GreaterEqual => write!(f, ">="),

            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),

            TokenKind::Identifier => write!(f, "Identifier"),

            TokenKind::Number => write!(f, "Number"),
            TokenKind::Str => write!(f, "String"),

            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::None => write!(f, "None"),

            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::While => write!(f, "while"),
            TokenKind::For => write!(f, "for"),
            TokenKind::Loop => write!(f, "loop"),
            TokenKind::Let => write!(f, "let"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Yield => write!(f, "yield"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Function => write!(f, "fn"),
            TokenKind::In => write!(f, "in"),
            TokenKind::Of => write!(f, "of"),
            TokenKind::Delete => write!(f, "delete"),
            TokenKind::New => write!(f, "new"),

            TokenKind::Import => write!(f, "import"),
            TokenKind::Export => write!(f, "export"),
            TokenKind::As => write!(f, "as"),

            TokenKind::Await => write!(f, "await"),
            TokenKind::Async => write!(f, "async"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenValue {
    None,
    Number(f64),
    Str(String),
    Identifier(String),
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenValue::None => write!(f, ""),
            TokenValue::Number(s) => write!(f, "{}", s),
            TokenValue::Str(s) => write!(f, "\"{}\"", s),
            TokenValue::Identifier(s) => write!(f, "{}", s),
        }
    }
}
