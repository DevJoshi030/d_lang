use d_lang::{
    lexer::Lexer,
    token::{Token, TokenType::*},
};
use macros::string_from;

#[test]
fn test_next_token() {
    let input = "let five = 5;
    let ten = 10;
    let add = fn(x, y) {
    x + y;
    };
    let result = add(five, ten);"
        .chars()
        .collect();

    let literals = vec![
        "let", "five", "=", "5", ";", "let", "ten", "=", "10", ";", "let", "add", "=", "fn", "(",
        "x", ",", "y", ")", "{", "x", "+", "y", ";", "}", ";", "let", "result", "=", "add", "(",
        "five", ",", "ten", ")", ";", "\0",
    ];
    let token_types = vec![
        LET, IDENT, ASSIGN, INT, SEMICOLON, LET, IDENT, ASSIGN, INT, SEMICOLON, LET, IDENT, ASSIGN,
        FUNCTION, LPAREN, IDENT, COMMA, IDENT, RPAREN, LBRACE, IDENT, PLUS, IDENT, SEMICOLON,
        RBRACE, SEMICOLON, LET, IDENT, ASSIGN, IDENT, LPAREN, IDENT, COMMA, IDENT, RPAREN,
        SEMICOLON, EOF,
    ];

    let test_tokens: Vec<Token> = token_types
        .iter()
        .zip(literals)
        .map(|(token_type, literal)| Token {
            token_type: *token_type,
            literal: string_from!(literal),
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
