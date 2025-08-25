use crate::state::AppState;
use egui::{Context, Ui};

pub mod nav_bar;
pub mod window_button;
pub mod window_renderer;

pub trait Component: Sized {
    fn ui(self, ui: &mut Ui);
}

pub trait ContentComponent {
    fn show(
        self,
        ctx: &Context,
        state: &mut AppState,
        content: impl FnOnce(&mut Ui, &mut AppState),
    );
}
