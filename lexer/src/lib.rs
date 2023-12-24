mod lexer;
mod types;

pub use lexer::Lexer;

#[cfg(test)]
mod tests {
    use crate::types::{Kind, Token, TokenValue::*};
    use crate::Lexer;
    #[test]
    fn plus() {
        let mut lexer = Lexer::new("./test.syms", "2x^2 + 2y^2 == 2");
    }
}
