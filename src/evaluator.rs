use crate::{
    ast::{Expression, Statement},
    object::Object,
};

pub fn eval_statements(statements: Vec<Statement>, p_req: bool) -> Object {
    let mut result = Object::Null {};
    statements.iter().for_each(|stmt| {
        result = eval(stmt.clone());
        if p_req {
            println!("{:#?}", result);
        }
    });
    result
}

pub fn eval(stmt: Statement) -> Object {
    match stmt {
        Statement::LetStatement { token, name, value } => todo!(),
        Statement::ReturnStatement { token, value } => todo!(),
        Statement::ExpressionStatement {
            token: _,
            expression,
        } => eval_expr(expression.clone()),
        Statement::BlockStatement {
            token: _,
            statements,
        } => eval_statements(statements, false),
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
            token: _,
            operator,
            right,
        } => eval_prefix_expr(operator, eval_expr(*right)),
        Expression::Infix {
            token: _,
            left,
            operator,
            right,
        } => eval_infix_expr(eval_expr(*left), operator, eval_expr(*right)),
        Expression::IfExpression {
            token: _,
            condition,
            consequence,
            alternative,
        } => eval_if_expr(condition, consequence, alternative),
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

fn eval_infix_expr(left: Object, operator: String, right: Object) -> Object {
    return match left {
        Object::Integer { value: left_val } => {
            if let Object::Integer { value: right_val } = right {
                return eval_int_infix_expr(left_val, operator, right_val);
            }
            Object::Null {}
        }
        Object::Boolean { value: left_val } => {
            if let Object::Boolean { value: right_val } = right {
                return eval_bool_infix_expr(left_val, operator, right_val);
            }
            Object::Null {}
        }
        _ => Object::Null {},
    };
}

fn eval_int_infix_expr(left: i64, operator: String, right: i64) -> Object {
    match operator.as_str() {
        "+" => Object::Integer {
            value: left + right,
        },
        "-" => Object::Integer {
            value: left - right,
        },
        "*" => Object::Integer {
            value: left * right,
        },
        "/" => Object::Integer {
            value: left / right,
        },
        "<" => Object::Boolean {
            value: left < right,
        },
        ">" => Object::Boolean {
            value: left > right,
        },
        "==" => Object::Boolean {
            value: left == right,
        },
        "!=" => Object::Boolean {
            value: left != right,
        },
        _ => Object::Null {},
    }
}

fn eval_bool_infix_expr(left: bool, operator: String, right: bool) -> Object {
    match operator.as_str() {
        "==" => Object::Boolean {
            value: left == right,
        },
        "!=" => Object::Boolean {
            value: left != right,
        },
        _ => Object::Null {},
    }
}

fn eval_if_expr(
    condition: Box<Expression>,
    consequence: Box<Statement>,
    alternative: Box<Option<Statement>>,
) -> Object {
    let cond = eval_expr(*condition);
    let cond_val: bool = match cond {
        Object::Boolean { value } => value,
        Object::Null {} => false,
        _ => true,
    };

    if cond_val {
        eval(*consequence)
    } else if let Some(stmt) = *alternative {
        eval(stmt)
    } else {
        Object::Null {}
    }
}
