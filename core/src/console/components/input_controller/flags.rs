use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct InputControllerFlags: u8 {
        const KEY_UP = 0b0000_0001;
        const KEY_DOWN = 0b0000_0010;
        const KEY_LEFT = 0b0000_0100;
        const KEY_RIGHT = 0b0000_1000;
        const KEY_A = 0b0001_0000;
        const KEY_B = 0b0010_0000;
        const KEY_START = 0b0100_0000;
        const TOUCH = 0b1000_0000;
    }
}

impl InputControllerFlags {
    #[inline(always)]
    pub fn set_bits(&mut self, value: u8) {
        *self = Self::from_bits_truncate(value);
    }
}
