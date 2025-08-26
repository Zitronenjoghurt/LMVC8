use crate::console::components::cpu::CPU;

#[derive(Debug, Default)]
pub struct EmulatorState {
    pub cpu_snapshot: CPU,
    pub is_running: bool,
    pub is_halting: bool,
    #[cfg(feature = "debugger")]
    pub breakpoints: std::collections::HashSet<crate::console::types::address::Address>,
    pub last_frame_mics: u64,
}

impl EmulatorState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
