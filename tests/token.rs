use d_lang::{
    lexer::Lexer,
    token::{self, Token},
};

#[test]
fn test_next_token() {
    let input = "=+(){},;".chars().collect();

    let tests = vec![
        Token {
            token_type: token::TokenType::ASSIGN,
            literal: '=',
        },
        Token {
            token_type: token::TokenType::PLUS,
            literal: '+',
        },
        Token {
            token_type: token::TokenType::LPAREN,
            literal: '(',
        },
        Token {
            token_type: token::TokenType::RPAREN,
            literal: ')',
        },
        Token {
            token_type: token::TokenType::LBRACE,
            literal: '{',
        },
        Token {
            token_type: token::TokenType::RBRACE,
            literal: '}',
        },
        Token {
            token_type: token::TokenType::COMMA,
            literal: ',',
        },
        Token {
            token_type: token::TokenType::SEMICOLON,
            literal: ';',
        },
        Token {
            token_type: token::TokenType::EOF,
            literal: '\0',
        },
    ];

    let mut l = Lexer::new(input);

    tests.iter().for_each(|tt| {
        let tok = l.next_token();
        if tok != *tt {
            panic!(
                "Error in token: expected: {}, got {}",
                tt.token_type.value(),
                tok.token_type.value()
            );
        }
    });
}
