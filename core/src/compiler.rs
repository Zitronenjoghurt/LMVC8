use crate::compiler::node::Node;
use crate::console::components::cpu::instructions::CPUInstruction;

mod layers;
mod node;

#[derive(Debug, Default)]
pub struct Compiler {
    nodes: Vec<Node>,
    push_position: u16,
}

pub struct CompilationContext<'a> {
    data: &'a mut Vec<u8>,
}

impl Compiler {
    pub fn compile(self) -> Vec<u8> {
        let mut data = Vec::new();
        let mut context = CompilationContext { data: &mut data };
        self.push_instruction(CPUInstruction::Halt)
            .nodes
            .iter()
            .for_each(|node| node.compile(&mut context));
        data
    }
}
