use macros::sf;

use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> &str;
}

pub trait StatementTrait: Node {}
#[derive(Clone, Copy)]
pub struct Statement {}

impl StatementTrait for Statement {}

impl Node for Statement {
    fn token_literal(&self) -> &str {
        "st"
    }
}

pub trait ExpressionTrait: Node {}
pub struct Expression {}

impl ExpressionTrait for Expression {}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        "ex"
    }
}

pub struct Program<T>
where
    T: Node,
{
    pub statements: Vec<Box<T>>,
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

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl ExpressionTrait for Identifier {}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.value
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl LetStatement {
    pub fn new() -> LetStatement {
        LetStatement {
            token: Token {
                token_type: crate::token::TokenType::LET,
                literal: sf!("let"),
            },
            name: Identifier {
                token: Token {
                    token_type: crate::token::TokenType::LET,
                    literal: sf!("let"),
                },
                value: sf!("let"),
            },
            value: Expression {},
        }
    }
}

impl StatementTrait for LetStatement {}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.value.token_literal()
    }
}
