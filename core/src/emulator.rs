use crate::console::cartridge::Cartridge;
use crate::console::types::address::Address;
use crate::console::Console;
use crate::emulator::command::{EmulatorCommand, EmulatorCommandSender};
use crate::emulator::event::{EmulatorEvent, EmulatorEventReceiver};
use crate::emulator::state::EmulatorState;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub mod command;
pub mod event;
pub mod state;
pub mod thread;

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
            #[cfg(not(feature = "debugger"))]
            thread::emulator_thread(console, thread_state, command_receiver, event_sender);

            #[cfg(feature = "debugger")]
            {
                let debugger = crate::debugger::Debugger::new();
                thread::emulator_thread(
                    console,
                    debugger,
                    thread_state,
                    command_receiver,
                    event_sender,
                );
            }
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

    pub fn pause(&self) {
        self.command_sender.pause();
    }

    pub fn step(&self) {
        self.command_sender.step()
    }

    pub fn reset(&self) {
        self.command_sender.reset();
    }

    pub fn set_clock_speed(&self, cycles_per_second: u64) {
        self.command_sender.set_clock_speed(cycles_per_second);
    }

    pub fn load_cartridge(&self, cartridge: Cartridge) {
        self.command_sender.load(Box::new(cartridge));
    }

    pub fn with_state<T, F>(&self, f: F) -> Option<T>
    where
        F: FnOnce(&EmulatorState) -> T,
    {
        let state_lock = self.state.try_lock().ok()?;
        Some(f(&state_lock))
    }

    pub fn with_state_mut<T, F>(&self, f: F) -> Option<T>
    where
        F: FnOnce(&mut EmulatorState) -> T,
    {
        let mut state_lock = self.state.try_lock().ok()?;
        Some(f(&mut state_lock))
    }

    #[cfg(feature = "debugger")]
    pub fn set_breakpoint(&self, address: Address) {
        self.command_sender.set_breakpoint(address);
    }

    #[cfg(feature = "debugger")]
    pub fn remove_breakpoint(&self, address: Address) {
        self.command_sender.remove_breakpoint(address);
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
