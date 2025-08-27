use egui::Ui;

pub mod clock_speed_edit;
pub mod cpu_snapshot_display;
pub mod rom_display;
pub mod window_button;
pub mod window_renderer;

pub trait Component: Sized {
    fn ui(self, ui: &mut Ui);
}
