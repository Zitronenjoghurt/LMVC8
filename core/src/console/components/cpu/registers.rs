use crate::console::components::bus::Bus;
use crate::console::components::cpu::alu::ALUFlags;
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;
use crate::console::types::word::Word;
use std::fmt::Display;

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
    #[inline(always)]
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

    #[inline(always)]
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

    #[inline(always)]
    pub fn increment_r8(&mut self, bus: &mut Bus, r8: R8) {
        let before_value = self.get_r8(bus, r8);
        self.set_r8(bus, r8, before_value.increment().0);
    }

    #[inline(always)]
    pub fn decrement_r8(&mut self, bus: &mut Bus, r8: R8) {
        let before_value = self.get_r8(bus, r8);
        self.set_r8(bus, r8, before_value.decrement().0);
    }

    #[inline(always)]
    pub fn get_r16(&self, r16: R16) -> Word {
        match r16 {
            R16::BC => Word::from_le(self.c, self.b),
            R16::DE => Word::from_le(self.e, self.d),
            R16::HL => Word::from_le(self.l, self.h),
            R16::SP => self.sp,
        }
    }

    #[inline(always)]
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

    #[inline(always)]
    pub fn increment_r16(&mut self, r16: R16) {
        self.set_r16(r16, self.get_r16(r16).increment().0);
    }

    #[inline(always)]
    pub fn decrement_r16(&mut self, r16: R16) {
        self.set_r16(r16, self.get_r16(r16).decrement().0);
    }

    #[inline(always)]
    pub fn get_r16s(&self, r16s: R16S, flags: ALUFlags) -> Word {
        match r16s {
            R16S::AF => Word::from_le(Byte::new(flags.bits()), self.a),
            R16S::BC => Word::from_le(self.c, self.b),
            R16S::DE => Word::from_le(self.e, self.d),
            R16S::HL => Word::from_le(self.l, self.h),
        }
    }

    #[inline(always)]
    pub fn set_r16s(&mut self, r16s: R16S, flags: &mut ALUFlags, word: Word) {
        match r16s {
            R16S::AF => {
                self.a = word.high_byte();
                flags.set_bits(word.low_byte().value());
            }
            R16S::BC => {
                self.b = word.high_byte();
                self.c = word.low_byte();
            }
            R16S::DE => {
                self.d = word.high_byte();
                self.e = word.low_byte();
            }
            R16S::HL => {
                self.h = word.high_byte();
                self.l = word.low_byte();
            }
        }
    }
}

/// Outside access
impl GeneralRegisters {
    pub fn get_a(&self) -> Byte {
        self.a
    }

    pub fn get_b(&self) -> Byte {
        self.b
    }

    pub fn get_c(&self) -> Byte {
        self.c
    }

    pub fn get_d(&self) -> Byte {
        self.d
    }

    pub fn get_e(&self) -> Byte {
        self.e
    }

    pub fn get_h(&self) -> Byte {
        self.h
    }

    pub fn get_l(&self) -> Byte {
        self.l
    }

    pub fn get_sp(&self) -> Word {
        self.sp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
    pub const ACC: R8 = R8::A;
    pub const ALL: [R8; 8] = [R8::A, R8::B, R8::C, R8::D, R8::E, R8::H, R8::L, R8::HL];
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
pub enum R16 {
    BC = 0,
    DE = 1,
    HL = 2,
    SP = 3,
}

impl R16 {
    pub const ACC: R16 = R16::BC;
    pub const ALL: [R16; 4] = [R16::BC, R16::DE, R16::HL, R16::SP];
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
/// Register pairs for stack operations
pub enum R16S {
    AF = 0,
    BC = 1,
    DE = 2,
    HL = 3,
}

impl R16S {
    pub const ALL: [R16S; 4] = [R16S::AF, R16S::BC, R16S::DE, R16S::HL];
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

impl Display for R8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

impl Display for R16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<R16S> for u8 {
    fn from(value: R16S) -> Self {
        value as u8
    }
}

impl Display for R16S {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
