#[derive(PartialEq, Debug, Clone, Copy)]
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

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "fn" => TokenType::FUNCTION,
            "let" => TokenType::LET,
            "true" => TokenType::TRUE,
            "false" => TokenType::FALSE,
            "if" => TokenType::IF,
            "else" => TokenType::ELSE,
            "return" => TokenType::RETURN,
            _ => TokenType::IDENT,
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type.eq(&other.token_type) && self.literal.eq(&other.literal)
    }
}
