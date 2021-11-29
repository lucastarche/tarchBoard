use crate::clock::format_time;
use crate::view::View;
use crate::{
    clock::{stopwatch::StopwatchWidget, timezone_dropdown::TimezoneDropdown},
    view::UiWidget,
};

use chrono::{TimeZone, Utc};
use eframe::egui::{self, menu, Ui, Window};

pub struct ClockWidget {
    timezones: Vec<TimezoneDropdown>,
    config_open: bool,
    stopwatch: StopwatchWidget,
    state: State,
}

#[derive(Eq, PartialEq)]
enum State {
    CLOCK,
    STOPWATCH,
}

impl Default for ClockWidget {
    fn default() -> Self {
        Self {
            timezones: vec![TimezoneDropdown::new(0)],
            config_open: false,
            stopwatch: Default::default(),
            state: State::CLOCK,
        }
    }
}

impl UiWidget for ClockWidget {
    fn name(&self) -> &'static str {
        "Clock"
    }

    fn show(&mut self, ctx: &eframe::egui::CtxRef) {
        Window::new(self.name())
            .resizable(false)
            .show(ctx, |ui| self.ui(ui));

        if self.config_open {
            // We create this extra variable since we cant pass self as &mut twice
            let mut open = true;
            let mut must_delete: Vec<usize> = Vec::new();

            Window::new("Timezone Config")
                .open(&mut open)
                .resizable(false)
                .default_width(200.0)
                .show(ctx, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            ui.heading("Timezones");
                            ui.horizontal(|ui| {
                                ui.vertical_centered_justified(|ui| {
                                    for timezone in self.timezones.iter_mut() {
                                        timezone.ui(ui);
                                    }
                                });

                                ui.vertical_centered_justified(|ui| {
                                    for i in 0..self.timezones.len() {
                                        if ui.button("x").clicked() {
                                            must_delete.push(i);
                                        }
                                    }
                                });
                            });

                            if ui.button("+").clicked() {
                                self.timezones
                                    .push(TimezoneDropdown::new(self.timezones.len() as u64));
                            }
                        },
                    );
                });

            for idx in must_delete {
                self.timezones.remove(idx);
            }

            for (idx, timezone) in &mut self.timezones.iter_mut().enumerate() {
                timezone.change_id(idx as u64);
            }

            self.config_open = open;
        }
    }
}

impl View for ClockWidget {
    fn ui(&mut self, ui: &mut Ui) {
        ui.ctx().request_repaint();

        menu::bar(ui, |ui| {
            if ui.button("Change Timezones").clicked() {
                self.config_open = true;
            }

            ui.selectable_value(&mut self.state, State::CLOCK, "Clock");
            ui.selectable_value(&mut self.state, State::STOPWATCH, "Stopwatch");
        });

        ui.separator();

        match self.state {
            State::CLOCK => {
                let now = Utc::now().naive_utc();

                ui.vertical_centered(|ui| {
                    for timezone in &self.timezones {
                        let local_time = timezone.selected().from_utc_datetime(&now);

                        ui.heading(format!(
                            "{}: {}",
                            timezone.selected().name(),
                            format_time(local_time)
                        ));
                    }
                });
            }
            State::STOPWATCH => self.stopwatch.ui(ui),
        }
    }
}
