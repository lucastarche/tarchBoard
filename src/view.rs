use eframe::egui::{self, CtxRef};

pub trait UiWidget {
    fn name(&self) -> &'static str;
    fn show(&mut self, ctx: &CtxRef);
}

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}
