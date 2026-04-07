mod interface;

fn main() -> eframe::Result<()> {
    // Создаем окно с интерфейсом
    eframe::run_native(
        "ObsiTray",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(interface::MyApp::default()))),
    )
}