use symboscript_lexer::Lexer;
use symboscript_types::{
    lexer::{Token, TokenKind, TokenValue},
    parser::*,
};
use symboscript_utils::report_error;

#[macro_use]
mod macro_utils;

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
            program: self.program(),
        };
    }

    // -------------------- program ------------------------

    fn program(&mut self) -> Program {
        Program {
            node: Node {
                start: 0,
                end: self.source.len(),
            },
            body: self.body(),
        }
    }

    fn body(&mut self) -> Vec<Statement> {
        let mut body = vec![];
        while self.cur_kind() != TokenKind::Eof {
            body.push(self.statement());
        }

        body
    }

    // -------------------- statements ---------------------

    fn statement(&mut self) -> Statement {
        match self.cur_kind() {
            _ => self.expression_statement(),
        }
    }

    // -------------------- expressions --------------------

    fn expression_statement(&mut self) -> Statement {
        let expression = self.expr();
        self.eat(TokenKind::Semicolon);

        Statement::ExpressionStatement(expression)
    }

    /// full expression
    fn expr(&mut self) -> Expression {
        self.comma(false)
    }

    /// word_expression , word_expression | word_expression
    fn comma(&mut self, only_sequence: bool) -> Expression {
        let start = self.cur_token.start;
        let mut nodes = vec![];

        nodes.push(self.yield_expr());
        while self.cur_kind() == TokenKind::Comma {
            self.advance();
            nodes.push(self.yield_expr());
        }

        if !only_sequence && nodes.len() == 1 {
            return nodes[0].clone();
        }

        self.sequence_expression(start, nodes)
    }

    /// yield assign | assign
    fn yield_expr(&mut self) -> Expression {
        word_right_associative!(self, TokenKind::Yield, assign, expr, yield_expression)
    }

    ///ternary (Assign | FormulaAssign | PlusAssign | MinusAssign | MultiplyAssign | DivideAssign | PowerAssign | ModuloAssign) ternary
    fn assign(&mut self) -> Expression {
        binary_right_associative!(
            self,
            ternary,
            [
                TokenKind::Assign,
                TokenKind::FormulaAssign,
                TokenKind::PlusAssign,
                TokenKind::MinusAssign,
                TokenKind::MultiplyAssign,
                TokenKind::DivideAssign,
                TokenKind::PowerAssign,
                TokenKind::ModuloAssign
            ]
        )
    }

    /// range ? range : range | range
    fn ternary(&mut self) -> Expression {
        let start = self.cur_token.start;
        let mut node = self.range();

        while self.cur_kind() == TokenKind::Question {
            self.advance();
            let consequent = self.range();
            self.eat(TokenKind::Colon);

            let alternate = self.expr();

            node = self.conditional_expression(start, node, consequent, alternate);
        }

        node
    }

    /// logical_or .. logical_or | logical_or
    fn range(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::Range], logical_or)
    }

    /// logical_and || logical_and
    fn logical_or(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::Or], logical_and)
    }

    /// cmp && cmp
    fn logical_and(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::And], cmp)
    }

    /// bit_or (< | <= | > | >= | == | !=) bit_or
    fn cmp(&mut self) -> Expression {
        binary_left_associative!(
            self,
            [
                TokenKind::Less,
                TokenKind::LessEqual,
                TokenKind::Greater,
                TokenKind::GreaterEqual,
                TokenKind::Equal,
                TokenKind::NotEqual,
            ],
            bit_or
        )
    }

    ///bit_xor | bit_xor
    fn bit_or(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::BitOr], bit_xor)
    }

    /// bit_and bxor bit_and
    fn bit_xor(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::BitXor], bit_and)
    }

    /// shift & shift
    fn bit_and(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::BitAnd], shift)
    }

    /// add_sub (>> | <<) add_sub
    fn shift(&mut self) -> Expression {
        binary_left_associative!(
            self,
            [TokenKind::BitRightShift, TokenKind::BitLeftShift],
            add_sub
        )
    }

    /// term (Plus | Minus) term
    fn add_sub(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::Plus, TokenKind::Minus], term)
    }

    /// (power (Star | Slash | Modulo) power)* | (power power)*
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
            self.advance();

            let right = self.power();
            expr = self.binary_expression(start, expr, right, operator);
        }

        expr
    }

    /// factor (Power) factor
    fn power(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::Power], factor)
    }

    /// Number | LParen expr Rparen | Identifier | (! | ++ | -- | ~)factor
    fn factor(&mut self) -> Expression {
        let token = self.cur_token.clone();

        match token.kind {
            TokenKind::Number | TokenKind::Str | TokenKind::True | TokenKind::False => {
                self.advance();
                return Expression::Literal(token);
            }
            TokenKind::LParen => {
                self.advance();
                let node = self.expr();
                self.eat_with_start(TokenKind::RParen, token.start);
                return node;
            }

            TokenKind::LBracket => self.read_seq_expr(token),
            TokenKind::LBrace => self.read_map_expr(),

            TokenKind::Not
            | TokenKind::PlusPlus
            | TokenKind::MinusMinus
            | TokenKind::BitNot
            | TokenKind::Minus
            | TokenKind::Plus => {
                self.advance();

                let right = self.factor();
                return self.unary_expression(token.start, token.kind, right);
            }
            _ => return self.await_expr(),
        }
    }

    fn read_seq_expr(&mut self, token: Token) -> Expression {
        self.advance();

        match self.cur_kind() {
            TokenKind::RBracket => {
                self.advance();
                return self.sequence_expression(token.start, vec![]);
            }
            _ => {}
        }

        let mut node = self.comma(true);
        self.eat_with_start(TokenKind::RBracket, token.start);

        match node {
            Expression::SequenceExpression(seq_exp) => {
                node = self.sequence_expression(token.start, seq_exp.expressions);
            }
            _ => node = self.sequence_expression(token.start, vec![node]),
        }

        return node;
    }

    fn read_map_expr(&mut self) -> Expression {
        let start = self.cur_token.start;
        self.advance();
        let mut properties = vec![self.read_map_key_value()];

        while self.cur_kind() == TokenKind::Semicolon {
            self.advance();

            match self.cur_kind() {
                TokenKind::RBrace => {
                    self.advance();
                    break;
                }
                _ => {
                    properties.push(self.read_map_key_value());
                }
            }
        }

        return Expression::MapExpression(Box::new(MapExpression {
            node: Node::new(start, self.cur_token.end),
            properties,
        }));
    }

    fn read_map_key_value(&mut self) -> Property {
        let ident_name = self.cur_token.clone();

        self.eat(TokenKind::Identifier);
        self.eat(TokenKind::Colon);
        let value = self.expr();

        // return (Expression::Identifier(ident_name), value);

        return Property {
            node: Node::new(ident_name.start, self.cur_token.end),
            key: Expression::Identifier(ident_name),
            value: value,
        };
    }

    /// await delete_expr | delete_expr
    fn await_expr(&mut self) -> Expression {
        word_right_associative!(
            self,
            TokenKind::Await,
            delete_expr,
            await_expr,
            await_expression
        )
    }

    /// delete new_expr | new_expr
    fn delete_expr(&mut self) -> Expression {
        word_right_associative!(
            self,
            TokenKind::Delete,
            new_expr,
            delete_expr,
            delete_expression
        )
    }

    fn new_expr(&mut self) -> Expression {
        word_right_associative!(self, TokenKind::New, dot, new_expr, new_expression)
    }

    fn dot(&mut self) -> Expression {
        member_left_associative!(self, [TokenKind::Dot], call)
    }

    fn call(&mut self) -> (Expression, bool) {
        let token = self.cur_token.clone();

        self.eat_with_start(TokenKind::Identifier, token.start);

        match self.cur_kind() {
            TokenKind::LBracket => {
                let sequence_start = self.cur_token.start;

                self.advance();

                match self.cur_kind() {
                    TokenKind::RBracket => {
                        self.advance();

                        let node = self.sequence_expression(sequence_start, vec![]);

                        return (
                            self.call_expression(
                                sequence_start,
                                Expression::Identifier(token),
                                node,
                            ),
                            true,
                        );
                    }
                    _ => {}
                }

                let mut node = self.expr();
                self.eat_with_start(TokenKind::RBracket, token.start);

                match node {
                    Expression::SequenceExpression(seq_exp) => {
                        node = self.sequence_expression(sequence_start, seq_exp.expressions);
                    }
                    _ => node = self.sequence_expression(sequence_start, vec![node]),
                }

                return (
                    self.call_expression(token.start, Expression::Identifier(token), node),
                    true,
                );
            }
            _ => {
                return (Expression::Identifier(token), true);
            }
        }
    }

    // ------------------------------ Expression builders ------------------------------

    // -------------------------- Word expression builders --------------------------
    fn yield_expression(&mut self, start: usize, argument: Expression) -> Expression {
        word_expr_build!(self, TokenKind::Yield, start, argument)
    }

    fn await_expression(&mut self, start: usize, argument: Expression) -> Expression {
        word_expr_build!(self, TokenKind::Await, start, argument)
    }

    fn delete_expression(&mut self, start: usize, argument: Expression) -> Expression {
        word_expr_build!(self, TokenKind::Delete, start, argument)
    }

    fn new_expression(&mut self, start: usize, argument: Expression) -> Expression {
        word_expr_build!(self, TokenKind::New, start, argument)
    }

    // -------------------------- Other expression builders --------------------------
    fn call_expression(
        &mut self,
        start: usize,
        callee: Expression,
        arguments: Expression,
    ) -> Expression {
        Expression::CallExpression(Box::new(CallExpression {
            node: Node::new(start, self.cur_token.end),
            callee,
            arguments,
        }))
    }

    fn member_expression(
        &mut self,
        start: usize,
        object: Expression,
        property: Expression,
        is_expr: bool,
    ) -> Expression {
        Expression::MemberExpression(Box::new(MemberExpression {
            node: Node::new(start, self.cur_token.end),
            object,
            property,
            is_expr,
        }))
    }

    fn sequence_expression(&mut self, start: usize, expressions: Vec<Expression>) -> Expression {
        Expression::SequenceExpression(Box::new(SequenceExpression {
            node: Node::new(start, self.cur_token.end),
            expressions,
        }))
    }

    fn conditional_expression(
        &mut self,
        start: usize,
        test: Expression,
        consequent: Expression,
        alternate: Expression,
    ) -> Expression {
        Expression::ConditionalExpression(Box::new(ConditionalExpression {
            node: Node::new(start, self.cur_token.end),
            test,
            consequent,
            alternate,
        }))
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

    fn unary_expression(
        &mut self,
        start: usize,
        operator: TokenKind,
        right: Expression,
    ) -> Expression {
        Expression::UnaryExpression(Box::new(UnaryExpression {
            node: Node::new(start, self.cur_token.end),
            operator,
            right,
        }))
    }

    // ------------------------------- Utility functions -------------------------------

    fn eat(&mut self, kind: TokenKind) {
        self.eat_with_start(kind, self.cur_token.start);
    }

    fn eat_with_start(&mut self, kind: TokenKind, start: usize) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }

        self.report_expected(start, kind, self.cur_kind());
        unreachable!("Report ends proccess");
    }

    fn report_expected<T: std::fmt::Display>(&self, start: usize, expected: T, got: TokenKind) {
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
