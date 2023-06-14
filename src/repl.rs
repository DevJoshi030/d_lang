use std::io::{self, Write};

use crate::{lexer::Lexer, parser::Parser};

const PROMT: &str = ">>> ";

pub fn run() {
    loop {
        print!("{}", PROMT);
        let mut input = String::new();
        io::stdout().flush().expect("Failed to flush output!");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to parse input!");

        let l = Lexer::new(input.chars().collect());
        let mut p = Parser::new(l);

        let program = p.parse_program();
        p.check_parse_errors();

        for statement in program.statements {
            println!("{:#?}", statement);
        }
    }
}
