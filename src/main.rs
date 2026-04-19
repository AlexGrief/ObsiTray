mod files;

use eframe::egui;
use files::task::Task;

struct MyApp {
    config: Result<files::AppConfig, String>,
    tasks: Vec<Task>,
    task_count: u32,
}

impl MyApp {
    fn new() -> Self {
        let config = files::AppConfig::new();
        let (tasks, task_count) = match &config {
            Ok(cfg) => {
                let t = files::collect_all_tasks(cfg);
                let c = t.len() as u32;
                (t, c)
            }
            Err(_) => (Vec::new(), 0),
        };
        Self { config, tasks, task_count }
    }

    fn reload(&mut self) {
        self.config = files::AppConfig::new();
        let (tasks, task_count) = match &self.config {
            Ok(cfg) => {
                let t = files::collect_all_tasks(cfg);
                let c = t.len() as u32;
                (t, c)
            }
            Err(_) => (Vec::new(), 0),
        };
        self.tasks = tasks;
        self.task_count = task_count;
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("ObsiTray 📋");
                if ui.button("🔄").clicked() {
                    self.reload();
                }
            });
            ui.separator();

            match &self.config {
                Ok(_) => {
                    ui.label(format!("В вашем хранилище {} задач", self.task_count));
                    ui.separator();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for task in &self.tasks {
                            ui.horizontal(|ui| {
                                ui.label(if task.done { "☑" } else { "☐" });
                                ui.label(&task.text);
                            });
                        }
                    });
                }
                Err(e) => {
                    ui.colored_label(egui::Color32::RED, format!("Ошибка: {}", e));
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "ObsiTray",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}