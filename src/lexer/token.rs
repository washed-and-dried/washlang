#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // keyword
    KeywordStart,
    LET,
    FN,
    KeywordEnd,

    // data types
    INT,

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
    MoreThan,
    MoreThanEqual,
    LogicalNot,
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

    pub fn is_keyword(literal: &str) -> Option<TokenType> {
        todo!();
    }
}
