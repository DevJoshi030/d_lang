pub struct Token {
    pub token_type: TokenType,
    pub literal: char,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type.value().eq(&other.token_type.value()) && self.literal.eq(&other.literal)
    }
}

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

    LT,
    GT,

    EQ,
    NOTEQ,

    // Delimiters;
    COMMA,
    SEMICOLON,
    COLON,

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

impl TokenType {
    pub fn value(&self) -> String {
        match *self {
            TokenType::ILLEGAL => String::from("ILLEGAL"),
            TokenType::EOF => String::from("EOF"),
            TokenType::IDENT => String::from("IDENT"),
            TokenType::INT => String::from("INT"),
            TokenType::STRING => String::from("STRING"),
            TokenType::ASSIGN => String::from("="),
            TokenType::PLUS => String::from("+"),
            TokenType::MINUS => String::from("-"),
            TokenType::BANG => String::from("!"),
            TokenType::ASTERISK => String::from("*"),
            TokenType::SLASH => String::from("/"),
            TokenType::LT => String::from("<"),
            TokenType::GT => String::from(">"),
            TokenType::EQ => String::from("=="),
            TokenType::NOTEQ => String::from("!="),
            TokenType::COMMA => String::from(","),
            TokenType::SEMICOLON => String::from(";"),
            TokenType::COLON => String::from(":"),
            TokenType::LPAREN => String::from("("),
            TokenType::RPAREN => String::from(")"),
            TokenType::LBRACE => String::from("{"),
            TokenType::RBRACE => String::from("}"),
            TokenType::LBRACKET => String::from("["),
            TokenType::RBRACKET => String::from("]"),
            TokenType::FUNCTION => String::from("FUNCTION"),
            TokenType::LET => String::from("LET"),
            TokenType::TRUE => String::from("TRUE"),
            TokenType::FALSE => String::from("FALSE"),
            TokenType::IF => String::from("IF"),
            TokenType::ELSE => String::from("ELSE"),
            TokenType::RETURN => String::from("RETURN"),
        }
    }
}
