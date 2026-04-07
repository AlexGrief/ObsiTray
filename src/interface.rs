use eframe::egui;

#[derive(Default)]
pub struct MyApp {
    input: String,
    result: String,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("ObsiTray");
            
            ui.label("На сегодня у вас есть {count} задач", count);
        });
    }
}