use crate::console::components::bus::Bus;
use crate::console::components::cpu::alu::ALU;
use crate::console::components::cpu::instructions::CPUInstruction;
use crate::console::components::cpu::interrupts::{InterruptFlags, IV_INPUT, IV_TIMER};
use crate::console::components::cpu::registers::{GeneralRegisters, R16, R16S, R8};
use crate::console::components::cpu::step_flags::CPUStepFlags;
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;
use crate::console::types::word::Word;

pub mod alu;
pub mod instructions;
pub mod interrupts;
pub mod registers;
pub mod step_flags;

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
    pub fn step(&mut self, bus: &mut Bus) -> CPUStepFlags {
        self.handle_interrupt(bus);

        self.fetch(bus);
        let instruction = self.decode(bus);
        let do_halt = self.execute(bus, instruction);

        let mut step_flags = CPUStepFlags::empty();
        step_flags.set(CPUStepFlags::HALT, do_halt);

        step_flags
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
        match instruction {
            CPUInstruction::NoOp => {}
            CPUInstruction::Halt => return true,
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
            CPUInstruction::Push(r16s) => self.push(bus, r16s),
            CPUInstruction::Pop(r16s) => self.pop(bus, r16s),
        }
        false
    }

    #[inline(always)]
    fn handle_interrupt(&mut self, bus: &mut Bus) {
        let interrupt_flags = self.read_ie(bus) & self.read_ia(bus);
        if let Some(interrupt) = interrupt_flags.first_set() {
            self.ime = false;
            self.push_word(bus, self.pc);
            match interrupt {
                InterruptFlags::TIMER => self.pc = IV_TIMER.into(),
                InterruptFlags::INPUT => self.pc = IV_INPUT.into(),
                _ => {}
            }
        }
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

    #[inline(always)]
    fn push_byte(&mut self, bus: &mut Bus, byte: Byte) {
        bus.write(Address::from(self.registers.get_r16(R16::SP)), byte);
        self.registers.decrement_r16(R16::SP);
    }

    #[inline(always)]
    fn push_word(&mut self, bus: &mut Bus, word: Word) {
        self.push_byte(bus, word.high_byte());
        self.push_byte(bus, word.low_byte());
    }

    #[inline(always)]
    fn pop_byte(&mut self, bus: &mut Bus) -> Byte {
        let byte = bus.read(Address::from(self.registers.get_r16(R16::SP)));
        self.registers.increment_r16(R16::SP);
        byte
    }

    #[inline(always)]
    fn pop_word(&mut self, bus: &mut Bus) -> Word {
        let low = self.pop_byte(bus);
        let high = self.pop_byte(bus);
        Word::from_le(low, high)
    }

    #[inline(always)]
    fn read_ie(self, bus: &mut Bus) -> InterruptFlags {
        bus.read(Bus::INTERRUPT_ENABLE.into()).into()
    }

    #[inline(always)]
    fn read_ia(self, bus: &mut Bus) -> InterruptFlags {
        bus.read(Bus::INTERRUPT_ACTIVE.into()).into()
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

    #[inline(always)]
    pub fn push(&mut self, bus: &mut Bus, r16s: R16S) {
        let value = self.registers.get_r16s(r16s, self.alu.get_flags());
        self.push_word(bus, value);
    }

    #[inline(always)]
    pub fn pop(&mut self, bus: &mut Bus, r16s: R16S) {
        let value = self.pop_word(bus);
        self.registers
            .set_r16s(r16s, self.alu.get_flags_mut(), value);
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
