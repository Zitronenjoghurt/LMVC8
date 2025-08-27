use crate::components::clock_speed_edit::ClockSpeedEdit;
use crate::components::cpu_snapshot_display::CPUSnapshotDisplay;
use crate::components::rom_display::ROMDisplay;
use crate::components::window_button::WindowButton;
use crate::components::window_renderer::WindowRenderer;
use crate::components::Component;
use crate::demos::Demo;
use crate::state::AppState;
use crate::views::{View, ViewID};
use crate::windows::settings::SettingsWindow;
use egui::{Context, MenuBar, SidePanel, TopBottomPanel, Ui, Widget};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DebuggerView {
    settings_window: SettingsWindow,
}

impl DebuggerView {
    fn render_top_menu(&mut self, ui: &mut Ui, state: &mut AppState) {
        ui.menu_button("ROM", |ui| {
            ui.menu_button("Load", |ui| {
                if ui.button("From File").clicked() {};
                ui.menu_button("From DEMO", |ui| {
                    if ui.button("Simple Add").clicked() {
                        state.debugger.load_demo(Demo::SimpleAdd);
                    };
                })
            });
        });

        ui.separator();

        ui.menu_button("Console", |ui| {
            ui.menu_button("Clock Speed", |ui| {
                ClockSpeedEdit::new(
                    state.debugger.action_context(),
                    state.debugger.cycles_per_second,
                )
                .ui(ui);
            });
        });
    }

    fn render_right_panel(&mut self, ui: &mut Ui, state: &mut AppState) {
        ui.horizontal(|ui| {
            if state.debugger.is_running {
                if ui.button("⏸").clicked() {
                    state.debugger.pause();
                }
            } else if ui.button("▶").clicked() {
                state.debugger.run();
            }

            if ui.button("↩").clicked() {
                state.debugger.step();
            };
            if ui.button("⟲").clicked() {
                state.debugger.reset();
            }

            ui.small(format!(
                "{} at {} ms/frame",
                state.debugger.format_clock_speed(),
                (state.debugger.last_frame_mics as f32) / 1000.0,
            ));
        });

        ui.separator();

        CPUSnapshotDisplay::new(&state.debugger.cpu_snapshot).ui(ui);

        ui.separator();

        ROMDisplay::new(
            &state.debugger.disassembled_binary,
            &state.debugger.breakpoints,
            state.debugger.cpu_snapshot.get_pc(),
        )
        .debugger_actions(state.debugger.action_context())
        .ui(ui);
    }
}

impl View for DebuggerView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        WindowRenderer::new(ctx, state)
            .window(&mut self.settings_window)
            .render();

        TopBottomPanel::top("debugger_top_panel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                if ui.button(" 🏠 ").clicked() {
                    state.switch_view(ViewID::MainMenu)
                }
                ui.separator();

                WindowButton::new(&mut self.settings_window, " 🛠 ").ui(ui);
                ui.separator();

                ui.label("Debugger");
                ui.separator();

                self.render_top_menu(ui, state);
            });
        });

        SidePanel::right("debugger_right_panel").show(ctx, |ui| {
            self.render_right_panel(ui, state);
        });
    }
}
