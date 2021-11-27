use crate::view::View;
use crate::{clock::timezone_dropdown::TimezoneDropdown, view::UiWidget};

use chrono::{TimeZone, Timelike, Utc};
use eframe::egui::{vec2, Ui, Window};

pub struct ClockWidget {
    timezone: TimezoneDropdown,
}

impl Default for ClockWidget {
    fn default() -> Self {
        Self {
            timezone: Default::default(),
        }
    }
}

impl UiWidget for ClockWidget {
    fn name(&self) -> &'static str {
        "Clock"
    }

    fn show(&mut self, ctx: &eframe::egui::CtxRef) {
        Window::new(self.name())
            .default_size(vec2(300.0, 150.0))
            .vscroll(false)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl View for ClockWidget {
    fn ui(&mut self, ui: &mut Ui) {
        let now = Utc::now().naive_utc();
        let local_time = self.timezone.selected().from_utc_datetime(&now);

        ui.ctx().request_repaint();
        ui.heading(format!(
            "{}: {}",
            self.timezone.selected().name(),
            format_time(local_time)
        ));
        self.timezone.ui(ui);
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
