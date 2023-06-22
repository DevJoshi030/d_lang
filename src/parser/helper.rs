use crate::token::TokenType;

use super::{precedence::Precedence, Parser};

impl Parser {
    pub fn curr_token_is(&self, token_type: TokenType) -> bool {
        self.curr_token.token_type == token_type
    }

    pub fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        }
        false
    }

    pub fn peek_error(&mut self, token_type: TokenType) {
        self.errors.push(format!(
            "expected token={:#?}, got {:#?}",
            token_type, self.peek_token
        ));
    }

    pub fn no_prefix_parse_fn_error(&mut self, token_type: TokenType) {
        self.errors
            .push(format!("no prefix parse fn for {:#?} found", token_type));
    }

    pub fn check_parse_errors(&self) {
        if self.errors.len() == 0 {
            return;
        }

        for err in self.errors.iter() {
            println!("{}", err);
        }

        panic!("error reported in parsing!!!");
    }

    pub fn peek_precedence(&self) -> Precedence {
        Precedence::lookup_precedence(self.peek_token.token_type)
    }

    pub fn curr_precedence(&self) -> Precedence {
        Precedence::lookup_precedence(self.curr_token.token_type)
    }
}
