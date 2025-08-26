use crate::state::debugger::DebuggerState;
use crate::state::settings::SettingsState;
use crate::views::ViewID;
use serde::{Deserialize, Serialize};

mod debugger;
pub mod settings;

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    current_view: ViewID,
    settings: SettingsState,
    #[serde(skip)]
    pub debugger: DebuggerState,
}

impl AppState {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.settings.update(ctx);

        if self.current_view == ViewID::Debugger {
            self.debugger.update();
        }
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
