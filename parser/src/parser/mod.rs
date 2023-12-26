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

    /// add_sub
    fn expr(&mut self) -> Expression {
        self.add_sub()
    }

    /// term (Plus | Minus) term
    fn add_sub(&mut self) -> Expression {
        let start = self.cur_token.start;
        let mut node = self.term();

        while [TokenKind::Plus, TokenKind::Minus].contains(&self.cur_token.kind) {
            let current_token = self.cur_token.clone();

            self.eat(current_token.kind);

            let right = self.term();
            node = self.binary_expression(start, node, right, current_token.kind);
        }

        node
    }

    /// (factor (Star | Slash | Modulo) factor)* | (factor factor)*
    fn term(&mut self) -> Expression {
        let start = self.cur_token.start;
        let mut expr = self.power();

        while [TokenKind::Identifier, TokenKind::LParen].contains(&self.cur_token.kind) {
            let right = self.power();
            expr = self.binary_expression(start, expr, right, TokenKind::Multiply);
        }

        while [TokenKind::Multiply, TokenKind::Divide, TokenKind::Modulo]
            .contains(&self.cur_token.kind)
        {
            let operator = self.cur_token.kind;
            self.eat(operator);

            let right = self.power();
            expr = self.binary_expression(start, expr, right, operator);
        }

        expr
    }

    /// power: factor (Power) factor
    fn power(&mut self) -> Expression {
        let start = self.cur_token.start;
        let mut expr = self.factor();

        while self.cur_token.kind == TokenKind::Power {
            self.eat(TokenKind::Power);

            let right_expr = self.factor();
            expr = self.binary_expression(start, expr, right_expr, TokenKind::Power)
        }

        expr
    }

    /// factor : Number | LParen expr Rparen | Identifier | (! | ++ | -- | ~)factor
    fn factor(&mut self) -> Expression {
        let token = self.cur_token.clone();

        match token.kind {
            TokenKind::Number | TokenKind::Str | TokenKind::True | TokenKind::False => {
                self.eat(token.kind);
                return Expression::Literal(token);
            }
            TokenKind::LParen => {
                self.eat(token.kind);
                let node = self.expr();
                self.eat_with_start(TokenKind::RParen, token.start);
                return node;
            }
            TokenKind::Identifier => {
                self.eat(token.kind);
                return Expression::Identifier(token);
            }
            TokenKind::Not | TokenKind::PlusPlus | TokenKind::MinusMinus | TokenKind::BitNot => {
                self.eat(token.kind);
                return Expression::UnaryExpression(Box::new(UnaryExpression {
                    node: Node::new(token.start, self.cur_token.end),
                    operator: token.kind,
                    right: self.factor(),
                }));
            }
            _ => {}
        }

        self.report_expected(token.start, TokenKind::Number, token.kind);
        unreachable!("Report ends proccess")
    }

    fn binary_expression(
        &mut self,
        start: usize,
        left: Expression,
        right: Expression,
        operator: TokenKind,
    ) -> Expression {
        Expression::BinaryExpression(Box::new(BinaryExpression {
            node: Node::new(start, self.cur_token.end),
            left,
            operator,
            right,
        }))
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        self.eat_with_start(kind, self.cur_token.start)
    }

    fn eat_with_start(&mut self, kind: TokenKind, start: usize) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }

        self.report_expected(start, kind, self.cur_kind());
        unreachable!("Report ends proccess");
    }

    fn report_expected(&self, start: usize, expected: TokenKind, got: TokenKind) {
        report_error(
            self.path,
            self.source,
            &format!("Expected {expected} but got {got}"),
            start,
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
