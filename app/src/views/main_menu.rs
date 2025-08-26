use crate::components::window_button::WindowButton;
use crate::components::window_renderer::WindowRenderer;
use crate::components::Component;
use crate::state::AppState;
use crate::views::{View, ViewID};
use crate::windows::settings::SettingsWindow;
use egui::{CentralPanel, Context, Frame, MenuBar, TopBottomPanel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MainMenuView {
    settings_window: SettingsWindow,
}

impl MainMenuView {
    fn render_center(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        ui.vertical_centered_justified(|ui| {
            let available_height = ui.available_height();
            let available_width = ui.available_width();
            ui.set_max_width(available_width / 3.0);

            ui.add_space(available_height / 10.0);

            Frame::default()
                .inner_margin(10.0)
                .corner_radius(4.0)
                .shadow(ui.style().visuals.window_shadow)
                .fill(ui.style().visuals.window_fill)
                .stroke(ui.style().visuals.widgets.noninteractive.bg_stroke)
                .show(ui, |ui| {
                    ui.heading("LMVC8");
                    ui.label("An emulator for a made-up console");

                    ui.separator();

                    if ui.button("Debugger").clicked() {
                        self.on_debugger_clicked(state);
                    }
                })
        });
    }

    fn on_debugger_clicked(&mut self, state: &mut AppState) {
        state.switch_view(ViewID::Debugger);
    }
}

impl View for MainMenuView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        WindowRenderer::new(ctx, state)
            .window(&mut self.settings_window)
            .render();

        TopBottomPanel::top("main_menu_top_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                if ui.button("  ").clicked() {
                    webbrowser::open("https://github.com/Zitronenjoghurt/LMVC8").ok();
                }
                ui.separator();

                WindowButton::new(&mut self.settings_window, " 🛠 ").ui(ui);
                ui.separator();

                ui.label("Main Menu");
                ui.separator();
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            self.render_center(ui, state);
        });
    }
}
