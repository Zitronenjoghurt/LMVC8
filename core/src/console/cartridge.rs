use crate::error::LMVC8Result;
use std::path::Path;

#[derive(Debug)]
pub struct Cartridge {
    pub binary: Vec<u8>,
}

impl Cartridge {
    pub fn new(binary: Vec<u8>) -> Self {
        Self { binary }
    }

    pub fn dump_to_file(&self, path: &Path) -> LMVC8Result<()> {
        Ok(std::fs::write(path, self.binary.clone())?)
    }

    pub fn load_from_file(path: &Path) -> LMVC8Result<Self> {
        let binary = std::fs::read(path)?;
        Ok(Self::new(binary))
    }
}
