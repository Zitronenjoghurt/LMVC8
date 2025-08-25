use crate::console::Console;
use crate::emulator::command::{EmulatorCommand, EmulatorCommandReceiver};
use crate::emulator::event::EmulatorEventSender;

pub fn emulator_thread(
    mut console: Console,
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
                        halt = emulator_step(&mut console);
                    }
                }
            }
        }

        if running {
            halt = emulator_step(&mut console);
        }

        if halt {
            break;
        }
    }

    event_sender.halted(console);
}

fn emulator_step(console: &mut Console) -> bool {
    let step = console.step();
    !step.do_continue
}
