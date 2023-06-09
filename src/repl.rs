use std::io::{self, Write};

use crate::{lexer::Lexer, token::TokenType};

const PROMT: &str = ">> ";

pub fn run() {
    loop {
        print!("{}", PROMT);
        let mut input = String::new();
        io::stdout().flush().expect("Failed to flush output!");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to parse input!");

        let mut l = Lexer::new(input.chars().collect());

        loop {
            let l = l.next_token();
            if l.token_type == TokenType::EOF {
                break;
            } else {
                println!("(Type:{:#?}, Literal: {})", l.token_type, l.literal);
            }
        }
    }
}
