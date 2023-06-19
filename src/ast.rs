use macros::sf;

use crate::token::{Token, TokenType};

pub trait Node {
    fn token_literal(&self) -> &str;
    fn to_string(&self) -> String;
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
    ExpressionStatement {
        token: Token,
        expression: Expression,
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

    fn to_string(&self) -> String {
        match self {
            Statement::LetStatement { token, name, value } => {
                format!("{} {} = {};", token.literal, name.value, value.to_string())
            }
            Statement::ReturnStatement { token, value } => {
                format!("{} {:#?};", token.literal, value.to_string())
            }
            Statement::ExpressionStatement {
                token: _,
                expression,
            } => {
                format!("{}", expression.to_string())
            }
            _ => sf!("\0"),
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
                value: Expression { value: sf!("\0") },
            },
            TokenType::RETURN => Statement::ReturnStatement {
                token: Token {
                    token_type: TokenType::RETURN,
                    literal: sf!("return"),
                },
                value: Expression { value: sf!("\0") },
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

pub struct Program<Statement> {
    pub statements: Vec<Statement>,
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

    fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Expression {
    pub value: String,
}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        &self.value
    }

    fn to_string(&self) -> String {
        self.value.clone()
    }
}
