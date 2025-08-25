use crate::state::AppState;
use crate::windows::RenderableWindow;

pub struct WindowRenderer<'a> {
    ctx: &'a egui::Context,
    state: &'a mut AppState,
    windows: Vec<&'a mut dyn RenderableWindow>,
}

impl<'a> WindowRenderer<'a> {
    pub fn new(ctx: &'a egui::Context, state: &'a mut AppState) -> Self {
        Self {
            ctx,
            state,
            windows: Vec::new(),
        }
    }

    pub fn window(mut self, window: &'a mut dyn RenderableWindow) -> Self {
        self.windows.push(window);
        self
    }

    pub fn render(self) {
        for window in self.windows {
            window.window_render(self.ctx, self.state);
        }
    }
}
