use crate::console::components::bus::Bus;
use crate::console::components::cpu::registers::{R16, R16S, R8};
use crate::console::types::address::Address;
use crate::console::types::byte::Byte;
use crate::console::types::word::Word;
use crate::console::Console;
use rstest::rstest;

const OP_HALT: u8 = 0x10;
const OP_LDR8_A_B: u8 = 0x20;
const OP_LDR8_A_C: u8 = 0x21;
const OP_LDR8_A_D: u8 = 0x22;
const OP_LDR8_A_E: u8 = 0x23;
const OP_LDR8_A_H: u8 = 0x24;
const OP_LDR8_A_L: u8 = 0x25;
const OP_LDR8_A_HL: u8 = 0x26;
const OP_LDR8_B_A: u8 = 0x27;
const OP_LDR8_B_C: u8 = 0x28;
const OP_LDR8_B_D: u8 = 0x29;
const OP_LDR8_B_E: u8 = 0x2A;
const OP_LDR8_B_H: u8 = 0x2B;
const OP_LDR8_B_L: u8 = 0x2C;
const OP_LDR8_B_HL: u8 = 0x2D;
const OP_LDR8_C_A: u8 = 0x2E;
const OP_LDR8_C_B: u8 = 0x2F;
const OP_LDR8_C_D: u8 = 0x30;
const OP_LDR8_C_E: u8 = 0x31;
const OP_LDR8_C_H: u8 = 0x32;
const OP_LDR8_C_L: u8 = 0x33;
const OP_LDR8_C_HL: u8 = 0x34;
const OP_LDR8_D_A: u8 = 0x35;
const OP_LDR8_D_B: u8 = 0x36;
const OP_LDR8_D_C: u8 = 0x37;
const OP_LDR8_D_E: u8 = 0x38;
const OP_LDR8_D_H: u8 = 0x39;
const OP_LDR8_D_L: u8 = 0x3A;
const OP_LDR8_D_HL: u8 = 0x3B;
const OP_LDR8_E_A: u8 = 0x3C;
const OP_LDR8_E_B: u8 = 0x3D;
const OP_LDR8_E_C: u8 = 0x3E;
const OP_LDR8_E_D: u8 = 0x3F;
const OP_LDR8_E_H: u8 = 0x40;
const OP_LDR8_E_L: u8 = 0x41;
const OP_LDR8_E_HL: u8 = 0x42;
const OP_LDR8_H_A: u8 = 0x43;
const OP_LDR8_H_B: u8 = 0x44;
const OP_LDR8_H_C: u8 = 0x45;
const OP_LDR8_H_D: u8 = 0x46;
const OP_LDR8_H_E: u8 = 0x47;
const OP_LDR8_H_L: u8 = 0x48;
const OP_LDR8_H_HL: u8 = 0x49;
const OP_LDR8_L_A: u8 = 0x4A;
const OP_LDR8_L_B: u8 = 0x4B;
const OP_LDR8_L_C: u8 = 0x4C;
const OP_LDR8_L_D: u8 = 0x4D;
const OP_LDR8_L_E: u8 = 0x4E;
const OP_LDR8_L_H: u8 = 0x4F;
const OP_LDR8_L_HL: u8 = 0x50;
const OP_LDR8_HL_A: u8 = 0x51;
const OP_LDR8_HL_B: u8 = 0x52;
const OP_LDR8_HL_C: u8 = 0x53;
const OP_LDR8_HL_D: u8 = 0x54;
const OP_LDR8_HL_E: u8 = 0x55;
const OP_LDR8_HL_H: u8 = 0x56;
const OP_LDR8_HL_L: u8 = 0x57;
const OP_PUSH_AF: u8 = 0x80;
const OP_PUSH_BC: u8 = 0x81;
const OP_PUSH_DE: u8 = 0x82;
const OP_PUSH_HL: u8 = 0x83;
const OP_POP_AF: u8 = 0x84;
const OP_POP_BC: u8 = 0x85;
const OP_POP_DE: u8 = 0x86;
const OP_POP_HL: u8 = 0x87;

