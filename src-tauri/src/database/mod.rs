pub mod schema;
pub mod migrations;

use std::path::Path;
use rusqlite::{Connection, Result};
use tauri::Manager;

/// Initialize the database
pub fn init(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let data_dir = app_handle.path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    
    std::fs::create_dir_all(&data_dir)
        .map_err(|e| format!("Failed to create data dir: {}", e))?;
    
    let db_path = data_dir.join("sloth-cleaner.db");
    
    // Run migrations
    migrations::run_migrations(&db_path)
        .map_err(|e| format!("Failed to run migrations: {}", e))?;
    
    log::info!("Database initialized at: {:?}", db_path);
    Ok(())
}

/// Get database connection
pub fn get_connection(db_path: &Path) -> Result<Connection, String> {
    Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))
}

/// Save scan results to database
pub fn save_scan(conn: &Connection, scan_id: &str, total_size: u64, file_count: u64, duration_ms: u64, categories_json: &str) -> Result<(), String> {
    conn.execute(
        "INSERT OR REPLACE INTO scans (id, timestamp, total_size, file_count, duration_ms, categories_json) VALUES (?1, datetime('now'), ?2, ?3, ?4, ?5)",
        (scan_id, total_size, file_count, duration_ms, categories_json),
    ).map_err(|e| format!("Failed to save scan: {}", e))?;
    
    Ok(())
}

/// Save cleanup results to database
pub fn save_cleanup(conn: &Connection, cleanup_id: &str, scan_id: &str, freed_space: u64, files_deleted: u64, snapshot_id: Option<&str>, status: &str) -> Result<(), String> {
    conn.execute(
        "INSERT OR REPLACE INTO cleanups (id, scan_id, timestamp, freed_space, files_deleted, snapshot_id, status) VALUES (?1, ?2, datetime('now'), ?3, ?4, ?5, ?6)",
        (cleanup_id, scan_id, freed_space, files_deleted, snapshot_id, status),
    ).map_err(|e| format!("Failed to save cleanup: {}", e))?;
    
    Ok(())
}

/// Record user decision for ML training
pub fn record_user_decision(
    conn: &Connection,
    file_path: &str,
    file_size: u64,
    category: &str,
    file_age_days: u64,
    decision: &str,
    ai_score: f32,
) -> Result<(), String> {
    let id = uuid::Uuid::new_v4().to_string();
    
    conn.execute(
        "INSERT INTO file_decisions (id, file_path, file_size, category, file_age_days, decision, timestamp, ai_score) VALUES (?1, ?2, ?3, ?4, ?5, ?6, datetime('now'), ?7)",
        (id, file_path, file_size, category, file_age_days, decision, ai_score),
    ).map_err(|e| format!("Failed to record decision: {}", e))?;
    
    Ok(())
}

/// Get analytics data
pub fn get_analytics_data(conn: &Connection, days: u32) -> Result<Vec<(String, u64, u32, u32)>, String> {
    let mut stmt = conn.prepare(
        "SELECT date, COALESCE(SUM(space_freed), 0), COALESCE(SUM(scans_count), 0), COALESCE(SUM(cleanups_count), 0) FROM analytics WHERE date >= date('now', ?1) GROUP BY date ORDER BY date"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let rows = stmt.query_map([format!("-{} days", days)], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, u64>(1)?,
            row.get::<_, u32>(2)?,
            row.get::<_, u32>(3)?,
        ))
    }).map_err(|e| format!("Failed to query analytics: {}", e))?;
    
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect results: {}", e))
}
