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
}

impl<'a> Parser<'a> {
    pub fn new(path: &'a str, source: &'a str, lexer: Lexer<'a>) -> Self {
        Self {
            path,
            source,
            lexer,

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

        loop {
            match self.cur_kind() {
                TokenKind::Eof | TokenKind::RBrace => break,
                _ => {
                    body.push(self.statement());
                }
            }
        }

        body
    }

    // -------------------- statements ---------------------

    fn statement(&mut self) -> Statement {
        match self.cur_kind() {
            TokenKind::Let => self.var_decl(false),
            TokenKind::Function => self.fn_decl(),

            TokenKind::If => self.if_stmt(),

            TokenKind::For => self.for_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::Loop => self.loop_stmt(),

            TokenKind::Continue => self.continue_stmt(),
            TokenKind::Break => self.break_stmt(),

            TokenKind::Return => self.return_stmt(),
            TokenKind::Yield => self.yield_stmt(),

            _ => self.expr_stmt(),
        }
    }

    fn block_stmt(&mut self) -> Vec<Statement> {
        let mut body = vec![];

        if self.cur_kind() == TokenKind::LBrace {
            body = {
                let start = self.cur_token.start;
                self.eat(TokenKind::LBrace);
                let consequent = self.body();
                self.eat_with_start(TokenKind::RBrace, start);
                consequent
            }
        } else {
            body.push(self.statement());
        }

        return body;
    }

    // --------------- loop statement ------------------

    fn loop_stmt(&mut self) -> Statement {
        let start = self.cur_token.start;
        self.eat(TokenKind::Loop);
        let body = self.block_stmt();

        Statement::LoopStatement(LoopStatement {
            node: Node::new(start, self.cur_token.end),
            body,
        })
    }

    // --------------- while statement ------------------

    fn while_stmt(&mut self) -> Statement {
        let start = self.cur_token.start;
        self.eat(TokenKind::While);

        let test = {
            let start = self.cur_token.start;
            self.eat(TokenKind::LParen);
            let test = self.expr();
            self.eat_with_start(TokenKind::RParen, start);
            test
        };

        let body = self.block_stmt();

        Statement::WhileStatement(WhileStatement {
            node: Node::new(start, self.cur_token.end),
            test,
            body,
        })
    }

    // --------------- for statement ------------------

    fn for_stmt(&mut self) -> Statement {
        let start = self.cur_token.start;

        self.eat(TokenKind::For);
        self.eat(TokenKind::LParen);

        let init = self.var_decl(true);

        let test = {
            let start = self.cur_token.start;
            let test = self.expr();
            self.eat_with_start(TokenKind::Semicolon, start);
            test
        };

        let update = {
            let start = self.cur_token.start;
            let update = self.expr();
            self.eat_with_start(TokenKind::RParen, start);
            update
        };

        let body = self.block_stmt();

        Statement::ForStatement(Box::new(ForStatement {
            node: Node::new(start, self.cur_token.end),
            init,
            test,
            update,
            body,
        }))
    }

    // --------------- if statement -------------------

    fn if_stmt(&mut self) -> Statement {
        let start = self.cur_token.start;

        self.eat(TokenKind::If);

        let test = {
            let start = self.cur_token.start;
            self.eat(TokenKind::LParen);
            let test = self.expr();
            self.eat_with_start(TokenKind::RParen, start);
            test
        };

        let consequent = self.block_stmt();

        let mut alternate = vec![];

        if self.cur_kind() == TokenKind::Else {
            self.advance();
            alternate = self.block_stmt();
        }

        Statement::IfStatement(IfStatement {
            node: Node::new(start, self.cur_token.end),
            test,
            consequent,
            alternate,
        })
    }

    // -------------- word statements -----------------

    fn return_stmt(&mut self) -> Statement {
        word_stmt!(self, TokenKind::Return, ReturnStatement)
    }

    fn yield_stmt(&mut self) -> Statement {
        word_stmt!(self, TokenKind::Yield, YieldStatement)
    }

    fn continue_stmt(&mut self) -> Statement {
        let start = self.cur_token.start;
        self.eat(TokenKind::Continue);

        Statement::ContinueStatement(Node::new(start, self.cur_token.end))
    }

    fn break_stmt(&mut self) -> Statement {
        let start = self.cur_token.start;
        self.eat(TokenKind::Break);

        Statement::BreakStatement(Node::new(start, self.cur_token.end))
    }

    // --------------- function declaration -----------------

    fn fn_decl(&mut self) -> Statement {
        let start = self.cur_token.start;
        self.advance();

        let id = self.cur_token.clone();
        self.eat(TokenKind::Identifier);

        let params = {
            let start = self.cur_token.start;
            self.eat(TokenKind::LBracket);
            let params = self.parse_params();
            self.eat_with_start(TokenKind::RBracket, start);
            params
        };

        let body = {
            let start = self.cur_token.start;
            self.eat(TokenKind::LBrace);
            let body = self.body();
            self.eat_with_start(TokenKind::RBrace, start);
            body
        };

        Statement::FunctionDeclaration(uni_builder!(
            self,
            FunctionDeclarator,
            start,
            [id, params, body]
        ))
    }

    fn parse_params(&mut self) -> Vec<Token> {
        let mut params = vec![];

        if self.cur_kind() == TokenKind::RBracket {
            self.advance();
            return params;
        }

        params.push(self.cur_token.clone());
        self.eat(TokenKind::Identifier);

        while self.cur_kind() == TokenKind::Comma {
            self.advance();
            params.push(self.cur_token.clone());
            self.eat(TokenKind::Identifier);
        }

        params
    }

    // -------------- variable declaration -----------------

    fn var_decl(&mut self, only_with_init: bool) -> Statement {
        let start = self.cur_token.start;
        self.advance();

        let id = self.cur_token.clone();
        self.eat(TokenKind::Identifier);

        let init = {
            let start = self.cur_token.start;
            if self.cur_kind() == TokenKind::Assign {
                self.advance();
                self.expr()
            } else if !only_with_init {
                Expression::None
            } else {
                self.report_expected(start, TokenKind::Assign, self.cur_kind());
                unreachable!("Report ends proccess");
            }
        };

        self.eat(TokenKind::Semicolon);

        Statement::VariableDeclaration(uni_builder!(self, VariableDeclarator, start, [id, init]))
    }

    // -------------------- expressions --------------------

    fn expr_stmt(&mut self) -> Statement {
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

        nodes.push(self.assign());
        while self.cur_kind() == TokenKind::Comma {
            self.advance();
            nodes.push(self.assign());
        }

        if !only_sequence && nodes.len() == 1 {
            return nodes[0].clone();
        }

        self.sequence_expression(start, nodes)
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
        word_right_associative_expr!(self, TokenKind::Await, delete_expr, await_expr)
    }

    /// delete new_expr | new_expr
    fn delete_expr(&mut self) -> Expression {
        word_right_associative_expr!(self, TokenKind::Delete, new_expr, delete_expr)
    }

    /// new dot | dot
    fn new_expr(&mut self) -> Expression {
        word_right_associative_expr!(self, TokenKind::New, dot, new_expr)
    }

    /// call.call | call
    fn dot(&mut self) -> Expression {
        member_left_associative!(self, [TokenKind::Dot], call)
    }

    /// identifier[expr] | identifier
    fn call(&mut self) -> (Expression, bool) {
        let token = self.cur_token.clone();

        match self.cur_kind() {
            TokenKind::Identifier => {
                self.advance();

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
                                node =
                                    self.sequence_expression(sequence_start, seq_exp.expressions);
                            }
                            _ => node = self.sequence_expression(sequence_start, vec![node]),
                        }

                        return (
                            self.call_expression(token.start, Expression::Identifier(token), node),
                            true,
                        );
                    }
                    _ => {
                        return (Expression::Identifier(token), false);
                    }
                }
            }
            TokenKind::LBracket => {
                self.advance();

                let node = self.expr();
                self.eat_with_start(TokenKind::RBracket, token.start);

                return (node, true);
            }
            got => {
                self.report_expected(token.start, "Identifier or [", got);
                unreachable!("Report ends proccess");
            }
        }
    }

    // ------------------------------ Expression builders ------------------------------

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

        self.report_expected(
            start,
            kind,
            format!("{}({})", self.cur_kind(), self.cur_token),
        );
        unreachable!("Report ends proccess");
    }

    fn report_expected<T: std::fmt::Display, U: std::fmt::Display>(
        &self,
        start: usize,
        expected: T,
        got: U,
    ) {
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
