use lmvc8_core::compiler::Compiler;
use lmvc8_core::console::cpu::registers::R8;

fn main() {
    // Proof of concept
    let rom = Compiler::new()
        .load_r8i(R8::A, 5)
        .load_r8i(R8::B, 12)
        .add_r8(R8::B)
        .compile();
}
