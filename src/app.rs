use crate::{clock::ClockWidget, view::UiWidget, weather::WeatherWidget};

use eframe::epi;

pub struct App {
    pub weather: WeatherWidget,
    pub clock: ClockWidget,
}

impl Default for App {
    fn default() -> Self {
        Self {
            weather: Default::default(),
            clock: Default::default(),
        }
    }
}

impl App {
    pub fn set_weather_query(&mut self, place: String) {
        self.weather.set_query(place);
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "tarch_board"
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        self.weather.show(ctx);
        self.clock.show(ctx);
    }
}
