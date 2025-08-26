use crate::console::types::word::Word;
use binrw::{BinRead, BinWrite};
use std::fmt::Display;

#[derive(Debug, Default, Copy, Clone, BinWrite, BinRead)]
#[repr(transparent)]
pub struct Byte(u8);

impl Byte {
    #[inline]
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    #[inline]
    pub fn value(&self) -> u8 {
        self.0
    }

    #[inline]
    pub fn set_value(&mut self, value: u8) {
        self.0 = value;
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_negative(&self) -> bool {
        self.0 & 0b1000_0000 != 0
    }

    #[inline]
    pub fn increment(&self) -> (Self, bool) {
        let (value, overflow) = self.0.overflowing_add(1);
        (value.into(), overflow)
    }

    #[inline]
    pub fn decrement(&self) -> (Self, bool) {
        let (value, overflow) = self.0.overflowing_sub(1);
        (value.into(), overflow)
    }

    #[inline]
    pub fn add(&self, byte: Byte) -> (Self, bool) {
        let (value, overflow) = self.0.overflowing_add(byte.0);
        (value.into(), overflow)
    }

    #[inline]
    pub fn sub(&self, byte: Byte) -> (Self, bool) {
        let (value, overflow) = self.0.overflowing_sub(byte.0);
        (value.into(), overflow)
    }
}

impl From<u8> for Byte {
    #[inline]
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl From<Byte> for u8 {
    #[inline]
    fn from(value: Byte) -> Self {
        value.0
    }
}

impl From<u16> for Byte {
    #[inline]
    fn from(value: u16) -> Self {
        Self::new(value as u8)
    }
}

impl From<Byte> for u16 {
    #[inline]
    fn from(value: Byte) -> Self {
        value.0 as u16
    }
}

impl Display for Byte {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:02X}", self.0)
    }
}
