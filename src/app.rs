use crate::{view::View, weather::WeatherWidget};

use eframe::{egui, epi};

pub struct App {
    pub weather: Option<WeatherWidget>,
}

impl Default for App {
    fn default() -> Self {
        Self { weather: None }
    }
}

impl App {
    pub fn set_weather_query(&mut self, place: String) {
        if self.weather.is_none() {
            self.weather = Some(WeatherWidget::default());
        }
        // UNWRAP SAFETY: self.weather will always be Some at this point, since we set it in the block above
        self.weather.as_mut().unwrap().set_query(place);
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "tarch_board"
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(widget) = &mut self.weather {
                widget.ui(ui);
            }
        });
    }
}
