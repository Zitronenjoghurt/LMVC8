use crate::console::components::bus::Bus;
use crate::console::components::cpu::alu::ALU;
use crate::console::components::cpu::instructions::CPUInstruction;
use crate::console::components::cpu::registers::{GeneralRegisters, R16, R8};
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;
use crate::console::types::word::Word;

mod alu;
pub mod instructions;
pub mod registers;

#[derive(Debug, Default, Copy, Clone)]
pub struct CPU {
    registers: GeneralRegisters,
    /// Interrupt master enable
    ime: bool,
    /// Instruction register
    ir: Byte,
    /// Program counter
    pc: Word,
    /// Arithmetic logic unit
    alu: ALU,
}

impl CPU {
    pub fn step(&mut self, bus: &mut Bus) -> bool {
        self.fetch(bus);

        let instruction = self.decode(bus);

        self.execute(bus, instruction)
    }

    fn fetch(&mut self, bus: &mut Bus) {
        self.ir = self.read_byte(bus);
    }

    fn decode(&self, bus: &mut Bus) -> CPUInstruction {
        bus.tick();
        CPUInstruction::from(self.ir.value())
    }

    fn execute(&mut self, bus: &mut Bus, instruction: CPUInstruction) -> bool {
        bus.tick();

        let mut do_continue = true;

        match instruction {
            CPUInstruction::NoOp => {}
            CPUInstruction::Halt => do_continue = false,
            CPUInstruction::AddR8(r8) => self.add_r8(bus, r8),
            CPUInstruction::AddR16(r16) => self.add_r16(r16),
            CPUInstruction::SubR8(r8) => self.sub_r8(bus, r8),
            CPUInstruction::SubR16(r16) => self.sub_r16(r16),
            CPUInstruction::LoadR8((target, source)) => self.load_r8(bus, target, source),
            CPUInstruction::LoadR16((target, source)) => self.load_r16(target, source),
            CPUInstruction::LoadR8i(r8) => self.load_r8i(bus, r8),
            CPUInstruction::LoadR16i(r16) => self.load_r16i(bus, r16),
        }

        do_continue
    }

    fn read_byte(&mut self, bus: &mut Bus) -> Byte {
        let byte = bus.read(Address::from(self.pc));
        self.pc = self.pc.increment().0;
        byte
    }

    fn read_word(&mut self, bus: &mut Bus) -> Word {
        let low = self.read_byte(bus);
        let high = self.read_byte(bus);
        Word::from_le(low, high)
    }
}

// Instructions
impl CPU {
    fn add_r8(&mut self, bus: &mut Bus, r8: R8) {
        let acc = self.registers.get_r8(bus, R8::ACC);
        let value = self.registers.get_r8(bus, r8);
        let result = self.alu.add_bytes(acc, value);
        self.registers.set_r8(bus, R8::ACC, result)
    }

    fn add_r16(&mut self, r16: R16) {
        let acc = self.registers.get_r16(R16::ACC);
        let value = self.registers.get_r16(r16);
        let result = self.alu.add_words(acc, value);
        self.registers.set_r16(R16::ACC, result);
    }

    pub fn sub_r8(&mut self, bus: &mut Bus, r8: R8) {
        let acc = self.registers.get_r8(bus, R8::ACC);
        let value = self.registers.get_r8(bus, r8);
        let result = self.alu.sub_bytes(acc, value);
        self.registers.set_r8(bus, R8::ACC, result)
    }

    pub fn sub_r16(&mut self, r16: R16) {
        let acc = self.registers.get_r16(R16::ACC);
        let value = self.registers.get_r16(r16);
        let result = self.alu.sub_words(acc, value);
        self.registers.set_r16(R16::ACC, result);
    }

    pub fn load_r8(&mut self, bus: &mut Bus, target: R8, source: R8) {
        let value = self.registers.get_r8(bus, source);
        self.registers.set_r8(bus, target, value);
    }

    pub fn load_r16(&mut self, target: R16, source: R16) {
        let value = self.registers.get_r16(source);
        self.registers.set_r16(target, value);
    }

    pub fn load_r8i(&mut self, bus: &mut Bus, r8: R8) {
        let value = self.read_byte(bus);
        self.registers.set_r8(bus, r8, value);
    }

    pub fn load_r16i(&mut self, bus: &mut Bus, r16: R16) {
        let value = self.read_word(bus);
        self.registers.set_r16(r16, value);
    }
}
