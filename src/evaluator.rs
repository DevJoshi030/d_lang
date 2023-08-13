use macros::sf;

use crate::{
    ast::{Expression, Statement},
    object::Object,
};

pub fn eval_statements(statements: Vec<Statement>, p_req: bool) -> Object {
    let mut result = Object::Null {};
    for stmt in statements {
        result = eval(stmt.clone());
        if p_req {
            println!("{:#?}", result);
        }
        match result {
            Object::Return { value } => return *value,
            Object::Error { .. } => return result,
            _ => (),
        }
    }
    result
}

pub fn eval(stmt: Statement) -> Object {
    match stmt {
        Statement::LetStatement { token, name, value } => todo!(),
        Statement::ReturnStatement { value, .. } => {
            let evaluated = eval_expr(value);
            if let Object::Error { .. } = evaluated {
                return evaluated;
            }
            Object::Return {
                value: Box::new(evaluated),
            }
        }
        Statement::ExpressionStatement { expression, .. } => eval_expr(expression),
        Statement::BlockStatement { statements, .. } => {
            let evaluated = eval_statements(statements, false);
            if let Object::Error { .. } = evaluated {
                return evaluated;
            }
            Object::Return {
                value: Box::new(evaluated),
            }
        }
    }
}

fn eval_expr(expr: Expression) -> Object {
    match expr {
        Expression::Identifier { token, value } => todo!(),
        Expression::IntegerLiteral { value, .. } => Object::Integer {
            value: value.into(),
        },
        Expression::BooleanLiteral { value, .. } => Object::get_bool_obj(value),
        Expression::Prefix {
            operator, right, ..
        } => {
            let evaluated = eval_expr(*right);
            if let Object::Error { .. } = evaluated {
                return evaluated;
            }
            eval_prefix_expr(operator, evaluated)
        }
        Expression::Infix {
            left,
            operator,
            right,
            ..
        } => {
            let evaluated_left = eval_expr(*left);
            if let Object::Error { .. } = evaluated_left {
                return evaluated_left;
            }
            let evaluated_right = eval_expr(*right);
            if let Object::Error { .. } = evaluated_right {
                return evaluated_right;
            }
            eval_infix_expr(evaluated_left, operator, evaluated_right)
        }
        Expression::IfExpression {
            condition,
            consequence,
            alternative,
            ..
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
        _ => Object::Error {
            message: sf!(format!(
                "unknown operator: {}{}",
                operator,
                right.get_type()
            )),
        },
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
        _ => Object::Error {
            message: sf!(format!("unknown operator: {}{}", "-", right.get_type())),
        },
    }
}

fn eval_infix_expr(left: Object, operator: String, right: Object) -> Object {
    return match left {
        Object::Integer { value: left_val } => {
            if let Object::Integer { value: right_val } = right {
                let evaluated = eval_int_infix_expr(left_val, operator.clone(), right_val);
                if let Object::Null {} = evaluated {
                    return Object::Error {
                        message: sf!(format!(
                            "unknown operator: {} {} {}",
                            left.get_type(),
                            operator,
                            right.get_type()
                        )),
                    };
                }
                return evaluated;
            }
            Object::Error {
                message: sf!(format!(
                    "type mismatch: {} {} {}",
                    left.get_type(),
                    operator,
                    right.get_type()
                )),
            }
        }
        Object::Boolean { value: left_val } => {
            if let Object::Boolean { value: right_val } = right {
                let evaluated = eval_bool_infix_expr(left_val, operator.clone(), right_val);
                if let Object::Null {} = evaluated {
                    return Object::Error {
                        message: sf!(format!(
                            "unknown operator: {} {} {}",
                            left.get_type(),
                            operator,
                            right.get_type()
                        )),
                    };
                }
                return evaluated;
            }
            Object::Error {
                message: sf!(format!(
                    "type mismatch: {} {} {}",
                    left.get_type(),
                    operator,
                    right.get_type()
                )),
            }
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
    if let Object::Error { .. } = cond {
        return cond;
    }
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
