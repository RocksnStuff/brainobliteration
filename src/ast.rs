use std::fmt::Debug;

pub mod ast_node;
pub mod tokens;

/*#[derive(Debug)]
pub struct Ast<'a> {
    nodes: Vec<Box<dyn AstNode<'a>>>
}

impl<'a> Ast<'a> {
    pub fn new(capacity: usize) -> Ast<'a> {
        Ast { nodes: Vec::with_capacity(capacity) }
    }

    pub fn add(&mut self, node: Box<dyn AstNode<'a> + 'a>) -> NodeRef<'a> {
        self.nodes.push(node);
        NodeRef::Element(&node)
    }
    
    pub fn add_loop(&mut self, node: Box<Loop>) -> NodeRef {
        self.nodes.push(node);
        NodeRef::Loop(&node)
    }

    pub fn get(&self, index: usize) -> &Box<dyn AstNode> {
        &self.nodes[index]
    }
}*/