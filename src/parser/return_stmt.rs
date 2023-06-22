use crate::{ast::Statement, token::TokenType};

use super::Parser;

impl Parser {
    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        let stmt = Statement::new(TokenType::RETURN);

        self.next_token();

        // TODO: Skipping expression till semicolon

        while !self.curr_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        return Some(stmt);
    }
}
