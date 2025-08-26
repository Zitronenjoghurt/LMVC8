use crate::console::types::byte::Byte;
use binrw::{BinRead, BinWrite};
use std::fmt::Display;

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, BinWrite, BinRead)]
#[brw(little)]
#[repr(transparent)]
pub struct Word(u16);

impl Word {
    #[inline]
    pub fn zero() -> Self {
        Self(0)
    }

    #[inline]
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    #[inline]
    pub fn from_le(low: Byte, high: Byte) -> Self {
        Self((u16::from(high)) << 8 | u16::from(low))
    }

    #[inline]
    pub fn value(&self) -> u16 {
        self.0
    }

    #[inline]
    pub fn set_value(&mut self, value: u16) {
        self.0 = value;
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.0 & 0b10000000_00000000 != 0
    }

    #[inline]
    pub fn low_byte(&self) -> Byte {
        Byte::new(self.0 as u8)
    }

    #[inline]
    pub fn high_byte(&self) -> Byte {
        Byte::new((self.0 >> 8) as u8)
    }

    #[inline]
    pub fn set_low_byte(&mut self, byte: Byte) {
        self.0 = (self.0 & 0xFF00) | u16::from(byte);
    }

    #[inline]
    pub fn set_high_byte(&mut self, byte: Byte) {
        self.0 = (self.0 & 0x00FF) | (u16::from(byte) << 8);
    }

    #[inline]
    pub fn increment(&self) -> (Self, bool) {
        let (result, overflow) = self.0.overflowing_add(1);
        (result.into(), overflow)
    }

    #[inline]
    pub fn decrement(&self) -> (Self, bool) {
        let (result, overflow) = self.0.overflowing_sub(1);
        (result.into(), overflow)
    }

    #[inline]
    pub fn add_byte_low(&self, byte: Byte) -> (Self, bool) {
        let (result, overflow) = self.0.overflowing_add(u16::from(byte));
        (result.into(), overflow)
    }

    #[inline]
    pub fn add_byte_high(&self, byte: Byte) -> (Self, bool) {
        let (result, overflow) = self.0.overflowing_add(u16::from(byte) << 8);
        (result.into(), overflow)
    }

    #[inline]
    pub fn add_word(&self, word: Word) -> (Self, bool) {
        let (result, overflow) = self.0.overflowing_add(word.0);
        (result.into(), overflow)
    }

    #[inline]
    pub fn sub_byte_low(&self, byte: Byte) -> (Self, bool) {
        let (result, overflow) = self.0.overflowing_sub(u16::from(byte));
        (result.into(), overflow)
    }

    #[inline]
    pub fn sub_byte_high(&self, byte: Byte) -> (Self, bool) {
        let (result, overflow) = self.0.overflowing_sub(u16::from(byte) << 8);
        (result.into(), overflow)
    }

    #[inline]
    pub fn sub_word(&self, word: Word) -> (Self, bool) {
        let (result, overflow) = self.0.overflowing_sub(word.0);
        (result.into(), overflow)
    }
}

impl From<u8> for Word {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value as u16)
    }
}

impl From<Word> for u8 {
    #[inline]
    fn from(word: Word) -> Self {
        word.0 as u8
    }
}

impl From<u16> for Word {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<Word> for u16 {
    #[inline]
    fn from(word: Word) -> Self {
        word.0
    }
}

impl Display for Word {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}
