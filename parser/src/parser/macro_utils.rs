#[macro_export]
macro_rules! parser {
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
}
