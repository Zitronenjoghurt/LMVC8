use crate::state::AppState;
use egui::Context;
use serde::{Deserialize, Serialize};

mod debugger;
mod main_menu;

#[derive(Debug, Default, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum ViewID {
    #[default]
    MainMenu,
    Debugger,
}

pub trait View: Default {
    fn render(&mut self, ctx: &Context, state: &mut AppState);
}

#[derive(Default, Serialize, Deserialize)]
pub struct ViewManager {
    debugger: debugger::DebuggerView,
    main_menu: main_menu::MainMenuView,
}

impl View for ViewManager {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        match state.current_view() {
            ViewID::Debugger => self.debugger.render(ctx, state),
            ViewID::MainMenu => self.main_menu.render(ctx, state),
        }
    }
}
