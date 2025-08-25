use crate::compiler::Compiler;

impl Compiler {
    /// Repeats a set of compiler functions a specified amount of times
    pub fn repeat<F>(mut self, times: usize, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        for _ in 0..times {
            self = f(self);
        }
        self
    }
}
