use crate::console::components::cpu::instructions::CPUInstruction;
use crate::disassembler::node::Node;

pub mod node;

pub struct Disassembler<'a> {
    binary: &'a [u8],
    offset: usize,
    nodes: Vec<Node>,
}

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct DisassembledBinary(Vec<Node>);

impl DisassembledBinary {
    pub fn nodes(&self) -> &[Node] {
        &self.0
    }
}

impl<'a> Disassembler<'a> {
    pub fn new(binary: &'a [u8]) -> Self {
        Self {
            binary,
            offset: 0,
            nodes: Vec::new(),
        }
    }

    fn has_data(&self) -> bool {
        self.offset < self.binary.len()
    }

    pub fn disassemble(mut self) -> DisassembledBinary {
        while self.has_data() {
            let value = self.read();
            let instruction = CPUInstruction::from(value);
            self.push_instruction(instruction);

            let seek_count = instruction.byte_count() - 1;
            if seek_count > 0 {
                for _ in 0..seek_count {
                    self.push_read();
                }
            }
        }

        DisassembledBinary(self.nodes)
    }

    fn read(&mut self) -> u8 {
        let value = self.binary[self.offset];
        self.offset += 1;
        value
    }

    fn push_read(&mut self) {
        let value = self.read();
        self.push_byte(value);
    }

    fn push_byte(&mut self, byte: u8) {
        self.nodes.push(Node::byte(byte));
    }

    fn push_instruction(&mut self, instruction: CPUInstruction) {
        self.nodes.push(Node::instruction(instruction));
    }
}
