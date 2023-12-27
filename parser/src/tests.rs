pub mod expr_tests {

    use crate::parser::Parser;
    use symboscript_lexer::Lexer;

    #[macro_use]
    mod utils {
        macro_rules! assert_parser {
            ($str: expr, $ast_str: expr) => {
                let test_str = $str;
                let mut parser = Parser::new("test", test_str, Lexer::new("test", test_str));

                let ast = format!("{}", parser.parse());
                assert_eq!(ast, $ast_str);
            };
        }
    }

    #[test]
    fn binary_ops() {
        assert_parser!("1+2;", "(1+2)");
        assert_parser!("1-2;", "(1-2)");
        assert_parser!("1*2;", "(1*2)");
        assert_parser!("1/2;", "(1/2)");
        assert_parser!("1%2;", "(1%2)");
        assert_parser!("1^2;", "(1^2)");
        assert_parser!("1&2;", "(1&2)");
        assert_parser!("1|2;", "(1|2)");
        assert_parser!("1<<2;", "(1<<2)");
        assert_parser!("1>>2;", "(1>>2)");
        assert_parser!("1==2;", "(1==2)");
        assert_parser!("1!=2;", "(1!=2)");
        assert_parser!("1<2;", "(1<2)");
        assert_parser!("1>2;", "(1>2)");
        assert_parser!("1<=2;", "(1<=2)");
        assert_parser!("1>=2;", "(1>=2)");
    }

    #[test]
    fn unary_ops() {
        assert_parser!("!1;", "(!1)");
        assert_parser!("~1;", "(~1)");
        assert_parser!("-1;", "(-1)");
        assert_parser!("++1;", "(++1)");
        assert_parser!("--1;", "(--1)");
    }

    #[test]
    fn ternary_op() {
        assert_parser!("a ? b : c;", "(a ? b : c)");
        assert_parser!("a ? b : c ? d : e;", "(a ? b : (c ? d : e))");
        assert_parser!(
            "a ? b : c ? d : e ? f : g;",
            "(a ? b : (c ? d : (e ? f : g)))"
        );

        assert_parser!("(a ? b : c) ? d : e;", "((a ? b : c) ? d : e)");
    }
}
