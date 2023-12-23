mod lexer;
mod types;

pub use lexer::Lexer;

#[cfg(test)]
mod tests {
    use crate::types::{Kind, Token, TokenValue::*};
    use crate::Lexer;
    #[test]
    fn plus() {
        let mut lexer = Lexer::new("2x^2 + 2y^2 == 2");
        assert_eq!(
            lexer.tokenize(),
            [
                Token {
                    kind: Kind::Number,
                    start: 0,
                    end: 1,
                    value: Number(2.0,),
                },
                Token {
                    kind: Kind::Identifier,
                    start: 1,
                    end: 2,
                    value: Identifier("x".to_string()),
                },
                Token {
                    kind: Kind::Number,
                    start: 2,
                    end: 4,
                    value: Number(0.0,),
                },
                Token {
                    kind: Kind::Plus,
                    start: 4,
                    end: 6,
                    value: None,
                },
                Token {
                    kind: Kind::Number,
                    start: 6,
                    end: 8,
                    value: Number(2.0,),
                },
                Token {
                    kind: Kind::Identifier,
                    start: 8,
                    end: 9,
                    value: Identifier("y".to_string()),
                },
                Token {
                    kind: Kind::Number,
                    start: 9,
                    end: 11,
                    value: Number(0.0,),
                },
                Token {
                    kind: Kind::Number,
                    start: 11,
                    end: 16,
                    value: Number(0.0,),
                },
            ]
        );
    }
}
