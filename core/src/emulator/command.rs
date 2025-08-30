use crate::console::cartridge::Cartridge;
use crate::console::input::ConsoleInput;
use crate::console::types::address::Address;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub enum EmulatorCommand {
    Load(Box<Cartridge>),
    Step,
    Reset,
    Run,
    Pause,
    Shutdown,
    Input(ConsoleInput),
    SetClockSpeed(u64),
    #[cfg(feature = "debugger")]
    SetBreakpoint(Address),
    #[cfg(feature = "debugger")]
    RemoveBreakpoint(Address),
}

impl EmulatorCommand {
    pub fn channel() -> (EmulatorCommandSender, EmulatorCommandReceiver) {
        let (sender, receiver) = std::sync::mpsc::channel();
        (
            EmulatorCommandSender(sender),
            EmulatorCommandReceiver(receiver),
        )
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct EmulatorCommandSender(Sender<EmulatorCommand>);

impl EmulatorCommandSender {
    pub fn send(&self, command: EmulatorCommand) {
        self.0.send(command).ok();
    }

    pub fn pause(&self) {
        self.send(EmulatorCommand::Pause);
    }

    pub fn run(&self) {
        self.send(EmulatorCommand::Run);
    }

    pub fn step(&self) {
        self.send(EmulatorCommand::Step);
    }

    pub fn reset(&self) {
        self.send(EmulatorCommand::Reset);
    }

    pub fn shutdown(&self) {
        self.send(EmulatorCommand::Shutdown);
    }

    pub fn load(&self, cartridge: Box<Cartridge>) {
        self.send(EmulatorCommand::Load(cartridge));
    }

    pub fn set_clock_speed(&self, clock_speed: u64) {
        self.send(EmulatorCommand::SetClockSpeed(clock_speed));
    }

    #[cfg(feature = "debugger")]
    pub fn set_breakpoint(&self, address: Address) {
        self.send(EmulatorCommand::SetBreakpoint(address));
    }

    #[cfg(feature = "debugger")]
    pub fn remove_breakpoint(&self, address: Address) {
        self.send(EmulatorCommand::RemoveBreakpoint(address));
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct EmulatorCommandReceiver(Receiver<EmulatorCommand>);

impl EmulatorCommandReceiver {
    pub fn poll(&self) -> Option<EmulatorCommand> {
        self.0.try_recv().ok()
    }
}
