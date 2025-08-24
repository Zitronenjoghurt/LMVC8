use binrw::{BinRead, BinWrite};

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
