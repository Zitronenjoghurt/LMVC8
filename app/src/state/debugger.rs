use crate::demos::Demo;
use lmvc8_core::console::cartridge::Cartridge;
use lmvc8_core::console::components::cpu::CPU;
use lmvc8_core::disassembler::{DisassembledBinary, Disassembler};
use lmvc8_core::emulator::event::EmulatorEvent;
use lmvc8_core::emulator::Emulator;

#[derive(Default)]
pub struct DebuggerState {
    emulator: Emulator,
    pub cpu_snapshot: CPU,
    pub disassembled_binary: DisassembledBinary,
}

impl DebuggerState {
    pub fn update(&mut self) {
        if let Some(snapshot) = self.emulator.get_cpu_snapshot() {
            self.cpu_snapshot = snapshot;
        }

        if let Some(event) = self.emulator.poll_event() {
            self.handle_event(event);
        };
    }

    pub fn step(&self) {
        self.emulator.step();
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        self.disassembled_binary = Disassembler::new(&cartridge.binary).disassemble();
        self.emulator.load_cartridge(cartridge);
    }

    pub fn load_demo(&mut self, demo: Demo) {
        let cartridge = demo.build_cartridge();
        self.load_cartridge(cartridge);
    }

    fn handle_event(&mut self, event: EmulatorEvent) {
        match event {
            EmulatorEvent::CartridgeLoadSuccess => {}
            EmulatorEvent::CartridgeLoadFailed => {}
            EmulatorEvent::Shutdown(_) => {}
        }
    }
}
