use crate::view::View;

use eframe::egui::{self, ScrollArea, Ui};
use std::time::{Duration, Instant};

use super::format_duration;

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
            ui.heading(format_duration(&self.stopwatch.elapsed()));

            ScrollArea::vertical()
                .max_height(75.0)
                .auto_shrink([true; 2])
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        for (idx, lap) in self.laps.iter().enumerate() {
                            ui.label(format!("Lap #{}: {}", idx + 1, format_duration(lap)));
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

struct Stopwatch {
    current: Option<Instant>,
    duration: Duration,
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self {
            current: None,
            duration: Duration::default(),
        }
    }
}

impl Stopwatch {
    pub fn play_pause(&mut self) {
        if let Some(instant) = &self.current {
            self.duration = self.duration + instant.elapsed();
            self.current = None;
        } else {
            self.current = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        self.duration = Duration::default();
        self.current = None;
    }

    pub fn elapsed(&self) -> Duration {
        if let Some(instant) = &self.current {
            self.duration + instant.elapsed()
        } else {
            self.duration
        }
    }

    pub fn running(&self) -> bool {
        self.current.is_some()
    }
}
