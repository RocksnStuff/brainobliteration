use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub enum NodeRef<'a> {
    Element(&'a dyn AstNode<'a>),
    Loop(&'a Loop<'a>),
    None
}

pub trait AstNode<'a>: Debug + 'a{
    fn next(&self) -> NodeRef;
}

macro_rules! ast_node_impl {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name<'a> {
            next: NodeRef<'a>
        }
        
        impl<'a> $name<'a> {
            pub fn new(next: NodeRef) -> $name<'a> { $name{next} }
        }
        
        impl<'a> AstNode<'a> for $name<'a> { fn next(&self) -> NodeRef { self.next } }
    };
}

ast_node_impl!{Increment}

ast_node_impl!{Decrement}

ast_node_impl!{Toggle}

ast_node_impl!{Output}

ast_node_impl!{Input}

#[derive(Debug)]
pub struct Loop<'a> {
    next: NodeRef<'a>,
    body: NodeRef<'a>
}

impl<'a> Loop<'a> { 
    pub fn new(next: NodeRef, body: NodeRef) -> Loop<'a> { Loop{next, body} }
    
    pub fn body(&self) -> NodeRef { self.body }
}

impl<'a> AstNode<'a> for Loop<'a> {
    fn next(&self) -> NodeRef { self.next }
}

macro_rules! visitor_fns {
    ($( $name:ident,$fn_name:ident ),*) => {
        $(
        fn $fn_name(&self, node: & $name) -> T;
        )*
    };
}

pub trait Visitor<T> {
    visitor_fns!{Increment,visit_increment,Decrement,visit_decrement,Toggle,visit_toggle,Output,visit_output,Input,visit_input,Loop,visit_loop}
}