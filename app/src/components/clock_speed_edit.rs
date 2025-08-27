use crate::components::Component;
use crate::state::debugger::action::DebuggerActionContext;
use egui::{Slider, Ui, Widget};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum ClockSpeedUnit {
    Hz,
    KHz,
    MHz,
}

pub struct ClockSpeedEdit<'a> {
    debug_actions: &'a DebuggerActionContext,
    cycles_per_second: u64,
}

impl<'a> ClockSpeedEdit<'a> {
    pub fn new(debug_actions: &'a DebuggerActionContext, cycles_per_second: u64) -> Self {
        Self {
            debug_actions,
            cycles_per_second,
        }
    }
}

impl Component for ClockSpeedEdit<'_> {
    fn ui(self, ui: &mut Ui) {
        let (mut value, mut unit) = if self.cycles_per_second < 1_000 {
            (self.cycles_per_second, ClockSpeedUnit::Hz)
        } else if self.cycles_per_second < 1_000_000 {
            (self.cycles_per_second / 1_000, ClockSpeedUnit::KHz)
        } else {
            (self.cycles_per_second / 1_000_000, ClockSpeedUnit::MHz)
        };

        let old_value = value;
        let old_unit = unit;

        ui.vertical(|ui| {
            Slider::new(&mut value, 1..=1000).ui(ui);
            ui.horizontal(|ui| {
                ui.radio_value(&mut unit, ClockSpeedUnit::Hz, "Hz");
                ui.radio_value(&mut unit, ClockSpeedUnit::KHz, "KHz");
                ui.radio_value(&mut unit, ClockSpeedUnit::MHz, "MHz");
            });
        });

        if value != old_value || unit != old_unit {
            let cycles_per_second = match unit {
                ClockSpeedUnit::Hz => value,
                ClockSpeedUnit::KHz => value * 1_000,
                ClockSpeedUnit::MHz => value * 1_000_000,
            };
            self.debug_actions.set_clock_speed(cycles_per_second);
        }
    }
}
