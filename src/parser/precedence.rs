use crate::token::TokenType;

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    BLANK,
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

impl Precedence {
    pub fn lookup_precedence(token_type: TokenType) -> Precedence {
        match token_type {
            TokenType::EQ => Precedence::EQUALS,
            TokenType::NOTEQ => Precedence::EQUALS,
            TokenType::LT => Precedence::LESSGREATER,
            TokenType::GT => Precedence::LESSGREATER,
            TokenType::PLUS => Precedence::SUM,
            TokenType::MINUS => Precedence::SUM,
            TokenType::SLASH => Precedence::PRODUCT,
            TokenType::ASTERISK => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }
}
