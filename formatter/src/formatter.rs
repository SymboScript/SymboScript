use symboscript_lexer::Lexer;
use symboscript_types::lexer::{Token, TokenKind};

pub struct Formatter<'a> {
    /// Source Text
    source: &'a str,

    /// Lexer
    lexer: Lexer<'a>,

    cur_token: Token,
    prev_token: Token,

    cur_indent: usize,
    string: String,
}

impl<'a> Formatter<'a> {
    pub fn new(source: &'a str, lexer: Lexer<'a>) -> Self {
        Self {
            source,
            lexer,
            cur_indent: 0,
            string: String::new(),
            cur_token: Token::default(),
            prev_token: Token::default(),
        }
    }

    pub fn format(&mut self) -> String {
        self.next_token();
        loop {
            match self.cur_kind() {
                TokenKind::Eof => break,
                TokenKind::LBrace | TokenKind::LBracket => {
                    self.indent();
                    self.write("\n");
                }
                TokenKind::RBrace | TokenKind::RBracket => {
                    self.dedent();
                    self.write("\n");
                }

                TokenKind::Semicolon => self.write("\n"),

                TokenKind::Comment | TokenKind::DocComment => self.write_cur_token(),
                TokenKind::Let => {
                    self.write_indent();
                    self.space();
                    self.write_to_semicolon();
                }

                _ => {
                    self.write_indent();
                    self.write_to_semicolon();
                }
            }
        }

        self.string.clone()
    }

    fn indent(&mut self) {
        self.cur_indent += 1;
    }

    fn dedent(&mut self) {
        if self.cur_indent != 0 {
            self.cur_indent -= 1;
        }
    }

    fn write_to(&mut self, end: TokenKind) {
        while self.cur_kind() != end {
            self.write_cur_token();
        }
    }

    fn write_to_semicolon(&mut self) {
        self.write_to(TokenKind::Semicolon);
        self.write("\n");
    }

    fn write(&mut self, end: &'a str) {
        self.write_cur_token();
        self.string.push_str(end);
    }

    fn write_cur_token(&mut self) {
        let token_start = self.cur_token.start;
        let token_end = self.cur_token.end;

        self.string.push_str(&self.source[token_start..token_end]);
        self.next_token();
    }

    fn write_indent(&mut self) {
        self.string.push_str(&" ".repeat(self.cur_indent * 4));
    }

    fn space(&mut self) {
        self.write_cur_token();
        self.string.push_str(" ");
    }

    fn next_token(&mut self) {
        self.prev_token = self.cur_token.clone();
        self.cur_token = self.lexer.next_token();
    }

    fn cur_kind(&self) -> TokenKind {
        self.cur_token.kind
    }
}
