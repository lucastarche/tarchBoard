mod stopwatch;
mod timezone_dropdown;
mod widget;

use std::time::Duration;

use chrono::Timelike;
pub use widget::ClockWidget;

fn format_time<T: chrono::TimeZone>(time: chrono::DateTime<T>) -> String {
    let hour = format_time_number(time.hour(), 2);
    let minute = format_time_number(time.minute(), 2);
    let second = format_time_number(time.second(), 2);

    format!("{}:{}:{}", hour, minute, second)
}

fn format_duration(duration: &Duration) -> String {
    let hour = format_time_number((duration.as_secs() / 3600) as u32, 2);
    let minute = format_time_number((duration.as_secs() / 60 % 60) as u32, 2);
    let second = format_time_number((duration.as_secs() % 60) as u32, 2);
    let millis = format_time_number(duration.subsec_millis(), 3);

    format!("{}:{}:{}:{}", hour, minute, second, millis)
}

fn format_time_number(x: u32, mut width: u32) -> String {
    let mut ans = String::new();
    let mut pow = 1;
    if x == 0 {
        width -= 1;
    } else {
        while pow <= x {
            pow *= 10;
            width -= 1;
        }
    }

    for _ in 0..width {
        ans.push('0');
    }

    format!("{}{}", ans, x.to_string())
}
