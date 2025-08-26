use std::cell::RefCell;

pub enum DebuggerAction {
    SetBreakpoint(u16),
    RemoveBreakpoint(u16),
}

#[derive(Default)]
pub struct DebuggerActionContext {
    action_queue: RefCell<Vec<DebuggerAction>>,
}

impl DebuggerActionContext {
    pub fn push_action(&self, action: DebuggerAction) {
        if let Ok(mut queue) = self.action_queue.try_borrow_mut() {
            queue.push(action)
        }
    }

    pub fn drain_actions(&self) -> impl Iterator<Item = DebuggerAction> {
        if let Ok(mut queue) = self.action_queue.try_borrow_mut() {
            queue.drain(..).collect::<Vec<_>>().into_iter()
        } else {
            Vec::new().into_iter()
        }
    }

    pub fn set_breakpoint(&self, addr: u16) {
        self.push_action(DebuggerAction::SetBreakpoint(addr));
    }

    pub fn remove_breakpoint(&self, addr: u16) {
        self.push_action(DebuggerAction::RemoveBreakpoint(addr));
    }
}
