use lmvc8_core::compiler::Compiler;
use lmvc8_core::console::cartridge::Cartridge;
use lmvc8_core::console::components::cpu::registers::R8;

pub fn build_cartridge() -> Cartridge {
    let binary = Compiler::new()
        .load_r8i(R8::A, 12)
        .load_r8i(R8::B, 13)
        .add_r8(R8::B)
        .compile();
    Cartridge::new(binary)
}
