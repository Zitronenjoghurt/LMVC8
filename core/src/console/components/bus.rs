use crate::console::components::ram::RAM;
use crate::console::components::rom::ROM;
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;

pub const ADR_RAM_START: u16 = 0x8000;

#[derive(Debug, Default, Clone)]
pub struct Bus {
    pub rom: ROM,
    pub ram: RAM,
    pub step_cycles: u64,
}

impl Bus {
    #[inline(always)]
    pub fn tick(&mut self) {
        self.step_cycles += 1;
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.ram.reset();
    }

    #[inline(always)]
    pub fn reset_step_cycles(&mut self) -> u64 {
        let cycles = self.step_cycles;
        self.step_cycles = 0;
        cycles
    }

    #[inline(always)]
    pub fn read(&mut self, addr: Address) -> Byte {
        self.tick();
        match u16::from(addr) {
            0x0..ADR_RAM_START => self.rom.read(addr),
            ADR_RAM_START..=u16::MAX => self.ram.read(addr - ADR_RAM_START.into()),
        }
    }

    #[inline(always)]
    pub fn write(&mut self, addr: Address, value: Byte) {
        self.tick();
        match u16::from(addr) {
            0x0..ADR_RAM_START => {}
            ADR_RAM_START..=u16::MAX => self.ram.write(addr - ADR_RAM_START.into(), value),
        }
    }
}
