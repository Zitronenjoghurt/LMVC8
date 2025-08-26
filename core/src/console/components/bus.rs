use crate::console::components::ram::RAM;
use crate::console::components::rom::ROM;
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;

pub struct Bus<'a> {
    pub rom: &'a mut ROM,
    pub ram: &'a mut RAM,
    pub step_cycles: u64,
}

impl<'a> Bus<'a> {
    #[inline(always)]
    pub fn tick(&mut self) {
        self.step_cycles += 1;
    }

    #[inline(always)]
    pub fn read(&mut self, addr: Address) -> Byte {
        self.tick();
        match u16::from(addr) {
            0x0..0x8000 => self.rom.read(addr),
            0x8000..=u16::MAX => self.ram.read(addr),
        }
    }

    #[inline(always)]
    pub fn write(&mut self, addr: Address, value: Byte) {
        self.tick();
        match u16::from(addr) {
            0x0..0x8000 => {}
            0x8000..=u16::MAX => self.ram.write(addr - 0x8000.into(), value),
        }
    }
}
