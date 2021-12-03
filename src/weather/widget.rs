use crate::{
    message::{MessageSender, OneshotReceiver},
    view::{UiWidget, View},
    weather::query::{retrieve_weather, WeatherResponse},
};

use eframe::egui::{Ui, Window};
use std::time::{Duration, Instant};

pub struct WeatherWidget {
    place: String,
    response: Option<WeatherResponse>,
    last_update: Instant,
    tx: MessageSender,
    weather_receiver: Option<OneshotReceiver<WeatherResponse>>,
}

impl WeatherWidget {
    pub fn new(tx: MessageSender) -> Self {
        let mut widget = Self {
            place: String::new(), // Empty query makes wttr.in use your current IP instead
            response: Default::default(),
            last_update: Instant::now(),
            tx,
            weather_receiver: Default::default(),
        };

        widget.update_weather();
        widget
    }

    pub fn set_query(&mut self, place: String) {
        self.place = place;
        self.update_weather();
    }

    fn update_weather(&mut self) {
        self.last_update = Instant::now();
        self.weather_receiver = Some(retrieve_weather(self.tx.clone(), &self.place));
    }
}

impl UiWidget for WeatherWidget {
    fn name(&self) -> &'static str {
        "Weather"
    }

    fn show(&mut self, ctx: &eframe::egui::CtxRef) {
        Window::new(self.name())
            .vscroll(false)
            .resizable(false)
            .show(ctx, |ui| self.ui(ui));
    }
}

impl View for WeatherWidget {
    fn ui(&mut self, ui: &mut Ui) {
        const ONE_HOUR: Duration = Duration::from_secs(3600);

        if self.last_update.elapsed() > ONE_HOUR {
            self.update_weather();
        }

        if let Some(recv) = &mut self.weather_receiver {
            if let Ok(response) = recv.try_recv() {
                self.response = Some(response);
            }
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
