use std::process::ExitCode;
use std::time::Instant;
use crate::ast::ast_node::Visitor;
use crate::interpret::{Interpreter, VirtualMachine};
use crate::parse::parse;

mod ast;
mod parse;
mod interpret;

fn main() -> ExitCode {
    let input = String::from("[[[>>]]][[]]");

    let (ast_node, time) = {
        let now = Instant::now();
        let ast_node = parse(&input);
        (ast_node, now.elapsed())
    };

    match ast_node {
        Ok(ref node) => {
            println!("Compile time: {}us", time.as_micros());
            println!("{:?}", node);

            let mut interpreter = Interpreter{ virtual_machine: VirtualMachine::new() };
            match interpreter.visit(node) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{:?}", e);
                    return ExitCode::FAILURE;
                }
            }
        },
        Err(e) => {
            eprintln!("{:?}", e);
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}