#[rstest]
#[case::ldr8_a_b(OP_LDR8_A_B, R8::A, R8::B)]
#[case::ldr8_a_c(OP_LDR8_A_C, R8::A, R8::C)]
#[case::ldr8_a_d(OP_LDR8_A_D, R8::A, R8::D)]
#[case::ldr8_a_e(OP_LDR8_A_E, R8::A, R8::E)]
#[case::ldr8_a_h(OP_LDR8_A_H, R8::A, R8::H)]
#[case::ldr8_a_l(OP_LDR8_A_L, R8::A, R8::L)]
#[case::ldr8_a_hl(OP_LDR8_A_HL, R8::A, R8::HL)]
#[case::ldr8_b_a(OP_LDR8_B_A, R8::B, R8::A)]
#[case::ldr8_b_c(OP_LDR8_B_C, R8::B, R8::C)]
#[case::ldr8_b_d(OP_LDR8_B_D, R8::B, R8::D)]
#[case::ldr8_b_e(OP_LDR8_B_E, R8::B, R8::E)]
#[case::ldr8_b_h(OP_LDR8_B_H, R8::B, R8::H)]
#[case::ldr8_b_l(OP_LDR8_B_L, R8::B, R8::L)]
#[case::ldr8_b_hl(OP_LDR8_B_HL, R8::B, R8::HL)]
#[case::ldr8_c_a(OP_LDR8_C_A, R8::C, R8::A)]
#[case::ldr8_c_b(OP_LDR8_C_B, R8::C, R8::B)]
#[case::ldr8_c_d(OP_LDR8_C_D, R8::C, R8::D)]
#[case::ldr8_c_e(OP_LDR8_C_E, R8::C, R8::E)]
#[case::ldr8_c_h(OP_LDR8_C_H, R8::C, R8::H)]
#[case::ldr8_c_l(OP_LDR8_C_L, R8::C, R8::L)]
#[case::ldr8_c_hl(OP_LDR8_C_HL, R8::C, R8::HL)]
#[case::ldr8_d_a(OP_LDR8_D_A, R8::D, R8::A)]
#[case::ldr8_d_b(OP_LDR8_D_B, R8::D, R8::B)]
#[case::ldr8_d_c(OP_LDR8_D_C, R8::D, R8::C)]
#[case::ldr8_d_e(OP_LDR8_D_E, R8::D, R8::E)]
#[case::ldr8_d_h(OP_LDR8_D_H, R8::D, R8::H)]
#[case::ldr8_d_l(OP_LDR8_D_L, R8::D, R8::L)]
#[case::ldr8_d_hl(OP_LDR8_D_HL, R8::D, R8::HL)]
#[case::ldr8_e_a(OP_LDR8_E_A, R8::E, R8::A)]
#[case::ldr8_e_b(OP_LDR8_E_B, R8::E, R8::B)]
#[case::ldr8_e_c(OP_LDR8_E_C, R8::E, R8::C)]
#[case::ldr8_e_d(OP_LDR8_E_D, R8::E, R8::D)]
#[case::ldr8_e_h(OP_LDR8_E_H, R8::E, R8::H)]
#[case::ldr8_e_l(OP_LDR8_E_L, R8::E, R8::L)]
#[case::ldr8_e_hl(OP_LDR8_E_HL, R8::E, R8::HL)]
#[case::ldr8_h_a(OP_LDR8_H_A, R8::H, R8::A)]
#[case::ldr8_h_b(OP_LDR8_H_B, R8::H, R8::B)]
#[case::ldr8_h_c(OP_LDR8_H_C, R8::H, R8::C)]
#[case::ldr8_h_d(OP_LDR8_H_D, R8::H, R8::D)]
#[case::ldr8_h_e(OP_LDR8_H_E, R8::H, R8::E)]
#[case::ldr8_h_l(OP_LDR8_H_L, R8::H, R8::L)]
#[case::ldr8_l_a(OP_LDR8_L_A, R8::L, R8::A)]
#[case::ldr8_l_b(OP_LDR8_L_B, R8::L, R8::B)]
#[case::ldr8_l_c(OP_LDR8_L_C, R8::L, R8::C)]
#[case::ldr8_l_d(OP_LDR8_L_D, R8::L, R8::D)]
#[case::ldr8_l_e(OP_LDR8_L_E, R8::L, R8::E)]
#[case::ldr8_l_h(OP_LDR8_L_H, R8::L, R8::H)]
#[case::ldr8_hl_a(OP_LDR8_HL_A, R8::HL, R8::A)]
#[case::ldr8_hl_b(OP_LDR8_HL_B, R8::HL, R8::B)]
#[case::ldr8_hl_c(OP_LDR8_HL_C, R8::HL, R8::C)]
#[case::ldr8_hl_d(OP_LDR8_HL_D, R8::HL, R8::D)]
#[case::ldr8_hl_e(OP_LDR8_HL_E, R8::HL, R8::E)]
fn test_load_r8(#[case] opcode: u8, #[case] r8_1: R8, #[case] r8_2: R8) {
    const VALUE_1: u8 = 0x71;
    const VALUE_2: u8 = 0x23;

    let mut console = Console::builder()
        .r16(R16::HL, Bus::RAM_START)
        .r8(r8_1, VALUE_1)
        .r8(r8_2, VALUE_2)
        .rom(opcode)
        .rom(OP_HALT)
        .build();

    assert_eq!(
        console.cpu.get_registers().get_r8(&mut console.bus, r8_1),
        Byte::new(VALUE_1)
    );
    assert_eq!(
        console.cpu.get_registers().get_r8(&mut console.bus, r8_2),
        Byte::new(VALUE_2)
    );

    console.step_till_halt();

    assert_eq!(
        console.cpu.get_registers().get_r8(&mut console.bus, r8_1),
        Byte::new(VALUE_2)
    );
    assert_eq!(
        console.cpu.get_registers().get_r8(&mut console.bus, r8_2),
        Byte::new(VALUE_2)
    );
}

