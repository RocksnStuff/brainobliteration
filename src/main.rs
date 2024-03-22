use std::process::ExitCode;
use std::time::Instant;
use crate::parse::parse;

mod ast;
mod parse;
mod interpret;

fn main() -> ExitCode {
    let input = String::from("[[[]]]");

    let (ast, time) = {
        let now = Instant::now();
        let ast = parse(&input);
        (ast, now.elapsed())
    };

    match ast {
        Ok(ast) => {
            println!("Compile time: {}us", time.as_micros());
            println!("{:?}", ast);
            ExitCode::SUCCESS
        },
        Err(e) => {
            eprintln!("{:?}", e);
            ExitCode::FAILURE
        }
    }
}