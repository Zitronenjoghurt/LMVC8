use crate::components::cpu_snapshot_display::CPUSnapshotDisplay;
use crate::components::nav_bar::NavBar;
use crate::components::window_renderer::WindowRenderer;
use crate::components::{Component, ContentComponent};
use crate::state::AppState;
use crate::views::View;
use crate::windows::settings::SettingsWindow;
use egui::{Context, SidePanel, Ui};
use lmvc8_core::console::components::cpu::CPU;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DebuggerView {
    settings_window: SettingsWindow,
    #[serde(skip, default)]
    cpu_snapshot: CPU,
}

impl DebuggerView {
    fn update(&mut self, state: &mut AppState) {
        if let Some(snapshot) = state.emulator.get_cpu_snapshot() {
            self.cpu_snapshot = snapshot;
        }
    }

    fn render_right_panel(&mut self, ui: &mut Ui, state: &mut AppState) {
        CPUSnapshotDisplay::new(&self.cpu_snapshot).ui(ui);
    }
}

impl View for DebuggerView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        self.update(state);

        WindowRenderer::new(ctx, state)
            .window(&mut self.settings_window)
            .render();

        NavBar::new("debugger_nav")
            .label("Debugger")
            .settings_window(&mut self.settings_window)
            .show(ctx, state, |_, _| {});

        SidePanel::right("debugger_right_panel").show(ctx, |ui| {
            self.render_right_panel(ui, state);
        });
    }
}
