use crate::console::components::cpu::CPU;
use crate::console::Console;

#[derive(Debug, Default)]
pub struct EmulatorState {
    pub cpu: CPU,
}

impl EmulatorState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, console: &Console) {
        self.cpu = console.cpu;
    }
}
