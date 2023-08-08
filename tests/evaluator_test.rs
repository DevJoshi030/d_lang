use d_lang::{evaluator::eval_statements, lexer::Lexer, object::Object, parser::Parser};
use macros::sf;

#[test]
fn test_eval_int_expr() {
    let input: Vec<String> = vec![sf!("5"), sf!("10")];
    let results: Vec<i64> = vec![5, 10];

    results.iter().enumerate().for_each(|(i, r)| {
        let evaluated = test_eval(input.get(i).unwrap().clone());
        test_int_obj(evaluated, *r);
    })
}

#[test]
fn test_eval_bool_expr() {
    let input: Vec<String> = vec![sf!("true"), sf!("false")];
    let results: Vec<bool> = vec![true, false];

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

fn test_eval(input: String) -> Object {
    let l = Lexer::new(input.chars().collect());
    let mut p = Parser::new(l);
    let program = p.parse_program();

    println!("{:#?}", program.statements);

    eval_statements(program.statements)
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
