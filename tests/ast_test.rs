use d_lang::{
    ast::{Expression, Identifier, Node, Program, Statement},
    token::{Token, TokenType},
};
use macros::sf;

// Testing string: let my_var = another_var;
#[test]
fn test_string() {
    let program = Program {
        statements: vec![Statement::LetStatement {
            token: Token {
                token_type: TokenType::LET,
                literal: sf!("let"),
            },
            name: Identifier {
                token: Token {
                    token_type: TokenType::IDENT,
                    literal: sf!("my_var"),
                },
                value: sf!("my_var"),
            },
            value: Expression {
                value: sf!("another_var"),
            },
        }],
    };

    if program.statements.get(0).unwrap().to_string() != "let my_var = another_var;" {
        panic!(
            "program.to_string() wrong. got={}",
            program.statements.get(0).unwrap().to_string()
        )
    }
}
