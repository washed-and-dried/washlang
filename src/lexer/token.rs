#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // keyword
    KeywordStart,
    LET,
    FN,
    KeywordEnd,

    // expression token
    ExprStart,
    PLUS,
    MINUS,
    ASTERIC,
    SLASH,
    MODULO,
    ASSIGN,
    EQUAL,
    NotEqual,
    LessThan,
    LessThanEqual,
    MORE,
    MoreThanEqual,
    ExprEnd,

    // misc
    LITERAL,
    ILLEGAL,
    EOF,
    NULL,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub tok_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(literal: String, tok_type: TokenType) -> Self {
        Token { literal, tok_type }
    }
}
