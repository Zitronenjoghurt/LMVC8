use crate::compiler::Compiler;
use crate::console::components::cpu::instructions::CPUInstruction;
use crate::console::components::cpu::registers::{R16, R16S, R8};

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

    /// Increment the specified register
    pub fn increment_r8(self, r8: R8) -> Self {
        self.push_instruction(CPUInstruction::IncR8(r8))
    }

    /// Decrement the specified register
    pub fn decrement_r8(self, r8: R8) -> Self {
        self.push_instruction(CPUInstruction::DecR8(r8))
    }

    /// Increment the specified register
    pub fn increment_r16(self, r16: R16) -> Self {
        self.push_instruction(CPUInstruction::IncR16(r16))
    }

    /// Decrement the specified register
    pub fn decrement_r16(self, r16: R16) -> Self {
        self.push_instruction(CPUInstruction::DecR16(r16))
    }

    /// Push a register pair value onto the stack
    pub fn stack_push(self, r16s: R16S) -> Self {
        self.push_instruction(CPUInstruction::Push(r16s))
    }

    /// Pop the value at the top of the stack into the specified register pair
    pub fn stack_pop(self, r16s: R16S) -> Self {
        self.push_instruction(CPUInstruction::Pop(r16s))
    }

    /// Enable interrupts, usually after critical sections (e.g. interrupt service routines)
    pub fn enable_interrupts(self) -> Self {
        self.push_instruction(CPUInstruction::EnableInterrupts)
    }

    /// Disable interrupts, usually before critical sections
    pub fn disable_interrupts(self) -> Self {
        self.push_instruction(CPUInstruction::DisableInterrupts)
    }

    /// Call a function at the specified address, will push the current PC onto the stack and jump to the address
    pub fn call(self, address: u16) -> Self {
        self.push_instruction(CPUInstruction::Call)
            .push_word(address)
    }

    /// Return from a previously called function, will pop an address from stack and jump there
    pub fn ret(self) -> Self {
        self.push_instruction(CPUInstruction::Return)
    }
}
