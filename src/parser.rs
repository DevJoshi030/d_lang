use crate::{
    ast::{Identifier, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};
use macros::sf;

pub struct Parser {
    l: Lexer,
    curr_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
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
            errors: vec![],
        };

        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program<Statement> {
        let mut program = Program { statements: vec![] };

        while self.curr_token.token_type != TokenType::EOF {
            let stmt = self.parse_statement();

            match stmt {
                Some(st) => program.statements.push(st),
                None => {}
            };

            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.curr_token.token_type {
            TokenType::LET => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let mut stmt = Statement::new_let_statement();

        if !self.expect_peek(TokenType::IDENT) {
            self.peek_error(TokenType::IDENT);
            return None;
        }

        stmt.set_name(Identifier {
            token: stmt.get_token(),
            value: self.curr_token.literal.clone(),
        });

        if !self.expect_peek(TokenType::ASSIGN) {
            self.peek_error(TokenType::ASSIGN);
            return None;
        }

        // TODO: Skipping expression till semicolon

        while !self.curr_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        return Some(stmt);
    }

    fn curr_token_is(&self, token_type: TokenType) -> bool {
        self.curr_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        }
        false
    }

    fn peek_error(&mut self, token_type: TokenType) {
        self.errors.push(format!(
            "expected token={:#?}, got {:#?}",
            token_type, self.peek_token
        ));
    }
}
