use crate::console::cpu::instructions::CPUInstruction;
use crate::console::cpu::registers::{R16, R8};

#[derive(Debug, Default)]
pub struct Compiler {
    instructions: Vec<CPUInstruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Will do nothing
    pub fn no_op(mut self) -> Self {
        self.instructions.push(CPUInstruction::NoOp);
        self
    }

    /// Adds the specified register to the A register (wrapping)
    pub fn add_r8(mut self, r8: R8) -> Self {
        self.instructions.push(CPUInstruction::AddR8(r8));
        self
    }

    /// Adds the specified register to the AB register (wrapping)
    pub fn add_r16(mut self, r16: R16) -> Self {
        self.instructions.push(CPUInstruction::AddR16(r16));
        self
    }

    /// Repeats a set of function a specified amount of times
    pub fn repeat<F>(mut self, times: usize, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        for _ in 0..times {
            self = f(self);
        }
        self
    }
}
