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

            let (right, computed) = $self.$SubOp();
            node = $self.member_expression(start, node, right, computed);
        }

        node
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
