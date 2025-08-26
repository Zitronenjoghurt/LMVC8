use crate::demos::Demo;
use crate::state::debugger::action::{DebuggerAction, DebuggerActionContext};
use lmvc8_core::console::cartridge::Cartridge;
use lmvc8_core::console::components::cpu::CPU;
use lmvc8_core::console::types::address::Address;
use lmvc8_core::disassembler::{DisassembledBinary, Disassembler};
use lmvc8_core::emulator::event::EmulatorEvent;
use lmvc8_core::emulator::Emulator;
use std::collections::HashSet;

pub mod action;

pub struct DebuggerState {
    emulator: Emulator,
    debugger_action_context: DebuggerActionContext,
    pub cpu_snapshot: CPU,
    pub is_running: bool,
    pub is_halting: bool,
    pub nanos_per_cycle: u64,
    pub cycles_per_second: u64,
    pub disassembled_binary: DisassembledBinary,
    pub breakpoints: HashSet<Address>,
}

impl Default for DebuggerState {
    fn default() -> Self {
        Self {
            emulator: Emulator::default(),
            debugger_action_context: DebuggerActionContext::default(),
            cpu_snapshot: CPU::default(),
            is_running: false,
            is_halting: false,
            nanos_per_cycle: 0,
            cycles_per_second: 0,
            disassembled_binary: DisassembledBinary::default(),
            breakpoints: HashSet::new(),
        }
    }
}

impl DebuggerState {
    pub fn update(&mut self) {
        self.emulator.with_state_mut(|state| {
            self.cpu_snapshot = state.cpu_snapshot;
            self.is_running = state.is_running;
            self.is_halting = state.is_halting;
            self.cycles_per_second = state.cycles_per_second;
            state.nanos_per_cycle = self.nanos_per_cycle;
            self.breakpoints = state.breakpoints.clone();
        });

        if let Some(event) = self.emulator.poll_event() {
            self.handle_event(event);
        };

        let debugger_actions = self
            .debugger_action_context
            .drain_actions()
            .collect::<Vec<_>>();
        for action in debugger_actions {
            self.handle_action(action);
        }
    }

    pub fn action_context(&self) -> &DebuggerActionContext {
        &self.debugger_action_context
    }

    pub fn reset(&self) {
        self.emulator.reset();
    }

    pub fn run(&self) {
        self.emulator.run();
    }

    pub fn pause(&self) {
        self.emulator.pause();
    }

    pub fn step(&self) {
        self.emulator.step();
    }

    pub fn format_cycles_per_second(&self) -> String {
        self.cycles_per_second
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",")
    }

    pub fn set_breakpoint(&self, address: u16) {
        self.emulator.set_breakpoint(address.into());
    }

    pub fn remove_breakpoint(&self, address: u16) {
        self.emulator.remove_breakpoint(address.into());
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

    fn handle_action(&mut self, action: DebuggerAction) {
        match action {
            DebuggerAction::SetBreakpoint(address) => self.set_breakpoint(address),
            DebuggerAction::RemoveBreakpoint(address) => self.remove_breakpoint(address),
        }
    }
}
