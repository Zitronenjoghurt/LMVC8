use crate::compiler::node::Node;
use crate::compiler::Compiler;
use crate::console::components::cpu::instructions::CPUInstruction;

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_data(mut self, data: Vec<u8>) -> Self {
        self.nodes.push(Node::data(data));
        self
    }

    pub fn push_byte(mut self, byte: u8) -> Self {
        self.nodes.push(Node::data(vec![byte]));
        self
    }

    pub fn push_word(mut self, word: u16) -> Self {
        self.nodes.push(Node::data(word.to_le_bytes().to_vec()));
        self
    }

    pub fn push_instruction(mut self, instruction: CPUInstruction) -> Self {
        self.nodes.push(Node::instruction(instruction));
        self
    }
}
