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
    ExprEnd,

    // misc
    LITERAL,
    ILLEGAL,
    NULL,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub tok_type: TokenType,
    pub literal: String,
}
