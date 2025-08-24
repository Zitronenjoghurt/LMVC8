use crate::console::types::address::Address;
use crate::console::types::byte::Byte;

pub const ROM_SIZE: usize = 0x8000; // 32KiB

#[derive(Debug, Clone)]
pub struct ROM {
    data: [u8; ROM_SIZE],
}

impl Default for ROM {
    fn default() -> Self {
        Self {
            data: [0; ROM_SIZE],
        }
    }
}

impl ROM {
    pub fn read(&self, addr: Address) -> Byte {
        self.data[(u16::from(addr) & 0x8000) as usize].into()
    }
}
