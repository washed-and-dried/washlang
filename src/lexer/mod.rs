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

            //TODO: handle literals, keywords and other related things.
            _ => Token::new(self.ch.to_string(), TokenType::ILLEGAL),
        };

        // put the next character in stream to `self.ch`
        self.read_next();

        tok
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
        while self.ch == '\n' || self.ch == '\r' || self.ch == '\t' {
            self.read_next();
        }
    }

    fn char_at(&self, index: usize) -> char {
        return self.input.chars().nth(index).unwrap();
    }
}