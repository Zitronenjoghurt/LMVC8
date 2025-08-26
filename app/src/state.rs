use crate::state::settings::SettingsState;
use crate::views::ViewID;
use lmvc8_core::emulator::Emulator;
use serde::{Deserialize, Serialize};

pub mod settings;

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    current_view: ViewID,
    settings: SettingsState,
    #[serde(skip)]
    pub emulator: Emulator,
}

impl AppState {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.settings.update(ctx);
    }

    pub fn current_view(&self) -> ViewID {
        self.current_view
    }

    pub fn switch_view(&mut self, view: ViewID) {
        self.current_view = view;
    }

    pub fn settings(&self) -> &SettingsState {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut SettingsState {
        &mut self.settings
    }
}
