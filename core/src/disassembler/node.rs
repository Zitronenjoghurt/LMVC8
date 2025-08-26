use crate::console::components::cpu::instructions::CPUInstruction;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Node {
    Byte(u8),
    Instruction(CPUInstruction),
}

impl Node {
    pub fn byte(byte: u8) -> Self {
        Self::Byte(byte)
    }

    pub fn instruction(instruction: CPUInstruction) -> Self {
        Self::Instruction(instruction)
    }

    pub fn is_byte(&self) -> bool {
        matches!(self, Self::Byte(_))
    }

    pub fn is_instruction(&self) -> bool {
        matches!(self, Self::Instruction(_))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Byte(byte) => write!(f, "0x{:02X}", byte),
            Self::Instruction(instruction) => write!(f, "{instruction}"),
        }
    }
}
