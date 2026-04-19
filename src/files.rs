use std::fs;
use walkdir::WalkDir;
use configparser::ini::Ini;
use std::env;
mod task;
use task::Task;

#[derive(Debug)]
pub struct AppConfig {
    vault_path: String,
    max_depth: usize,
    show_done_tasks: bool,
    theme: String,
    path: String,
}
impl AppConfig {
    pub fn new() -> Result<Self, String> {
        let path = config_path();
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

        Ok(Self {
            vault_path,
            max_depth,
            show_done_tasks,
            theme,
            path,
        })
    }
}
fn load_config(path: &str) -> Result<AppConfig, String> {
    let mut ini = Ini::new();
    ini.load(path).map_err(|e| format!("Не удалось загрузить конфиг: {}", e))?;

    // читаем значения: get(секция, ключ) -> Option<String>
    let vault_path = ini
        .get("obsidian", "vault_path")
        .ok_or("Не найден vault_path")?;

    let max_depth = ini
        .get("obsidian", "max_depth")
        .unwrap_or("5".to_string())       // дефолтное значение
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

    Ok(AppConfig {
        vault_path,
        max_depth,
        show_done_tasks,
        theme,
        path: path.to_string(),
    })
}
fn parse_tasks(content: &str, source_file: &str) -> Vec<Task> {
    let mut tasks = Vec::new();

    for (line_idx, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Ищем паттерн: "- [ ] текст" или "- [x] текст"
        if let Some(rest) = trimmed.strip_prefix("- [") {
            if let Some(inner) = rest.get(..1) {
                let done = inner.eq_ignore_ascii_case("x");
                let is_valid_checkbox = inner == " " || inner.eq_ignore_ascii_case("x");

                if is_valid_checkbox {
                    if let Some(text_part) = rest.get(3..) { // пропускаем "] "
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

fn collect_all_tasks(config: &AppConfig) -> Vec<Task> {
    let mut all_tasks: Vec<Task> = Vec::new();

    for entry in WalkDir::new(&config.vault_path)  // <-- используем путь из конфига
        .max_depth(config.max_depth)                // <-- используем глубину
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

                // фильтруем выполненные если нужно
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
	let mut counter: u32 = 0;

	let tasks = collect_all_tasks(config);

	for task in tasks {
		counter += 1;
	}

	counter
}
    pub fn config_path() -> String {
        // папка где лежит сам .exe
        let exe_dir = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        //self.path = exe_dir.to_string_lossy().to_string();
        exe_dir.join("../settings.ini").to_string_lossy().to_string()

    }