use lmvc8_core::console::Console;
use lmvc8_core::emulator::command::EmulatorCommand;
use lmvc8_core::emulator::event::EmulatorEvent;
use lmvc8_core::emulator::state::EmulatorState;
use lmvc8_core::emulator::thread::EmulatorThreadContext;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let console = Console::new();
    let state = Arc::new(Mutex::new(EmulatorState::new()));
    let (command_sender, command_receiver) = EmulatorCommand::channel();
    let (event_sender, _) = EmulatorEvent::channel();

    let handle = std::thread::spawn(move || {
        command_sender.run();
        std::thread::sleep(Duration::from_secs(10));
        command_sender.shutdown();
    });

    let thread_context = EmulatorThreadContext::new(console, state, command_receiver, event_sender);
    thread_context.run();
    handle.join().unwrap();
}
