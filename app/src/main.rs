use lmvc8_core::compiler::Compiler;
use lmvc8_core::console::components::cpu::registers::R8;
use lmvc8_core::console::Console;
use lmvc8_core::emulator::event::EmulatorEvent;
use lmvc8_core::emulator::Emulator;

fn main() {
    let rom = Compiler::new()
        .load_r8i(R8::A, 5)
        .load_r8i(R8::B, 12)
        .add_r8(R8::B)
        .compile();

    let console = Console::new().load_rom(rom);
    let emulator = Emulator::start(console);

    emulator.run();
    loop {
        if let Some(event) = emulator.poll_event() {
            match event {
                EmulatorEvent::Halted(console) => {
                    println!("{:?}", console.cpu);
                    break;
                }
            }
        }
    }
}
