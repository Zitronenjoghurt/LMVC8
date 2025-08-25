use crate::components::window_button::WindowButton;
use crate::components::{Component, ContentComponent};
use crate::state::AppState;
use crate::views::ViewID;
use crate::windows::settings::SettingsWindow;
use egui::{Context, MenuBar, TopBottomPanel, Ui};

pub struct NavBar<'a> {
    id: &'a str,
    label: &'a str,
    show_home_button: bool,
    show_github_button: bool,
    settings_window: Option<&'a mut SettingsWindow>,
}

impl<'a> NavBar<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            label: id,
            show_home_button: true,
            show_github_button: false,
            settings_window: None,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn hide_home_button(mut self) -> Self {
        self.show_home_button = false;
        self
    }

    pub fn show_github_button(mut self) -> Self {
        self.show_github_button = true;
        self
    }

    pub fn settings_window(mut self, settings_window: &'a mut SettingsWindow) -> Self {
        self.settings_window = Some(settings_window);
        self
    }
}

impl ContentComponent for NavBar<'_> {
    fn show(
        self,
        ctx: &Context,
        state: &mut AppState,
        content: impl FnOnce(&mut Ui, &mut AppState),
    ) {
        let Self {
            id,
            label,
            show_home_button,
            show_github_button,
            settings_window,
        } = self;

        TopBottomPanel::top(id.to_string()).show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                if show_home_button {
                    if ui.button(" 🏠 ").clicked() {
                        state.switch_view(ViewID::MainMenu)
                    }
                    ui.separator();
                }

                if show_github_button {
                    if ui.button("  ").clicked() {
                        webbrowser::open("https://github.com/Zitronenjoghurt/LMVC8").ok();
                    }
                    ui.separator();
                }

                if let Some(settings_window) = settings_window {
                    WindowButton::new(settings_window, " 🛠 ").ui(ui);
                    ui.separator();
                }

                ui.label(label);
                ui.separator();
                content(ui, state);
            });
        });
    }
}
