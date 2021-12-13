use crate::{
    clock::ClockWidget, kanban::KanbanWidget, load_image::load_image, message::MessageSender,
    view::UiWidget, weather::WeatherWidget,
};

use diesel::SqliteConnection;
use eframe::{egui, epi};
use std::rc::Rc;

pub struct App {
    tx: MessageSender,
    db: Rc<SqliteConnection>,

    background: Option<(egui::Vec2, egui::TextureId)>,

    weather: WeatherWidget,
    clock: ClockWidget,
    kanban: KanbanWidget,
}

impl App {
    pub fn new(tx: MessageSender, db: SqliteConnection) -> Self {
        let db = Rc::new(db);
        Self {
            background: None,
            weather: WeatherWidget::new(tx.clone()),
            clock: Default::default(),
            kanban: KanbanWidget::new(db.clone()),
            tx,
            db,
        }
    }

    pub fn set_weather_query(&mut self, place: String) {
        self.weather.set_query(place);
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "tarch_board"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        match load_image(frame, "img/background.png") {
            Ok(res) => self.background = Some(res),
            Err(why) => println!("Error loading background: {:?}", why),
        }
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        if let Some((size, id)) = self.background {
            let available_rect = ctx.available_rect();
            let dx = (available_rect.width() - size.x) / 2.0;
            let dy = (available_rect.height() - size.y) / 2.0;
            let pos = available_rect.min + egui::vec2(dx, dy);

            egui::Area::new("background")
                .fixed_pos(pos)
                .interactable(false)
                .order(egui::Order::Background)
                .show(ctx, |ui| {
                    ui.image(id, size);
                });
        }

        self.weather.show(ctx);
        self.clock.show(ctx);
        self.kanban.show(ctx);
    }
}
