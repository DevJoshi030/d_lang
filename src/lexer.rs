use crate::token::{self, Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        self.ch = *self.input.get(self.read_position).unwrap_or(&'\0');
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();
        let mut literal = self.ch.to_string();
        let t = Token {
            token_type: match self.ch {
                '=' => token::TokenType::ASSIGN,
                ';' => token::TokenType::SEMICOLON,
                '(' => token::TokenType::LPAREN,
                ')' => token::TokenType::RPAREN,
                ',' => token::TokenType::COMMA,
                '+' => token::TokenType::PLUS,
                '-' => token::TokenType::MINUS,
                '{' => token::TokenType::LBRACE,
                '}' => token::TokenType::RBRACE,
                '!' => token::TokenType::BANG,
                '*' => token::TokenType::ASTERISK,
                '<' => token::TokenType::LT,
                '>' => token::TokenType::GT,
                '/' => token::TokenType::SLASH,
                '\0' => token::TokenType::EOF,
                ch => {
                    let t_type: TokenType;
                    if Lexer::is_letter(ch) {
                        literal = self.read_identifier().into_iter().collect();
                        t_type = Token::lookup_ident(&literal);
                        return Token {
                            token_type: t_type,
                            literal,
                        };
                    } else if Lexer::is_digit(ch) {
                        literal = self.read_number().into_iter().collect();
                        t_type = TokenType::INT;
                        return Token {
                            token_type: t_type,
                            literal,
                        };
                    } else {
                        t_type = token::TokenType::ILLEGAL
                    }
                    t_type
                }
            },
            literal,
        };

        self.read_char();
        return t;
    }

    fn read_identifier(&mut self) -> Vec<char> {
        let pos = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char();
        }

        self.input[pos..self.position].into()
    }

    fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

    fn skip_white_space(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Vec<char> {
        let pos = self.position;
        while Lexer::is_digit(self.ch) {
            self.read_char();
        }

        self.input[pos..self.position].into()
    }
}
