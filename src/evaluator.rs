use macros::sf;

use crate::{
    ast::{Expression, Node, Statement},
    environment::Environment,
    object::Object,
};

pub fn eval_statements(statements: Vec<Statement>, env: &mut Environment, p_req: bool) -> Object {
    let mut result = Object::Null {};
    for stmt in statements {
        result = eval(stmt.clone(), env);
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

pub fn eval(stmt: Statement, env: &mut Environment) -> Object {
    match stmt {
        Statement::LetStatement { name, value, .. } => {
            let evaluated = eval_expr(value, env);
            let mut final_eval = evaluated.clone();
            if let Object::Error { .. } = evaluated {
                return evaluated;
            }
            if let Object::Return { value } = evaluated.clone() {
                final_eval = *value;
            }
            if let Expression::Identifier {
                value: key_name, ..
            } = name
            {
                env.set(key_name, final_eval);
            }
            return evaluated;
        }
        Statement::ReturnStatement { value, .. } => {
            let evaluated = eval_expr(value, env);
            if let Object::Error { .. } = evaluated {
                return evaluated;
            }
            Object::Return {
                value: Box::new(evaluated),
            }
        }
        Statement::ExpressionStatement { expression, .. } => eval_expr(expression, env),
        Statement::BlockStatement { statements, .. } => {
            let evaluated = eval_statements(statements, env, false);
            if let Object::Error { .. } = evaluated {
                return evaluated;
            }
            Object::Return {
                value: Box::new(evaluated),
            }
        }
    }
}

fn eval_expr(expr: Expression, env: &mut Environment) -> Object {
    match expr {
        Expression::Identifier { value, .. } => {
            let ident = env.get(value.clone());
            if let Some(obj) = ident {
                return obj.clone();
            }
            Object::Error {
                message: sf!(format!("identifier not found: {}", value)),
            }
        }
        Expression::IntegerLiteral { value, .. } => Object::Integer {
            value: value.into(),
        },
        Expression::BooleanLiteral { value, .. } => Object::get_bool_obj(value),
        Expression::Prefix {
            operator, right, ..
        } => {
            let evaluated = eval_expr(*right, env);
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
            let evaluated_left = eval_expr(*left, env);
            if let Object::Error { .. } = evaluated_left {
                return evaluated_left;
            }
            let evaluated_right = eval_expr(*right, env);
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
        } => eval_if_expr(condition, consequence, alternative, env),
        Expression::FuncExpression {
            parameters, body, ..
        } => Object::Function { parameters, body },
        Expression::CallExpression { func, args, .. } => {
            let function = eval_expr(*func, env);
            if let Object::Error { .. } = function {
                return function;
            }
            let args = eval_args(args, env);
            if args.len() == 1 {
                if let Object::Error { .. } = args[0] {
                    return args[0].clone();
                }
            }
            apply_function(function, args, env)
        }
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
    env: &mut Environment,
) -> Object {
    let cond = eval_expr(*condition, env);
    if let Object::Error { .. } = cond {
        return cond;
    }
    let cond_val: bool = match cond {
        Object::Boolean { value } => value,
        Object::Null {} => false,
        _ => true,
    };

    if cond_val {
        eval(*consequence, env)
    } else if let Some(stmt) = *alternative {
        eval(stmt, env)
    } else {
        Object::Null {}
    }
}

fn eval_args(args: Vec<Expression>, env: &mut Environment) -> Vec<Object> {
    let mut results = vec![];
    for arg in args {
        let evaluated = eval_expr(arg, env);
        if let Object::Error { .. } = evaluated {
            return vec![evaluated];
        }
        results.push(evaluated);
    }
    results
}

fn apply_function(function: Object, args: Vec<Object>, env: &mut Environment) -> Object {
    if let Object::Function { parameters, body } = function {
        parameters
            .iter()
            .zip(args)
            .for_each(|(p, a)| env.set(p.to_string(), a));

        let evaluated = eval(*body, env);
        if let Object::Return { value } = evaluated {
            return *value;
        }
        evaluated
    } else {
        panic!("not a function: {}", function.get_type());
    }
}
