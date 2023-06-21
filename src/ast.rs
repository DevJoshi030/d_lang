use macros::sf;

use crate::token::{Token, TokenType};

pub trait Node {
    fn token_literal(&self) -> &str;
    fn to_string(&self) -> String;
}

#[derive(Clone, Debug)]
pub enum Expression {
    Identifier {
        token: Token,
        value: String,
    },
    IntegerLiteral {
        token: Token,
        value: i32,
    },
    Prefix {
        token: Token,
        operator: String,
        right: i32,
    },
    NoExpression,
}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier { token, value: _ } => &token.literal,
            Expression::IntegerLiteral { token, value: _ } => &token.literal,
            Expression::Prefix {
                token,
                operator: _,
                right: _,
            } => &token.literal,
            Expression::NoExpression => "\0",
        }
    }

    fn to_string(&self) -> String {
        match self {
            Expression::Identifier { token: _, value } => value.clone(),
            Expression::IntegerLiteral { token: _, value } => value.to_string(),
            Expression::Prefix {
                token: _,
                operator,
                right,
            } => sf!(format!("({}{})", operator, right.to_string())),
            Expression::NoExpression => sf!("\0"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    LetStatement {
        token: Token,
        name: Expression,
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
                format!(
                    "{} {} = {};",
                    token.literal,
                    name.token_literal(),
                    value.to_string()
                )
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
                name: Expression::Identifier {
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
            token_type => Statement::ExpressionStatement {
                token: Token {
                    token_type: token_type,
                    literal: sf!("\0"),
                },
                expression: Expression::NoExpression,
            },
        }
    }

    pub fn set_let_name(&mut self, ident: Expression) {
        match self {
            Statement::LetStatement {
                token: _,
                ref mut name,
                value: _,
            } => *name = ident,
            _ => (),
        }
    }

    pub fn set_expression(&mut self, expr: Expression) {
        match self {
            Statement::ExpressionStatement {
                token: _,
                ref mut expression,
            } => *expression = expr,
            _ => (),
        }
    }

    pub fn set_expression_literal(&mut self) {
        match self {
            Statement::ExpressionStatement {
                ref mut token,
                expression,
            } => token.literal = sf!(expression.token_literal()),
            _ => (),
        }
    }
}

pub struct Program<Statement> {
    pub statements: Vec<Statement>,
}
