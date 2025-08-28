use crate::console::cartridge::Cartridge;
use crate::console::components::rom::ROM;
use crate::console::step::ConsoleStep;
use crate::error::LMVC8Result;
use components::{bus, cpu};

pub mod cartridge;
pub mod components;
pub mod step;
pub mod types;

#[derive(Debug, Default, Clone)]
pub struct Console {
    pub cpu: cpu::CPU,
    pub bus: bus::Bus,
}

impl Console {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.bus.reset();
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) -> LMVC8Result<()> {
        self.reset();
        self.bus.rom = ROM::from_cartridge(cartridge)?;
        Ok(())
    }

    pub fn step(&mut self) -> ConsoleStep {
        let cpu_step_flags = self.cpu.step(&mut self.bus);
        let cycles = self.bus.take_step_cycles();

        ConsoleStep {
            cycles,
            cpu_step_flags,
        }
    }

    pub fn step_till_halt(&mut self) {
        loop {
            let step = self.step();
            if step.cpu_step_flags.is_halt() {
                break;
            }
        }
    }
}
