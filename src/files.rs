use std::fs;
use std::env;
use walkdir::WalkDir;
use configparser::ini::Ini;
pub mod task;
use task::Task;

#[derive(Debug)]
pub struct AppConfig {
    pub vault_path: String,
    pub max_depth: usize,
    pub show_done_tasks: bool,
    pub theme: String,
    pub path: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, String> {
        let path = config_path();

        if !std::path::Path::new(&path).exists() {
            generate_default_config(&path)?;
            return Err(format!(
                "Конфиг не найден — создан шаблон по пути: {}\nЗаполните vault_path и перезапустите.",
                path
            ));
        }
        
        let mut ini = Ini::new();
        ini.load(&path).map_err(|e| format!("Не удалось загрузить конфиг: {}", e))?;

        let vault_path = ini
            .get("obsidian", "vault_path")
            .ok_or("Не найден vault_path")?;
        let max_depth = ini
            .get("obsidian", "max_depth")
            .unwrap_or("5".to_string())
            .parse::<usize>()
            .unwrap_or(5);
        let show_done_tasks = ini
            .get("app", "show_done_tasks")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .unwrap_or(false);
        let theme = ini
            .get("app", "theme")
            .unwrap_or("dark".to_string());

        Ok(Self { vault_path, max_depth, show_done_tasks, theme, path })
    }
}

pub fn config_path() -> String {
    let exe_dir = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    exe_dir.join("../settings.ini").to_string_lossy().to_string()
}

pub fn parse_tasks(content: &str, source_file: &str) -> Vec<Task> {
    let mut tasks = Vec::new();
    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("- [") {
            if let Some(inner) = rest.get(..1) {
                let done = inner.eq_ignore_ascii_case("x");
                let is_valid_checkbox = inner == " " || inner.eq_ignore_ascii_case("x");
                if is_valid_checkbox {
                    if let Some(text_part) = rest.get(3..) {
                        let text = text_part.trim_start_matches("] ").trim().to_string();
                        if !text.is_empty() {
                            tasks.push(Task {
                                done,
                                text,
                                file: source_file.to_string(),
                                line: line_idx + 1,
                            });
                        }
                    }
                }
            }
        }
    }
    tasks
}

pub fn collect_all_tasks(config: &AppConfig) -> Vec<Task> {
    let mut all_tasks: Vec<Task> = Vec::new();
    for entry in WalkDir::new(&config.vault_path)
        .max_depth(config.max_depth)
        .into_iter()
        .filter_entry(|e| {
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Ok(content) = fs::read_to_string(path) {
                let mut tasks = parse_tasks(&content, &path.to_string_lossy());
                if !config.show_done_tasks {
                    tasks.retain(|t| !t.done);
                }
                all_tasks.extend(tasks);
            }
        }
    }
    all_tasks
}

pub fn count(config: &AppConfig) -> u32 {
    collect_all_tasks(config).len() as u32
}

pub fn generate_default_config(path: &str) -> Result<(), String> {
    let mut ini = Ini::new();

    ini.set("obsidian", "vault_path", Some("/path/to/your/vault".to_string()));
    ini.set("obsidian", "max_depth", Some("5".to_string()));

    ini.set("app", "show_done_tasks", Some("false".to_string()));
    ini.set("app", "theme", Some("dark".to_string()));

    ini.write(path).map_err(|e| format!("Не удалось записать конфиг: {}", e))?;

    Ok(())
}