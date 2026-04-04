// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod scanner;
mod cleaner;
mod ai;
mod database;
mod os;
mod utils;
mod commands;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application state shared across commands
#[derive(Clone)]
pub struct AppState {
    pub db_path: PathBuf,
    pub data_dir: PathBuf,
    pub snapshot_dir: PathBuf,
}

impl AppState {
    pub fn new(data_dir: PathBuf) -> Self {
        let db_path = data_dir.join("sloth-cleaner.db");
        let snapshot_dir = data_dir.join("snapshots");
        
        Self {
            db_path,
            data_dir,
            snapshot_dir,
        }
    }
}

/// System information structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub hostname: String,
    pub cpu_brand: String,
    pub cpu_cores: u32,
    pub total_memory_gb: f64,
    pub available_memory_gb: f64,
    pub disk_total_gb: f64,
    pub disk_available_gb: f64,
}

fn main() {
    // Setup logger
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::start_scan,
            commands::get_scan_progress,
            commands::get_scan_results,
            commands::propose_cleanup,
            commands::execute_cleanup,
            commands::list_snapshots,
            commands::restore_snapshot,
            commands::get_analytics,
            commands::ai_chat,
            commands::get_ai_recommendations,
            commands::update_user_feedback,
            commands::get_system_info,
            commands::export_logs,
            commands::reset_learning_data,
        ])
        .setup(|app| {
            // Initialize database
            let app_handle = app.handle().clone();
            database::init(&app_handle)?;
            
            // Initialize AI model (lazy load)
            ai::init_model(&app_handle)?;
            
            log::info!("SlothCleaner initialized successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
