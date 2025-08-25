use crate::console::types::byte::Byte;
use crate::console::types::word::Word;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ALU {
    zero: bool,
    carry: bool,
    negative: bool,
}

impl ALU {
    pub fn add_bytes(&mut self, a: Byte, b: Byte) -> Byte {
        let (result, carry) = a.add(b);
        self.zero = result.is_zero();
        self.carry = carry;
        self.negative = result.is_negative();
        result
    }

    pub fn add_words(&mut self, a: Word, b: Word) -> Word {
        let (result, carry) = a.add_word(b);
        self.zero = result.is_zero();
        self.carry = carry;
        self.negative = result.is_negative();
        result
    }

    pub fn sub_bytes(&mut self, a: Byte, b: Byte) -> Byte {
        let (result, carry) = a.sub(b);
        self.zero = result.is_zero();
        self.carry = carry;
        self.negative = result.is_negative();
        result
    }

    pub fn sub_words(&mut self, a: Word, b: Word) -> Word {
        let (result, carry) = a.sub_word(b);
        self.zero = result.is_zero();
        self.carry = carry;
        self.negative = result.is_negative();
        result
    }
}
