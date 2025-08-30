use crate::console::types::word::Word;
use binrw::{BinRead, BinWrite};
use std::fmt::{Display, Formatter};
use std::ops::{RangeBounds, Sub};

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, BinWrite, BinRead)]
#[brw(little)]
#[repr(transparent)]
pub struct Address(Word);

impl Address {
    #[inline(always)]
    pub fn new(value: Word) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub fn value(&self) -> Word {
        self.0
    }

    #[inline(always)]
    pub fn set_value(&mut self, value: Word) {
        self.0 = value;
    }

    #[inline(always)]
    pub fn is_in_range(&self, range: impl RangeBounds<u16>) -> bool {
        range.contains(&self.0.into())
    }
}

impl From<u16> for Address {
    #[inline(always)]
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl From<Address> for u16 {
    #[inline(always)]
    fn from(value: Address) -> Self {
        value.0.into()
    }
}

impl From<Word> for Address {
    #[inline(always)]
    fn from(value: Word) -> Self {
        Self::new(value)
    }
}

impl From<Address> for Word {
    #[inline(always)]
    fn from(value: Address) -> Self {
        value.value()
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.to_string().fmt(f)
    }
}

impl Sub for Address {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        self.0.sub_word(rhs.0).0.into()
    }
}
