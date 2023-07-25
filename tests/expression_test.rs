use d_lang::{
    ast::{Node, Statement},
    lexer::Lexer,
    parser::Parser,
};
use macros::sf;

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
    let results: Vec<String> = vec![sf!("(!5)"), sf!("(-14)")];

    results.iter().enumerate().for_each(|(idx, result)| {
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

        if expr.to_string() != *result {
            panic!("expr is not {}, got={}", result, expr.to_string());
        }
    })
}

#[test]
fn test_infix_expressions() {
    let input: Vec<String> = vec![
        sf!("1 + 2 + 3;"),
        sf!("3 - 4;"),
        sf!("5 * 6;"),
        sf!("7 / 8;"),
        sf!("9 > 10;"),
        sf!("11 < 12;"),
        sf!("13 == 14;"),
        sf!("15 != 16;"),
    ];

    let results: Vec<String> = vec![
        sf!("((1 + 2) + 3)"),
        sf!("(3 - 4)"),
        sf!("(5 * 6)"),
        sf!("(7 / 8)"),
        sf!("(9 > 10)"),
        sf!("(11 < 12)"),
        sf!("(13 == 14)"),
        sf!("(15 != 16)"),
    ];

    results.iter().enumerate().for_each(|(idx, result)| {
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

        if expr.to_string() != *result {
            panic!("expr is not {}, got={}", result, expr.to_string());
        }
    })
}

#[test]
fn test_if_expression() {
    let input: Vec<char> = "
    if x < y {
        a
    } else if x > y {
        b
    } else {
        c
    };"
    .chars()
    .collect();

    let result = "if (x < y) { a } else if (x > y) { b } else { c }";

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

    if expr.to_string() != result {
        panic!(
            "expr is not correct expected={}, got={}",
            result,
            expr.to_string()
        );
    }
}

#[test]
fn test_func_expression() {
    let input: Vec<char> = "
    fn(x, y) {
        if x < y {
            a
        } else if x > y {
            b
        } else {
            c
        }
    };"
    .chars()
    .collect();

    let result = "fn(x, y) { if (x < y) { a } else if (x > y) { b } else { c } }";

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

    if expr.to_string() != result {
        panic!(
            "expr is not correct expected={}, got={}",
            result,
            expr.to_string()
        );
    }
}

#[test]
fn test_call_expression() {
    let input: Vec<char> = "add(1, 2 * 3, 4 + 5);".chars().collect();

    let result = "add(1, (2 * 3), (4 + 5))";

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

    if expr.to_string() != result {
        panic!(
            "expr is not correct expected={}, got={}",
            result,
            expr.to_string()
        );
    }
}
