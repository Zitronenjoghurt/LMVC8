use crate::console::bus::Bus;
use crate::console::cpu::registers::CPURegisters;

mod registers;

#[derive(Debug, Default, Copy, Clone)]
pub struct CPU {
    registers: CPURegisters,
}

impl CPU {
    pub fn step(&mut self, bus: &mut Bus) {}
}
