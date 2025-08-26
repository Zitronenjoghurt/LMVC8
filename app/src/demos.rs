use lmvc8_core::console::cartridge::Cartridge;

mod simple_add;

#[derive(Debug, Clone, Copy)]
pub enum Demo {
    SimpleAdd,
}

impl Demo {
    pub fn build_cartridge(&self) -> Cartridge {
        match self {
            Self::SimpleAdd => simple_add::build_cartridge(),
        }
    }
}
