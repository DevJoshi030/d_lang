use std::{
    fs,
    io::{self, Write},
};

use crate::{environment::Environment, evaluator::eval_statements, lexer::Lexer, parser::Parser};

const PROMT: &str = ">>> ";

pub fn lpe(input: String, env: &mut Environment) {
    let l = Lexer::new(input.chars().collect());
    let mut p = Parser::new(l);

    let program = p.parse_program();
    p.check_parse_errors();

    eval_statements(program.statements, env, true);
}

pub fn run() {
    let mut env = Environment::new();
    loop {
        print!("{}", PROMT);
        let mut input = String::new();
        io::stdout().flush().expect("Failed to flush output!");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to parse input!");
        lpe(input, &mut env);
    }
}

pub fn run_file(filename: String) {
    let mut env = Environment::new();
    let contents = fs::read_to_string(filename.clone())
        .unwrap_or_else(|_| panic!("No such file `{}`", filename));
    lpe(contents, &mut env);
}
