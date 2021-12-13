use crate::view::View;

use eframe::egui::{ScrollArea, Ui};
use std::time::Duration;

use super::{format_duration, Stopwatch};

pub struct StopwatchWidget {
    stopwatch: Stopwatch,
    laps: Vec<Duration>,
}

impl Default for StopwatchWidget {
    fn default() -> Self {
        Self {
            stopwatch: Default::default(),
            laps: Vec::new(),
        }
    }
}

impl View for StopwatchWidget {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.heading(format_duration(&self.stopwatch.elapsed(), true));

            ScrollArea::vertical()
                .max_height(75.0)
                .auto_shrink([true; 2])
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        for (idx, lap) in self.laps.iter().enumerate() {
                            ui.label(format!("Lap #{}: {}", idx + 1, format_duration(lap, true)));
                        }
                    })
                });

            ui.columns(3, |cols| {
                cols[0].vertical_centered_justified(|ui| {
                    if ui.button("Play/Pause").clicked() {
                        self.stopwatch.play_pause();
                    }
                });

                cols[1].vertical_centered_justified(|ui| {
                    if ui.button("Stop").clicked() {
                        self.stopwatch.stop();
                        self.laps.clear();
                    }
                });

                cols[2].vertical_centered_justified(|ui| {
                    if ui.button("Lap").clicked() {
                        self.laps.push(self.stopwatch.elapsed());
                    }
                });
            });
        });
    }
}
