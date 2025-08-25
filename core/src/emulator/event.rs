use crate::console::Console;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub enum EmulatorEvent {
    Halted(Box<Console>),
}

impl EmulatorEvent {
    pub fn channel() -> (EmulatorEventSender, EmulatorEventReceiver) {
        let (sender, receiver) = std::sync::mpsc::channel();
        (EmulatorEventSender(sender), EmulatorEventReceiver(receiver))
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct EmulatorEventSender(Sender<EmulatorEvent>);

impl EmulatorEventSender {
    pub fn send(&self, event: EmulatorEvent) {
        self.0.send(event).ok();
    }

    pub fn halted(&self, console: Console) {
        self.send(EmulatorEvent::Halted(Box::new(console)));
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct EmulatorEventReceiver(Receiver<EmulatorEvent>);

impl EmulatorEventReceiver {
    pub fn poll(&self) -> Option<EmulatorEvent> {
        self.0.try_recv().ok()
    }
}
