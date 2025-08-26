use crate::console::Console;
use crate::emulator::command::{EmulatorCommand, EmulatorCommandReceiver};
use crate::emulator::event::EmulatorEventSender;
use crate::emulator::state::EmulatorState;
use std::sync::{Arc, Mutex};
use std::time::Duration;

struct EmulatorThreadContext {
    console: Console,
    #[cfg(feature = "debugger")]
    debugger: crate::debugger::Debugger,
    state: Arc<Mutex<EmulatorState>>,
    command_receiver: EmulatorCommandReceiver,
    event_sender: EmulatorEventSender,
    running: bool,
    halt: bool,
    nanos_per_cycle: u64,
}

impl EmulatorThreadContext {
    fn new(
        console: Console,
        #[cfg(feature = "debugger")] debugger: crate::debugger::Debugger,
        state: Arc<Mutex<EmulatorState>>,
        command_receiver: EmulatorCommandReceiver,
        event_sender: EmulatorEventSender,
    ) -> Self {
        Self {
            console,
            #[cfg(feature = "debugger")]
            debugger,
            state,
            command_receiver,
            event_sender,
            running: false,
            halt: false,
            nanos_per_cycle: 0,
        }
    }

    fn run(mut self) {
        loop {
            if let Some(command) = self.command_receiver.poll() {
                let shutdown = self.handle_command(command);
                if shutdown {
                    break;
                }
            }

            if self.running && !self.halt {
                self.step();
            } else {
                std::thread::sleep(Duration::from_millis(10));
            }
        }

        self.event_sender.shutdown(self.console);
    }

    fn halt(&mut self) {
        self.halt = true;
        self.running = false;
        self.update_state();
    }

    fn step(&mut self) {
        let before = std::time::Instant::now();
        let step = self.console.step();
        self.update_state();
        let after = std::time::Instant::now();

        let elapsed = after.duration_since(before);
        std::thread::sleep(
            Duration::from_nanos(self.nanos_per_cycle * step.cycles).saturating_sub(elapsed),
        );

        #[cfg(feature = "debugger")]
        self.debug();

        if !step.do_continue {
            self.halt();
        }
    }

    fn update_state(&mut self) {
        if let Ok(mut state_lock) = self.state.try_lock() {
            state_lock.cpu_snapshot = self.console.cpu;
            state_lock.is_running = self.running;
            state_lock.is_halting = self.halt;
            self.nanos_per_cycle = state_lock.nanos_per_cycle;
        }
    }

    #[cfg(feature = "debugger")]
    fn update_state_debug(&mut self) {
        if let Ok(mut state_lock) = self.state.try_lock() {
            state_lock.breakpoints = self.debugger.get_breakpoints().clone();
        }
    }

    fn handle_command(&mut self, command: EmulatorCommand) -> bool {
        match command {
            EmulatorCommand::Load(cartridge) => {
                self.running = false;
                self.halt = false;
                match self.console.load_cartridge(*cartridge) {
                    Ok(_) => self.event_sender.cartridge_load_success(),
                    Err(_) => self.event_sender.cartridge_load_failed(),
                }
                self.update_state();
            }
            EmulatorCommand::Reset => {
                self.running = false;
                self.halt = false;
                self.console.reset();
                self.update_state();
            }
            EmulatorCommand::Run => self.running = true,
            EmulatorCommand::Pause => {
                self.running = false;
                self.update_state();
            }
            EmulatorCommand::Shutdown => return true,
            EmulatorCommand::Step => {
                if !self.running && !self.halt {
                    self.step();
                }
            }
            #[cfg(feature = "debugger")]
            EmulatorCommand::SetBreakpoint(address) => {
                self.debugger.set_breakpoint(address);
                self.update_state_debug();
            }
            #[cfg(feature = "debugger")]
            EmulatorCommand::RemoveBreakpoint(address) => {
                self.debugger.remove_breakpoint(address);
                self.update_state_debug();
            }
        }

        false
    }

    #[cfg(feature = "debugger")]
    fn debug(&mut self) {
        let debugger_events = self.debugger.inspect(&self.console);

        for event in debugger_events {
            match event {
                crate::debugger::event::DebuggerEvent::Breakpoint => {
                    self.running = false;
                    self.update_state();
                }
            }
        }
    }
}

pub fn emulator_thread(
    console: Console,
    #[cfg(feature = "debugger")] debugger: crate::debugger::Debugger,
    state: Arc<Mutex<EmulatorState>>,
    command_receiver: EmulatorCommandReceiver,
    event_sender: EmulatorEventSender,
) {
    #[cfg(feature = "debugger")]
    let context = EmulatorThreadContext::new(
        console,
        #[cfg(feature = "debugger")]
        debugger,
        state,
        command_receiver,
        event_sender,
    );

    #[cfg(not(feature = "debugger"))]
    let context = EmulatorThreadContext::new(console, state, command_receiver, event_sender);
    context.run();
}
