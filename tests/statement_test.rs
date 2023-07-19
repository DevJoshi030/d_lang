use d_lang::{
    ast::{Node, Statement},
    lexer::Lexer,
    parser::Parser,
};

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
