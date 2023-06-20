use std::{collections::HashMap, ops::Deref};

use crate::{
    ast::{Expression, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};
use macros::sf;

enum Precedence {
    BLANK,
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

pub struct Parser {
    l: Lexer,
    curr_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,

    prefix_parse_fns: HashMap<TokenType, Box<dyn for<'a> Fn(&'a Parser) -> Expression>>,
    infix_parse_fns: HashMap<TokenType, Box<dyn for<'a> Fn(&'a Parser) -> Expression>>,
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

    fn parse_let_statement(&mut self) -> Option<Statement> {
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

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let stmt = Statement::new(TokenType::RETURN);

        self.next_token();

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

    pub fn check_parse_errors(&self) {
        if self.errors.len() == 0 {
            return;
        }

        for err in self.errors.iter() {
            println!("{}", err);
        }

        panic!("error reported in parsing!!!");
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let mut stmt = Statement::new(self.curr_token.token_type);

        stmt.set_expression(self.parse_expression(Precedence::LOWEST));
        stmt.set_expression_literal();

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    fn parse_expression(&mut self, prec: Precedence) -> Expression {
        let prefix_option = self.prefix_parse_fns.get(&self.curr_token.token_type);

        let prefix = match prefix_option {
            Some(prefix) => prefix,
            None => return Expression::NoExpression,
        }
        .deref();

        prefix(&self)
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier {
            token: Token {
                token_type: TokenType::IDENT,
                literal: self.curr_token.literal.clone(),
            },
            value: self.curr_token.literal.clone(),
        }
    }

    fn parse_integer_literal(&self) -> Expression {
        Expression::IntegerLiteral {
            token: Token {
                token_type: TokenType::INT,
                literal: self.curr_token.literal.clone(),
            },
            value: self.curr_token.literal.clone().parse().expect(&format!(
                "Failed to parse int, got={}",
                self.curr_token.literal.clone(),
            )),
        }
    }

    fn register_prefix(
        &mut self,
        token_type: TokenType,
        func: for<'a> fn(&'a Parser) -> Expression,
    ) {
        self.prefix_parse_fns.insert(token_type, Box::new(func));
    }

    fn register_infix(
        &mut self,
        token_type: TokenType,
        func: for<'a> fn(&'a Parser) -> Expression,
    ) {
        self.infix_parse_fns.insert(token_type, Box::new(func));
    }
}
