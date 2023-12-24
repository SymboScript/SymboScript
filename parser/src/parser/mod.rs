use symboscript_lexer::Lexer;
use symboscript_types::{
    lexer::{self, Token, TokenKind, TokenValue},
    parser::{Ast, BinaryExpression, Expression, Node, Program, Statement},
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
            program: Program {
                node: Node {
                    start: 0,
                    end: self.source.len(),
                },
                body: vec![Statement::ExpressionStatement(self.expr())],
            },
        };
    }

    pub fn expr(&mut self) -> Expression {
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

        node
    }

    pub fn term(&mut self) -> Expression {
        let start = self.cur_token.start;
        let mut expr = self.factor();

        while [TokenKind::Star, TokenKind::Slash].contains(&self.cur_token.kind) {
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

    pub fn factor(&mut self) -> Expression {
        let token = self.cur_token.clone();
        if token.kind == TokenKind::Number {
            self.eat(TokenKind::Number);
            return Expression::NumberLiteral(token);
        } else if token.kind == TokenKind::LParen {
            self.eat(TokenKind::LParen);
            let node = self.expr();
            self.eat(TokenKind::RParen);
            return node;
        }

        report_error(
            self.path,
            self.source,
            "Expected number literal",
            token.start,
            token.end,
        );

        std::process::exit(1);
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }

        report_error(
            self.path,
            self.source,
            &format!(
                "Expected token {:?} but got {:?}",
                kind, self.cur_token.kind
            ),
            self.cur_token.start + 1,
            self.cur_token.end + 1,
        );

        false
    }

    /// Move to the next token
    fn advance(&mut self) {
        let token = self.lexer.next_token();
        self.prev_token_end = self.cur_token.end;
        self.cur_token = token;
    }

    fn start_node(&self) -> Node {
        let token = self.cur_token();
        Node::new(token.start, 0)
    }

    fn finish_node(&self, node: Node) -> Node {
        Node::new(node.start, self.prev_token_end)
    }

    fn cur_token(&self) -> &Token {
        &self.cur_token
    }

    fn cur_kind(&self) -> TokenKind {
        self.cur_token.kind
    }

    /// Checks if the current index has token `TokenKind`
    fn at(&self, kind: TokenKind) -> bool {
        self.cur_kind() == kind
    }

    /// Advance if we are at `TokenKind`
    fn bump(&mut self, kind: TokenKind) {
        if self.at(kind) {
            self.advance();
        }
    }

    /// Advance any token
    fn bump_any(&mut self) {
        self.advance();
    }
}
