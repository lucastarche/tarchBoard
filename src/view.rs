use eframe::egui;

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}
