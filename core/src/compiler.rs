use crate::compiler::node::Node;
use crate::console::cpu::instructions::CPUInstruction;
use crate::console::rom::ROM;

mod layers;
mod node;

#[derive(Debug, Default)]
pub struct Compiler {
    nodes: Vec<Node>,
}

pub struct CompilationContext<'a> {
    data: &'a mut Vec<u8>,
}

impl Compiler {
    pub fn compile(self) -> ROM {
        let mut data = Vec::new();
        let mut context = CompilationContext { data: &mut data };
        self.push_instruction(CPUInstruction::Halt)
            .nodes
            .iter()
            .for_each(|node| node.compile(&mut context));
        ROM::new(data).unwrap()
    }
}
