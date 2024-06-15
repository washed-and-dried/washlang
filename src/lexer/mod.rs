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

    pub fn next_token() -> Token {
        Token {
            tok_type: TokenType::NULL,
            literal: String::from(""),
        }
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

    fn char_at(&self, index: usize) -> char {
        return self.input.chars().nth(index).unwrap();
    }
}
