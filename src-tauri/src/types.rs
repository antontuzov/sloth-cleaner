use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use chrono::Utc;

// ============================================================
// Serializable Types (Frontend-facing) - All camelCase for JS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResults {
    pub scan_id: String,
    pub timestamp: String,
    pub total_size: u64,
    pub file_count: u64,
    pub duration_ms: u64,
    pub categories: Vec<CategoryResults>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryResults {
    pub category: String,
    pub size: u64,
    pub file_count: u64,
    pub files: Vec<FileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub category: String,
    pub modified: String,
    pub safety_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanProgress {
    pub scan_id: String,
    pub files_scanned: u64,
    pub total_size_scanned: u64,
    pub categories_found: u32,
    pub progress_percent: f32,
    pub is_complete: bool,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupProposal {
    pub proposal_id: String,
    pub scan_id: String,
    pub categories: Vec<CategoryProposal>,
    pub total_size_to_free: u64,
    pub total_file_count: u32,
    pub estimated_time_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryProposal {
    pub category: String,
    pub size: u64,
    pub file_count: u32,
    pub safety_score: f32,
    pub files: Vec<FileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanupResult {
    pub proposal_id: String,
    pub success: bool,
    pub freed_space: u64,
    pub files_deleted: u64,
    pub errors: Vec<String>,
    pub snapshot_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotInfo {
    pub id: String,
    pub timestamp: String,
    pub files_count: u32,
    pub total_size: u64,
    pub restore_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalyticsData {
    pub days: Vec<DayAnalytics>,
    pub total_space_freed_gb: f64,
    pub total_scans: u32,
    pub total_cleanups: u32,
    pub average_savings_per_cleanup_gb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayAnalytics {
    pub date: String,
    pub space_freed_gb: f64,
    pub scans_count: u32,
    pub cleanups_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIResponse {
    pub message: String,
    pub actions: Vec<AIAction>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIAction {
    pub label: String,
    pub action_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AIContext {
    pub scan_results: Option<ScanResults>,
    pub system_info: Option<SystemInfo>,
    pub user_preferences: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub potential_savings_gb: f64,
    pub safety_score: f32,
    pub action_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

// ============================================================
// Internal State Types (Backend-only, not serialized to frontend)
// ============================================================

pub struct ScanState {
    pub scan_id: String,
    pub is_running: bool,
    pub is_paused: bool,
    pub files_scanned: u64,
    pub total_size_scanned: u64,
    pub categories_found: u32,
    pub start_time: chrono::DateTime<Utc>,
    pub results: Option<ScanResults>,
    pub category_map: HashMap<String, CategoryResults>,
}

impl ScanState {
    pub fn new(scan_id: String) -> Self {
        Self {
            scan_id,
            is_running: false,
            is_paused: false,
            files_scanned: 0,
            total_size_scanned: 0,
            categories_found: 0,
            start_time: Utc::now(),
            results: None,
            category_map: HashMap::new(),
        }
    }

    pub fn progress_percent(&self) -> f32 {
        (self.files_scanned as f32 / 100000.0).min(1.0) * 100.0
    }

    pub fn elapsed_ms(&self) -> u64 {
        Utc::now().signed_duration_since(self.start_time).num_milliseconds() as u64
    }
}

pub struct ProposalState {
    pub proposal_id: String,
    pub scan_id: String,
    pub categories: Vec<CategoryProposal>,
    pub total_size_to_free: u64,
    pub total_file_count: u32,
    pub estimated_time_seconds: u32,
}

/// Application state shared across Tauri commands
pub struct AppState {
    pub db_path: PathBuf,
    pub data_dir: PathBuf,
    pub snapshot_dir: PathBuf,
    pub scans: Arc<Mutex<HashMap<String, ScanState>>>,
    pub proposals: Arc<Mutex<HashMap<String, ProposalState>>>,
}

impl AppState {
    pub fn new(data_dir: PathBuf) -> Self {
        let db_path = data_dir.join("sloth-cleaner.db");
        let snapshot_dir = data_dir.join("snapshots");

        Self {
            db_path,
            data_dir,
            snapshot_dir,
            scans: Arc::new(Mutex::new(HashMap::new())),
            proposals: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            db_path: self.db_path.clone(),
            data_dir: self.data_dir.clone(),
            snapshot_dir: self.snapshot_dir.clone(),
            scans: Arc::clone(&self.scans),
            proposals: Arc::clone(&self.proposals),
        }
    }
}
