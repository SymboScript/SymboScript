use symboscript_parser::Parser;
use symboscript_types::parser::Ast;

pub fn value() -> Ast {
    Parser::new(
        "native/lang/hashmap.syms",
        "fn set[key, value] {std.hashmap.set[this, key, value];}",
    )
    .parse()
}
