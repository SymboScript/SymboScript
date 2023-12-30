#[macro_export]
macro_rules! binary_left_associative {
    ($self:ident, $Kinds: expr, $SubOp: ident) => {{
        let start = $self.cur_token.start;
        let mut node = $self.$SubOp();

        while $Kinds.contains(&$self.cur_token.kind) {
            let current_token = $self.cur_token.clone();

            $self.eat(current_token.kind);

            let right = $self.$SubOp();
            node = $self.binary_expression(start, node, right, current_token.kind);
        }

        node
    }};

    ($self:ident, $SubOp: ident, [$($Kinds: expr),+], [$($EatOrNot: expr),+], [$($SubKind: expr),+]) => {{
        let start = $self.cur_token.start;
        let mut node = $self.$SubOp();

        $(
            while $Kinds.contains(&$self.cur_token.kind) {

                if $EatOrNot {
                    $self.eat($self.cur_token.kind);
                }

                let operator = if $SubKind != TokenKind::Unexpected {
                    $SubKind
                } else {
                    $self.cur_token.kind
                };

                let right = $self.$SubOp();
                node = $self.binary_expression(start, node, right, operator);
            }
        )+

        node
    }};
}

#[macro_export]
macro_rules! member_left_associative {
    ($self:ident, $Kinds: expr, $SubOp: ident) => {{
        let start = $self.cur_token.start;
        let (mut node, _) = $self.$SubOp();

        while $Kinds.contains(&$self.cur_token.kind) {
            let current_token = $self.cur_token.clone();

            $self.eat(current_token.kind);

            let (right, is_expr) = $self.$SubOp();
            node = $self.member_expression(start, node, right, is_expr);
        }

        node
    }};
}

#[macro_export]
macro_rules! word_right_associative {
    ($self:ident, $Kind: path, $SubOp: ident, $SelfOp: ident, $WordFn: ident) => {{
        let start = $self.cur_token.start;
        match $self.cur_kind() {
            $Kind => {
                $self.advance();

                let argument = $self.$SelfOp();
                return $self.$WordFn(start, argument);
            }

            _ => {
                return $self.$SubOp();
            }
        }
    }};
}

#[macro_export]
macro_rules! binary_right_associative {
    ($self:ident,  $SubOp: ident, $Kinds: expr) => {{
        let start = $self.cur_token.start;
        let mut node = $self.$SubOp();

        while $Kinds.contains(&$self.cur_token.kind) {
            let current_token = $self.cur_token.clone();

            $self.eat(current_token.kind);

            let right = $self.expr();
            node = $self.binary_expression(start, node, right, current_token.kind);
        }

        node
    }};
}

#[macro_export]
macro_rules! word_expr_build {
    ($self:ident, $operator: path, $start: ident, $argument: ident) => {{
        Expression::WordExpression(Box::new(WordExpression {
            node: Node::new($start, $self.cur_token.end),
            argument: $argument,
            operator: $operator,
        }))
    }};
}

#[macro_export]
macro_rules! uni_builder {
    ($self:ident, $Expr: ident, $start: ident,[$($properties: ident),+]) => {
        $Expr {
            node: Node::new($start, $self.cur_token.end),
            $(
                $properties,
            )+
        }
    };
}
