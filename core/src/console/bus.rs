use crate::console::ram::RAM;
use crate::console::rom::ROM;
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;

pub struct Bus<'a> {
    pub rom: &'a mut ROM,
    pub ram: &'a mut RAM,
}

impl<'a> Bus<'a> {
    pub fn tick(&mut self) {}

    pub fn read(&mut self, addr: Address) -> Byte {
        self.tick();
        match u16::from(addr) {
            0x0..0x8000 => self.rom.read(addr),
            0x8000..=u16::MAX => self.ram.read(addr),
        }
    }

    pub fn write(&mut self, addr: Address, value: Byte) {
        self.tick();
        match u16::from(addr) {
            0x0..0x8000 => {}
            0x8000..=u16::MAX => self.ram.write(addr, value),
        }
    }
}
