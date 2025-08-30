use crate::console::components::cpu::interrupts::InterruptFlags;
use crate::console::components::input_controller::InputController;
use crate::console::components::ram::RAM;
use crate::console::components::rom::ROM;
use crate::console::input::ConsoleInput;
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;
use std::ops::{RangeInclusive, Sub};

#[derive(Debug, Default, Clone)]
pub struct Bus {
    pub rom: ROM,
    pub ram: RAM,
    pub ic: InputController,
    /// Interrupt enable, memory mapped, but CPU-internal register
    pub ie: InterruptFlags,
    /// Interrupt active, memory mapped, but CPU-internal register
    pub ia: InterruptFlags,
    pub step_cycles: u64,
}

impl Bus {
    // ROM
    pub const ROM_START: u16 = 0x0000;
    pub const ROM_END: u16 = 0x7FFF;
    pub const RANGE_ROM: RangeInclusive<u16> = Self::ROM_START..=Self::ROM_END;
    // RAM
    pub const RAM_START: u16 = 0x8000;
    pub const RAM_END: u16 = 0xFFFA;
    pub const RANGE_RAM: RangeInclusive<u16> = Self::RAM_START..=Self::RAM_END;
    pub const DEFAULT_SP: u16 = Self::RAM_END;
    // Input controller
    pub const IC_START: u16 = 0xFFFB;
    pub const IC_END: u16 = 0xFFFD;
    pub const RANGE_IC: RangeInclusive<u16> = Self::IC_START..=Self::IC_END;
    pub const INPUT_TOUCH_X: u16 = 0xFFFB;
    pub const INPUT_TOUCH_Y: u16 = 0xFFFC;
    pub const INPUT_FLAGS: u16 = 0xFFFD;
    // Special flag registers
    pub const SFR_START: u16 = 0xFFFE;
    pub const SFR_END: u16 = 0xFFFF;
    pub const RANGE_SFR: RangeInclusive<u16> = Self::SFR_START..=Self::SFR_END;
    pub const INTERRUPT_ACTIVE: u16 = 0xFFFE;
    pub const INTERRUPT_ENABLE: u16 = 0xFFFF;

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
        if !addr.is_in_range(Self::RANGE_SFR) {
            self.tick();
        }

        match u16::from(addr) {
            Self::ROM_START..=Self::ROM_END => self.rom.read(self.address_rom(addr)),
            Self::RAM_START..=Self::RAM_END => self.ram.read(self.address_ram(addr)),
            Self::IC_START..=Self::IC_END => self.ic.read(self.address_ic(addr)),
            Self::INTERRUPT_ACTIVE => self.ia.into(),
            Self::INTERRUPT_ENABLE => self.ie.into(),
            _ => unreachable!(),
        }
    }

    #[allow(unreachable_patterns)]
    #[inline(always)]
    pub fn write(&mut self, addr: Address, value: Byte) {
        if !addr.is_in_range(Self::RANGE_SFR) {
            self.tick();
        }

        match u16::from(addr) {
            Self::ROM_START..=Self::ROM_END => {}
            Self::RAM_START..=Self::RAM_END => self.ram.write(self.address_ram(addr), value),
            Self::IC_START..=Self::IC_END => self.ic.write(self.address_ic(addr), value),
            Self::INTERRUPT_ACTIVE => self.ia = value.into(),
            Self::INTERRUPT_ENABLE => self.ie = value.into(),
            _ => unreachable!(),
        }
    }

    pub fn input(&mut self, input: ConsoleInput) {
        self.ia.set_input();
        self.ic.input(input);
    }

    #[inline(always)]
    fn address_rom(&self, addr: Address) -> Address {
        addr
    }

    #[inline(always)]
    fn address_ram(&self, addr: Address) -> Address {
        addr.sub(Self::RAM_START.into())
    }

    #[inline(always)]
    fn address_ic(&self, addr: Address) -> Address {
        addr.sub(Self::IC_START.into())
    }
}

pub trait MemoryMapped {
    fn read(&mut self, addr: Address) -> Byte;
    fn write(&mut self, addr: Address, value: Byte);
}
