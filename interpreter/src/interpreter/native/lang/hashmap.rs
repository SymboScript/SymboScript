use symboscript_parser::Parser;
use symboscript_types::parser::Ast;

pub fn value() -> Ast {
    Parser::new(
        "native/lang/hashmap.syms",
        "
            fn set[key, value] std.hashmap.set[this, key, value]; 
            fn get[key] return std.hashmap.get[this, key]; 
            fn del[key] std.hashmap.del[this, key];
            fn has[key] return std.hashmap.has[this, key];
            fn keys[] return std.hashmap.keys[this];
            fn values[] return std.hashmap.values[this];
            fn clear[] std.hashmap.clear[this];
            fn len[] return std.hashmap.len[this] - 9;
        ",
    )
    .parse()
}
