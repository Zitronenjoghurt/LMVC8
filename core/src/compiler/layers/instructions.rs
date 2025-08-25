use crate::compiler::Compiler;
use crate::console::components::cpu::instructions::CPUInstruction;
use crate::console::components::cpu::registers::{R16, R8};

impl Compiler {
    /// Will do nothing
    pub fn no_op(self) -> Self {
        self.push_instruction(CPUInstruction::NoOp)
    }

    /// Halt
    pub fn halt(self) -> Self {
        self.push_instruction(CPUInstruction::Halt)
    }

    /// Adds the specified register to the A register (wrapping)
    pub fn add_r8(self, r8: R8) -> Self {
        self.push_instruction(CPUInstruction::AddR8(r8))
    }

    /// Adds the specified register to the BC register (wrapping)
    pub fn add_r16(self, r16: R16) -> Self {
        self.push_instruction(CPUInstruction::AddR16(r16))
    }

    /// Subtracts the specified register from the A register (wrapping)
    pub fn sub_r8(self, r8: R8) -> Self {
        self.push_instruction(CPUInstruction::SubR8(r8))
    }

    /// Subtracts the specified register from the BC register (wrapping)
    pub fn sub_r16(self, r16: R16) -> Self {
        self.push_instruction(CPUInstruction::SubR16(r16))
    }

    /// Load the value from the source register into the target register\
    /// **Loading into the same register is not part of the CPUs instruction set!**
    pub fn load_r8(self, target: R8, source: R8) -> Self {
        self.push_instruction(CPUInstruction::LoadR8((target, source)))
    }

    /// Load the value from the source register into the target register\
    /// **Loading into the same register is not part of the CPUs instruction set!**
    pub fn load_r16(self, target: R16, source: R16) -> Self {
        self.push_instruction(CPUInstruction::LoadR16((target, source)))
    }

    /// Load an immediate into a specified register
    pub fn load_r8i(self, r8: R8, immediate: u8) -> Self {
        self.push_instruction(CPUInstruction::LoadR8i(r8))
            .push_byte(immediate)
    }

    /// Load an immediate into a specified register
    pub fn load_r16i(self, r16: R16, immediate: u16) -> Self {
        self.push_instruction(CPUInstruction::LoadR16i(r16))
            .push_word(immediate)
    }
}
