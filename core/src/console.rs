use crate::console::bus::Bus;

mod bus;
mod cpu;
mod ram;
mod types;

#[derive(Debug, Default, Clone)]
pub struct Console {
    cpu: cpu::CPU,
    ram: ram::RAM,
}

impl Console {
    pub fn step(&mut self) {
        let mut bus = Bus { ram: &mut self.ram };
        self.cpu.step(&mut bus);
    }
}
