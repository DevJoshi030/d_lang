use d_lang::{
    lexer::Lexer,
    token::{self, Token},
};
use macros::string_from;

#[test]
fn test_next_token() {
    let input = "=+(){},;".chars().collect();

    let tests = vec![
        Token {
            token_type: token::TokenType::ASSIGN,
            literal: string_from!("="),
        },
        Token {
            token_type: token::TokenType::PLUS,
            literal: string_from!("+"),
        },
        Token {
            token_type: token::TokenType::LPAREN,
            literal: string_from!("("),
        },
        Token {
            token_type: token::TokenType::RPAREN,
            literal: string_from!(")"),
        },
        Token {
            token_type: token::TokenType::LBRACE,
            literal: string_from!("{"),
        },
        Token {
            token_type: token::TokenType::RBRACE,
            literal: string_from!("}"),
        },
        Token {
            token_type: token::TokenType::COMMA,
            literal: string_from!(","),
        },
        Token {
            token_type: token::TokenType::SEMICOLON,
            literal: string_from!(";"),
        },
        Token {
            token_type: token::TokenType::EOF,
            literal: string_from!("\0"),
        },
    ];

    let mut l = Lexer::new(input);

    tests.iter().for_each(|test_token| {
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
