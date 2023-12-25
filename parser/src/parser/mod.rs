use symboscript_lexer::Lexer;
use symboscript_types::{
    lexer::{Token, TokenKind, TokenValue},
    parser::*,
};
use symboscript_utils::report_error;

pub struct Parser<'a> {
    /// Path of the source file
    path: &'a str,

    /// Source Text
    source: &'a str,

    /// Lexer
    lexer: Lexer<'a>,

    cur_token: Token,
    prev_token_end: usize,
}

impl<'a> Parser<'a> {
    pub fn new(path: &'a str, source: &'a str, lexer: Lexer<'a>) -> Self {
        Self {
            path,
            source,
            lexer,

            prev_token_end: 0,
            cur_token: Token {
                kind: TokenKind::Start,
                start: 0,
                end: 0,
                value: TokenValue::None,
            },
        }
    }

    pub fn parse(&mut self) -> Ast {
        self.eat(TokenKind::Start);
        return Ast {
            program: Statement::Program(Program {
                node: Node {
                    start: 0,
                    end: self.source.len(),
                },
                body: vec![Statement::ExpressionStatement(self.expr())],
            }),
        };
    }

    fn expr(&mut self) -> Expression {
        let start = self.cur_token.start;
        let mut node = self.term();

        while [TokenKind::Plus, TokenKind::Minus].contains(&self.cur_token.kind) {
            let token = self.cur_token.clone();

            match token.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    self.eat(token.kind);
                }

                _ => {}
            };

            node = Expression::BinaryExpression(Box::new(BinaryExpression {
                node: Node::new(start, self.cur_token.end),
                left: node,
                operator: token.kind,
                right: self.term(),
            }));
        }

        // self.eat(TokenKind::Semicolon);

        node
    }

    /// term : (factor (Star | Slash | Modulo | Power | Range) factor)* | (factor factor)*
    fn term(&mut self) -> Expression {
        let start = self.cur_token.start;
        let mut expr = self.factor();

        while [TokenKind::Identifier, TokenKind::LParen].contains(&self.cur_token.kind) {
            expr = Expression::BinaryExpression(Box::new(BinaryExpression {
                node: Node::new(start, self.cur_token.end),
                left: expr,
                operator: TokenKind::Multiply,
                right: self.factor(),
            }));
        }

        while [
            TokenKind::Multiply,
            TokenKind::Divide,
            TokenKind::Modulo,
            TokenKind::Power,
            TokenKind::Range,
        ]
        .contains(&self.cur_token.kind)
        {
            let operator = self.cur_token.kind;
            self.eat(operator);

            expr = Expression::BinaryExpression(Box::new(BinaryExpression {
                node: Node::new(start, self.cur_token.end),
                left: expr,
                operator,
                right: self.factor(),
            }));
        }

        expr
    }

    /// factor : Number | LParen expr Rparen | Identifier | !factor | ++factor | --factor
    fn factor(&mut self) -> Expression {
        let token = self.cur_token.clone();

        match token.kind {
            TokenKind::Number => {
                self.eat(token.kind);
                return Expression::NumberLiteral(token);
            }
            TokenKind::True | TokenKind::False => {
                self.eat(token.kind);
                return Expression::BooleanLiteral(token);
            }
            TokenKind::Str => {
                self.eat(token.kind);
                return Expression::StrLiteral(token);
            }
            TokenKind::LParen => {
                self.eat(token.kind);
                let node = self.expr();
                self.eat(TokenKind::RParen);
                return node;
            }
            TokenKind::Identifier => {
                self.eat(token.kind);
                return Expression::Identifier(token);
            }
            TokenKind::Not | TokenKind::PlusPlus | TokenKind::MinusMinus => {
                self.eat(token.kind);
                return Expression::UnaryExpression(Box::new(UnaryExpression {
                    node: Node::new(token.start, self.cur_token.end),
                    operator: token.kind,
                    right: self.factor(),
                }));
            }
            _ => {}
        }

        self.report_expected(TokenKind::Number, token.kind);
        unreachable!("Report ends proccess")
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }

        self.report_expected(kind, self.cur_kind());
        unreachable!("Report ends proccess");
    }

    fn report_expected(&self, expected: TokenKind, got: TokenKind) {
        report_error(
            self.path,
            self.source,
            &format!("Expected {expected} but got {got}"),
            self.cur_token.start,
            self.cur_token.end,
        );
    }

    /// Move to the next token
    fn advance(&mut self) {
        let token = self.lexer.next_token();
        self.prev_token_end = self.cur_token.end;
        self.cur_token = token;
    }

    fn cur_kind(&self) -> TokenKind {
        self.cur_token.kind
    }

    /// Checks if the current index has token `TokenKind`
    fn at(&self, kind: TokenKind) -> bool {
        self.cur_kind() == kind
    }
}
