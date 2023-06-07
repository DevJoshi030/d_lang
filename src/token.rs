#[derive(PartialEq, Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifiers + literals;
    IDENT,
    INT,
    STRING,

    // Operators;
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    // Comparision
    LT,
    GT,
    EQ,
    NOTEQ,

    // Delimiters;
    COMMA,
    SEMICOLON,
    COLON,

    // Brackets
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,

    // Keywords;
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: char,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type.eq(&other.token_type) && self.literal.eq(&other.literal)
    }
}
