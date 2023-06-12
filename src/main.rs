mod ast;
pub mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    println!("d_lang!!!");
    repl::run();
}
