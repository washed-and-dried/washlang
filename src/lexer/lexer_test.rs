#[cfg(test)]
mod lexertest {
    use crate::lexer::*;

    #[test]
    fn testing_lexer() {
        // basic expressions
        test_tokens(
            "+-*/%",
            vec![
                Token::new(String::from("+"), TokenType::PLUS),
                Token::new(String::from("-"), TokenType::MINUS),
                Token::new(String::from("*"), TokenType::ASTERIC),
                Token::new(String::from("/"), TokenType::SLASH),
                Token::new(String::from("%"), TokenType::MODULO),
            ],
        );

        // peek tokens such as =, ==, FIXME: =>, >, <, =<
        test_tokens(
            "=+==
            ===
            ",
            vec![
                Token::new(String::from("="), TokenType::ASSIGN),
                Token::new(String::from("+"), TokenType::PLUS),
                Token::new(String::from("=="), TokenType::EQUAL),
                Token::new(String::from("=="), TokenType::EQUAL),
                Token::new(String::from("="), TokenType::ASSIGN),
            ],
        );
    }

    fn test_tokens(input: &str, mut toks: Vec<Token>) {
        let mut lexer = Lexer::new(input.to_string());
        toks.push(Token::new("\0".to_owned(), TokenType::EOF)); // additional EOF for every test

        for tok in toks {
            assert_eq!(tok, lexer.next_token());
        }
    }
}
