use macros::sf;

use crate::token::{Token, TokenType};

pub trait Node {
    fn token_literal(&self) -> &str;
}

#[derive(Clone, Debug)]
pub enum Statement {
    LetStatement {
        token: Token,
        name: Identifier,
        value: Expression,
    },
    ReturnStatement {
        token: Token,
        value: Expression,
    },
    IllegalStatement,
}

impl Node for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Statement::LetStatement {
                token,
                name: _,
                value: _,
            } => &token.literal,
            Statement::ReturnStatement { token, value: _ } => &token.literal,
            _ => "\0",
        }
    }
}

impl Statement {
    pub fn new(token_type: TokenType) -> Statement {
        match token_type {
            TokenType::LET => Statement::LetStatement {
                token: Token {
                    token_type: TokenType::LET,
                    literal: sf!("let"),
                },
                name: Identifier {
                    token: Token {
                        token_type: TokenType::LET,
                        literal: sf!("let"),
                    },
                    value: sf!("let"),
                },
                value: Expression::NoExpression,
            },
            TokenType::RETURN => Statement::ReturnStatement {
                token: Token {
                    token_type: TokenType::RETURN,
                    literal: sf!("return"),
                },
                value: Expression::NoExpression,
            },
            _ => Statement::IllegalStatement,
        }
    }

    pub fn set_let_name(&mut self, ident: Identifier) {
        match self {
            Statement::LetStatement {
                token: _,
                ref mut name,
                value: _,
            } => *name = ident,
            _ => (),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    NoExpression,
}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        "ex"
    }
}

pub struct Program<T>
where
    T: Node,
{
    pub statements: Vec<T>,
}

impl<T> Node for Program<T>
where
    T: Node,
{
    fn token_literal(&self) -> &str {
        if self.statements.len() > 0 {
            self.statements.get(0).unwrap().token_literal()
        } else {
            ""
        }
    }
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.value
    }
}
