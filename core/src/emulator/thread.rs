use crate::console::step::ConsoleStep;
use crate::console::Console;
use crate::emulator::command::{EmulatorCommand, EmulatorCommandReceiver};
use crate::emulator::event::EmulatorEventSender;
use crate::emulator::state::EmulatorState;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const CYCLES_PER_SECOND: u64 = 600_000_000;
const FRAMES_PER_SECOND: u64 = 60;
const FRAME_TIME: Duration = Duration::from_micros(1_000_000 / FRAMES_PER_SECOND);
const CYCLES_PER_FRAME: u64 = CYCLES_PER_SECOND / FRAMES_PER_SECOND;

pub struct EmulatorThreadContext {
    console: Console,
    #[cfg(feature = "debugger")]
    debugger: crate::debugger::Debugger,
    state: Arc<Mutex<EmulatorState>>,
    command_receiver: EmulatorCommandReceiver,
    event_sender: EmulatorEventSender,
    running: bool,
    halt: bool,
    last_frame_mics: u64,
    frame_start: Instant,
}

impl EmulatorThreadContext {
    pub fn new(
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
            last_frame_mics: 0,
            frame_start: Instant::now(),
        }
    }

    pub fn run(mut self) {
        loop {
            self.frame_start = Instant::now();

            let _ = if self.running && !self.halt {
                self.run_frame()
            } else {
                0
            };

            if let Some(command) = self.command_receiver.poll() {
                let shutdown = self.handle_command(command);
                if shutdown {
                    break;
                }
            }

            self.update_state();

            let elapsed = self.frame_start.elapsed();
            self.last_frame_mics = elapsed.as_micros() as u64;
            let sleep_time = FRAME_TIME.saturating_sub(elapsed);
            std::thread::sleep(sleep_time);
        }

        self.event_sender.shutdown(self.console);
    }

    fn halt(&mut self) {
        self.halt = true;
        self.running = false;
        self.update_state();
    }

    fn run_frame(&mut self) -> u64 {
        let mut cycles = 0;

        while cycles < CYCLES_PER_FRAME {
            if self.halt || !self.running {
                break;
            }

            let step = self.step();
            cycles += step.cycles;
        }

        cycles
    }

    fn step(&mut self) -> ConsoleStep {
        let step = self.console.step();

        if !step.do_continue {
            self.halt();
        }

        #[cfg(feature = "debugger")]
        self.debug();

        step
    }

    fn update_state(&mut self) {
        if let Ok(mut state_lock) = self.state.try_lock() {
            state_lock.cpu_snapshot = self.console.cpu;
            state_lock.is_running = self.running;
            state_lock.is_halting = self.halt;
            state_lock.last_frame_mics = self.last_frame_mics;
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
