mod app;
mod weather;

use app::App;

fn main() {
    let mut app = App::default();
    app.set_weather_query("http://wttr.in/Detroit?format=j1".to_string());
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
