#[cfg(test)]
mod lexertest {
    use crate::lexer::*;

    #[test]
    fn testing_basic_expression() {
        let input = "+-*/%1";

        let mut lexer = Lexer::new(input.to_string());

        let toks: Vec<Token> = vec![
            Token::new(String::from("+"), TokenType::PLUS),
            Token::new(String::from("-"), TokenType::MINUS),
            Token::new(String::from("*"), TokenType::ASTERIC),
            Token::new(String::from("/"), TokenType::SLASH),
            Token::new(String::from("%"), TokenType::MODULO),

            Token::new(String::from("1"), TokenType::ILLEGAL),

            Token::new(String::from("\0"), TokenType::EOF)
        ];

        for tok in toks {
            assert_eq!(tok, lexer.next_token());
        }
    }
}
