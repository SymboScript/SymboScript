use crate::types::{Kind, Token, TokenValue};
use std::str::Chars;
pub struct Lexer<'a> {
    /// Source Text
    source: &'a str,

    /// The remaining characters
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
        }
    }

    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            if token.kind == Kind::Eof {
                break;
            }
            tokens.push(token);
        }

        tokens
    }

    pub fn next_kind(&mut self) -> Kind {
        while let Some(c) = self.chars.next() {
            match c {
                '+' => return Kind::Plus,
                '-' => return Kind::Minus,
                '*' => return Kind::Star,
                '/' => return Kind::Slash,
                '^' => return Kind::Power,
                '(' => return Kind::LParen,
                ')' => return Kind::RParen,
                '{' => return Kind::LBrace,
                '}' => return Kind::RBrace,
                '=' => match self.peek() {
                    Some('=') => {
                        self.next();
                        return Kind::Equal;
                    }
                    _ => return Kind::Equation,
                },
                '0'..='9' => return self.read_number(),
                'a'..='z' | 'A'..='Z' | '_' => return self.read_identifier(),
                ' ' | '\t' | '\n' | '\r' => {}
                _ => return Kind::Unexpected,
            };
        }
        Kind::Eof
    }

    fn read_number(&mut self) -> Kind {
        while let Some(c) = self.peek() {
            match c {
                '0'..='9' => {
                    self.next();
                }
                _ => break,
            };
        }

        Kind::Number
    }

    fn read_identifier(&mut self) -> Kind {
        while let Some(c) = self.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    self.next();
                }
                _ => break,
            };
        }

        Kind::Identifier
    }

    fn next_token(&mut self) -> Token {
        let start = self.offset();
        let mut kind = self.next_kind();
        let end = self.offset();

        let s = self.source[start..end].trim();

        let mut value = TokenValue::String(s.to_string());

        match kind {
            Kind::Number => {
                value = TokenValue::Number(s.trim().parse::<f64>().unwrap_or_default());
            }
            Kind::Identifier => {
                kind = self.match_keyword(&s);

                match kind {
                    Kind::If | Kind::While | Kind::For => {}
                    _ => {
                        value = TokenValue::Identifier(s.to_string());
                    }
                }
            }
            Kind::Unexpected => {
                self.report_error(format!("Unexpected character: `{}`", s), start, end)
            }
            _ => {}
        }

        Token {
            kind,
            start,
            end,
            value,
        }
    }

    fn match_keyword(&self, ident: &str) -> Kind {
        // all keywords are 1 <= length <= 10
        if ident.len() == 1 || ident.len() > 10 {
            return Kind::Identifier;
        }

        match ident {
            "if" => Kind::If,
            "else" => Kind::Else,
            "while" => Kind::While,
            "loop" => Kind::Loop,
            "for" => Kind::For,
            "let" => Kind::Let,
            "fn" => Kind::Function,
            "return" => Kind::Return,
            "break" => Kind::Break,
            "continue" => Kind::Continue,

            "true" => Kind::True,
            "false" => Kind::False,

            _ => Kind::Identifier,
        }
    }

    /// Get the length offset from the source text, in UTF-8 bytes
    fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.as_str().chars().next()
    }

    fn report_error(&self, error: String, start: usize, end: usize) {
        let line = self.source[..start].lines().count();
        let line_end = self.source[start..end]
            .rfind('\n')
            .map_or(end, |i| i + start);

        let column = start - self.source[..start].rfind('\n').unwrap_or(0);
        let column_end = end - self.source[start..end].rfind('\n').unwrap_or(0);

        println!(
            "{} from {}:{} to {}:{}",
            error, line, column, line_end, column_end
        );

        std::process::exit(1);
    }
}
