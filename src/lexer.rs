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
            ch: ' ',
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
        let mut t = Token {
            token_type: token::TokenType::ILLEGAL,
            literal: self.ch,
        };

        match self.ch {
            '=' => {
                t.token_type = token::TokenType::ASSIGN;
            }
            ';' => {
                t.token_type = token::TokenType::SEMICOLON;
            }
            '(' => {
                t.token_type = token::TokenType::LPAREN;
            }
            ')' => {
                t.token_type = token::TokenType::RPAREN;
            }
            ',' => {
                t.token_type = token::TokenType::COMMA;
            }
            '+' => {
                t.token_type = token::TokenType::PLUS;
            }
            '{' => {
                t.token_type = token::TokenType::LBRACE;
            }
            '}' => {
                t.token_type = token::TokenType::RBRACE;
            }
            '\0' => {
                t.token_type = token::TokenType::EOF;
            }
            _ => {
                t.token_type = token::TokenType::ILLEGAL;
            }
        }

        self.read_char();
        return t;
    }
}
