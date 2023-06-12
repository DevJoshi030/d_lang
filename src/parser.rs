use crate::{
    ast::{LetStatement, Node, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};
use macros::sf;

pub struct Parser {
    l: Lexer,
    curr_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,
            curr_token: Token {
                token_type: TokenType::ILLEGAL,
                literal: sf!("\0"),
            },
            peek_token: Token {
                token_type: TokenType::ILLEGAL,
                literal: sf!("\0"),
            },
        };

        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) {
        let mut program = Program { statements: vec![] };

        while self.curr_token.token_type != TokenType::EOF {
            let stmt = self.parse_statement();

            match stmt {
                Some(st) => program.statements.push(st.deref()),
                None => {}
            };

            self.next_token();
        }
    }

    fn parse_statement<T>(&self) -> Option<T>
    where
        T: Node,
    {
        match self.curr_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement<T>(&mut self) -> Option<T>
    where
        T: Node,
    {
        let stmt = LetStatement::new();

        // if !self.expect_peek(TokenType::IDENT) {
        //     return None;
        // }

        // if !self.expect_peek(TokenType::ASSIGN) {
        //     return None;
        // }

        // while !self.curr_token_is(TokenType::SEMICOLON) {
        //     self.next_token();
        // }

        return Some(stmt);
    }
}
