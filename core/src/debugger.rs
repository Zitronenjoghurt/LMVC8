use crate::console::types::address::Address;
use crate::console::Console;
use crate::debugger::event::DebuggerEvent;
use std::collections::HashSet;

pub mod event;

#[derive(Debug, Default, Clone)]
pub struct Debugger {
    breakpoints: HashSet<Address>,
}

impl Debugger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inspect(&mut self, console: &Console) -> Vec<DebuggerEvent> {
        let mut events = Vec::new();

        let pc = Address::from(console.cpu.get_pc());
        if self.breakpoints.contains(&pc) {
            events.push(DebuggerEvent::Breakpoint);
        }

        events
    }

    pub fn get_breakpoints(&self) -> &HashSet<Address> {
        &self.breakpoints
    }

    pub fn set_breakpoint(&mut self, address: Address) {
        self.breakpoints.insert(address);
    }

    pub fn remove_breakpoint(&mut self, address: Address) {
        self.breakpoints.remove(&address);
    }
}
