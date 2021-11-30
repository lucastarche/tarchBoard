mod stopwatch;
mod timer;
mod timezone_dropdown;
mod widget;

use std::time::{Duration, Instant};

use chrono::Timelike;
pub use widget::ClockWidget;

fn format_time<T: chrono::TimeZone>(time: chrono::DateTime<T>) -> String {
    let hour = format_time_number(time.hour(), 2);
    let minute = format_time_number(time.minute(), 2);
    let second = format_time_number(time.second(), 2);

    format!("{}:{}:{}", hour, minute, second)
}

fn format_duration(duration: &Duration, show_millis: bool) -> String {
    let hour = format_time_number((duration.as_secs() / 3600) as u32, 2);
    let minute = format_time_number((duration.as_secs() / 60 % 60) as u32, 2);
    let second = format_time_number((duration.as_secs() % 60) as u32, 2);

    if show_millis {
        let millis = format_time_number(duration.subsec_millis(), 3);
        format!("{}:{}:{}:{}", hour, minute, second, millis)
    } else {
        format!("{}:{}:{}", hour, minute, second)
    }
}

fn format_time_number(x: u32, mut width: u32) -> String {
    let mut ans = String::new();
    let mut pow = 1;
    if x == 0 {
        width -= 1;
    } else {
        while pow <= x && width > 0 {
            pow *= 10;
            width -= 1;
        }
    }

    for _ in 0..width {
        ans.push('0');
    }

    format!("{}{}", ans, x.to_string())
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
