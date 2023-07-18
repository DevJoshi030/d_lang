use d_lang::{
    ast::{Expression, Node, Statement},
    lexer::Lexer,
    parser::Parser,
};
use macros::sf;

#[test]
fn test_let_statements() {
    let input: Vec<char> = "let x = 5;
    let y = 10;
    let foobar = 838383;"
        .chars()
        .collect();

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    p.check_parse_errors();

    if program.statements.len() > 3 {
        panic!(
            "program.statements does not contain 3 statements. got {}",
            program.statements.len()
        );
    }

    let tests = vec!["x", "y", "foobar"];

    tests.iter().enumerate().for_each(|(index, exp_ident)| {
        if !test_let_statement(
            program
                .statements
                .get(index)
                .expect("No statement found!")
                .clone(),
            exp_ident.to_string(),
        ) {
            return;
        }
    });
}

fn test_let_statement(stmt: Statement, name: String) -> bool {
    if stmt.token_literal() != "let" {
        panic!("literal is not 'let', got={}", stmt.token_literal());
    }

    let stmt_name = match stmt {
        Statement::LetStatement {
            token: _,
            name,
            value: _,
        } => name.clone(),
        _ => panic!("Statement is not LET"),
    };

    if stmt_name.to_string() != name {
        panic!("name not {}, got={}", stmt_name.to_string(), name);
    }

    true
}

#[test]
fn test_return_statements() {
    let input: Vec<char> = "return 5;
    return 10;
    return 993322;"
        .chars()
        .collect();

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    p.check_parse_errors();

    if program.statements.len() > 3 {
        panic!(
            "program.statements does not contain 3 statements. got {}",
            program.statements.len()
        );
    }

    for stmt in program.statements.iter() {
        if stmt.token_literal() != "return" {
            panic!("literal is not 'return', got={}", stmt.token_literal());
        }
    }
}

#[test]
fn test_identifier_expression() {
    let input: Vec<char> = "foobar;".chars().collect();

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    p.check_parse_errors();

    if program.statements.len() != 1 {
        panic!(
            "program.statements does not have enough statements. got {}",
            program.statements.len()
        );
    }

    let stmt = program.statements.get(0).unwrap();

    let expr = match stmt {
        Statement::ExpressionStatement {
            token: _,
            expression,
        } => expression.clone(),
        _ => panic!("Statement is not EXPRESSION"),
    };

    if expr.to_string() != "foobar" {
        panic!("expr is not foobar, got={}", expr.to_string());
    }
}

#[test]
fn test_integer_literal_expression() {
    let input: Vec<char> = "5;".chars().collect();

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    p.check_parse_errors();

    if program.statements.len() != 1 {
        panic!(
            "program.statements does not have enough statements. got {}",
            program.statements.len()
        );
    }

    let stmt = program.statements.get(0).unwrap();

    let expr = match stmt {
        Statement::ExpressionStatement {
            token: _,
            expression,
        } => expression.clone(),
        _ => panic!("Statement is not EXPRESSION"),
    };

    if expr.to_string() != "5" {
        panic!("expr is not 5, got={}", expr.to_string());
    }
}

#[test]
fn test_prefix_expressions() {
    let input: Vec<String> = vec![sf!("!5;"), sf!("-14;")];

    let operator: Vec<String> = vec![sf!("!"), sf!("-")];
    let integer_value = vec![5, 14];

    operator
        .iter()
        .zip(integer_value)
        .enumerate()
        .for_each(|(idx, (op, int_val))| {
            let l = Lexer::new(input.get(idx).unwrap().chars().collect());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            p.check_parse_errors();

            if program.statements.len() != 1 {
                panic!(
                    "program.statements does not have enough statements. got {}",
                    program.statements.len()
                );
            }

            let stmt = program.statements.get(0).unwrap();

            let expr = match stmt {
                Statement::ExpressionStatement {
                    token: _,
                    expression,
                } => expression.clone(),
                _ => panic!("Statement is not EXPRESSION"),
            };

            let oper = match expr {
                Expression::Prefix {
                    token: _,
                    ref operator,
                    right: _,
                } => operator.clone(),
                _ => sf!("\0"),
            };

            if oper.as_str() != op {
                panic!("expr is not {}, got={}", op, oper);
            }

            if !test_integer_literal(&expr, int_val, 0, op.clone()) {
                return;
            }
        })
}

#[test]
fn test_infix_expressions() {
    let input: Vec<String> = vec![
        sf!("1 + 2;"),
        sf!("3 - 4;"),
        sf!("5 * 6;"),
        sf!("7 / 8;"),
        sf!("9 > 10;"),
        sf!("11 < 12;"),
        sf!("13 == 14;"),
        sf!("15 != 16;"),
    ];

    let lefts = vec![1, 3, 5, 7, 9, 11, 13, 15];
    let operators: Vec<String> = vec![
        sf!("+"),
        sf!("-"),
        sf!("*"),
        sf!("/"),
        sf!(">"),
        sf!("<"),
        sf!("=="),
        sf!("!="),
    ];
    let rights = vec![2, 4, 6, 8, 10, 12, 14, 16];

    lefts
        .iter()
        .zip(operators)
        .zip(rights)
        .enumerate()
        .for_each(|(idx, ((left, op), right))| {
            let l = Lexer::new(input.get(idx).unwrap().chars().collect());
            let mut p = Parser::new(l);
            let program = p.parse_program();
            p.check_parse_errors();

            if program.statements.len() != 1 {
                panic!(
                    "program.statements does not have enough statements. got {}",
                    program.statements.len()
                );
            }

            let stmt = program.statements.get(0).unwrap();

            let expr = match stmt {
                Statement::ExpressionStatement {
                    token: _,
                    expression,
                } => expression.clone(),
                _ => panic!("Statement is not EXPRESSION"),
            };

            let oper = match expr {
                Expression::Infix {
                    token: _,
                    left: _,
                    ref operator,
                    right: _,
                } => operator.clone(),
                _ => sf!("\0"),
            };

            if oper.as_str() != op {
                panic!("expr is not {}, got={}", op, oper);
            }

            if !test_integer_literal(&expr, right, *left, op) {
                return;
            }
        })
}

fn test_integer_literal(expr: &Expression, right: i32, left: i32, op: String) -> bool {
    match expr {
        Expression::Prefix {
            token: _,
            operator: _,
            right: _,
        } => test_val(format!("({}{})", op, right), expr.to_string()),
        Expression::Infix {
            token: _,
            left: _,
            operator: _,
            right: _,
        } => test_val(format!("({} {} {})", left, op, right), expr.to_string()),
        _ => (),
    }

    if expr.token_literal() != op {
        panic!("token literal is not {}, got={}", op, expr.token_literal());
    }

    true
}

fn test_val(first: String, sec: String) {
    if first.ne(&sec) {
        panic!("value is not {}, got={}", first, sec);
    }
}
