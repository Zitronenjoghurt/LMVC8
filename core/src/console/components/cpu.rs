use crate::console::components::bus::Bus;
use crate::console::components::cpu::alu::ALU;
use crate::console::components::cpu::instructions::CPUInstruction;
use crate::console::components::cpu::registers::{GeneralRegisters, R16, R8};
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;
use crate::console::types::word::Word;

pub mod alu;
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
    pub fn reset(&mut self) {
        self.registers = GeneralRegisters::default();
        self.ime = false;
        self.ir = Byte::default();
        self.pc = Word::default();
        self.alu = ALU::default();
    }

    #[inline(always)]
    pub fn step(&mut self, bus: &mut Bus) -> bool {
        self.fetch(bus);

        let instruction = self.decode(bus);

        self.execute(bus, instruction)
    }

    #[inline(always)]
    fn fetch(&mut self, bus: &mut Bus) {
        self.ir = self.read_byte(bus);
    }

    #[inline(always)]
    fn decode(&self, bus: &mut Bus) -> CPUInstruction {
        bus.tick();
        CPUInstruction::from(self.ir.value())
    }

    #[inline(always)]
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
            CPUInstruction::IncR8(r8) => self.increment_r8(bus, r8),
            CPUInstruction::DecR8(r8) => self.decrement_r8(bus, r8),
            CPUInstruction::IncR16(r16) => self.increment_r16(r16),
            CPUInstruction::DecR16(r16) => self.decrement_r16(r16),
        }

        do_continue
    }

    #[inline(always)]
    fn read_byte(&mut self, bus: &mut Bus) -> Byte {
        let byte = bus.read(Address::from(self.pc));
        self.pc = self.pc.increment().0;
        byte
    }

    #[inline(always)]
    fn read_word(&mut self, bus: &mut Bus) -> Word {
        let low = self.read_byte(bus);
        let high = self.read_byte(bus);
        Word::from_le(low, high)
    }
}

// Instructions
impl CPU {
    #[inline(always)]
    fn add_r8(&mut self, bus: &mut Bus, r8: R8) {
        let acc = self.registers.get_r8(bus, R8::ACC);
        let value = self.registers.get_r8(bus, r8);
        let result = self.alu.add_bytes(acc, value);
        self.registers.set_r8(bus, R8::ACC, result)
    }

    #[inline(always)]
    fn add_r16(&mut self, r16: R16) {
        let acc = self.registers.get_r16(R16::ACC);
        let value = self.registers.get_r16(r16);
        let result = self.alu.add_words(acc, value);
        self.registers.set_r16(R16::ACC, result);
    }

    #[inline(always)]
    pub fn sub_r8(&mut self, bus: &mut Bus, r8: R8) {
        let acc = self.registers.get_r8(bus, R8::ACC);
        let value = self.registers.get_r8(bus, r8);
        let result = self.alu.sub_bytes(acc, value);
        self.registers.set_r8(bus, R8::ACC, result)
    }

    #[inline(always)]
    pub fn sub_r16(&mut self, r16: R16) {
        let acc = self.registers.get_r16(R16::ACC);
        let value = self.registers.get_r16(r16);
        let result = self.alu.sub_words(acc, value);
        self.registers.set_r16(R16::ACC, result);
    }

    #[inline(always)]
    pub fn load_r8(&mut self, bus: &mut Bus, target: R8, source: R8) {
        let value = self.registers.get_r8(bus, source);
        self.registers.set_r8(bus, target, value);
    }

    #[inline(always)]
    pub fn load_r16(&mut self, target: R16, source: R16) {
        let value = self.registers.get_r16(source);
        self.registers.set_r16(target, value);
    }

    #[inline(always)]
    pub fn load_r8i(&mut self, bus: &mut Bus, r8: R8) {
        let value = self.read_byte(bus);
        self.registers.set_r8(bus, r8, value);
    }

    #[inline(always)]
    pub fn load_r16i(&mut self, bus: &mut Bus, r16: R16) {
        let value = self.read_word(bus);
        self.registers.set_r16(r16, value);
    }

    #[inline(always)]
    pub fn increment_r8(&mut self, bus: &mut Bus, r8: R8) {
        self.registers.increment_r8(bus, r8);
    }

    #[inline(always)]
    pub fn decrement_r8(&mut self, bus: &mut Bus, r8: R8) {
        self.registers.decrement_r8(bus, r8);
    }

    #[inline(always)]
    pub fn increment_r16(&mut self, r16: R16) {
        self.registers.increment_r16(r16);
    }

    #[inline(always)]
    pub fn decrement_r16(&mut self, r16: R16) {
        self.registers.decrement_r16(r16);
    }
}

/// Outside access
impl CPU {
    pub fn get_pc(&self) -> Word {
        self.pc
    }

    pub fn get_ir(&self) -> Byte {
        self.ir
    }

    pub fn get_registers(&self) -> GeneralRegisters {
        self.registers
    }

    pub fn get_alu(&self) -> ALU {
        self.alu
    }
}

impl CPU {
    pub fn set_r8(&mut self, bus: &mut Bus, r8: R8, value: u8) {
        self.registers.set_r8(bus, r8, value.into());
    }

    pub fn set_r16(&mut self, r16: R16, value: u16) {
        self.registers.set_r16(r16, value.into())
    }
}
