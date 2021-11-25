use crate::view::View;
use crate::weather::query::{retrieve_weather, WeatherResponse};

use eframe::egui::Ui;
use std::time::{Duration, Instant};

pub struct WeatherWidget {
    place: String,
    query: String,
    response: Option<WeatherResponse>,
    last_update: Instant,
}

impl Default for WeatherWidget {
    fn default() -> Self {
        Self {
            place: Default::default(),
            query: Default::default(),
            response: None,
            last_update: Instant::now(),
        }
    }
}

impl View for WeatherWidget {
    fn ui(&mut self, ui: &mut Ui) {
        const ONE_HOUR: Duration = Duration::from_secs(3600);

        if self.last_update.elapsed() > ONE_HOUR {
            self.update_weather();
        }

        if let Some(response) = &self.response {
            let current_condition = &response.current_condition;
            let nearest_area = &response.nearest_area;

            ui.heading(format!("Weather report: {}", self.place));
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
        } else {
            ui.label("Could not retrieve weather data");
        }
    }
}

impl WeatherWidget {
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
