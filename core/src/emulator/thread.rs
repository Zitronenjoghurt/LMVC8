use crate::console::Console;
use crate::emulator::command::{EmulatorCommand, EmulatorCommandReceiver};
use crate::emulator::event::EmulatorEventSender;
use crate::emulator::state::EmulatorState;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn emulator_thread(
    mut console: Console,
    state: Arc<Mutex<EmulatorState>>,
    command_receiver: EmulatorCommandReceiver,
    event_sender: EmulatorEventSender,
) {
    let mut running = false;
    let mut halt = false;
    let mut shutdown = false;

    loop {
        if shutdown {
            break;
        }

        if let Some(command) = command_receiver.poll() {
            match command {
                EmulatorCommand::Load(cartridge) => {
                    running = false;
                    halt = false;
                    match console.load_cartridge(*cartridge) {
                        Ok(_) => event_sender.cartridge_load_success(),
                        Err(_) => event_sender.cartridge_load_failed(),
                    }
                    update_state(&state, &console);
                }
                EmulatorCommand::Reset => {
                    running = false;
                    halt = false;
                    console.reset();
                }
                EmulatorCommand::Run => running = true,
                EmulatorCommand::Pause => running = false,
                EmulatorCommand::Shutdown => shutdown = true,
                EmulatorCommand::Step => {
                    if !running && !halt {
                        halt = emulator_step(&mut console, &state);
                    }
                }
            }
        }

        if running && !halt {
            halt = emulator_step(&mut console, &state);
        } else {
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    event_sender.shutdown(console);
}

fn emulator_step(console: &mut Console, state: &Arc<Mutex<EmulatorState>>) -> bool {
    let step = console.step();
    update_state(state, console);
    !step.do_continue
}

fn update_state(state: &Arc<Mutex<EmulatorState>>, console: &Console) {
    if let Ok(mut state_lock) = state.try_lock() {
        state_lock.update(console);
    }
}
