use crate::console::Console;
use crate::emulator::command::{EmulatorCommand, EmulatorCommandReceiver};
use crate::emulator::event::EmulatorEventSender;
use crate::emulator::state::EmulatorState;
use std::sync::{Arc, Mutex};

pub fn emulator_thread(
    mut console: Console,
    state: Arc<Mutex<EmulatorState>>,
    command_receiver: EmulatorCommandReceiver,
    event_sender: EmulatorEventSender,
) {
    let mut running = false;
    let mut halt = false;

    loop {
        if let Some(command) = command_receiver.poll() {
            match command {
                EmulatorCommand::Run => running = true,
                EmulatorCommand::Pause => running = false,
                EmulatorCommand::Shutdown => halt = true,
                EmulatorCommand::Step => {
                    if !running {
                        halt = emulator_step(&mut console, &state);
                    }
                }
            }
        }

        if running {
            halt = emulator_step(&mut console, &state);
        }

        if halt {
            break;
        }
    }

    event_sender.halted(console);
}

fn emulator_step(console: &mut Console, state: &Arc<Mutex<EmulatorState>>) -> bool {
    let step = console.step();

    if let Ok(mut state_lock) = state.try_lock() {
        state_lock.update(console);
    }

    !step.do_continue
}
