use eframe::egui::DragValue;

use crate::view::View;

use super::{format_duration, Stopwatch};

use std::time::Duration;

#[derive(Default)]
pub struct TimerWidget {
    duration: u64,
    second_input: u64,
    minute_input: u64,
    hour_input: u64,
    stopwatch: Stopwatch,
}

impl View for TimerWidget {
    fn ui(&mut self, ui: &mut eframe::egui::Ui) {
        let duration = Duration::from_secs(self.duration);
        let time_left = if duration < self.stopwatch.elapsed() {
            Duration::default()
        } else {
            duration - self.stopwatch.elapsed()
        };

        ui.vertical_centered(|ui| {
            if self.stopwatch.elapsed() == Duration::default() {
                ui.heading(format_duration(
                    &Duration::from_secs(
                        self.second_input + self.minute_input * 60 + self.hour_input * 3600,
                    ),
                    false,
                ));
            } else {
                ui.heading(format_duration(&time_left, false));
            }

            ui.columns(3, |cols| {
                cols[0].vertical_centered_justified(|ui| {
                    ui.add(DragValue::new(&mut self.hour_input).speed(0.5))
                });
                cols[1].vertical_centered_justified(|ui| {
                    ui.add(
                        DragValue::new(&mut self.minute_input)
                            .speed(0.5)
                            .clamp_range(0..=59),
                    )
                });
                cols[2].vertical_centered_justified(|ui| {
                    ui.add(
                        DragValue::new(&mut self.second_input)
                            .speed(0.5)
                            .clamp_range(0..=59),
                    )
                });
            });

            ui.columns(2, |cols| {
                cols[0].vertical_centered_justified(|ui| {
                    if ui.button("Play/Pause").clicked() {
                        if self.stopwatch.elapsed() == Duration::default() {
                            self.duration =
                                self.second_input + self.minute_input * 60 + self.hour_input * 3600;
                        }
                        self.stopwatch.play_pause();
                    }
                });

                cols[1].vertical_centered_justified(|ui| {
                    if self.stopwatch.elapsed() == Duration::default() {
                        if ui.button("Start").clicked() {
                            self.duration =
                                self.second_input + self.minute_input * 60 + self.hour_input * 3600;
                            self.stopwatch.play_pause();
                        }
                    } else {
                        if ui.button("Stop").clicked() {
                            self.stopwatch.stop();
                        }
                    }
                });
            });
        });
    }
}
