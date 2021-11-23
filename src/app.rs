use crate::weather::{retrieve_weather, WeatherResponse};
use eframe::{egui, epi};
use std::time::Duration;
use std::time::Instant;

pub struct App {
    weather_query: Option<String>,
    weather: Option<WeatherResponse>,
    last_weather_update: Instant,
}

impl App {
    pub fn set_weather_query(&mut self, weather_query: String) {
        self.weather_query = Some(weather_query);
        self.update_weather();
    }

    fn update_weather(&mut self) {
        self.last_weather_update = Instant::now();
        match retrieve_weather("http://wttr.in/Detroit?format=j1") {
            Ok(weather) => self.weather = Some(weather),
            Err(why) => println!("Error retrieving weather: {}", why),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            weather_query: None,
            weather: None,
            last_weather_update: Instant::now(),
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "tarch_board"
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        const ONE_HOUR: Duration = Duration::from_secs(3600);

        if self.last_weather_update.elapsed() > ONE_HOUR {
            self.update_weather();
        }

        if let Some(weather) = &self.weather {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(&format!(
                    "Temperature: {}°C",
                    weather.current_condition.temperature
                ));
                ui.label(&format!(
                    "Feels like: {}°C",
                    weather.current_condition.feels_like
                ));
                ui.label(&format!(
                    "{}, {}, {}",
                    weather.nearest_area.country,
                    weather.nearest_area.region,
                    weather.nearest_area.area_name
                ));
                ui.label(&weather.current_condition.last_update_local_time);
            });
        } else {
            egui::CentralPanel::default()
                .show(ctx, |ui| ui.label("Could not retrieve weather data :("));
        }
    }
}
