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

#[macro_export]
macro_rules! mut_values_hm {
    ($amount:expr, $interpreter:ident, $call_expr:ident, $args:ident) => {{
        expect_args!($amount, $interpreter, $call_expr, $args);

        let scope_ref = $args[0].clone();

        let scope = match_scope(&scope_ref, $interpreter, $call_expr);

        &mut $interpreter.vault.get_mut(&scope).unwrap().values
    }};
}
