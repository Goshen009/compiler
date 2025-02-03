mod lexer;
mod parser;
mod errorq;
mod ast_types;
mod semantics;

use lexer::objects::Lexer;
use ast_types::Program;
use parser::Parser;

const FILEPATH: &str = "./src.txt";

fn main() { // on finish let the user be able to enter a filepath
    let src_code = get_source_code(FILEPATH);

    /* Lexer! */
    let lexer = Lexer::new(src_code);
    if !lexer.completed_without_errors() {
        return;
    }

    /* Parser! */
    let mut parser = Parser::new(lexer);
    let mut program = Program::new();

    let global_scope = parser::parse(&mut parser, &mut program, true);
    if !parser.completed_without_errors(&program) {
        return;
    }

    /* Semantics! */
    let semantics = semantics::start(&program, global_scope);
    if !semantics.completed_without_errors() {
        return;
    }

    println!("\nWe are ready for LLVM");
}

pub fn get_source_code(filepath: &str) -> String {
    let file = std::fs::read_to_string(filepath);
    match file {
        Ok(file_contents) => file_contents,
        Err(error) => {
            println!("Error getting source code. Check if it is the correct file name");
            panic!("{}", error);
        }
    }
}