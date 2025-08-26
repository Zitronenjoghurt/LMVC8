use crate::console::cartridge::Cartridge;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub enum EmulatorCommand {
    Load(Box<Cartridge>),
    Step,
    Reset,
    Run,
    Pause,
    Shutdown,
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

    pub fn run(&self) {
        self.send(EmulatorCommand::Run);
    }

    pub fn step(&self) {
        self.send(EmulatorCommand::Step);
    }

    pub fn shutdown(&self) {
        self.send(EmulatorCommand::Shutdown);
    }

    pub fn load(&self, cartridge: Box<Cartridge>) {
        self.send(EmulatorCommand::Load(cartridge));
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
