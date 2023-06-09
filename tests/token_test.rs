use d_lang::{
    lexer::Lexer,
    token::{Token, TokenType::*},
};
use macros::sf;

#[test]
fn test_next_token() {
    let input = "=+(){},;".chars().collect();

    let literals = vec!["=", "+", "(", ")", "{", "}", ",", ";", "\0"];
    let token_types = vec![
        ASSIGN, PLUS, LPAREN, RPAREN, LBRACE, RBRACE, COMMA, SEMICOLON, EOF,
    ];

    let test_tokens: Vec<Token> = token_types
        .iter()
        .zip(literals)
        .map(|(token_type, literal)| Token {
            token_type: *token_type,
            literal: sf!(literal),
        })
        .collect();

    let mut l = Lexer::new(input);

    test_tokens.iter().for_each(|test_token| {
        let input_token = l.next_token();
        if input_token != *test_token {
            panic!(
                "Error in token: expected: ({:#?}, {:#?}), got ({:#?}, {:#?})",
                test_token.token_type,
                test_token.literal,
                input_token.token_type,
                input_token.literal
            );
        }
    });
}
