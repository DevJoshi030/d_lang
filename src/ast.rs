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
}

impl Node for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Statement::LetStatement {
                token,
                name: _,
                value: _,
            } => &token.literal,
        }
    }
}

impl Statement {
    pub fn new_let_statement() -> Statement {
        Statement::LetStatement {
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
        }
    }

    pub fn get_name(&self) -> Identifier {
        match self {
            Statement::LetStatement {
                token: _,
                name,
                value: _,
            } => name.clone(),
        }
    }

    pub fn get_token(&self) -> Token {
        match self {
            Statement::LetStatement {
                token,
                name: _,
                value: _,
            } => token.clone(),
        }
    }

    pub fn set_name(&mut self, ident: Identifier) {
        match self {
            Statement::LetStatement {
                token: _,
                ref mut name,
                value: _,
            } => *name = ident,
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
