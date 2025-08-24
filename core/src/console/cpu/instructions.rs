use crate::console::cpu::registers::{R16, R8};

#[derive(Debug, Default, Copy, Clone)]
pub enum CPUInstruction {
    #[default]
    NoOp,
    AddR8(R8),
    AddR16(R16),
}

impl From<u8> for CPUInstruction {
    fn from(value: u8) -> Self {
        match value {
            0x00 => CPUInstruction::NoOp,
            0x04 => CPUInstruction::AddR16(R16::AB),
            0x05 => CPUInstruction::AddR16(R16::CD),
            0x06 => CPUInstruction::AddR16(R16::PC),
            0x07 => CPUInstruction::AddR16(R16::SP),
            0x08 => CPUInstruction::AddR8(R8::A),
            0x09 => CPUInstruction::AddR8(R8::B),
            0x0A => CPUInstruction::AddR8(R8::C),
            0x0B => CPUInstruction::AddR8(R8::D),
            0x0C => CPUInstruction::AddR8(R8::E),
            0x0D => CPUInstruction::AddR8(R8::F),
            0x0E => CPUInstruction::AddR8(R8::G),
            0x0F => CPUInstruction::AddR8(R8::H),
            _ => CPUInstruction::NoOp,
        }
    }
}

impl From<CPUInstruction> for u8 {
    fn from(value: CPUInstruction) -> Self {
        match value {
            CPUInstruction::NoOp => 0x00,
            CPUInstruction::AddR16(r16) => match r16 {
                R16::AB => 0x04,
                R16::CD => 0x05,
                R16::PC => 0x06,
                R16::SP => 0x07,
            },
            CPUInstruction::AddR8(r8) => match r8 {
                R8::A => 0x08,
                R8::B => 0x09,
                R8::C => 0x0A,
                R8::D => 0x0B,
                R8::E => 0x0C,
                R8::F => 0x0D,
                R8::G => 0x0E,
                R8::H => 0x0F,
            },
        }
    }
}
