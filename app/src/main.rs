use crate::app::LMVC8App;

mod app;
mod components;
mod demos;
mod state;
mod views;
mod windows;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_title("LMVC8"),
        persist_window: true,
        ..Default::default()
    };

    eframe::run_native(
        "LMVC8",
        native_options,
        Box::new(|cc| Ok(Box::new(LMVC8App::new(cc)))),
    )
    .expect("Failed to run egui application.");
}
