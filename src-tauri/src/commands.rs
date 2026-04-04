use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::AppState;
use crate::scanner::ScanResults;
use crate::cleaner::CleanupResult;

// Scan types
#[derive(Debug, Serialize, Deserialize)]
pub struct ScanId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanProgress {
    pub scan_id: String,
    pub files_scanned: u64,
    pub total_size_scanned: u64,
    pub categories_found: u32,
    pub progress_percent: f32,
    pub is_complete: bool,
    pub elapsed_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanupProposal {
    pub proposal_id: String,
    pub scan_id: String,
    pub categories: Vec<CategoryProposal>,
    pub total_size_to_free: u64,
    pub total_file_count: u32,
    pub estimated_time_seconds: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryProposal {
    pub category: String,
    pub size: u64,
    pub file_count: u32,
    pub safety_score: f32,
    pub files: Vec<FileInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub category: String,
    pub modified: String,
    pub safety_score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub timestamp: String,
    pub files_count: u32,
    pub total_size: u64,
    pub restore_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyticsData {
    pub days: Vec<DayAnalytics>,
    pub total_space_freed_gb: f64,
    pub total_scans: u32,
    pub total_cleanups: u32,
    pub average_savings_per_cleanup_gb: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DayAnalytics {
    pub date: String,
    pub space_freed_gb: f64,
    pub scans_count: u32,
    pub cleanups_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIResponse {
    pub message: String,
    pub actions: Vec<AIAction>,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIAction {
    pub label: String,
    pub action_type: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIContext {
    pub scan_results: Option<ScanResults>,
    pub system_info: Option<crate::SystemInfo>,
    pub user_preferences: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub potential_savings_gb: f64,
    pub safety_score: f32,
    pub action_label: String,
}

// Command implementations

#[tauri::command]
pub async fn start_scan(app_state: State<'_, AppState>) -> Result<ScanId, String> {
    log::info!("Starting new scan...");
    let scan_id = Uuid::new_v4().to_string();
    
    // TODO: Implement actual scanning
    // For now, return a placeholder
    Ok(ScanId { id: scan_id })
}

#[tauri::command]
pub async fn get_scan_progress(scan_id: String) -> Result<ScanProgress, String> {
    // TODO: Track actual progress
    Ok(ScanProgress {
        scan_id,
        files_scanned: 0,
        total_size_scanned: 0,
        categories_found: 0,
        progress_percent: 0.0,
        is_complete: false,
        elapsed_ms: 0,
    })
}

#[tauri::command]
pub async fn get_scan_results(scan_id: String) -> Result<ScanResults, String> {
    // TODO: Return actual scan results
    Err(format!("Scan results not found for: {}", scan_id))
}

#[tauri::command]
pub async fn propose_cleanup(scan_id: String) -> Result<CleanupProposal, String> {
    // TODO: Generate cleanup proposal from scan results
    Err(format!("No scan results found for: {}", scan_id))
}

#[tauri::command]
pub async fn execute_cleanup(proposal_id: String, dry_run: bool) -> Result<CleanupResult, String> {
    log::info!("Executing cleanup: {} (dry_run: {})", proposal_id, dry_run);
    
    // TODO: Implement actual cleanup
    Ok(CleanupResult {
        proposal_id,
        success: true,
        freed_space: 0,
        files_deleted: 0,
        errors: vec![],
        snapshot_id: None,
    })
}

#[tauri::command]
pub async fn list_snapshots() -> Result<Vec<Snapshot>, String> {
    // TODO: List actual snapshots
    Ok(vec![])
}

#[tauri::command]
pub async fn restore_snapshot(snapshot_id: String) -> Result<(), String> {
    // TODO: Implement snapshot restore
    Err(format!("Snapshot not found: {}", snapshot_id))
}

#[tauri::command]
pub async fn get_analytics(days: u32) -> Result<AnalyticsData, String> {
    // TODO: Query analytics from database
    Ok(AnalyticsData {
        days: vec![],
        total_space_freed_gb: 0.0,
        total_scans: 0,
        total_cleanups: 0,
        average_savings_per_cleanup_gb: 0.0,
    })
}

#[tauri::command]
pub async fn ai_chat(message: String, context: AIContext) -> Result<AIResponse, String> {
    log::info!("AI chat message received: {}", message);
    
    // TODO: Implement AI chat with local model
    // For now, return a placeholder response
    Ok(AIResponse {
        message: format!("I received your message: \"{}\". AI integration is coming soon!", message),
        actions: vec![],
        confidence: 0.5,
    })
}

#[tauri::command]
pub async fn get_ai_recommendations() -> Result<Vec<Recommendation>, String> {
    // TODO: Generate AI-based recommendations
    Ok(vec![])
}

#[tauri::command]
pub async fn update_user_feedback(file_path: String, decision: String) -> Result<(), String> {
    log::info!("User feedback: {} -> {}", file_path, decision);
    
    // TODO: Store feedback for ML training
    Ok(())
}

#[tauri::command]
pub async fn get_system_info() -> Result<crate::SystemInfo, String> {
    use sysinfo::{System, Disks};

    let mut system = System::new_all();
    system.refresh_all();

    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    let cpu_brand = system.cpus().first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let cpu_cores = system.cpus().len() as u32;

    let total_memory_gb = system.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let available_memory_gb = system.available_memory() as f64 / (1024.0 * 1024.0 * 1024.0);

    // Get disk info - sysinfo 0.30 uses new() + refresh()
    let mut disks = Disks::new();
    disks.refresh();
    let (disk_total_gb, disk_available_gb) = disks.iter()
        .next()
        .map(|d| {
            (
                d.total_space() as f64 / (1024.0 * 1024.0 * 1024.0),
                d.available_space() as f64 / (1024.0 * 1024.0 * 1024.0),
            )
        })
        .unwrap_or((0.0, 0.0));

    Ok(crate::SystemInfo {
        os_name,
        os_version,
        hostname,
        cpu_brand,
        cpu_cores,
        total_memory_gb,
        available_memory_gb,
        disk_total_gb,
        disk_available_gb,
    })
}

#[tauri::command]
pub async fn export_logs(app_state: State<'_, AppState>) -> Result<String, String> {
    // TODO: Export logs to file
    Ok(app_state.data_dir.join("logs.txt").to_string_lossy().to_string())
}

#[tauri::command]
pub async fn reset_learning_data() -> Result<(), String> {
    // TODO: Reset ML training data
    Ok(())
}
