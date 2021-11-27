use crate::{clock::ClockWidget, view::UiWidget, weather::WeatherWidget};

use eframe::epi;

pub struct App {
    pub weather: Option<WeatherWidget>,
    pub clock: ClockWidget,
}

impl Default for App {
    fn default() -> Self {
        Self {
            weather: None,
            clock: Default::default(),
        }
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
        if let Some(widget) = &mut self.weather {
            widget.show(ctx);
        }
        self.clock.show(ctx);
    }
}
