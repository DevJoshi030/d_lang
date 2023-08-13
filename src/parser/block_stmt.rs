use crate::{ast::Statement, token::TokenType};

use super::Parser;

impl Parser {
    pub fn parse_block_statement(&mut self) -> Option<Statement> {
        let mut stmt = Statement::new(TokenType::BLOCK);

        stmt.set_block_token(self.curr_token.clone());

        self.next_token();

        while !self.curr_token_is(TokenType::RBRACE) || self.curr_token_is(TokenType::EOF) {
            let parsed_stmt = self.parse_statement().unwrap();
            stmt.add_block_stmt(parsed_stmt);
            self.next_token();
        }

        return Some(stmt);
    }
}
