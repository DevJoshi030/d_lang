use crate::{ast::Statement, token::TokenType};

use super::Parser;

impl Parser {
    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        let mut stmt = Statement::new(TokenType::RETURN);

        self.next_token();
        stmt.set_value(self.parse_expression(super::precedence::Precedence::LOWEST));

        while !self.curr_token_is(TokenType::SEMICOLON) && !self.curr_token_is(TokenType::EOF) {
            self.next_token();
        }

        return Some(stmt);
    }
}