#[rstest]
#[case::ldr8_h_hl(OP_LDR8_H_HL, 0x71, R8::H, 0x71)]
#[case::ldr8_l_hl(OP_LDR8_L_HL, 0x71, R8::L, 0x71)]
#[case::ldr8_hl_h(OP_LDR8_HL_H, 0x71, R8::HL, (Bus::RAM_START >> 8) as u8)]
#[case::ldr8_hl_l(OP_LDR8_HL_L, 0x71, R8::HL, Bus::RAM_START as u8)]
fn test_load_r8_h_l_hl(
    #[case] opcode: u8,
    #[case] value_hl: u8,
    #[case] r8_test: R8,
    #[case] expected: u8,
) {
    let mut console = Console::builder()
        .r16(R16::HL, Bus::RAM_START)
        .r8(R8::HL, value_hl)
        .rom(opcode)
        .rom(OP_HALT)
        .build();

    console.step_till_halt();

    assert_eq!(
        console
            .cpu
            .get_registers()
            .get_r8(&mut console.bus, r8_test),
        Byte::new(expected)
    );
}

#[rstest]
#[case::push_af(OP_PUSH_AF, R16S::AF)]
#[case::push_bc(OP_PUSH_BC, R16S::BC)]
#[case::push_de(OP_PUSH_DE, R16S::DE)]
#[case::push_hl(OP_PUSH_HL, R16S::HL)]
fn test_push(#[case] opcode: u8, #[case] register: R16S) {
    const VALUE: u16 = 0x1234;

    let mut console = Console::builder()
        .r16(R16::SP, Bus::DEFAULT_SP)
        .r16(R16::BC, VALUE)
        .r16(R16::DE, VALUE)
        .r16(R16::HL, VALUE)
        .r8(R8::A, (VALUE >> 8) as u8)
        .rom(opcode)
        .rom(OP_HALT)
        .build();

    console.step_till_halt();

    assert_eq!(
        console.bus.read(Address::from(Bus::DEFAULT_SP)),
        Byte::new((VALUE >> 8) as u8)
    );

    if register != R16S::AF {
        assert_eq!(
            console.bus.read(Address::from(Bus::DEFAULT_SP - 1)),
            Byte::new(VALUE as u8)
        );
    }
}

#[rstest]
#[case::pop_af(OP_POP_AF, R16S::AF)]
#[case::pop_bc(OP_POP_BC, R16S::BC)]
#[case::pop_de(OP_POP_DE, R16S::DE)]
#[case::pop_hl(OP_POP_HL, R16S::HL)]
fn test_op(#[case] opcode: u8, #[case] register: R16S) {
    let mut console = Console::builder()
        .r16(R16::SP, Bus::DEFAULT_SP - 1)
        .write(Bus::DEFAULT_SP, 0x75)
        .write(Bus::DEFAULT_SP - 1, 0x01)
        .rom(opcode)
        .rom(OP_HALT)
        .build();

    console.step_till_halt();

    match register {
        R16S::AF => {
            assert_eq!(
                console.cpu.get_registers().get_r8(&mut console.bus, R8::A),
                Byte::new(0x75)
            );
            assert_eq!(console.cpu.get_alu().get_flags().bits(), 0x01)
        }
        R16S::BC => {
            assert_eq!(
                console.cpu.get_registers().get_r16(R16::BC),
                Word::new(0x7501)
            );
        }
        R16S::DE => {
            assert_eq!(
                console.cpu.get_registers().get_r16(R16::DE),
                Word::new(0x7501)
            );
        }
        R16S::HL => {
            assert_eq!(
                console.cpu.get_registers().get_r16(R16::HL),
                Word::new(0x7501)
            );
        }
    }
}
