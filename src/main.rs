use std::env;
use crate::program::Program;

pub mod program;
pub mod parser;
pub mod lexer;
pub mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let input = std::fs::read_to_string(filename).expect("EXEC ERROR: Failed to read file");
    let mut program = Program::new(&input);
    println!("{}", program.run());
}

