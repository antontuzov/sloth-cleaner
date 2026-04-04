// Library module exports for SlothCleaner
pub mod scanner;
pub mod cleaner;
pub mod ai;
pub mod database;
pub mod os;
pub mod utils;
pub mod commands;

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
