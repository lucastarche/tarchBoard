mod app;
mod clock;
mod view;
mod weather;

use app::App;

fn main() {
    let mut app = App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
