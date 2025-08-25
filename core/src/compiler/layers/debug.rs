use crate::compiler::node::Node;
use crate::compiler::Compiler;

impl Compiler {
    pub fn breakpoint(mut self) -> Self {
        self.nodes.push(Node::BreakPoint);
        self
    }
}
