use crate::weather::{retrieve_weather, WeatherResponse};
use eframe::{egui, epi};
use std::time::Duration;
use std::time::Instant;

pub struct App {
    pub weather: Option<WeatherState>,
}

pub struct WeatherState {
    place: String,
    query: String,
    response: Option<WeatherResponse>,
    last_update: Instant,
}

impl Default for WeatherState {
    fn default() -> Self {
        Self {
            place: Default::default(),
            query: Default::default(),
            response: None,
            last_update: Instant::now(),
        }
    }
}

impl WeatherState {
    pub fn set_query(&mut self, place: String, arguments: String) {
        self.query = format!("https://wttr.in/{}?{}", place, arguments);
        self.place = place;
        self.update_weather();
    }

    pub fn update_weather(&mut self) {
        self.last_update = Instant::now();
        match retrieve_weather(&self.query) {
            Ok(weather) => self.response = Some(weather),
            Err(why) => println!("Error retrieving weather: {}", why),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            weather: Default::default(),
        }
    }
}

impl App {
    pub fn set_weather_query(&mut self, place: String, arguments: String) {
        if self.weather.is_none() {
            self.weather = Some(WeatherState::default());
        }
        // UNWRAP SAFETY: self.weather will always be Some at this point, since we set it in the block above
        self.weather.as_mut().unwrap().set_query(place, arguments);
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "tarch_board"
    }

    fn clear_color(&self) -> egui::Rgba {
        egui::Rgba::from_black_alpha(0.0)
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        const ONE_HOUR: Duration = Duration::from_secs(3600);

        if let Some(weather) = &mut self.weather {
            if weather.last_update.elapsed() > ONE_HOUR {
                weather.update_weather();
            }

            if let Some(response) = &weather.response {
                let current_condition = &response.current_condition;
                let nearest_area = &response.nearest_area;
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading(format!("Weather report: {}", weather.place));
                    ui.label(&current_condition.weather_description);
                    ui.label(format!(
                        "{}°C ({}°C)",
                        current_condition.temperature, current_condition.feels_like
                    ));
                    ui.label(format!(
                        "{}, {}, {}",
                        nearest_area.country, nearest_area.region, nearest_area.area_name
                    ));
                    ui.label(format!(
                        "Last update: {}",
                        &current_condition.last_update_local_time
                    ));
                });
            } else {
                egui::CentralPanel::default()
                    .show(ctx, |ui| ui.label("Could not retrieve weather data"));
            }
        } else {
            egui::CentralPanel::default()
                .show(ctx, |ui| ui.label("No weather target set in place"));
        }
    }
}
