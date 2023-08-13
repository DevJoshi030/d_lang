use d_lang::{
    ast::Node, environment::Environment, evaluator::eval_statements, lexer::Lexer, object::Object,
    parser::Parser,
};
use macros::sf;

#[test]
fn test_eval_int_expr() {
    let input: Vec<String> = vec![
        sf!("5"),
        sf!("10"),
        sf!("-5"),
        sf!("-10"),
        sf!("5 + 5 + 5 + 5 - 10"),
        sf!("2 * 2 * 2 * 2 * 2"),
        sf!("-50 + 100 + -50"),
        sf!("5 * 2 + 10"),
        sf!("5 + 2 * 10"),
        sf!("20 + 2 * -10"),
        sf!("50 / 2 * 2 + 10"),
        sf!("2 * (5 + 10)"),
        sf!("3 * 3 * 3 + 10"),
        sf!("3 * (3 * 3) + 10"),
        sf!("(5 + 10 * 2 + 15 / 3) * 2 + -10"),
    ];
    let results: Vec<i64> = vec![5, 10, -5, -10, 10, 32, 0, 20, 25, 0, 60, 30, 37, 37, 50];

    results.iter().enumerate().for_each(|(i, r)| {
        let evaluated = test_eval(input.get(i).unwrap().clone());
        test_int_obj(evaluated, *r);
    })
}

#[test]
fn test_eval_bool_expr() {
    let input: Vec<String> = vec![
        sf!("true"),
        sf!("false"),
        sf!("1 < 2"),
        sf!("1 > 2"),
        sf!("1 < 1"),
        sf!("1 > 1"),
        sf!("1 == 1"),
        sf!("1 != 1"),
        sf!("1 == 2"),
        sf!("1 != 2"),
        sf!("true == true"),
        sf!("false == false"),
        sf!("true == false"),
        sf!("true != false"),
        sf!("false != true"),
        sf!("(1 < 2) == true"),
        sf!("(1 < 2) == false"),
        sf!("(1 > 2) == true"),
        sf!("(1 > 2) == false"),
    ];
    let results: Vec<bool> = vec![
        true, false, true, false, false, false, true, false, false, true, true, true, false, true,
        true, true, false, false, true,
    ];

    results.iter().enumerate().for_each(|(i, r)| {
        let evaluated = test_eval(input.get(i).unwrap().clone());
        test_bool_obj(evaluated, *r);
    })
}

#[test]
fn test_bang_oper() {
    let input: Vec<String> = vec![
        sf!("!true"),
        sf!("!false"),
        sf!("!5"),
        sf!("!!true"),
        sf!("!!false"),
        sf!("!!5"),
    ];
    let results: Vec<bool> = vec![false, true, false, true, false, true];

    results.iter().enumerate().for_each(|(i, r)| {
        let evaluated = test_eval(input.get(i).unwrap().clone());
        test_bool_obj(evaluated, *r);
    })
}

#[test]
fn test_if_else_expr() {
    let input: Vec<String> = vec![
        sf!("if (true) { 10 }"),
        sf!("if (false) { 10 }"),
        sf!("if (1) { 10 }"),
        sf!("if (1 < 2) { 10 }"),
        sf!("if (1 > 2) { 10 }"),
        sf!("if (1 > 2) { 10 } else { 20 }"),
        sf!("if (1 < 2) { 10 } else { 20 }"),
    ];
    let results: Vec<Option<i64>> =
        vec![Some(10), None, Some(10), Some(10), None, Some(20), Some(10)];

    results.iter().enumerate().for_each(|(i, r)| {
        let evaluated = test_eval(input.get(i).unwrap().clone());
        test_opt_int_obj(evaluated, *r);
    })
}

#[test]
fn test_return_expr() {
    let input: Vec<String> = vec![
        sf!("return 10;"),
        sf!("return 10;9;"),
        sf!("return 2 * 5; 9;"),
        sf!("9; return 2 * 5; 9;"),
        sf!("if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }
                return 1;
            }
        "),
    ];
    let results: Vec<i64> = vec![10, 10, 10, 10, 10];

    results.iter().enumerate().for_each(|(i, r)| {
        let evaluated = test_eval(input.get(i).unwrap().clone());
        test_int_obj(evaluated, *r);
    });
}

