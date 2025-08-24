use crate::console::bus::Bus;
use crate::console::cpu::instructions::CPUInstruction;
use crate::console::cpu::registers::{CPURegisters, R16, R8};
use crate::console::types::byte::Byte;

pub mod instructions;
pub mod registers;

#[derive(Debug, Default, Copy, Clone)]
pub struct CPU {
    /// General registers
    registers: CPURegisters,
    /// Interrupt master enable
    ime: bool,
    /// Instruction register
    ir: Byte,
}

impl CPU {
    pub fn step(&mut self, bus: &mut Bus) {
        self.fetch(bus);
        self.registers.increment_r16(R16::PC);

        let instruction = self.decode(bus);
        self.execute(bus, instruction);
    }

    fn fetch(&mut self, bus: &mut Bus) {
        self.ir = bus.read(self.registers.get_r16(R16::PC).into());
    }

    fn decode(&self, bus: &mut Bus) -> CPUInstruction {
        bus.tick();
        CPUInstruction::from(self.ir.value())
    }

    fn execute(&mut self, bus: &mut Bus, instruction: CPUInstruction) {
        bus.tick();

        match instruction {
            CPUInstruction::NoOp => {}
            CPUInstruction::AddR8(r8) => self.add_r8(r8),
            CPUInstruction::AddR16(r16) => self.add_r16(r16),
        }
    }
}

// Instructions
impl CPU {
    fn add_r8(&mut self, r8: R8) {
        self.registers.set_r8(
            R8::A,
            self.registers.get_r8(R8::A).add(self.registers.get_r8(r8)),
        )
    }

    fn add_r16(&mut self, r16: R16) {
        self.registers.set_r16(
            R16::AB,
            self.registers
                .get_r16(R16::AB)
                .add_word(self.registers.get_r16(r16)),
        )
    }
}
