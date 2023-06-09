use crate::token::{Token, TokenType};

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
                '=' => {
                    if self.peek_char() == '=' {
                        self.read_char();
                        literal.push(self.ch);
                        self.read_char();
                        return Token {
                            token_type: TokenType::EQ,
                            literal,
                        };
                    }
                    TokenType::ASSIGN
                }
                ';' => TokenType::SEMICOLON,
                '(' => TokenType::LPAREN,
                ')' => TokenType::RPAREN,
                ',' => TokenType::COMMA,
                '+' => TokenType::PLUS,
                '-' => TokenType::MINUS,
                '{' => TokenType::LBRACE,
                '}' => TokenType::RBRACE,
                '!' => {
                    if self.peek_char() == '=' {
                        self.read_char();
                        literal.push(self.ch);
                        self.read_char();
                        return Token {
                            token_type: TokenType::NOTEQ,
                            literal,
                        };
                    }
                    TokenType::BANG
                }
                '*' => TokenType::ASTERISK,
                '<' => TokenType::LT,
                '>' => TokenType::GT,
                '/' => TokenType::SLASH,
                '\0' => TokenType::EOF,
                ch => {
                    let t_type: TokenType;
                    if Lexer::is_letter(ch) {
                        literal = self.read_identifier().into_iter().collect();
                        return Token {
                            token_type: Token::lookup_ident(&literal),
                            literal,
                        };
                    } else if Lexer::is_digit(ch) {
                        literal = self.read_number().into_iter().collect();
                        return Token {
                            token_type: TokenType::INT,
                            literal,
                        };
                    } else {
                        t_type = TokenType::ILLEGAL
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

    fn peek_char(&self) -> char {
        *self.input.get(self.read_position).unwrap_or(&'\0')
    }
}
