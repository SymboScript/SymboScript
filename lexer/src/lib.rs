mod lexer;

pub use lexer::Lexer;

pub use symboscript_types::lexer as types;
pub use symboscript_utils as utils;

#[cfg(test)]
mod tests {
    use crate::types::{Token, TokenKind, TokenValue::*};
    use crate::Lexer;
    #[test]
    fn plus() {
        let mut lexer = Lexer::new("./test.syms", "2x^2 + 2y^2 == 2");
    }
}
