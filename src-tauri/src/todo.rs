use chrono::{Local, Datelike};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};

use crate::config;

// --- Data Structures ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub text: String,
    pub done: bool,
    pub done_at: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoDay {
    pub date: String,
    pub items: Vec<TodoItem>,
}

// --- Helper Functions ---

fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    (0..6)
        .map(|_| {
            let chars = b"abcdefghijklmnopqrstuvwxyz0123456789";
            chars[rng.gen_range(0..chars.len())] as char
        })
        .collect()
}

fn get_todo_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let config = config::load_config(app)?;
    if let Some(ref path) = config.todo_config.storage_path {
        Ok(PathBuf::from(path))
    } else {
        let config_dir = app
            .path()
            .app_config_dir()
            .map_err(|e| format!("Failed to get config dir: {}", e))?;
        Ok(config_dir.join("todo"))
    }
}

fn ensure_todo_dir(dir: &PathBuf) -> Result<(), String> {
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|e| format!("Failed to create todo dir: {}", e))?;
    }
    Ok(())
}

fn today_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

fn load_day(dir: &PathBuf, date: &str) -> Result<TodoDay, String> {
    let file_path = dir.join(format!("{}.json", date));
    if !file_path.exists() {
        return Ok(TodoDay {
            date: date.to_string(),
            items: Vec::new(),
        });
    }
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read todo file: {}", e))?;
    serde_json::from_str::<TodoDay>(&content)
        .map_err(|e| format!("Failed to parse todo file: {}", e))
}

fn save_day(dir: &PathBuf, day: &TodoDay) -> Result<(), String> {
    ensure_todo_dir(dir)?;
    let file_path = dir.join(format!("{}.json", day.date));
    let content = serde_json::to_string_pretty(day)
        .map_err(|e| format!("Failed to serialize todo: {}", e))?;
    fs::write(&file_path, content).map_err(|e| format!("Failed to write todo file: {}", e))
}

fn sort_items(items: &mut Vec<TodoItem>) {
    items.sort_by(|a, b| {
        if a.done != b.done {
            if !a.done { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
        } else if !a.done {
            a.created_at.cmp(&b.created_at)
        } else {
            b.done_at.cmp(&a.done_at)
        }
    });
}

pub fn format_day_text(day: &TodoDay) -> String {
    let mut lines = vec![day.date.clone()];
    for item in &day.items {
        let check = if item.done { "✅" } else { "☐" };
        lines.push(format!("{} {}", check, item.text));
    }
    lines.join("\n")
}

fn format_week_text(days: &[TodoDay]) -> String {
    let mut parts: Vec<String> = Vec::new();
    for day in days {
        let done_items: Vec<&TodoItem> = day.items.iter().filter(|i| i.done).collect();
        if !done_items.is_empty() {
            let mut lines = vec![day.date.clone()];
            for item in done_items {
                lines.push(format!("✅ {}", item.text));
            }
            parts.push(lines.join("\n"));
        }
    }
    parts.join("\n\n")
}

fn cleanup_old_files(dir: &PathBuf, retention_days: u32) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }
    let cutoff = Local::now() - chrono::Duration::days(retention_days as i64);
    let cutoff_str = cutoff.format("%Y-%m-%d").to_string();

    let entries = fs::read_dir(dir).map_err(|e| format!("Failed to read todo dir: {}", e))?;
    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if name.ends_with(".json") && name < cutoff_str.as_str() {
                let _ = fs::remove_file(entry.path());
            }
        }
    }
    Ok(())
}

// --- Tauri Commands ---

#[tauri::command]
pub async fn todo_add_item(
    app: AppHandle,
    text: String,
    is_done: Option<bool>,
    date: Option<String>,
) -> Result<TodoDay, String> {
    let dir = get_todo_dir(&app)?;
    let date = date.unwrap_or_else(today_date);
    let is_done = is_done.unwrap_or(true);
    let now = Local::now().timestamp();

    let mut day = load_day(&dir, &date)?;
    let item = TodoItem {
        id: generate_id(),
        text: text.trim().to_string(),
        done: is_done,
        done_at: if is_done { Some(now) } else { None },
        created_at: now,
    };
    day.items.push(item);
    sort_items(&mut day.items);
    save_day(&dir, &day)?;

    let _ = app.emit("todo-list-changed", ());
    Ok(day)
}

#[tauri::command]
pub async fn todo_toggle_item(
    app: AppHandle,
    id: String,
    date: Option<String>,
) -> Result<TodoDay, String> {
    let dir = get_todo_dir(&app)?;
    let date = date.unwrap_or_else(today_date);
    let now = Local::now().timestamp();

    let mut day = load_day(&dir, &date)?;
    if let Some(item) = day.items.iter_mut().find(|i| i.id == id) {
        item.done = !item.done;
        item.done_at = if item.done { Some(now) } else { None };
    }
    sort_items(&mut day.items);
    save_day(&dir, &day)?;

    let _ = app.emit("todo-list-changed", ());
    Ok(day)
}

#[tauri::command]
pub async fn todo_delete_item(
    app: AppHandle,
    id: String,
    date: Option<String>,
) -> Result<TodoDay, String> {
    let dir = get_todo_dir(&app)?;
    let date = date.unwrap_or_else(today_date);

    let mut day = load_day(&dir, &date)?;
    day.items.retain(|i| i.id != id);
    save_day(&dir, &day)?;

    let _ = app.emit("todo-list-changed", ());
    Ok(day)
}

#[tauri::command]
pub async fn todo_get_day(
    app: AppHandle,
    date: Option<String>,
) -> Result<TodoDay, String> {
    let dir = get_todo_dir(&app)?;
    let date = date.unwrap_or_else(today_date);
    let mut day = load_day(&dir, &date)?;
    sort_items(&mut day.items);
    Ok(day)
}

#[tauri::command]
pub async fn todo_get_week(app: AppHandle) -> Result<Vec<TodoDay>, String> {
    let dir = get_todo_dir(&app)?;
    let mut days = Vec::new();
    let today = Local::now();
    let weekday = today.weekday().num_days_from_monday() as i64;
    for i in (0..weekday + 1).rev() {
        let d = today - chrono::Duration::days(i);
        let date_str = d.format("%Y-%m-%d").to_string();
        if let Ok(mut day) = load_day(&dir, &date_str) {
            sort_items(&mut day.items);
            days.push(day);
        }
    }
    Ok(days)
}

#[tauri::command]
pub async fn todo_copy_day(
    app: AppHandle,
    date: Option<String>,
) -> Result<String, String> {
    let dir = get_todo_dir(&app)?;
    let date = date.unwrap_or_else(today_date);
    let day = load_day(&dir, &date)?;
    Ok(format_day_text(&day))
}

#[tauri::command]
pub async fn todo_copy_week(app: AppHandle) -> Result<String, String> {
    let days = todo_get_week(app).await?;
    Ok(format_week_text(&days))
}

#[tauri::command]
pub async fn todo_get_pending_count(app: AppHandle) -> Result<u32, String> {
    let dir = get_todo_dir(&app)?;
    let day = load_day(&dir, &today_date())?;
    Ok(day.items.iter().filter(|i| !i.done).count() as u32)
}

/// Run cleanup on app startup
pub fn run_cleanup(app: &AppHandle) {
    if let Ok(config) = config::load_config(app) {
        if let Ok(dir) = get_todo_dir(app) {
            let _ = cleanup_old_files(&dir, config.todo_config.retention_days);
        }
    }
}
