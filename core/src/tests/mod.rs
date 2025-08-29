use crate::console::components::cpu::registers::{R16, R8};
use crate::console::Console;

mod test_instructions;

impl Console {
    pub fn builder() -> ConsoleBuilder {
        ConsoleBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct ConsoleBuilder {
    console: Console,
    rom_offset: usize,
}

impl ConsoleBuilder {
    pub fn rom(mut self, value: u8) -> Self {
        self.console.bus.rom.data[self.rom_offset] = value;
        self.rom_offset += 1;
        self
    }

    pub fn write(mut self, address: u16, value: u8) -> Self {
        self.console.bus.write(address.into(), value.into());
        self
    }

    pub fn r8(mut self, r8: R8, value: u8) -> Self {
        self.console.cpu.set_r8(&mut self.console.bus, r8, value);
        self
    }

    pub fn r16(mut self, r16: R16, value: u16) -> Self {
        self.console.cpu.set_r16(r16, value);
        self
    }

    pub fn build(self) -> Console {
        self.console
    }
}
