mod bus;
pub mod cpu;
mod ram;
pub mod rom;
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

    pub fn load_rom(&mut self, rom: rom::ROM) {
        self.rom = rom;
    }

    pub fn step(&mut self) -> bool {
        let mut bus = bus::Bus {
            rom: &mut self.rom,
            ram: &mut self.ram,
        };
        self.cpu.step(&mut bus)
    }

    pub fn step_till_halt(&mut self) {
        while self.step() {}
    }
}
