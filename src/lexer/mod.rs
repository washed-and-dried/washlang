pub mod lexer_test;
pub mod token;
use token::*;

pub struct Lexer {
    input: String,
    position_current: usize,
    position_next: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lex = Self {
            input,
            position_current: 0,
            position_next: 0,
            ch: '\0',
        };

        lex.read_next();

        lex
    }

    pub fn next_token(&mut self) -> Token {
        // skip \n, \r and \t
        self.skip_whitespaces();

        let tok = match self.ch {
            // null character indicates end of file/line
            '\0' => Token::new(self.ch.to_string(), TokenType::EOF),

            // expression: +, -, /, *, %
            '+' => Token::new(self.ch.to_string(), TokenType::PLUS),
            '-' => Token::new(self.ch.to_string(), TokenType::MINUS),
            '*' => Token::new(self.ch.to_string(), TokenType::ASTERIC),
            '/' => Token::new(self.ch.to_string(), TokenType::SLASH),
            '%' => Token::new(self.ch.to_string(), TokenType::MODULO),

            // match =, ==, >=, <=
            '=' => {
                if self.match_next('=') {
                    self.read_next(); // skip =
                    Token::new(String::from("=="), TokenType::EQUAL)
                } else {
                    Token::new(self.ch.to_string(), TokenType::ASSIGN)
                }
            }

            //TODO: handle literals, keywords and other related things.
            _ => {
                if self.ch.is_ascii_digit() {
                    return self.read_digit();
                } else if self.ch.is_ascii_alphabetic() {
                    return self.read_literal();
                } else {
                    Token::new(self.ch.to_string(), TokenType::ILLEGAL)
                }
            }
        };

        // put the next character in stream to `self.ch`
        self.read_next();

        tok
    }

    fn match_next(&mut self, c: char) -> bool {
        self.char_at(self.position_next) == c
    }
    // TODO: Yet to implement, walks the entire self.input using self.read_next() until a ' ' space
    // is encountered. (maybe not just a space, @kishore needed)
    fn walk_rest(&mut self) -> String {
        let mut val: String = "".to_string();
        while self.ch != ' ' {
            // TODO: other checks here later please
            val.push(self.ch);
            self.read_next();
        }
        val
    }

    fn read_digit(&mut self) -> Token {
        let pos = self.position_current;

        while self.ch.is_ascii_digit() {
            self.read_next();
        }

        let literal = &self.input[pos..self.position_current];

        Token::new(literal.to_owned(), TokenType::INT)
    }

    fn read_literal(&mut self) -> Token {
        let pos = self.position_current;

        while self.ch.is_ascii_alphabetic() {
            self.read_next();
        }

        let literal = &self.input[pos..self.position_current];

        let mut toktype = TokenType::LITERAL;

        if let Some(keywordType) = Token::is_keyword(literal) {
            toktype = keywordType;
        }

        Token::new(literal.to_owned(), toktype)
    }

    fn read_next(&mut self) {
        if self.position_next >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.char_at(self.position_next);
        }

        self.position_current = self.position_next;
        self.position_next += 1;
    }

    fn skip_whitespaces(&mut self) {
        while self.ch.is_ascii_whitespace() || self.ch == '\n' || self.ch == '\r' || self.ch == '\t'
        {
            self.read_next();
        }
    }

    fn char_at(&self, index: usize) -> char {
        return self.input.chars().nth(index).unwrap();
    }
}
