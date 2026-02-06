use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::{State, AppHandle, Manager};
use serde::Serialize;
use screenshots::Screen;
use active_win_pos_rs::get_active_window;
use chrono::Local;
use std::fs;

struct TimerState {
    start_time: Option<Instant>,
    accumulated_time: Duration,
    is_running: bool,
}

impl TimerState {
    fn new() -> Self {
        Self {
            start_time: None,
            accumulated_time: Duration::new(0, 0),
            is_running: false,
        }
    }
}

pub struct AppState(Mutex<TimerState>);

#[derive(Serialize)]
struct TimeInfo {
    formatted_time: String,
    seconds: u64,
    is_running: bool,
}

#[derive(Serialize)]
struct ScreenshotResult {
    path: String,
    category: String,
    timestamp: String,
}

fn get_time_info_internal(timer: &TimerState) -> TimeInfo {
    let mut total = timer.accumulated_time;
    if timer.is_running {
        if let Some(start) = timer.start_time {
            total += start.elapsed();
        }
    }
    
    let secs = total.as_secs();
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    
    TimeInfo {
        formatted_time: format!("{:02}:{:02}:{:02}", h, m, s),
        seconds: secs,
        is_running: timer.is_running,
    }
}

#[tauri::command]
fn start_timer(state: State<AppState>) -> TimeInfo {
    let mut timer = state.0.lock().unwrap();
    if !timer.is_running {
        timer.start_time = Some(Instant::now());
        timer.is_running = true;
    }
    get_time_info_internal(&timer)
}

#[tauri::command]
fn pause_timer(state: State<AppState>) -> TimeInfo {
    let mut timer = state.0.lock().unwrap();
    if timer.is_running {
        if let Some(start) = timer.start_time {
            timer.accumulated_time += start.elapsed();
        }
        timer.start_time = None;
        timer.is_running = false;
    }
    get_time_info_internal(&timer)
}

#[tauri::command]
fn stop_timer(state: State<AppState>) -> TimeInfo {
    let mut timer = state.0.lock().unwrap();
    let info = get_time_info_internal(&timer);
    timer.start_time = None;
    timer.accumulated_time = Duration::new(0, 0);
    timer.is_running = false;
    info
}

#[tauri::command]
fn get_timer_time(state: State<AppState>) -> TimeInfo {
    let timer = state.0.lock().unwrap();
    get_time_info_internal(&timer)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn categorize_activity(app_name: &str, title: &str) -> String {
    let app_lower = app_name.to_lowercase();
    let title_lower = title.to_lowercase();
    
    if app_lower.contains("code") || app_lower.contains("intellij") || title_lower.contains("visual studio") || app_lower.contains("terminal") {
        return "Work (Coding)".to_string();
    }
    if app_lower.contains("zoom") || app_lower.contains("teams") || app_lower.contains("slack") || app_lower.contains("discord") || app_lower.contains("skype") {
        return "Meeting/Communication".to_string();
    }
    if app_lower.contains("spotify") || title_lower.contains("netflix") || title_lower.contains("youtube") || app_lower.contains("steam") || app_lower.contains("game") {
         return "Break".to_string();
    }
    
    "Work".to_string()
}

#[tauri::command]
async fn capture_screenshot(app_handle: AppHandle) -> Result<ScreenshotResult, String> {
    // 1. Get Active Window for categorization
    let (app_name, title) = match get_active_window() {
        Ok(win) => (win.app_name, win.title),
        Err(_) => ("Unknown".to_string(), "Unknown".to_string()),
    };

    let category = categorize_activity(&app_name, &title);

    // 2. Prepare path
    let app_dir = app_handle.path().app_local_data_dir().map_err(|e| e.to_string())?;
    let screenshots_dir = app_dir.join("screenshots");
    if !screenshots_dir.exists() {
        fs::create_dir_all(&screenshots_dir).map_err(|e| e.to_string())?;
    }

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let filename = format!("screenshot_{}.png", timestamp);
    let path = screenshots_dir.join(&filename);
    let path_str = path.to_string_lossy().to_string();
    let path_clone = path.clone();

    // 3. Capture Screen & Save in Blocking Task
    tauri::async_runtime::spawn_blocking(move || {
        let screens = Screen::all().map_err(|e| e.to_string())?;
        let screen = screens.first().ok_or("No screen found")?; 
        let image = screen.capture().map_err(|e| e.to_string())?;
        
        image.save(&path_clone).map_err(|e| e.to_string())?;
        Ok::<(), String>(())
    }).await.map_err(|e| e.to_string())??;

    Ok(ScreenshotResult {
        path: path_str,
        category,
        timestamp,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(TimerState::new())))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            start_timer, 
            pause_timer, 
            stop_timer, 
            get_timer_time,
            capture_screenshot
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
