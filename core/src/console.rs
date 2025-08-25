use crate::console::step::ConsoleStep;
use components::{bus, cpu, ram, rom};

pub mod components;
mod step;
pub mod types;

#[derive(Debug, Default, Clone)]
pub struct Console {
    pub cpu: cpu::CPU,
    pub rom: rom::ROM,
    pub ram: ram::RAM,
}

impl Console {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_rom(mut self, rom: rom::ROM) -> Self {
        self.rom = rom;
        self
    }

    pub fn step(&mut self) -> ConsoleStep {
        let mut bus = bus::Bus {
            rom: &mut self.rom,
            ram: &mut self.ram,
            step_cycles: 0,
        };

        let do_continue = self.cpu.step(&mut bus);

        ConsoleStep {
            cycles: bus.step_cycles,
            do_continue,
        }
    }

    pub fn step_till_halt(&mut self) {
        while self.step().do_continue {}
    }
}
