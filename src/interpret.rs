use crate::ast::Ast;
use crate::ast::ast_node::{Decrement, Increment, Input, Loop, Output, Toggle, Visitor};

pub fn interpret(ast: &mut Ast) {
    
}

struct Interpreter {}

impl Visitor<()> for Interpreter {
    fn visit_increment(&self, node: &Increment) -> () {
        node.next();
    }

    fn visit_decrement(&self, node: &Decrement) -> () {
        todo!()
    }

    fn visit_toggle(&self, node: &Toggle) -> () {
        todo!()
    }

    fn visit_output(&self, node: &Output) -> () {
        todo!()
    }

    fn visit_input(&self, node: &Input) -> () {
        todo!()
    }

    fn visit_loop(&self, node: &Loop) -> () {
        todo!()
    }
}