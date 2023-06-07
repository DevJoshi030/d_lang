use crate::token::{self, Token};

pub struct Lexer {
    input: Vec<char>,
    position: i32,
    read_position: i32,
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
        self.ch = *self.input.get(self.read_position as usize).unwrap_or(&'\0');
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let t = Token {
            token_type: match self.ch {
                '=' => token::TokenType::ASSIGN,
                ';' => token::TokenType::SEMICOLON,
                '(' => token::TokenType::LPAREN,
                ')' => token::TokenType::RPAREN,
                ',' => token::TokenType::COMMA,
                '+' => token::TokenType::PLUS,
                '{' => token::TokenType::LBRACE,
                '}' => token::TokenType::RBRACE,
                '\0' => token::TokenType::EOF,
                _ => token::TokenType::ILLEGAL,
            },
            literal: self.ch,
        };

        self.read_char();
        return t;
    }
}
