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
    BooleanLiteral {
        token: Token,
        value: bool,
    },
    Prefix {
        token: Token,
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        token: Token,
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    IfExpression {
        token: Token,
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Box<Option<Statement>>,
    },
    FuncExpression {
        token: Token,
        parameters: Vec<Expression>,
        body: Box<Statement>,
    },
    CallExpression {
        token: Token,
        func: Box<Expression>,
        args: Vec<Expression>,
    },
    NoExpression,
}

impl Expression {
    fn get_token(&self) -> TokenType {
        match self {
            Expression::Identifier { token, .. } => token.token_type,
            Expression::IntegerLiteral { token, .. } => token.token_type,
            Expression::BooleanLiteral { token, .. } => token.token_type,
            Expression::Prefix { token, .. } => token.token_type,
            Expression::Infix { token, .. } => token.token_type,
            Expression::IfExpression { token, .. } => token.token_type,
            Expression::FuncExpression { token, .. } => token.token_type,
            Expression::CallExpression { token, .. } => token.token_type,
            Expression::NoExpression => TokenType::ILLEGAL,
        }
    }
}

impl Node for Expression {
    fn token_literal(&self) -> &str {
        match self {
            Expression::Identifier { token, .. } => &token.literal,
            Expression::IntegerLiteral { token, .. } => &token.literal,
            Expression::BooleanLiteral { token, .. } => &token.literal,
            Expression::Prefix { token, .. } => &token.literal,
            Expression::Infix { token, .. } => &token.literal,
            Expression::IfExpression { token, .. } => &token.literal,
            Expression::FuncExpression { token, .. } => &token.literal,
            Expression::CallExpression { token, .. } => &token.literal,
            Expression::NoExpression => "\0",
        }
    }

    fn to_string(&self) -> String {
        match self {
            Expression::Identifier { value, .. } => value.clone(),
            Expression::IntegerLiteral { value, .. } => value.to_string(),
            Expression::BooleanLiteral { value, .. } => value.to_string(),
            Expression::Prefix {
                operator, right, ..
            } => sf!(format!("({}{})", operator, right.to_string())),
            Expression::Infix {
                left,
                operator,
                right,
                ..
            } => sf!(format!(
                "({} {} {})",
                left.to_string(),
                operator,
                right.to_string()
            )),
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
                ..
            } => {
                let mut if_part: String = sf!(format!(
                    "if {} {}",
                    condition.to_string(),
                    consequence.to_string()
                ));
                match *alternative.clone() {
                    Some(alt) => if_part.push_str(format!(" else {}", alt.to_string()).as_str()),
                    None => (),
                }
                if_part
            }
            Expression::FuncExpression {
                parameters, body, ..
            } => {
                let mut func = String::from("fn(");
                let len = parameters.len();
                parameters.iter().enumerate().for_each(|(i, param)| {
                    func.push_str(param.to_string().as_str());
                    if i != len - 1 {
                        func.push_str(", ");
                    }
                });
                func.push_str(") ");
                func.push_str(body.to_string().as_str());
                func
            }
            Expression::CallExpression { func, args, .. } => {
                let mut call = String::from(func.to_string() + "(");
                let len = args.len();
                args.iter().enumerate().for_each(|(i, arg)| {
                    call.push_str(arg.to_string().as_str());
                    if i != len - 1 {
                        call.push_str(", ");
                    }
                });
                call.push_str(")");
                call
            }
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
    BlockStatement {
        token: Token,
        statements: Vec<Statement>,
    },
}

impl Node for Statement {
    fn token_literal(&self) -> &str {
        match self {
            Statement::LetStatement { token, .. } => &token.literal,
            Statement::ReturnStatement { token, .. } => &token.literal,
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
            Statement::ExpressionStatement { expression, .. } => {
                format!("{}", expression.to_string())
            }
            Statement::BlockStatement { statements, .. } => {
                let mut blk_stmt = String::from("{ ");
                statements
                    .iter()
                    .for_each(|stmt| blk_stmt.push_str(stmt.to_string().as_str()));
                blk_stmt.push_str(" }");
                blk_stmt
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
            TokenType::BLOCK => Statement::BlockStatement {
                token: Token {
                    token_type: TokenType::BLOCK,
                    literal: sf!("block"),
                },
                statements: vec![],
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
            Statement::LetStatement { ref mut name, .. } => *name = ident,
            _ => (),
        }
    }

    pub fn set_value(&mut self, expr_value: Expression) {
        match self {
            Statement::LetStatement { ref mut value, .. } => *value = expr_value,
            Statement::ReturnStatement { ref mut value, .. } => *value = expr_value,
            _ => (),
        }
    }

    pub fn set_expression(&mut self, expr: Expression) {
        match self {
            Statement::ExpressionStatement {
                ref mut token,
                ref mut expression,
            } => {
                token.token_type = expr.get_token();
                *expression = expr
            }
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

    pub fn set_block_token(&mut self, blk_token: Token) {
        match self {
            Statement::BlockStatement { ref mut token, .. } => *token = blk_token,
            _ => (),
        }
    }

    pub fn add_block_stmt(&mut self, stmt: Statement) {
        match self {
            Statement::BlockStatement {
                ref mut statements, ..
            } => statements.push(stmt),
            _ => (),
        }
    }
}

pub struct Program<Statement> {
    pub statements: Vec<Statement>,
}
