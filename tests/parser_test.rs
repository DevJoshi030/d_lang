use d_lang::{
    ast::{LetStatement, Node, Program, Statement},
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
    let p = Parser::new(l);

    // let program  = p.parse_program();

    let program: Program<Statement> = Program { statements: vec![] };

    // if program == () {
    //     panic!("Failed to parse program!!!");
    // }

    if program.statements.len() > 3 {
        panic!(
            "program.statements does not contain 3 statements. got {}",
            program.statements.len()
        );
    }

    let tests = vec!["x", "y", "foobar"];

    tests.iter().enumerate().for_each(|(index, exp_ident)| {
        if !test_let_statement(
            *program.statements.get(index).expect("No statement found!"),
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

    let let_statement = LetStatement::new();

    if let_statement.name.value != name {
        panic!("name not {}, got={}", let_statement.name.value, name);
    }

    true
}
