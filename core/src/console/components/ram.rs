use crate::console::types::address::Address;
use crate::console::types::byte::Byte;

pub const RAM_SIZE: usize = 0x8000; // 32KiB

#[derive(Debug, Clone)]
pub struct RAM {
    data: [u8; RAM_SIZE],
}

impl Default for RAM {
    fn default() -> Self {
        Self {
            data: [0; RAM_SIZE],
        }
    }
}

impl RAM {
    pub fn read(&self, addr: Address) -> Byte {
        self.data[(u16::from(addr) & 0x7FFF) as usize].into()
    }

    pub fn write(&mut self, addr: Address, value: Byte) {
        self.data[(u16::from(addr) & 0x7FFF) as usize] = value.into();
    }
}
