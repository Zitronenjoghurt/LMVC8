use crate::components::Component;
use egui::{Grid, Ui};
use lmvc8_core::console::components::cpu::CPU;

pub struct CPUSnapshotDisplay<'a> {
    snapshot: &'a CPU,
}

impl<'a> CPUSnapshotDisplay<'a> {
    pub fn new(snapshot: &'a CPU) -> Self {
        Self { snapshot }
    }
}

impl Component for CPUSnapshotDisplay<'_> {
    fn ui(self, ui: &mut Ui) {
        let registers = self.snapshot.get_registers();
        ui.style_mut().override_font_id = Some(egui::FontId::monospace(14.0));

        Grid::new("cpu_snapshot_display_grid_right")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.label("PC");
                ui.label(format!("{}", self.snapshot.get_pc()));
                ui.label("A");
                ui.label(format!("{}", registers.get_a()));
                ui.end_row();

                ui.label("SP");
                ui.label(format!("{}", registers.get_sp()));
                ui.label("B");
                ui.label(format!("{}", registers.get_b()));
                ui.end_row();

                ui.label("IR");
                ui.label(format!("{}", self.snapshot.get_ir()));
                ui.label("C");
                ui.label(format!("{}", registers.get_c()));
                ui.end_row();

                ui.label("H");
                ui.label(format!("{}", registers.get_h()));
                ui.label("D");
                ui.label(format!("{}", registers.get_d()));
                ui.end_row();

                ui.label("L");
                ui.label(format!("{}", registers.get_l()));
                ui.label("E");
                ui.label(format!("{}", registers.get_e()));
            });
    }
}
