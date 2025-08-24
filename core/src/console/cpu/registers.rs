use crate::console::types::byte::Byte;
use crate::console::types::word::Word;

#[derive(Debug, Default, Clone, Copy)]
pub struct CPURegisters {
    /// Accumulator
    a: Byte,
    /// Flags
    f: Byte,
    b: Byte,
    c: Byte,
    d: Byte,
    e: Byte,
    g: Byte,
    h: Byte,
    pc: Word,
    sp: Word,
}

impl CPURegisters {
    pub fn get_r8(&self, r8: R8) -> Byte {
        match r8 {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::F => self.f,
            R8::G => self.g,
            R8::H => self.h,
        }
    }

    pub fn set_r8(&mut self, r8: R8, byte: Byte) {
        match r8 {
            R8::A => self.a = byte,
            R8::B => self.b = byte,
            R8::C => self.c = byte,
            R8::D => self.d = byte,
            R8::E => self.e = byte,
            R8::F => self.f = byte,
            R8::G => self.g = byte,
            R8::H => self.h = byte,
        }
    }

    pub fn get_r16(&self, r16: R16) -> Word {
        match r16 {
            R16::AF => Word::from_le(self.f, self.a),
            R16::BC => Word::from_le(self.c, self.b),
            R16::DE => Word::from_le(self.e, self.d),
            R16::GH => Word::from_le(self.h, self.g),
            R16::CD => Word::from_le(self.d, self.c),
            R16::EG => Word::from_le(self.g, self.e),
            R16::PC => self.pc,
            R16::SP => self.sp,
        }
    }

    pub fn set_r16(&mut self, r16: R16, word: Word) {
        match r16 {
            R16::AF => {
                self.a = word.high_byte();
                self.f = word.low_byte();
            }
            R16::BC => {
                self.b = word.high_byte();
                self.c = word.low_byte();
            }
            R16::DE => {
                self.d = word.high_byte();
                self.e = word.low_byte();
            }
            R16::GH => {
                self.g = word.high_byte();
            }
            R16::CD => {
                self.c = word.high_byte();
                self.d = word.low_byte();
            }
            R16::EG => {
                self.e = word.high_byte();
                self.g = word.low_byte();
            }
            R16::PC => self.pc = word,
            R16::SP => self.sp = word,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum R8 {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum R16 {
    AF = 0,
    BC = 1,
    DE = 2,
    GH = 3,
    CD = 4,
    EG = 5,
    PC = 6,
    SP = 7,
}

impl From<u8> for R8 {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            0 => R8::A,
            1 => R8::B,
            2 => R8::C,
            3 => R8::D,
            4 => R8::E,
            5 => R8::F,
            6 => R8::G,
            7 => R8::H,
            _ => unreachable!(),
        }
    }
}

impl From<R8> for u8 {
    fn from(value: R8) -> Self {
        value as u8
    }
}
