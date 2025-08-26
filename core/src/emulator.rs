use crate::console::components::cpu::CPU;
use crate::console::Console;
use crate::emulator::command::{EmulatorCommand, EmulatorCommandSender};
use crate::emulator::event::{EmulatorEvent, EmulatorEventReceiver};
use crate::emulator::state::EmulatorState;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub mod command;
pub mod event;
mod state;
mod thread;

pub struct Emulator {
    state: Arc<Mutex<EmulatorState>>,
    command_sender: EmulatorCommandSender,
    event_receiver: EmulatorEventReceiver,
    thread_handle: Option<JoinHandle<()>>,
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Emulator {
    pub fn new() -> Self {
        let (command_sender, command_receiver) = EmulatorCommand::channel();
        let (event_sender, event_receiver) = EmulatorEvent::channel();
        let state = Arc::new(Mutex::new(EmulatorState::new()));

        let console = Console::new();
        let thread_state = state.clone();
        let thread_handle = std::thread::spawn(move || {
            thread::emulator_thread(console, thread_state, command_receiver, event_sender);
        });

        Emulator {
            state,
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

    pub fn get_cpu_snapshot(&self) -> Option<CPU> {
        let state_lock = self.state.try_lock().ok()?;
        Some(state_lock.cpu_snapshot)
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
