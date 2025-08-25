use crate::console::bus::Bus;
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;
use crate::console::types::word::Word;

#[derive(Debug, Default, Clone, Copy)]
pub struct GeneralRegisters {
    /// Accumulator for 8-Bit operations
    /// For 16-bit operations, BC will act as the accumulator.
    a: Byte,
    b: Byte,
    c: Byte,
    d: Byte,
    e: Byte,
    h: Byte,
    l: Byte,
    sp: Word,
}

impl GeneralRegisters {
    pub fn get_r8(&self, bus: &mut Bus, r8: R8) -> Byte {
        match r8 {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::H => self.h,
            R8::L => self.l,
            R8::HL => bus.read(Address::from(self.get_r16(R16::HL))),
        }
    }

    pub fn set_r8(&mut self, bus: &mut Bus, r8: R8, byte: Byte) {
        match r8 {
            R8::A => self.a = byte,
            R8::B => self.b = byte,
            R8::C => self.c = byte,
            R8::D => self.d = byte,
            R8::E => self.e = byte,
            R8::H => self.h = byte,
            R8::L => self.l = byte,
            R8::HL => bus.write(Address::from(self.get_r16(R16::HL)), byte),
        }
    }

    pub fn get_r16(&self, r16: R16) -> Word {
        match r16 {
            R16::BC => Word::from_le(self.c, self.b),
            R16::DE => Word::from_le(self.e, self.d),
            R16::HL => Word::from_le(self.l, self.h),
            R16::SP => self.sp,
        }
    }

    pub fn set_r16(&mut self, r16: R16, word: Word) {
        match r16 {
            R16::BC => {
                self.b = word.high_byte();
                self.c = word.low_byte();
            }
            R16::DE => {
                self.d = word.high_byte();
                self.e = word.low_byte();
            }
            R16::HL => {
                self.h = word.high_byte();
                self.l = word.low_byte();
            }
            R16::SP => self.sp = word,
        }
    }

    pub fn increment_r16(&mut self, r16: R16) {
        match r16 {
            R16::BC => self.set_r16(R16::BC, self.get_r16(R16::BC).increment().0),
            R16::DE => self.set_r16(R16::DE, self.get_r16(R16::DE).increment().0),
            R16::HL => self.set_r16(R16::HL, self.get_r16(R16::HL).increment().0),
            R16::SP => self.sp = self.sp.increment().0,
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
    H = 5,
    L = 6,
    HL = 7,
}

impl R8 {
    pub const ACC: Self = Self::A;
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum R16 {
    BC = 0,
    DE = 1,
    HL = 2,
    SP = 3,
}

impl R16 {
    pub const ACC: Self = Self::BC;
}

impl From<u8> for R8 {
    fn from(value: u8) -> Self {
        match value & 0b111 {
            0 => R8::A,
            1 => R8::B,
            2 => R8::C,
            3 => R8::D,
            4 => R8::E,
            5 => R8::H,
            6 => R8::L,
            7 => R8::HL,
            _ => unreachable!(),
        }
    }
}

impl From<R8> for u8 {
    fn from(value: R8) -> Self {
        value as u8
    }
}

impl From<u8> for R16 {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0 => R16::BC,
            1 => R16::DE,
            2 => R16::HL,
            3 => R16::SP,
            _ => unreachable!(),
        }
    }
}

impl From<R16> for u8 {
    fn from(value: R16) -> Self {
        value as u8
    }
}
