use crate::compiler::node::{Node, NodeType};
use crate::compiler::Compiler;
use crate::console::components::cpu::instructions::CPUInstruction;

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_node(mut self, node_type: NodeType) -> Self {
        let address = self.push_position;
        self.push_position += node_type.size();
        let node = Node::new(node_type, address);
        self.nodes.push(node);
        self
    }

    pub fn push_data(self, data: Vec<u8>) -> Self {
        self.push_node(NodeType::Data(data))
    }

    pub fn push_byte(self, byte: u8) -> Self {
        self.push_node(NodeType::Data(vec![byte]))
    }

    pub fn push_word(self, word: u16) -> Self {
        self.push_node(NodeType::Data(word.to_le_bytes().to_vec()))
    }

    pub fn push_instruction(self, instruction: CPUInstruction) -> Self {
        self.push_node(NodeType::Instruction(instruction))
    }

    pub fn set_push_position(mut self, position: u16) -> Self {
        self.push_position = position;
        self
    }
}
