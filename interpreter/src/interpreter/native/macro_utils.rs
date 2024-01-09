#[macro_export]
macro_rules! expect_args {
    ($amount: expr, $interpreter:ident, $call_expr:ident, $args:ident ) => {
        if $args.len() != $amount {
            $interpreter.report(
                &format!("Wrong number of arguments (expected $amount)"),
                $call_expr.node.start,
                $call_expr.node.end,
            );
        }
    };
}
