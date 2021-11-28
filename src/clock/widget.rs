use crate::view::View;
use crate::{clock::timezone_dropdown::TimezoneDropdown, view::UiWidget};

use chrono::{TimeZone, Timelike, Utc};
use eframe::egui::{self, menu, Ui, Window};

pub struct ClockWidget {
    timezones: Vec<TimezoneDropdown>,
    config_open: bool,
}

impl Default for ClockWidget {
    fn default() -> Self {
        Self {
            timezones: vec![TimezoneDropdown::new(0)],
            config_open: false,
        }
    }
}

impl UiWidget for ClockWidget {
    fn name(&self) -> &'static str {
        "Clock"
    }

    fn show(&mut self, ctx: &eframe::egui::CtxRef) {
        Window::new(self.name())
            .vscroll(false)
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
                                ui.vertical(|ui| {
                                    for timezone in &mut self.timezones {
                                        timezone.ui(ui);
                                    }
                                });

                                ui.vertical(|ui| {
                                    for idx in 0..self.timezones.len() {
                                        if ui.button("x").clicked() {
                                            must_delete.push(idx);
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
        menu::bar(ui, |ui| {
            if ui.button("Change Timezones").clicked() {
                self.config_open = true;
            }
        });

        let now = Utc::now().naive_utc();

        for timezone in &self.timezones {
            let local_time = timezone.selected().from_utc_datetime(&now);
            ui.ctx().request_repaint();

            ui.heading(format!(
                "{}: {}",
                timezone.selected().name(),
                format_time(local_time)
            ));
        }
    }
}

fn format_time<T: chrono::TimeZone>(time: chrono::DateTime<T>) -> String {
    let hour = format_time_number(time.hour());
    let minute = format_time_number(time.minute());
    let second = format_time_number(time.second());

    format!("{}:{}:{}", hour, minute, second)
}

fn format_time_number(x: u32) -> String {
    if x < 10 {
        format!("0{}", x)
    } else {
        x.to_string()
    }
}