#[test]
fn test_errors() {
    let input: Vec<String> = vec![
        sf!("5 + true;"),
        sf!("5 + true; 5;"),
        sf!("-true;"),
        sf!("false + true;"),
        sf!("5; false + true; 5;"),
        sf!("if (10 > 1) { true + false; }"),
        sf!("if (10 > 1) {
                if (10 > 1) {
                    return true + false;
                }
                return 1;
            }"),
        sf!("foobar;"),
    ];
    let results: Vec<&str> = vec![
        "type mismatch: INTEGER + BOOLEAN",
        "type mismatch: INTEGER + BOOLEAN",
        "unknown operator: -BOOLEAN",
        "unknown operator: BOOLEAN + BOOLEAN",
        "unknown operator: BOOLEAN + BOOLEAN",
        "unknown operator: BOOLEAN + BOOLEAN",
        "unknown operator: BOOLEAN + BOOLEAN",
        "identifier not found: foobar",
    ];

    results.iter().enumerate().for_each(|(i, r)| {
        let evaluated = test_eval(input.get(i).unwrap().clone());
        if let Object::Error { message } = evaluated {
            if *r != message {
                panic!("wrong error message. expected={}, got={}", *r, message);
            }
        } else {
            panic!("No error object returned. got={:#?}", evaluated);
        }
    });
}

#[test]
fn test_let_statements() {
    let input: Vec<String> = vec![
        sf!("let a = 5; a;"),
        sf!("let a = 5 * 5; a;"),
        sf!("let a = 5; let b = a; b;"),
        sf!("let a = 5; let b = a; let c = a + b + 5; c;"),
    ];
    let results: Vec<i64> = vec![5, 25, 5, 15];

    results.iter().enumerate().for_each(|(i, r)| {
        let evaluated = test_eval(input.get(i).unwrap().clone());
        test_int_obj(evaluated, *r);
    });
}

#[test]
fn test_functions() {
    let input: Vec<String> = vec![sf!("fn(x) { x + 2; };")];

    input.iter().for_each(|inp| {
        let evaluated = test_eval(inp.clone());
        let (parameters, ..) = match evaluated {
            Object::Function {
                parameters, body, ..
            } => (parameters, body),
            _ => panic!("object is not Function. got={:#?}", evaluated),
        };

        if parameters.len() != 1 {
            panic!(
                "function has wrong parameters. Parameters={:#?}",
                parameters
            );
        }

        if parameters[0].to_string() != "x" {
            panic!("parameter is not 'x'. got={:#?}", parameters[0].to_string());
        }
    });
}

#[test]
fn test_function_calls() {
    let input: Vec<String> = vec![
        sf!("let identity = fn(x) { x; }; identity(5);"),
        sf!("let identity = fn(x) { return x; }; identity(5);"),
        sf!("let double = fn(x) { x * 2; }; double(5);"),
        sf!("let add = fn(x, y) { x + y; }; add(5, 5);"),
        sf!("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));"),
        sf!("fn(x) { x; }(5)"),
        sf!("let double = fn(x) { x + x; };
        let triple = fn(x, y) { x + y(x);};
        triple(2, double)"),
    ];
    let results: Vec<i64> = vec![5, 5, 10, 10, 20, 5, 6];

    results.iter().enumerate().for_each(|(i, r)| {
        let evaluated = test_eval(input.get(i).unwrap().clone());
        test_int_obj(evaluated, *r);
    });
}

fn test_eval(input: String) -> Object {
    let l = Lexer::new(input.chars().collect());
    let mut p = Parser::new(l);
    let program = p.parse_program();
    let mut env = Environment::new();

    // println!("{:#?}", program.statements);

    eval_statements(program.statements, &mut env, true)
}

fn test_int_obj(eval: Object, r: i64) {
    let eval_value;
    match eval {
        Object::Integer { value } => eval_value = value,
        _ => panic!("object is not integer, got={}", eval.get_type()),
    }

    if eval_value != r {
        panic!("value is not {}, got={}", r, eval_value);
    }
}

fn test_bool_obj(eval: Object, r: bool) {
    let eval_value;
    match eval {
        Object::Boolean { value } => eval_value = value,
        _ => panic!("object is not boolean, got={}", eval.get_type()),
    }

    if eval_value != r {
        panic!("value is not {}, got={}", r, eval_value);
    }
}

fn test_opt_int_obj(eval: Object, r: Option<i64>) {
    match r {
        Some(r_val) => {
            let eval_value;
            match eval {
                Object::Integer { value } => eval_value = value,
                _ => panic!("object is not integer, got={}", eval.get_type()),
            }
            if eval_value != r_val {
                panic!("value is not {}, got={}", r_val, eval_value);
            }
        }
        None => match eval {
            Object::Null {} => {}
            _ => panic!("object is not NULL, got={}", eval.get_type()),
        },
    }
}
