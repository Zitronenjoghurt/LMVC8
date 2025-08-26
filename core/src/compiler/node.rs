use crate::compiler::CompilationContext;
use crate::console::components::cpu::instructions::CPUInstruction;

#[derive(Debug)]
pub enum Node {
    #[cfg(feature = "debugger")]
    BreakPoint,
    Data(Vec<u8>),
    Instruction(CPUInstruction),
}

impl Node {
    pub fn data(data: Vec<u8>) -> Self {
        Self::Data(data)
    }

    pub fn instruction(instruction: CPUInstruction) -> Self {
        Self::Instruction(instruction)
    }

    pub fn compile(&self, ctx: &mut CompilationContext) {
        match self {
            #[cfg(feature = "debugger")]
            Self::BreakPoint => {}
            Self::Data(data) => ctx.data.extend(data),
            Self::Instruction(instr) => ctx.data.push(u8::try_from(*instr).unwrap()),
        }
    }
}
