use crate::components::Component;
use egui::Ui;
use egui_extras::{Column, TableBuilder};
use lmvc8_core::console::types::word::Word;
use lmvc8_core::disassembler::DisassembledBinary;

pub struct ROMDisplay<'a> {
    disassembled_binary: &'a DisassembledBinary,
    pc: Word,
}

impl<'a> ROMDisplay<'a> {
    pub fn new(disassembled_binary: &'a DisassembledBinary, pc: Word) -> Self {
        Self {
            disassembled_binary,
            pc,
        }
    }
}

impl Component for ROMDisplay<'_> {
    fn ui(self, ui: &mut Ui) {
        ui.style_mut().override_font_id = Some(egui::FontId::monospace(14.0));
        let text_height = ui.text_style_height(&egui::TextStyle::Body);

        let pc_index = self.pc.value() as usize;
        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().at_least(10.0))
            .column(Column::auto().at_least(50.0)) // Address column
            .column(Column::auto().at_least(80.0))
            .column(Column::remainder())
            .body(|body| {
                body.rows(text_height, (u16::MAX as usize) + 1, |mut row| {
                    let row_index = row.index();

                    row.col(|ui| {
                        if row_index == pc_index {
                            ui.label("⏵");
                        }
                    });

                    row.col(|ui| {
                        ui.label(format!("0x{:04X}", row_index));
                    });

                    row.col(|ui| {
                        if let Some(node) = self.disassembled_binary.nodes().get(row_index) {
                            ui.label(node.to_string());
                        } else {
                            ui.label("---------");
                        }
                    });

                    row.col(|ui| {
                        ui.small_button("⏺");
                    });
                });
            });
    }
}
