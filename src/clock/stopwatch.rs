use crate::view::View;

use eframe::egui::{self, Ui};
use std::time::{Duration, Instant};

use super::format_duration;

pub struct StopwatchWidget {
    stopwatch: Stopwatch,
}

impl Default for StopwatchWidget {
    fn default() -> Self {
        Self {
            stopwatch: Default::default(),
        }
    }
}

impl View for StopwatchWidget {
    fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered_justified(|ui| {
            ui.heading(format_duration(self.stopwatch.elapsed()));

            ui.horizontal(|ui| {
                if ui.button("Play/Pause").clicked() {
                    self.stopwatch.play_pause();
                }

                if ui.button("Stop").clicked() {
                    self.stopwatch.stop();
                }
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
