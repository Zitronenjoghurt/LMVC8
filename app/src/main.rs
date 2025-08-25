use lmvc8_core::compiler::Compiler;
use lmvc8_core::console::cpu::registers::{R16, R8};

fn main() {
    // Proof of concept
    let c = Compiler::new()
        .no_op()
        .add_r16(R16::SP)
        .add_r16(R16::HL)
        .repeat(5, |c| {
            c.add_r8(R8::A)
                .add_r8(R8::B)
                .add_r8(R8::C)
                .add_r8(R8::D)
                .add_r8(R8::E)
        });
}
