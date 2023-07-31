use crate::{
    ast::{Expression, Statement},
    object::Object,
};

pub fn eval_statements(statements: Vec<Statement>) -> Object {
    let mut result = Object::Null {};
    statements
        .iter()
        .for_each(|stmt| result = eval(stmt.clone()));
    result
}

fn eval(stmt: Statement) -> Object {
    match stmt {
        Statement::LetStatement { token, name, value } => todo!(),
        Statement::ReturnStatement { token, value } => todo!(),
        Statement::ExpressionStatement {
            token: _,
            expression,
        } => eval_expr(expression.clone()),
        Statement::BlockStatement { token, statements } => todo!(),
    }
}

fn eval_expr(expr: Expression) -> Object {
    match expr {
        Expression::Identifier { token, value } => todo!(),
        Expression::IntegerLiteral { token: _, value } => Object::Integer {
            value: value.into(),
        },
        Expression::BooleanLiteral { token: _, value } => Object::get_bool_obj(value),
        Expression::Prefix {
            token,
            operator,
            right,
        } => todo!(),
        Expression::Infix {
            token,
            left,
            operator,
            right,
        } => todo!(),
        Expression::IfExpression {
            token,
            condition,
            consequence,
            alternative,
        } => todo!(),
        Expression::FuncExpression {
            token,
            parameters,
            body,
        } => todo!(),
        Expression::CallExpression { token, func, args } => todo!(),
        Expression::NoExpression => todo!(),
    }
}
