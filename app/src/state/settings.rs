use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum UIScale {
    XXS,
    XS,
    S,
    #[default]
    M,
    L,
    XL,
    XXL,
}

impl UIScale {
    pub fn as_f32(self) -> f32 {
        match self {
            UIScale::XXS => 0.25,
            UIScale::XS => 0.5,
            UIScale::S => 0.75,
            UIScale::M => 1.0,
            UIScale::L => 1.25,
            UIScale::XL => 1.5,
            UIScale::XXL => 2.0,
        }
    }
}

impl Display for UIScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UIScale::XXS => write!(f, "XXS"),
            UIScale::XS => write!(f, "XS"),
            UIScale::S => write!(f, "S"),
            UIScale::M => write!(f, "M"),
            UIScale::L => write!(f, "L"),
            UIScale::XL => write!(f, "XL"),
            UIScale::XXL => write!(f, "XXL"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsState {
    ui_scale: UIScale,
    dirty: bool,
}

impl Default for SettingsState {
    fn default() -> Self {
        Self {
            ui_scale: UIScale::default(),
            dirty: true,
        }
    }
}

impl SettingsState {
    pub fn update(&mut self, ctx: &egui::Context) {
        if !self.dirty {
            return;
        }

        ctx.set_pixels_per_point(self.ui_scale.as_f32());
        self.dirty = false;
    }

    pub fn get_ui_scale(&self) -> UIScale {
        self.ui_scale
    }

    pub fn set_ui_scale(&mut self, ui_scale: UIScale) {
        if self.ui_scale == ui_scale {
            return;
        }
        self.ui_scale = ui_scale;
        self.dirty = true;
    }
}
