use crate::state::settings::UIScale;
use crate::state::AppState;
use crate::windows::ViewWindow;
use egui::{Id, Ui, WidgetText};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SettingsWindow {
    open: bool,
}

impl ViewWindow for SettingsWindow {
    fn id(&self) -> Id {
        Id::new("settings_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Settings"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        let mut ui_scale = state.settings_mut().get_ui_scale();
        egui::ComboBox::from_id_salt("ui_scale")
            .selected_text(ui_scale.to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut ui_scale, UIScale::XXS, UIScale::XXS.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::XS, UIScale::XS.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::S, UIScale::S.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::M, UIScale::M.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::L, UIScale::L.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::XL, UIScale::XL.to_string());
                ui.selectable_value(&mut ui_scale, UIScale::XXL, UIScale::XXL.to_string());
            });
        state.settings_mut().set_ui_scale(ui_scale);
    }
}
