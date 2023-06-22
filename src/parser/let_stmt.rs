use crate::{
    ast::{Expression, Statement},
    token::TokenType,
};

use super::Parser;

impl Parser {
    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        let mut stmt = Statement::new(TokenType::LET);

        if !self.expect_peek(TokenType::IDENT) {
            self.peek_error(TokenType::IDENT);
            return None;
        }

        stmt.set_let_name(Expression::Identifier {
            token: self.curr_token.clone(),
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
}
