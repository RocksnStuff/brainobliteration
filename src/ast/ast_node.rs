use std::fmt::Debug;
use crate::ast::ast_node::AstNode::{Decrement, Loop, Output};

#[derive(Debug, Copy, Clone)]
pub enum AstNode {
    Increment(Box<AstNode>),
    Decrement(Box<AstNode>),
    Output(Box<AstNode>),
    Input(Box<AstNode>),
    Loop(Box<AstNode>, Box<AstNode>),
    None
}