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
        } => eval_prefix_expr(operator, eval_expr(*right)),
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

fn eval_prefix_expr(operator: String, right: Object) -> Object {
    match operator.as_str() {
        "!" => eval_bang_operator_expr(right),
        "-" => eval_minus_operator_expr(right),
        _ => Object::Null {},
    }
}

fn eval_bang_operator_expr(right: Object) -> Object {
    match right {
        Object::Boolean { value } => Object::Boolean { value: !value },
        _ => Object::Boolean { value: false },
    }
}

fn eval_minus_operator_expr(right: Object) -> Object {
    match right {
        Object::Integer { value } => Object::Integer { value: -value },
        _ => Object::Null {},
    }
}
