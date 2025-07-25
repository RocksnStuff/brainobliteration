use std::rc::Rc;
use std::str::Chars;

use crate::ast::ast_node::*;
use crate::ast::ast_node::AstNode::{Decrement, Increment, Input, Loop, Null, Output, Toggle};
use crate::ast::tokens::*;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseError {
    UnmatchedParenthesis(String)
}

pub fn parse(input: &String) -> Result<Rc<AstNode>, ParseError> {
    get_node(Rc::new(Null), &mut input.chars(), false)
}

fn get_node(ast_node: Rc<AstNode>, chars: &mut Chars, in_loop: bool) -> Result<Rc<AstNode>, ParseError> {
    macro_rules! return_node {
        ($NODE_NAME:ident) => {
            {
                let next = get_node(ast_node, chars, in_loop)?;
                Ok(Rc::new($NODE_NAME(next)))
            }
        };
    }
    
    match chars.next() {
        Some(INCREMENT) => return_node!(Increment),
        Some(DECREMENT) => return_node!(Decrement),
        Some(TOGGLE) => return_node!(Toggle),
        Some(OUTPUT) => return_node!(Output),
        Some(INPUT) => return_node!(Input),
        Some(LEFT_LOOP) => get_loop(ast_node, chars, in_loop),
        Some(RIGHT_LOOP) => if in_loop {
            Ok(Rc::new(Null))
        } else {
            Err(ParseError::UnmatchedParenthesis(String::from("Unexpected right parenthesis.")))
        },
        None => if !in_loop {
            Ok(Rc::new(Null))
        } else {
            Err(ParseError::UnmatchedParenthesis(String::from("Found EOL, expected closing right parenthesis.")))
        }
        Some(_) => get_node(ast_node, chars, in_loop),
    }
}

fn get_loop(ast_node: Rc<AstNode>, chars: &mut Chars, in_loop: bool) -> Result<Rc<AstNode>, ParseError> {
    let body = get_node(Rc::clone(&ast_node), chars, true)?;
    
    let next = get_node(ast_node, chars, in_loop)?;
    Ok(Rc::new(Loop(next, body)))
}