use crate::console::Console;
use crate::emulator::command::{EmulatorCommand, EmulatorCommandSender};
use crate::emulator::event::{EmulatorEvent, EmulatorEventReceiver};
use std::thread::JoinHandle;

pub mod command;
pub mod event;
mod thread;

pub struct Emulator {
    command_sender: EmulatorCommandSender,
    event_receiver: EmulatorEventReceiver,
    thread_handle: Option<JoinHandle<()>>,
}

impl Emulator {
    pub fn start(console: Console) -> Self {
        let (command_sender, command_receiver) = EmulatorCommand::channel();
        let (event_sender, event_receiver) = EmulatorEvent::channel();

        let thread_handle = std::thread::spawn(move || {
            thread::emulator_thread(console, command_receiver, event_sender);
        });

        Emulator {
            command_sender,
            event_receiver,
            thread_handle: Some(thread_handle),
        }
    }

    pub fn poll_event(&self) -> Option<EmulatorEvent> {
        self.event_receiver.poll()
    }

    pub fn run(&self) {
        self.command_sender.run();
    }
}

impl Drop for Emulator {
    fn drop(&mut self) {
        self.command_sender.shutdown();
        if let Some(thread_handle) = self.thread_handle.take() {
            thread_handle.join().ok();
        }
    }
}
