use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::State;
use serde::Serialize;

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
    
    // Capture final time to return it (e.g. for logging)
    let info = get_time_info_internal(&timer);
    
    // Reset
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(TimerState::new())))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start_timer, pause_timer, stop_timer, get_timer_time])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
