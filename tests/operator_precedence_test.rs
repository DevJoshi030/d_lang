use d_lang::{
    ast::{Node, Statement},
    lexer::Lexer,
    parser::Parser,
};
use macros::sf;

#[test]
fn test_operator_precedence() {
    let input: Vec<String> = vec![
        sf!("true"),
        sf!("false"),
        sf!("3 > 5 == false"),
        sf!("3 < 5 == true"),
        sf!("1 + (2 + 3) + 4"),
        sf!("(5 + 5) * 2"),
        sf!("2 / (5 + 5)"),
        sf!("-(5 + 5)"),
        sf!("!(true == true)"),
    ];

    let results: Vec<String> = vec![
        sf!("true"),
        sf!("false"),
        sf!("((3 > 5) == false)"),
        sf!("((3 < 5) == true)"),
        sf!("((1 + (2 + 3)) + 4)"),
        sf!("((5 + 5) * 2)"),
        sf!("(2 / (5 + 5))"),
        sf!("(-(5 + 5))"),
        sf!("(!(true == true))"),
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
            Statement::ExpressionStatement { expression, .. } => expression.clone(),
            _ => panic!("Statement is not EXPRESSION"),
        };

        if expr.to_string() != *result {
            panic!("expr is not {}, got={}", result, expr.to_string());
        }
    })
}
