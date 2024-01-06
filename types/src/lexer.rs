use std::{fmt, ops};

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

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::Start,
            start: 0,
            end: 0,
            value: TokenValue::None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Eof, // end of file
    DocComment,
    Comment,
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
    Star,
    Slash,
    Caret,
    Range,
    Modulo,

    // Bitwise operators (Keyword2Operator)
    Ampersand,
    Pipe,
    Tilde,
    BitXor,
    BitLeftShift,
    BitRightShift,

    // Unary operators
    PlusPlus,
    MinusMinus,

    // Ternary operators
    Question,

    // Logic operators (Keyword2Operator)
    AmpersandAmpersand,
    PipePipe,
    Xor,

    // Unary logic operators
    ExclamationMark,

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
    LParen,  // (
    RParen,  // )
    LAngle,  // {
    RAngle,  // }
    LSquare, // [
    RSquare, // ]

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
    Scope, // Scope declaration
    Return,
    Yield,
    Break,
    Continue,
    Function,
    In,
    Of,
    Delete,
    Throw,

    Mut,

    // Import
    Import,
    As,

    // Async/Await
    Async,
    Await,

    Block,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::DocComment => write!(f, "DocComment"),
            TokenKind::Comment => write!(f, "Comment"),
            TokenKind::Unexpected => write!(f, "Unexpected"),
            TokenKind::Skip => write!(f, "Skip"),
            TokenKind::Start => write!(f, "Start"),

            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Dot => write!(f, "."),

            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::Range => write!(f, ".."),
            TokenKind::Modulo => write!(f, "%"),

            TokenKind::Ampersand => write!(f, "&"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::BitXor => write!(f, "^"),
            TokenKind::BitLeftShift => write!(f, "<<"),
            TokenKind::BitRightShift => write!(f, ">>"),

            TokenKind::PlusPlus => write!(f, "++"),
            TokenKind::MinusMinus => write!(f, "--"),
            TokenKind::Question => write!(f, "?"),

            TokenKind::AmpersandAmpersand => write!(f, "&&"),
            TokenKind::PipePipe => write!(f, "||"),
            TokenKind::Xor => write!(f, "xor"),
            TokenKind::ExclamationMark => write!(f, "!"),

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
            TokenKind::LAngle => write!(f, "{{"),
            TokenKind::RAngle => write!(f, "}}"),
            TokenKind::LSquare => write!(f, "["),
            TokenKind::RSquare => write!(f, "]"),

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
            TokenKind::Scope => write!(f, "scope"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Yield => write!(f, "yield"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Function => write!(f, "fn"),
            TokenKind::In => write!(f, "in"),
            TokenKind::Of => write!(f, "of"),
            TokenKind::Delete => write!(f, "delete"),
            TokenKind::Throw => write!(f, "throw"),

            TokenKind::Import => write!(f, "import"),
            TokenKind::As => write!(f, "as"),

            TokenKind::Await => write!(f, "await"),
            TokenKind::Async => write!(f, "async"),

            TokenKind::Block => write!(f, "block"),

            TokenKind::Mut => write!(f, "assign"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenValue {
    None,
    Number(f64),
    Str(String),
    Identifier(String),
    Bool(bool),
}

// ------------- Math -------------

#[macro_use]
mod math {
    macro_rules! token_math {
        ($Op:path, $fn: ident) => {
            impl $Op for Token {
                type Output = Token;

                fn $fn(self, rhs: Self) -> Self::Output {
                    Token {
                        kind: self.kind,
                        start: self.start,
                        end: rhs.end,
                        value: self.value + rhs.value,
                    }
                }
            }
        };
    }
}

impl fmt::Display for TokenValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenValue::None => write!(f, ""),
            TokenValue::Number(s) => write!(f, "{}", s),
            TokenValue::Str(s) => write!(f, "\"{}\"", s),
            TokenValue::Identifier(s) => write!(f, "{}", s),
            TokenValue::Bool(b) => write!(f, "{}", b),
        }
    }
}

token_math!(ops::Add, add);
token_math!(ops::Sub, sub);
token_math!(ops::Mul, mul);
token_math!(ops::Div, div);

impl ops::Add for TokenValue {
    type Output = TokenValue;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TokenValue::Number(n1), TokenValue::Number(n2)) => TokenValue::Number(n1 + n2),

            (TokenValue::Number(n), TokenValue::Str(str)) => {
                TokenValue::Str(format!("{}{}", n, str))
            }

            (TokenValue::Str(str), TokenValue::Number(n)) => {
                TokenValue::Str(format!("{}{}", str, n))
            }

            (TokenValue::Str(str1), TokenValue::Str(str2)) => TokenValue::Str(str1 + &str2),

            (TokenValue::Bool(b1), TokenValue::Bool(b2)) => TokenValue::Bool(b1 || b2),

            (TokenValue::Bool(_), _) | (_, TokenValue::Bool(_)) => TokenValue::None,

            (TokenValue::None, _) | (_, TokenValue::None) => TokenValue::None,
            (TokenValue::Identifier(_), _) | (_, TokenValue::Identifier(_)) => {
                panic!("Identifiers can't be added")
            }
        }
    }
}

impl ops::Sub for TokenValue {
    type Output = TokenValue;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TokenValue::Number(n1), TokenValue::Number(n2)) => TokenValue::Number(n1 - n2),

            (TokenValue::Identifier(_), _) | (_, TokenValue::Identifier(_)) => {
                panic!("Identifiers can't be subtracted")
            }

            _ => TokenValue::None,
        }
    }
}

impl ops::Mul for TokenValue {
    type Output = TokenValue;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TokenValue::Number(n1), TokenValue::Number(n2)) => TokenValue::Number(n1 * n2),

            (TokenValue::Str(str), TokenValue::Number(n)) => {
                TokenValue::Str(str.repeat(n as usize))
            }

            (TokenValue::Number(n), TokenValue::Str(str)) => {
                TokenValue::Str(str.repeat(n as usize))
            }

            (TokenValue::Identifier(_), _) | (_, TokenValue::Identifier(_)) => {
                panic!("Identifiers can't be multiplied")
            }

            _ => TokenValue::None,
        }
    }
}

impl ops::Div for TokenValue {
    type Output = TokenValue;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (TokenValue::Number(n1), TokenValue::Number(n2)) => TokenValue::Number(n1 / n2),

            (TokenValue::Identifier(_), _) | (_, TokenValue::Identifier(_)) => {
                panic!("Identifiers can't be divided")
            }

            _ => TokenValue::None,
        }
    }
}
