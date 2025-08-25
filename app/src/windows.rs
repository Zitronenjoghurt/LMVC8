use crate::state::AppState;
use crate::views::View;
use egui::{Context, Id, Ui, WidgetText};

pub mod settings;

pub trait ViewWindow: Default {
    fn id(&self) -> Id;
    fn title(&self) -> impl Into<WidgetText>;
    fn is_open(&self) -> bool;
    fn set_open(&mut self, open: bool);
    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState);

    fn resizable(&self) -> bool {
        true
    }

    fn movable(&self) -> bool {
        true
    }

    fn collapsible(&self) -> bool {
        true
    }
}

impl<T> View for T
where
    T: ViewWindow,
{
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        if !self.is_open() {
            return;
        }

        let mut is_open = self.is_open();
        egui::Window::new(self.title())
            .id(self.id())
            .open(&mut is_open)
            .resizable(self.resizable())
            .collapsible(self.collapsible())
            .movable(self.movable())
            .show(ctx, |ui| {
                self.render_content(ui, state);
            });
        self.set_open(is_open)
    }
}

pub trait OpenableWindow {
    fn window_is_open(&self) -> bool;
    fn window_set_open(&mut self, open: bool);
}

impl<T> OpenableWindow for T
where
    T: ViewWindow,
{
    fn window_is_open(&self) -> bool {
        self.is_open()
    }

    fn window_set_open(&mut self, open: bool) {
        self.set_open(open)
    }
}

pub trait RenderableWindow {
    fn window_render(&mut self, ctx: &Context, state: &mut AppState);
}

impl<T> RenderableWindow for T
where
    T: ViewWindow,
{
    fn window_render(&mut self, ctx: &Context, state: &mut AppState) {
        self.render(ctx, state);
    }
}
