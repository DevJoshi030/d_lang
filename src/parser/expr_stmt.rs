use crate::{
    ast::{Expression, Statement},
    token::{Token, TokenType},
};

use super::{precedence::Precedence, Parser};

impl Parser {
    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        let mut stmt = Statement::new(self.curr_token.token_type);

        stmt.set_expression(self.parse_expression(Precedence::LOWEST));
        stmt.set_expression_literal();

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(stmt)
    }

    pub fn parse_expression(&mut self, prec: Precedence) -> Expression {
        let prefix_option = self.prefix_parse_fns.get(&self.curr_token.token_type);

        let prefix = match prefix_option {
            Some(prefix) => prefix,
            None => {
                self.no_prefix_parse_fn_error(self.curr_token.token_type);
                return Expression::NoExpression;
            }
        };

        let mut left = prefix(self);

        while !self.peek_token_is(TokenType::SEMICOLON) && prec < self.peek_precedence() {
            let infix_option = self.infix_parse_fns.get(&self.peek_token.token_type);

            let infix = match infix_option {
                Some(infix) => infix,
                None => {
                    return left;
                }
            };

            left = infix(self, &left);
        }

        left
    }

    pub fn parse_prefix_expression(&mut self) -> Expression {
        Expression::Prefix {
            token: self.curr_token.clone(),
            operator: self.curr_token.literal.clone(),
            right: {
                self.next_token();
                Box::new(self.parse_expression(Precedence::PREFIX))
            },
        }
    }

    pub fn parse_infix_expression(&mut self, left_expr: &Expression) -> Expression {
        self.next_token();
        Expression::Infix {
            token: self.curr_token.clone(),
            left: Box::new(left_expr.clone()),
            operator: self.curr_token.literal.clone(),
            right: {
                let prec = self.curr_precedence();
                self.next_token();
                Box::new(self.parse_expression(prec))
            },
        }
    }

    pub fn parse_identifier(&mut self) -> Expression {
        Expression::Identifier {
            token: Token {
                token_type: TokenType::IDENT,
                literal: self.curr_token.literal.clone(),
            },
            value: self.curr_token.literal.clone(),
        }
    }

    pub fn parse_integer_literal(&mut self) -> Expression {
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

    pub fn parse_boolean(&mut self) -> Expression {
        Expression::BooleanLiteral {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone().parse().expect(&format!(
                "Failed to parse int, got={}",
                self.curr_token.literal.clone(),
            )),
        }
    }

    pub fn parse_grouped_expression(&mut self) -> Expression {
        self.next_token();
        let expr = self.parse_expression(Precedence::LOWEST);

        if !self.expect_peek(TokenType::RPAREN) {
            panic!("Error: no RPAREN found");
        }

        expr
    }

    pub fn parse_if_expression(&mut self) -> Expression {
        let curr_token = self.curr_token.clone();

        self.next_token();
        let cond = self.parse_expression(Precedence::LOWEST);

        if !self.expect_peek(TokenType::LBRACE) {
            panic!("LBRACE Expected after condition!!!");
        }

        let consequence = self.parse_block_statement().unwrap();
        let mut alternative = Box::new(None);

        if self.peek_token_is(TokenType::ELSE) {
            self.next_token();

            if self.peek_token_is(TokenType::IF) {
                self.next_token();
                let mut stmt = Statement::new(TokenType::IF);
                stmt.set_expression(self.parse_if_expression());
                alternative = Box::new(Some(stmt));
            } else {
                if !self.expect_peek(TokenType::LBRACE) {
                    panic!("LBRACE Expected after else!!!");
                }
                alternative = Box::new(self.parse_block_statement());
            }
        }

        Expression::IfExpression {
            token: curr_token,
            condition: Box::new(cond),
            consequence: Box::new(consequence),
            alternative,
        }
    }

    pub fn parse_func_expression(&mut self) -> Expression {
        let curr_token = self.curr_token.clone();

        if !self.expect_peek(TokenType::LPAREN) {
            panic!("LPAREN Expected after fn!!!");
        }

        let parameters = self.parse_func_parameters();

        if !self.expect_peek(TokenType::LBRACE) {
            panic!("LBRACE Expected after parameters!!!");
        }

        let body = self.parse_block_statement().unwrap();

        Expression::FuncExpression {
            token: curr_token,
            parameters,
            body: Box::new(body),
        }
    }

    fn parse_func_parameters(&mut self) -> Vec<Expression> {
        let mut identifiers = vec![];

        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return identifiers;
        }

        self.next_token();

        let ident = Expression::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        };

        identifiers.push(ident);

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            let ident = Expression::Identifier {
                token: self.curr_token.clone(),
                value: self.curr_token.literal.clone(),
            };

            identifiers.push(ident);
        }

        if !self.expect_peek(TokenType::RPAREN) {
            panic!("RPAREN Expected after LPAREN!!!")
        }

        identifiers
    }
}
