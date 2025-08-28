use crate::console::components::cpu::step_flags::CPUStepFlags;

pub struct ConsoleStep {
    pub cycles: u64,
    pub cpu_step_flags: CPUStepFlags,
}
