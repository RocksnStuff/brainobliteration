use std::str::Chars;

use crate::ast::{Ast};
use crate::ast::ast_node::*;
use crate::ast::tokens::*;

#[derive(Debug)]
pub enum ParseError {
    UnmatchedParenthesis(String)
}

pub fn parse(input: &String) -> Result<Ast, ParseError> {
    let ast = Ast::new(input.len());

    match get_node(ast, &mut input.chars(), false) {
        Ok((_, ast)) => Ok(ast),
        Err(err) => Err(err),
    }
}

fn get_node<'a>(mut ast: Ast, chars: &mut Chars, in_loop: bool) -> Result<(NodeRef<'a>, Ast<'a>), ParseError> {
    macro_rules! return_node {
        ($NODE_NAME:ident) => {
            {
                let next: NodeRef;
                (next, ast) = get_node(ast, chars, in_loop)?;
                Ok((
                    ast.add(Box::new($NODE_NAME::new(next))),
                    ast
                ))
            }
        };
    }
    
    match chars.next() {
        Some(INCREMENT) => return_node!(Increment),
        Some(DECREMENT) => return_node!(Decrement),
        Some(TOGGLE) => return_node!(Toggle),
        Some(OUTPUT) => return_node!(Output),
        Some(INPUT) => return_node!(Input),
        Some(LEFT_LOOP) => get_loop(ast, chars, in_loop),
        Some(RIGHT_LOOP) => if in_loop {
            Ok((NodeRef::None, ast)) 
        } else {
            Err(ParseError::UnmatchedParenthesis(String::from("Unexpected right parenthesis.")))
        },
        None => if !in_loop {
            Ok((NodeRef::None, ast))
        } else {
            Err(ParseError::UnmatchedParenthesis(String::from("Found EOL, expected closing right parenthesis.")))
        }
        Some(_) => get_node(ast, chars, in_loop),
    }
}

fn get_loop<'a>(mut ast: Ast, chars: &mut Chars, in_loop: bool) -> Result<(NodeRef<'a>, Ast<'a>), ParseError> {
    let body: NodeRef;
    (body, ast) = get_node(ast, chars, true)?;
    
    let next: NodeRef;
    (next, ast) = get_node(ast, chars, in_loop)?;
    Ok((
        ast.add_loop(Box::new(Loop::new(next, body))),
        ast
    ))
}