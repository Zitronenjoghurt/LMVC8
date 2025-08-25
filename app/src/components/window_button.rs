use crate::components::Component;
use crate::windows::OpenableWindow;
use egui::Ui;

pub struct WindowButton<'a> {
    window: &'a mut dyn OpenableWindow,
    label: &'a str,
}

impl<'a> WindowButton<'a> {
    pub fn new(window: &'a mut dyn OpenableWindow, label: &'a str) -> Self {
        Self { window, label }
    }
}

impl Component for WindowButton<'_> {
    fn ui(self, ui: &mut Ui) {
        let response = ui.selectable_label(self.window.window_is_open(), self.label);
        if response.clicked() {
            self.window.window_set_open(!self.window.window_is_open());
        };
    }
}
