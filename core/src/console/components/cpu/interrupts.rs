use crate::console::types::byte::Byte;
use bitflags::bitflags;

// ISR Vectors
pub const IV_TIMER: u16 = 0x0040;
pub const IV_INPUT: u16 = 0x0048;

bitflags! {
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct InterruptFlags: u8 {
        const TIMER = 0b0000_0001;
        const INPUT = 0b0000_0010;
    }
}

impl InterruptFlags {
    #[inline(always)]
    pub fn first_set(&self) -> Option<Self> {
        let bits = self.bits();
        if bits == 0 {
            None
        } else {
            Some(Self::from_bits_truncate(1 << bits.trailing_zeros()))
        }
    }
}

impl From<Byte> for InterruptFlags {
    #[inline(always)]
    fn from(byte: Byte) -> Self {
        Self::from_bits_truncate(byte.value())
    }
}

impl From<InterruptFlags> for Byte {
    #[inline(always)]
    fn from(flags: InterruptFlags) -> Self {
        Byte::new(flags.bits())
    }
}
