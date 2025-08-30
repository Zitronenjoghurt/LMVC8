use crate::compiler::node::NodeType;
use crate::compiler::Compiler;

impl Compiler {
    pub fn breakpoint(self) -> Self {
        self.push_node(NodeType::BreakPoint)
    }
}
