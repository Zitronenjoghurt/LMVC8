use crate::console::components::cpu::interrupts::InterruptFlags;
use crate::console::components::ram::RAM;
use crate::console::components::rom::ROM;
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;

pub const ADDR_RAM_START: u16 = 0x8000;
pub const ADDR_SAFE_SP_START: u16 = 0xFFFD;
pub const ADDR_IA: u16 = 0xFFFE;
pub const ADDR_IE: u16 = 0xFFFF;

#[derive(Debug, Default, Clone)]
pub struct Bus {
    pub rom: ROM,
    pub ram: RAM,
    /// Interrupt enable, memory mapped, but CPU-internal register
    pub ie: InterruptFlags,
    /// Interrupt active, memory mapped, but CPU-internal register
    pub ia: InterruptFlags,
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
    pub fn take_step_cycles(&mut self) -> u64 {
        let cycles = self.step_cycles;
        self.step_cycles = 0;
        cycles
    }

    #[allow(unreachable_patterns)]
    #[inline(always)]
    pub fn read(&mut self, addr: Address) -> Byte {
        if addr < 0xFFFE.into() {
            self.tick();
        }

        match u16::from(addr) {
            0x0..ADDR_RAM_START => self.rom.read(addr),
            ADDR_RAM_START..=0xFFFD => self.ram.read(addr - ADDR_RAM_START.into()),
            ADDR_IA => self.ia.into(),
            ADDR_IE => self.ie.into(),
            _ => unreachable!(),
        }
    }

    #[allow(unreachable_patterns)]
    #[inline(always)]
    pub fn write(&mut self, addr: Address, value: Byte) {
        if addr < 0xFFFE.into() {
            self.tick();
        }

        match u16::from(addr) {
            0x0..ADDR_RAM_START => {}
            ADDR_RAM_START..=0xFFFD => self.ram.write(addr - ADDR_RAM_START.into(), value),
            ADDR_IA => self.ia = value.into(),
            ADDR_IE => self.ie = value.into(),
            _ => unreachable!(),
        }
    }
}
