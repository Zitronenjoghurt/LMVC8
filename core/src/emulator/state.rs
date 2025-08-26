use crate::console::components::cpu::CPU;
use crate::console::types::address::Address;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct EmulatorState {
    pub cpu_snapshot: CPU,
    pub is_running: bool,
    pub is_halting: bool,
    pub nanos_per_cycle: u64,
    #[cfg(feature = "debugger")]
    pub breakpoints: HashSet<Address>,
    pub cycles_per_second: u64,
}

impl EmulatorState {
    pub fn new() -> Self {
        Self {
            nanos_per_cycle: 10,
            ..Default::default()
        }
    }
}
