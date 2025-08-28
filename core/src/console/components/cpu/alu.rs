use crate::console::types::byte::Byte;
use crate::console::types::word::Word;
use bitflags::bitflags;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ALU {
    flags: ALUFlags,
}

impl ALU {
    #[inline(always)]
    pub fn add_bytes(&mut self, a: Byte, b: Byte) -> Byte {
        let (result, carry) = a.add(b);
        let overflow = self.byte_add_overflow(a, b, result);
        self.flags.set(ALUFlags::ZERO, result.is_zero());
        self.flags.set(ALUFlags::CARRY, carry);
        self.flags.set(ALUFlags::NEGATIVE, result.is_negative());
        self.flags.set(ALUFlags::OVERFLOW, overflow);
        result
    }

    #[inline(always)]
    pub fn add_words(&mut self, a: Word, b: Word) -> Word {
        let (result, carry) = a.add_word(b);
        let overflow = self.word_add_overflow(a, b, result);
        self.flags.set(ALUFlags::ZERO, result.is_zero());
        self.flags.set(ALUFlags::CARRY, carry);
        self.flags.set(ALUFlags::NEGATIVE, result.is_negative());
        self.flags.set(ALUFlags::OVERFLOW, overflow);
        result
    }

    #[inline(always)]
    pub fn sub_bytes(&mut self, a: Byte, b: Byte) -> Byte {
        let (result, carry) = a.sub(b);
        let overflow = self.byte_sub_overflow(a, b, result);
        self.flags.set(ALUFlags::ZERO, result.is_zero());
        self.flags.set(ALUFlags::CARRY, carry);
        self.flags.set(ALUFlags::NEGATIVE, result.is_negative());
        self.flags.set(ALUFlags::OVERFLOW, overflow);
        result
    }

    #[inline(always)]
    pub fn sub_words(&mut self, a: Word, b: Word) -> Word {
        let (result, carry) = a.sub_word(b);
        let overflow = self.word_sub_overflow(a, b, result);
        self.flags.set(ALUFlags::ZERO, result.is_zero());
        self.flags.set(ALUFlags::CARRY, carry);
        self.flags.set(ALUFlags::NEGATIVE, result.is_negative());
        self.flags.set(ALUFlags::OVERFLOW, overflow);
        result
    }

    #[inline(always)]
    fn byte_add_overflow(&self, a: Byte, b: Byte, result: Byte) -> bool {
        (!(a.value() ^ b.value()) & (a.value() ^ result.value())) != 0
    }

    #[inline(always)]
    fn byte_sub_overflow(&self, a: Byte, b: Byte, result: Byte) -> bool {
        ((a.value() ^ b.value()) & (a.value() ^ result.value())) != 0
    }

    #[inline(always)]
    fn word_add_overflow(&self, a: Word, b: Word, result: Word) -> bool {
        (!(a.value() ^ b.value()) & (a.value() ^ result.value())) != 0
    }

    #[inline(always)]
    fn word_sub_overflow(&self, a: Word, b: Word, result: Word) -> bool {
        ((a.value() ^ b.value()) & (a.value() ^ result.value())) != 0
    }
}

bitflags! {
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ALUFlags: u8 {
        const ZERO = 0b0000_0001;
        const CARRY = 0b0000_0010;
        const NEGATIVE = 0b0000_0100;
        /// Signed overflow
        const OVERFLOW = 0b0000_1000;
    }
}

impl ALUFlags {
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.contains(ALUFlags::ZERO)
    }

    #[inline(always)]
    pub fn is_carry(&self) -> bool {
        self.contains(ALUFlags::CARRY)
    }

    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        self.contains(ALUFlags::NEGATIVE)
    }

    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        self.contains(ALUFlags::OVERFLOW)
    }
}
