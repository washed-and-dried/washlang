#[cfg(test)]
mod lexertest {
    use crate::lexer::*;

    #[test]
    fn testing_basic_expression() {
        let input = "
            +-*/%

            =+==
            ===
        ";

        let mut lexer = Lexer::new(input.to_string());

        // do not remove white spaces between the separated token group :O
        let toks: Vec<Token> = vec![
            Token::new(String::from("+"), TokenType::PLUS),
            Token::new(String::from("-"), TokenType::MINUS),
            Token::new(String::from("*"), TokenType::ASTERIC),
            Token::new(String::from("/"), TokenType::SLASH),
            Token::new(String::from("%"), TokenType::MODULO),


            Token::new(String::from("="), TokenType::ASSIGN),
            Token::new(String::from("+"), TokenType::PLUS),
            Token::new(String::from("=="), TokenType::EQUAL),

            Token::new(String::from("=="), TokenType::EQUAL),
            Token::new(String::from("="), TokenType::ASSIGN),

            Token::new(String::from("\0"), TokenType::EOF)
        ];

        for tok in toks {
            assert_eq!(tok, lexer.next_token());
        }
    }
}
