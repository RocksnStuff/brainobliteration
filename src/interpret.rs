use crate::ast::ast_node::AstNode::{Decrement, Increment, Input, Loop, Null, Output, Toggle};
use crate::ast::ast_node::{AstNode, Visitor};
use crate::interpret::RuntimeError::{IOError, PointerOutOfBounds};
use std::io;
use std::io::{stdin, stdout, Read, Write};

const MAX_POINTER: usize = 300;
const MAX_BIT_POINTER: u8 = 127;

#[derive(Debug)]
#[allow(dead_code)]
pub enum RuntimeError {
    PointerOutOfBounds(String),
    IOError(io::Error)
}

pub struct VirtualMachine {
    memory: [u128; MAX_POINTER],
    pointer: usize,
    bit_pointer: u8,

    out_buffer: u8,
    out_buffer_counter: u8,
    in_buffer: [u8; 1],
    in_buffer_counter: u8
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            memory: [0; MAX_POINTER],
            pointer: 0,
            bit_pointer: 0,
            out_buffer: 0,
            out_buffer_counter: 0,
            in_buffer: [0; 1],
            in_buffer_counter: 0
        }
    }
    
    fn increment(&mut self) -> Result<(), RuntimeError> {
        if self.pointer == MAX_POINTER && self.bit_pointer == MAX_BIT_POINTER {
            return Err(PointerOutOfBounds(format!(
                "Cannot increment pointer past the maximum size of {MAX_POINTER}."))) // TODO: Fix
        }

        if self.bit_pointer == MAX_BIT_POINTER {
            self.bit_pointer = 0;
            self.pointer += 1;
        } else {
            self.bit_pointer += 1;
        }

        Ok(())
    }

    fn decrement(&mut self) -> Result<(), RuntimeError> {
        if self.pointer == 0 && self.bit_pointer == 0 {
            return Err(PointerOutOfBounds(String::from("Cannot decrement pointer past 0.")))
        }

        if self.bit_pointer == 0 {
            self.bit_pointer = MAX_BIT_POINTER;
            self.pointer -= 1;
        } else {
            self.bit_pointer -= 1
        }

        self.bit_pointer -= 1;

        Ok(())
    }

    fn toggle(&mut self) {
        self.memory[self.pointer] = self.memory[self.pointer] ^ (1_u128 << self.bit_pointer);
    }

    fn set(&mut self, value: bool) {
        self.memory[self.pointer] = if value {
            self.memory[self.pointer] | (1_u128 << self.bit_pointer)
        } else {
            self.memory[self.pointer] & (!1_u128 << self.bit_pointer)
        }
    }

    fn get(&self) -> bool {
        (1_u128 << self.bit_pointer) & self.memory[self.pointer] > 0
    }

    fn read(&mut self) -> Result<bool, RuntimeError> {
        if self.in_buffer_counter == 8 {
            self.in_buffer_counter = 0;

            match stdin().read(&mut self.in_buffer) {
                Err(e) => return Err(IOError(e)),
                Ok(_) => ()
            }
        }

        let result = (1_u8 << self.in_buffer_counter) & self.in_buffer[0] > 0;
        self.in_buffer_counter += 1;
        Ok(result)
    }

    fn write(&mut self, value: bool) -> Result<(), RuntimeError> {
        self.out_buffer = if value {
            self.out_buffer | (1_u8 << self.out_buffer_counter)
        } else {
            self.out_buffer & (!1_u8 << self.out_buffer_counter)
        };

        self.out_buffer_counter += 1;

        if self.out_buffer_counter == 8 {
            self.out_buffer_counter = 0;

            return match stdout().write(&[self.out_buffer]) {
                Ok(_) => Ok(()),
                Err(e) => Err(IOError(e))
            };
        }

        Ok(())
    }
}

pub struct Interpreter {
    pub virtual_machine: VirtualMachine
}

impl Visitor<Result<(),RuntimeError>> for Interpreter {
    fn visit(&mut self, n: &AstNode) -> Result<(),RuntimeError> {
        match *n {
            Increment(ref next) => {
                self.virtual_machine.increment()?;

                self.visit(next)
            },
            Decrement(ref next) => {
                self.virtual_machine.decrement()?;

                self.visit(next)
            },
            Toggle(ref next) => {
                self.virtual_machine.toggle();

                self.visit(next)
            },
            Output(ref next) => {
                self.virtual_machine.write(self.virtual_machine.get())?;

                self.visit(next)
            },
            Input(ref next) => {
                let value = self.virtual_machine.read()?;
                self.virtual_machine.set(value);

                self.visit(next)
            },
            Loop(ref next, ref body) => {
                while self.virtual_machine.get() {
                    self.visit(body)?;
                }

                self.visit(next)
            },
            Null => Ok(())
        }
    }
}