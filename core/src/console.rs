use crate::console::bus::Bus;
use crate::console::rom::ROM;

mod bus;
pub mod cpu;
mod ram;
mod rom;
pub mod types;

#[derive(Debug, Default, Clone)]
pub struct Console {
    cpu: cpu::CPU,
    rom: rom::ROM,
    ram: ram::RAM,
}

impl Console {
    pub fn new(rom: ROM) -> Self {
        Self {
            rom,
            ..Default::default()
        }
    }

    pub fn step(&mut self) {
        let mut bus = Bus {
            rom: &mut self.rom,
            ram: &mut self.ram,
        };
        self.cpu.step(&mut bus);
    }
}
