mod block_stmt;
mod expr_stmt;
mod helper;
mod let_stmt;
mod precedence;
mod return_stmt;

use std::collections::HashMap;

use crate::{
    ast::{Expression, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};
use macros::sf;

#[derive(Debug)]
pub struct Parser {
    l: Lexer,
    curr_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,

    prefix_parse_fns: HashMap<TokenType, for<'a> fn(&'a mut Parser) -> Expression>,
    infix_parse_fns: HashMap<TokenType, for<'a> fn(&'a mut Parser, &Expression) -> Expression>,
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
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        p.register_prefix(TokenType::IDENT, Parser::parse_identifier);
        p.register_prefix(TokenType::INT, Parser::parse_integer_literal);
        p.register_prefix(TokenType::BANG, Parser::parse_prefix_expression);
        p.register_prefix(TokenType::MINUS, Parser::parse_prefix_expression);
        p.register_prefix(TokenType::TRUE, Parser::parse_boolean);
        p.register_prefix(TokenType::FALSE, Parser::parse_boolean);
        p.register_prefix(TokenType::LPAREN, Parser::parse_grouped_expression);
        p.register_prefix(TokenType::RPAREN, Parser::parse_grouped_expression);
        p.register_prefix(TokenType::PLUS, Parser::parse_grouped_expression);
        p.register_prefix(TokenType::IF, Parser::parse_if_expression);

        p.register_infix(TokenType::PLUS, Parser::parse_infix_expression);
        p.register_infix(TokenType::MINUS, Parser::parse_infix_expression);
        p.register_infix(TokenType::SLASH, Parser::parse_infix_expression);
        p.register_infix(TokenType::ASTERISK, Parser::parse_infix_expression);
        p.register_infix(TokenType::EQ, Parser::parse_infix_expression);
        p.register_infix(TokenType::NOTEQ, Parser::parse_infix_expression);
        p.register_infix(TokenType::LT, Parser::parse_infix_expression);
        p.register_infix(TokenType::GT, Parser::parse_infix_expression);

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
            TokenType::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn register_prefix(
        &mut self,
        token_type: TokenType,
        func: for<'a> fn(&'a mut Parser) -> Expression,
    ) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    fn register_infix(
        &mut self,
        token_type: TokenType,
        func: for<'a> fn(&'a mut Parser, &Expression) -> Expression,
    ) {
        self.infix_parse_fns.insert(token_type, func);
    }
}
