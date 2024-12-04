#[allow(warnings)]
mod bindings;

use crate::bindings::exports::component::rpn::types::Guest;
use std::cell::RefCell;
use bindings::exports::component::rpn::types::{GuestEngine, Operation};

struct CalcEngine {
    stack: RefCell<Vec<u32>>,
}


impl GuestEngine for CalcEngine {
    fn new() -> Self {
        CalcEngine {
            stack: RefCell::new(vec![])
        }
    }

    fn push_operand(&self, operand: u32) {
        self.stack.borrow_mut().push(operand);
    }

    fn push_operation(&self, operation: Operation) {
        let mut stack = self.stack.borrow_mut();
        let right = stack.pop().unwrap(); // TODO: error handling!
        let left = stack.pop().unwrap();
        let result = match operation {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        };
        stack.push(result);
    }

    fn execute(&self) -> u32 {
        self.stack.borrow_mut().pop().unwrap() // TODO: error handling!
    }
}

struct Implementation;
impl Guest for Implementation {
    type Engine = CalcEngine;
}

bindings::export!(Implementation with_types_in bindings);