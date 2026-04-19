mod files;
mod new_file;

use crate::files::AppConfig;
use std::path::PathBuf;
use eframe::egui;

#[derive(Default)]
pub struct MyApp {
    input: String,
    result: String,
}
impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let config = files::AppConfig::new();
            ui.label(format!("В вашем хранилище {} задач", files::count(config)));
        });
    }
}
fn main() -> eframe::Result<()> {
    // Создаем окно с интерфейсом
    eframe::run_native(
        "ObsiTray",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}