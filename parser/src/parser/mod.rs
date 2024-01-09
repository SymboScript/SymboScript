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
    pub fn new(path: &'a str, source: &'a str) -> Self {
        Self {
            path,
            source,
            lexer: Lexer::new(path, source, false),
            cur_token: Token::default(),
            prev_token_end: 0,
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
                TokenKind::Eof | TokenKind::RAngle => break,
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
            TokenKind::Function | TokenKind::Async => self.fn_decl(),
            TokenKind::Scope => self.scope_decl(),
            TokenKind::Context => self.context_decl(),

            TokenKind::If => self.if_stmt(),

            TokenKind::For => self.for_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::Loop => self.loop_stmt(),

            TokenKind::Continue => self.continue_stmt(),
            TokenKind::Break => self.break_stmt(),

            TokenKind::Throw => self.throw_stmt(),

            TokenKind::Return => self.return_stmt(),
            TokenKind::Yield => self.yield_stmt(),

            TokenKind::Import => self.import_statement(),

            TokenKind::Block => self.block_decl(),
            TokenKind::LAngle => Statement::BlockStatement(self.block_stmt()),

            TokenKind::Mut => self.assign_statement(),

            _ => self.expr_stmt(),
        }
    }

    fn block_stmt(&mut self) -> Vec<Statement> {
        let mut body = vec![];

        if self.cur_kind() == TokenKind::LAngle {
            body = {
                let start = self.cur_token.start;
                self.eat(TokenKind::LAngle);
                let consequent = self.body();
                self.eat_with_start(TokenKind::RAngle, start);
                consequent
            }
        } else {
            body.push(self.statement());
        }

        return body;
    }

    // --------------- import statement ----------------

    fn import_statement(&mut self) -> Statement {
        let start = self.cur_token.start;
        self.advance();

        let source = Identifier {
            node: Node::new(self.cur_token.start, self.cur_token.end),
            name: match self.cur_token.value.clone() {
                TokenValue::Str(s) => s,
                TokenValue::Identifier(s) => s,
                got => {
                    self.report_expected(self.cur_token.start, "Identifier or String", got);
                    unreachable!("Report ends proccess");
                }
            },
        };

        self.advance();

        let as_name = match self.cur_kind() {
            TokenKind::As => {
                self.advance();

                let id = Identifier {
                    node: Node::new(self.cur_token.start, self.cur_token.end),
                    name: format!("{}", self.cur_token.clone().value),
                };

                self.eat(TokenKind::Identifier);

                id
            }

            _ => source.clone(),
        };

        self.eat(TokenKind::Semicolon);

        Statement::ImportStatement(uni_builder!(
            self,
            ImportStatement,
            start,
            [source, as_name]
        ))
    }

    // --------------- scope declaration ---------------

    fn scope_decl(&mut self) -> Statement {
        let start = self.cur_token.start;
        self.eat(TokenKind::Scope);

        let id = format!("{}", self.cur_token.clone().value);
        self.eat(TokenKind::Identifier);

        let body = self.block_stmt();

        Statement::ScopeDeclaration(uni_builder!(self, ScopeDeclarator, start, [id, body]))
    }

    fn context_decl(&mut self) -> Statement {
        let start = self.cur_token.start;
        self.eat(TokenKind::Context);

        let id = format!("{}", self.cur_token.clone().value);
        self.eat(TokenKind::Identifier);

        let body = self.block_stmt();

        Statement::ContextDeclaration(uni_builder!(self, ContextDeclarator, start, [id, body]))
    }

    // --------------- loop statement ------------------

    fn loop_stmt(&mut self) -> Statement {
        let start = self.cur_token.start;
        self.eat(TokenKind::Loop);
        let body = self.block_stmt();

        Statement::LoopStatement(uni_builder!(self, LoopStatement, start, [body]))
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

        Statement::WhileStatement(uni_builder!(self, WhileStatement, start, [test, body]))
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

        Statement::ForStatement(Box::new(uni_builder!(
            self,
            ForStatement,
            start,
            [init, test, update, body]
        )))
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

        Statement::IfStatement(uni_builder!(
            self,
            IfStatement,
            start,
            [test, consequent, alternate]
        ))
    }

    // -------------- word statements -----------------

    fn return_stmt(&mut self) -> Statement {
        word_stmt!(self, TokenKind::Return, ReturnStatement)
    }

    fn yield_stmt(&mut self) -> Statement {
        word_stmt!(self, TokenKind::Yield, YieldStatement)
    }

    fn throw_stmt(&mut self) -> Statement {
        word_stmt!(self, TokenKind::Throw, ThrowStatement)
    }

    fn block_decl(&mut self) -> Statement {
        self.advance();

        Statement::BlockStatement(self.block_stmt())
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

        let is_async = {
            if self.cur_kind() == TokenKind::Async {
                self.eat(TokenKind::Async);
                true
            } else {
                false
            }
        };

        self.advance();

        let id = format!("{}", self.cur_token.clone().value);
        self.eat(TokenKind::Identifier);

        let params = {
            let start = self.cur_token.start;
            self.eat(TokenKind::LSquare);

            let params = self
                .parse_params()
                .into_iter()
                .map(|p| format!("{}", p.value))
                .collect();

            self.eat_with_start(TokenKind::RSquare, start);
            params
        };

        let body = self.block_stmt();

        Statement::FunctionDeclaration(uni_builder!(
            self,
            FunctionDeclarator,
            start,
            [id, params, body, is_async]
        ))
    }

    fn parse_params(&mut self) -> Vec<Token> {
        let mut params = vec![];

        if self.cur_kind() == TokenKind::RSquare {
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

        let id = format!("{}", self.cur_token.clone().value);
        self.eat(TokenKind::Identifier);

        let mut is_formula = false;

        let init = {
            let start = self.cur_token.start;

            match self.cur_kind() {
                TokenKind::Assign => {
                    self.advance();
                    self.expr()
                }
                TokenKind::FormulaAssign => {
                    is_formula = true;
                    self.advance();
                    self.expr()
                }
                _ if !only_with_init => Expression::None(None {
                    node: Node::new(start, self.cur_token.end),
                }),
                _ => {
                    self.report_expected(start, "Assign or FormulaAssign", self.cur_kind());
                    unreachable!("Report ends proccess");
                }
            }
        };

        self.eat(TokenKind::Semicolon);

        Statement::VariableDeclaration(uni_builder!(
            self,
            VariableDeclarator,
            start,
            [id, init, is_formula]
        ))
    }

    // ---------------- assign statement -------------------

    ///ternary (Assign | PlusAssign | MinusAssign | MultiplyAssign | DivideAssign | PowerAssign | ModuloAssign) ternary
    fn assign_statement(&mut self) -> Statement {
        let start = self.cur_token.start;

        self.eat(TokenKind::Mut);

        let left = self.cur_token.clone();
        self.eat(TokenKind::Identifier);

        let left = Identifier {
            node: Node::new(start, self.cur_token.end),
            name: format!("{}", left.value),
        };

        if [
            TokenKind::Assign,
            TokenKind::PlusAssign,
            TokenKind::MinusAssign,
            TokenKind::MultiplyAssign,
            TokenKind::DivideAssign,
            TokenKind::PowerAssign,
            TokenKind::ModuloAssign,
        ]
        .contains(&self.cur_token.kind)
        {
            let current_token = self.cur_token.clone();

            self.advance();

            let right = self.expr();
            let operator = self.kind_to_assign_op(current_token.kind);

            self.eat(TokenKind::Semicolon);
            Statement::AssignStatement(uni_builder!(
                self,
                AssignStatement,
                start,
                [left, right, operator]
            ))
        } else {
            self.report_expected(start, "= | += | -= | *= | /= | ^= | %=", self.cur_kind());
            unreachable!("Report ends proccess");
        }
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

        nodes.push(self.ternary());
        while self.cur_kind() == TokenKind::Comma {
            self.advance();
            nodes.push(self.ternary());
        }

        if !only_sequence && nodes.len() == 1 {
            return nodes[0].clone();
        }

        self.sequence_expression(start, nodes)
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
        binary_left_associative!(self, [TokenKind::PipePipe], logical_and)
    }

    /// cmp && cmp
    fn logical_and(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::AmpersandAmpersand], cmp)
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
        binary_left_associative!(self, [TokenKind::Pipe], bit_xor)
    }

    /// bit_and bxor bit_and
    fn bit_xor(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::BitXor], bit_and)
    }

    /// shift & shift
    fn bit_and(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::Ampersand], shift)
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

        while [TokenKind::Identifier, TokenKind::LParen, TokenKind::Number]
            .contains(&self.cur_kind())
        {
            let right = self.power();
            expr = self.binary_expression(start, expr, right, TokenKind::Star);
        }

        while [TokenKind::Star, TokenKind::Slash, TokenKind::Modulo].contains(&self.cur_kind()) {
            let operator = self.cur_kind();
            self.advance();

            let right = self.power();

            expr = self.binary_expression(start, expr, right, operator);
        }

        expr
    }

    /// factor (Power) factor
    fn power(&mut self) -> Expression {
        binary_left_associative!(self, [TokenKind::Caret], factor)
    }

    /// Number | LParen expr Rparen | Identifier | (! | ++ | -- | ~)factor
    fn factor(&mut self) -> Expression {
        let token = self.cur_token.clone();

        match token.kind {
            TokenKind::Number | TokenKind::Str => {
                self.advance();
                return Expression::Literal(Literal {
                    node: Node::new(token.start, token.end),
                    value: token.value,
                });
            }

            TokenKind::True => {
                self.advance();
                return Expression::Literal(Literal {
                    node: Node::new(token.start, token.end),
                    value: TokenValue::Bool(true),
                });
            }

            TokenKind::False => {
                self.advance();
                return Expression::Literal(Literal {
                    node: Node::new(token.start, token.end),
                    value: TokenValue::Bool(false),
                });
            }

            TokenKind::LParen => {
                self.advance();
                let node = self.expr();
                self.eat_with_start(TokenKind::RParen, token.start);
                return node;
            }

            TokenKind::LSquare => self.read_seq_expr(token),

            TokenKind::ExclamationMark
            | TokenKind::PlusPlus
            | TokenKind::MinusMinus
            | TokenKind::Tilde
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
            TokenKind::RSquare => {
                self.advance();
                return self.sequence_expression(token.start, vec![]);
            }
            _ => {}
        }

        let mut node = self.comma(true);
        self.eat_with_start(TokenKind::RSquare, token.start);

        match node {
            Expression::SequenceExpression(seq_exp) => {
                node = self.sequence_expression(token.start, seq_exp.expressions);
            }
            _ => node = self.sequence_expression(token.start, vec![node]),
        }

        return node;
    }

    /// await delete_expr | delete_expr
    fn await_expr(&mut self) -> Expression {
        word_right_associative_expr!(self, TokenKind::Await, delete_expr, await_expr)
    }

    /// delete new_expr | new_expr
    fn delete_expr(&mut self) -> Expression {
        word_right_associative_expr!(self, TokenKind::Delete, dot, delete_expr)
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
                    TokenKind::LSquare => {
                        let sequence_start = self.cur_token.start;

                        self.advance();

                        match self.cur_kind() {
                            TokenKind::RSquare => {
                                self.advance();

                                let node = self.sequence_expression(sequence_start, vec![]);

                                return (
                                    self.call_expression(
                                        sequence_start,
                                        format!("{}", token.value),
                                        node,
                                    ),
                                    false,
                                );
                            }
                            _ => {}
                        }

                        let mut node = self.expr();
                        self.eat_with_start(TokenKind::RSquare, token.start);

                        match node {
                            Expression::SequenceExpression(seq_exp) => {
                                node =
                                    self.sequence_expression(sequence_start, seq_exp.expressions);
                            }
                            _ => node = self.sequence_expression(sequence_start, vec![node]),
                        }

                        return (
                            self.call_expression(token.start, format!("{}", token.value), node),
                            false,
                        );
                    }
                    _ => {
                        return (
                            Expression::Identifier(Identifier {
                                node: Node::new(token.start, token.end),
                                name: format!("{}", token.value),
                            }),
                            false,
                        );
                    }
                }
            }
            TokenKind::LSquare => {
                self.advance();

                let node = self.expr();
                self.eat_with_start(TokenKind::RSquare, token.start);

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
        callee: String,
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
            operator: self.kind_to_bin_op(operator),
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
            operator: self.kind_to_un_op(operator),
            right,
        }))
    }

    // ------------------------------- Utility functions -------------------------------

    fn kind_to_word_op(&mut self, kind: TokenKind) -> WordOperator {
        match kind {
            TokenKind::Await => WordOperator::Await,
            TokenKind::Delete => WordOperator::Delete,

            got => unreachable!("This function can't be called for other tokens: ({})", got),
        }
    }

    fn kind_to_un_op(&mut self, kind: TokenKind) -> UnaryOperator {
        match kind {
            TokenKind::MinusMinus => UnaryOperator::MinusMinus,
            TokenKind::Tilde => UnaryOperator::BitNot,
            TokenKind::ExclamationMark => UnaryOperator::Not,
            TokenKind::PlusPlus => UnaryOperator::PlusPlus,

            TokenKind::Plus => UnaryOperator::Plus,
            TokenKind::Minus => UnaryOperator::Minus,

            got => unreachable!("This function can't be called for other tokens: ({})", got),
        }
    }

    fn kind_to_assign_op(&mut self, kind: TokenKind) -> AssignOperator {
        match kind {
            TokenKind::Assign => AssignOperator::Assign,
            TokenKind::PlusAssign => AssignOperator::PlusAssign,
            TokenKind::MinusAssign => AssignOperator::MinusAssign,
            TokenKind::MultiplyAssign => AssignOperator::MultiplyAssign,
            TokenKind::DivideAssign => AssignOperator::DivideAssign,
            TokenKind::PowerAssign => AssignOperator::PowerAssign,
            TokenKind::ModuloAssign => AssignOperator::ModuloAssign,

            got => unreachable!("This function can't be called for other tokens: ({})", got),
        }
    }

    fn kind_to_bin_op(&mut self, kind: TokenKind) -> BinaryOperator {
        match kind {
            TokenKind::Plus => BinaryOperator::Add,
            TokenKind::Minus => BinaryOperator::Substract,
            TokenKind::Star => BinaryOperator::Multiply,
            TokenKind::Slash => BinaryOperator::Divide,
            TokenKind::Caret => BinaryOperator::Power,
            TokenKind::Range => BinaryOperator::Range,
            TokenKind::Modulo => BinaryOperator::Modulo,

            TokenKind::AmpersandAmpersand => BinaryOperator::And,
            TokenKind::PipePipe => BinaryOperator::Or,
            TokenKind::Xor => BinaryOperator::Xor,

            TokenKind::Ampersand => BinaryOperator::BitAnd,
            TokenKind::Pipe => BinaryOperator::BitOr,
            TokenKind::BitXor => BinaryOperator::BitXor,

            TokenKind::BitLeftShift => BinaryOperator::BitLeftShift,
            TokenKind::BitRightShift => BinaryOperator::BitRightShift,

            TokenKind::Equal => BinaryOperator::Equal,
            TokenKind::NotEqual => BinaryOperator::NotEqual,
            TokenKind::Less => BinaryOperator::Less,
            TokenKind::LessEqual => BinaryOperator::LessEqual,
            TokenKind::Greater => BinaryOperator::Greater,
            TokenKind::GreaterEqual => BinaryOperator::GreaterEqual,
            _ => unreachable!("This function can't be called for other tokens: ({kind})"),
        }
    }

    fn eat(&mut self, kind: TokenKind) {
        self.eat_with_start(kind, self.cur_token.start);
    }

    fn eat_with_start(&mut self, kind: TokenKind, start: usize) -> bool {
        if self.at(kind) {
            self.advance();
            return true;
        }

        let val = self.cur_token.value.to_string();

        self.report_expected(
            start,
            kind,
            format!(
                "{} {}",
                self.cur_kind(),
                if self.cur_token.value == TokenValue::None {
                    ""
                } else {
                    &val
                }
            ),
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
        self.prev_token_end = self.cur_token.end;
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
