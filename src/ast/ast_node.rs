use std::rc::Rc;
use std::fmt::Debug;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AstNode {
    Increment(Rc<AstNode>),
    Decrement(Rc<AstNode>),
    Toggle(Rc<AstNode>),
    Output(Rc<AstNode>),
    Input(Rc<AstNode>),
    Loop(Rc<AstNode>, Rc<AstNode>),
    Null
}

pub trait Visitor<T> {
    fn visit(&mut self, n: &AstNode) -> T;
}