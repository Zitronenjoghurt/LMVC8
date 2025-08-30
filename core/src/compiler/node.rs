use crate::compiler::CompilationContext;
use crate::console::components::cpu::instructions::CPUInstruction;

#[derive(Debug)]
pub enum NodeType {
    #[cfg(feature = "debugger")]
    BreakPoint,
    Data(Vec<u8>),
    Instruction(CPUInstruction),
}

impl NodeType {
    pub fn size(&self) -> u16 {
        match self {
            NodeType::BreakPoint => 0,
            NodeType::Data(data) => data.len() as u16,
            NodeType::Instruction(instr) => instr.byte_count() as u16,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub address: u16,
}

impl Node {
    pub fn new(node_type: NodeType, address: u16) -> Self {
        Self { node_type, address }
    }

    pub fn compile(&self, ctx: &mut CompilationContext) {
        match &self.node_type {
            #[cfg(feature = "debugger")]
            NodeType::BreakPoint => {}
            NodeType::Data(data) => ctx.data.extend(data),
            NodeType::Instruction(instr) => ctx.data.push(u8::try_from(*instr).unwrap()),
        }
    }
}
