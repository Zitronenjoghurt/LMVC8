use crate::console::types::address::Address;
use crate::console::types::byte::Byte;
use crate::error::{LMVC8Error, LMVC8Result};
use std::path::Path;

pub const ROM_SIZE: usize = 0x8000; // 32KiB

#[derive(Debug, Clone)]
pub struct ROM {
    data: [u8; ROM_SIZE],
}

impl ROM {
    pub fn new(data: Vec<u8>) -> LMVC8Result<Self> {
        if data.len() > ROM_SIZE {
            return Err(LMVC8Error::ROMSizeExceeded);
        }

        let mut rom_data = [0u8; ROM_SIZE];
        rom_data[..data.len()].copy_from_slice(&data);

        Ok(Self { data: rom_data })
    }

    pub fn dump_to_file(&self, path: &Path) -> LMVC8Result<()> {
        Ok(std::fs::write(path, self.data)?)
    }

    pub fn load_from_file(path: &Path) -> LMVC8Result<Self> {
        let data = std::fs::read(path)?;
        Self::new(data)
    }
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
        self.data[(u16::from(addr) & 0x7FFF) as usize].into()
    }
}
