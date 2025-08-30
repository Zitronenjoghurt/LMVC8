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

    /// Move to a certain position only for a certain context, then move back to the previous position
    pub fn position_context<F>(self, position: u16, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        let previous_position = self.push_position;
        f(self.set_push_position(position)).set_push_position(previous_position)
    }
}
