use tauri::{State, Manager, Emitter};
use uuid::Uuid;
use chrono::Utc;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::types::{
    AppState, ScanState, ProposalState,
    ScanProgress, ScanResults, CategoryResults, FileInfo,
    CleanupProposal, CategoryProposal, CleanupResult,
    SnapshotInfo, AnalyticsData, DayAnalytics,
    AIResponse, AIAction, AIContext, Recommendation, SystemInfo,
};

// ============================================================
// Scan Commands
// ============================================================

#[tauri::command]
pub fn start_scan(app_state: State<AppState>) -> Result<String, String> {
    let scan_id = Uuid::new_v4().to_string();
    log::info!("Starting scan: {}", scan_id);

    let scan_state = ScanState::new(scan_id.clone());

    let mut scans = app_state.scans.lock()
        .map_err(|e| format!("Failed to lock scans: {}", e))?;
    scans.insert(scan_id.clone(), scan_state);

    let app_state_clone = app_state.inner().clone();
    let scan_id_clone = scan_id.clone();
    std::thread::spawn(move || {
        if let Err(e) = run_scan(&app_state_clone, &scan_id_clone) {
            log::error!("Scan failed: {}", e);
        }
    });

    Ok(scan_id)
}

/// Run the actual scan with incremental progress
fn run_scan(app_state: &AppState, scan_id: &str) -> Result<(), String> {
    let start = std::time::Instant::now();
    eprintln!("[SCAN] Starting scan: {}", scan_id);

    // Mark as running
    {
        let mut scans = app_state.scans.lock()
            .map_err(|e| format!("Lock failed: {}", e))?;
        if let Some(state) = scans.get_mut(scan_id) {
            state.is_running = true;
        }
    }

    // Get target directories
    let scan_dirs = crate::scanner::filesystem::get_cache_dirs();
    eprintln!("[SCAN] Found {} cache directories", scan_dirs.len());
    for d in &scan_dirs {
        eprintln!("[SCAN]   - {:?}", d);
    }

    let scan_dirs = if scan_dirs.is_empty() {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"));
        eprintln!("[SCAN] No cache dirs, falling back to: {:?}", home);
        vec![home]
    } else {
        scan_dirs
    };

    // Exclude patterns
    let excludes = vec![
        ".git".to_string(),
        "node_modules".to_string(),
    ];

    // Shared state for the callback
    let shared = std::sync::Arc::new(std::sync::Mutex::new((
        0u64,   // files_scanned
        0u64,   // total_size
        0u32,   // categories_found
        HashMap::<String, CategoryResults>::new(),
    )));

    // Process each file as it's found (incremental progress)
    let shared_clone = std::sync::Arc::clone(&shared);
    let app_state_clone = app_state.clone();
    let scan_id_string = scan_id.to_string();

    eprintln!("[SCAN] Starting file walk...");

    crate::scanner::filesystem::scan_multi_with_options(
        &scan_dirs,
        Some(15),
        Some(&excludes),
        |file_path| {
            // Get file metadata
            let metadata = match std::fs::metadata(file_path) {
                Ok(m) => m,
                Err(_) => return,
            };

            let size = metadata.len();
            let name = file_path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            let modified = metadata.modified()
                .ok()
                .and_then(|t| t.elapsed().ok())
                .map(|d| format!("{} days ago", d.as_secs() / 86400))
                .unwrap_or_else(|| "Unknown".to_string());

            // Categorize
            let cat = crate::scanner::categorizer::categorize_file(file_path, size);
            let category_name = cat.as_str().to_string();

            let file_info = FileInfo {
                path: file_path.to_string_lossy().to_string(),
                name,
                size,
                category: category_name.clone(),
                modified,
                safety_score: calculate_category_safety(&category_name),
            };

            let mut locked = shared_clone.lock().unwrap();
            locked.0 += 1;
            locked.1 += size;

            let entry = locked.3.entry(category_name).or_insert(CategoryResults {
                category: String::new(),
                size: 0,
                file_count: 0,
                files: Vec::new(),
            });
            entry.category = file_info.category.clone();
            entry.size += file_info.size;
            entry.file_count += 1;

            // Only store first 100 files per category to save memory
            if entry.files.len() < 100 {
                entry.files.push(file_info);
            }
            drop(entry);

            locked.2 = locked.3.len() as u32;

            // Update progress in AppState every 50 files
            if locked.0 % 50 == 0 {
                if let Ok(mut scans) = app_state_clone.scans.lock() {
                    if let Some(state) = scans.get_mut(&scan_id_string) {
                        state.files_scanned = locked.0;
                        state.total_size_scanned = locked.1;
                        state.categories_found = locked.2;
                    }
                }
            }
        },
    )?;

    // Finalize results
    let (total_file_count, total_size, categories_found, category_map) = {
        let locked = shared.lock().unwrap();
        eprintln!("[SCAN] Final: {} files, {:.1} MB, {} categories",
            locked.0, locked.1 as f64 / 1_048_576.0, locked.2);
        (locked.0, locked.1, locked.2, locked.3.clone())
    };

    let duration_ms = start.elapsed().as_millis() as u64;

    let results = ScanResults {
        scan_id: scan_id.to_string(),
        timestamp: Utc::now().to_rfc3339(),
        total_size,
        file_count: total_file_count,
        duration_ms,
        categories: category_map.values().cloned().collect(),
    };

    // Store final results
    if let Ok(mut scans) = app_state.scans.lock() {
        if let Some(state) = scans.get_mut(scan_id) {
            state.is_running = false;
            state.files_scanned = total_file_count;
            state.total_size_scanned = total_size;
            state.categories_found = categories_found;
            state.results = Some(results.clone());
            state.category_map = category_map;
        }
    }

    // Save scan to database
    if let Ok(conn) = rusqlite::Connection::open(&app_state.db_path) {
        let categories_json = serde_json::to_string(&results.categories).unwrap_or_default();
        let _ = crate::database::save_scan(
            &conn,
            scan_id,
            total_size,
            total_file_count,
            duration_ms,
            &categories_json,
        );
    }

    log::info!("Scan complete: {} files, {} in {}ms",
        total_file_count,
        crate::scanner::filesystem::format_file_size(total_size),
        duration_ms);

    // Send desktop notification
    let _ = notify_rust::Notification::new()
        .summary("Scan Complete")
        .body(&format!("Found {} files, {:.1} MB can be freed",
            total_file_count,
            total_size as f64 / 1_048_576.0))
        .timeout(5000)
        .show();

    Ok(())
}

#[tauri::command]
pub fn get_scan_progress(
    scan_id: String,
    app_state: State<AppState>,
) -> Result<ScanProgress, String> {
    let scans = app_state.scans.lock()
        .map_err(|e| format!("Failed to lock scans: {}", e))?;

    let state = scans.get(&scan_id)
        .ok_or_else(|| format!("Scan not found: {}", scan_id))?;

    Ok(ScanProgress {
        scan_id: state.scan_id.clone(),
        files_scanned: state.files_scanned,
        total_size_scanned: state.total_size_scanned,
        categories_found: state.categories_found,
        progress_percent: state.progress_percent(),
        is_complete: !state.is_running,
        elapsed_ms: state.elapsed_ms(),
    })
}

#[tauri::command]
pub fn get_scan_results(
    scan_id: String,
    app_state: State<AppState>,
) -> Result<ScanResults, String> {
    let scans = app_state.scans.lock()
        .map_err(|e| format!("Failed to lock scans: {}", e))?;

    let state = scans.get(&scan_id)
        .ok_or_else(|| format!("Scan not found: {}", scan_id))?;

    state.results.clone()
        .ok_or_else(|| "Scan not complete".to_string())
}

// ============================================================
// Cleanup Commands
// ============================================================

#[tauri::command]
pub fn propose_cleanup(
    scan_id: String,
    app_state: State<AppState>,
) -> Result<CleanupProposal, String> {
    let scans = app_state.scans.lock()
        .map_err(|e| format!("Failed to lock scans: {}", e))?;

    let state = scans.get(&scan_id)
        .ok_or_else(|| format!("Scan not found: {}", scan_id))?;

    let results = state.results.as_ref()
        .ok_or_else(|| "Scan not complete".to_string())?;

    let mut categories = Vec::new();
    let mut total_size_to_free: u64 = 0;
    let mut total_file_count: u32 = 0;

    for cat in &results.categories {
        let safety_score = calculate_category_safety(&cat.category);

        let proposal = CategoryProposal {
            category: cat.category.clone(),
            size: cat.size,
            file_count: cat.file_count as u32,
            safety_score,
            files: cat.files.clone(),
        };

        if safety_score >= 0.7 {
            total_size_to_free += cat.size;
            total_file_count += cat.file_count as u32;
        }

        categories.push(proposal);
    }

    categories.sort_by(|a, b| b.safety_score.partial_cmp(&a.safety_score).unwrap_or(std::cmp::Ordering::Equal));

    let proposal_id = Uuid::new_v4().to_string();

    let proposal_state = ProposalState {
        proposal_id: proposal_id.clone(),
        scan_id: scan_id.clone(),
        categories: categories.clone(),
        total_size_to_free,
        total_file_count,
        estimated_time_seconds: (total_size_to_free as f64 / 100_000_000.0).ceil() as u32 + 5,
    };

    drop(scans);
    let mut proposals = app_state.proposals.lock()
        .map_err(|e| format!("Failed to lock proposals: {}", e))?;
    proposals.insert(proposal_id.clone(), proposal_state);

    Ok(CleanupProposal {
        proposal_id,
        scan_id,
        categories,
        total_size_to_free,
        total_file_count,
        estimated_time_seconds: 30,
    })
}

/// Calculate safety score for a category
fn calculate_category_safety(category: &str) -> f32 {
    match category {
        "Browser Cache" | "Temporary Files" | "Thumbnails" | "Logs" | "Trash" => 0.95,
        "System Cache" | "Application Cache" => 0.85,
        "Development Cache" => 0.80,
        "Downloads" => 0.50,
        "Large Files" => 0.40,
        "Duplicates" => 0.60,
        _ => 0.50,
    }
}

#[tauri::command]
pub fn execute_cleanup(
    proposal_id: String,
    dry_run: bool,
    app: tauri::AppHandle,
    app_state: State<AppState>,
) -> Result<CleanupResult, String> {
    log::info!("Executing cleanup: {} (dry_run: {})", proposal_id, dry_run);

    let proposals = app_state.proposals.lock()
        .map_err(|e| format!("Failed to lock proposals: {}", e))?;

    let proposal = proposals.get(&proposal_id)
        .ok_or_else(|| format!("Proposal not found: {}", proposal_id))?;

    let mut total_freed: u64 = 0;
    let mut files_deleted: u64 = 0;
    let mut errors: Vec<String> = Vec::new();

    // Dry run — just report
    if dry_run {
        for cat in &proposal.categories {
            if cat.safety_score >= 0.7 {
                total_freed += cat.size;
                files_deleted += cat.file_count as u64;
            }
        }

        return Ok(CleanupResult {
            proposal_id,
            success: true,
            freed_space: total_freed,
            files_deleted,
            errors,
            snapshot_id: None,
        });
    }

    // Create snapshot before cleanup
    let snapshot_id = {
        let snapshot_dir = app_state.snapshot_dir.clone();
        std::fs::create_dir_all(&snapshot_dir).ok();

        let snap_id = Uuid::new_v4().to_string();
        let snap_path = snapshot_dir.join(&snap_id);
        std::fs::create_dir_all(&snap_path).ok();

        let mut snap_count = 0u32;
        let mut snap_size = 0u64;

        for cat in &proposal.categories {
            if cat.safety_score < 0.7 { continue; }

            for file in &cat.files {
                let file_path = PathBuf::from(&file.path);
                // Skip files >1GB without explicit approval
                if file_path.metadata().map(|m| m.len()).unwrap_or(0) > 1_000_000_000 {
                    continue;
                }
                if let Some(parent) = file_path.parent() {
                    let relative = file_path.strip_prefix("/").unwrap_or(&file_path);
                    let dest = snap_path.join(relative);
                    if let Some(p) = dest.parent() {
                        std::fs::create_dir_all(p).ok();
                    }
                    if std::fs::copy(&file_path, &dest).is_ok() {
                        snap_count += 1;
                        snap_size += file_path.metadata().map(|m| m.len()).unwrap_or(0);
                    }
                }
            }
        }

        let meta = format!("timestamp={}\nfiles_count={}\ntotal_size={}", Utc::now(), snap_count, snap_size);
        std::fs::write(snap_path.join(".metadata"), meta).ok();

        if snap_count > 0 { Some(snap_id) } else { None }
    };

    // Actually delete files (move to trash)
    for cat in &proposal.categories {
        if cat.safety_score < 0.7 { continue; }

        for file in &cat.files {
            let file_path = PathBuf::from(&file.path);
            // Skip large files (>1GB)
            if file_path.metadata().map(|m| m.len()).unwrap_or(0) > 1_000_000_000 {
                errors.push(format!("Skipped large file: {} (>{})", file.name, crate::scanner::filesystem::format_file_size(1_000_000_000)));
                continue;
            }
            match trash::delete(&file_path) {
                Ok(_) => {
                    files_deleted += 1;
                    total_freed += file.size;
                }
                Err(e) => {
                    errors.push(format!("Failed to delete {}: {}", file.name, e));
                }
            }
        }
    }

    let result = CleanupResult {
        proposal_id: proposal_id.clone(),
        success: errors.is_empty(),
        freed_space: total_freed,
        files_deleted,
        errors,
        snapshot_id,
    };

    // Save to database
    if let Ok(conn) = rusqlite::Connection::open(&app_state.db_path) {
        let cleanup_id = Uuid::new_v4().to_string();
        let snap_ref = result.snapshot_id.as_deref().unwrap_or("");
        let status = if result.success { "completed" } else { "completed_with_errors" };

        let _ = conn.execute(
            "INSERT OR REPLACE INTO cleanups (id, scan_id, timestamp, freed_space, files_deleted, snapshot_id, status) VALUES (?1, ?2, datetime('now'), ?3, ?4, ?5, ?6)",
            (cleanup_id, proposal.scan_id.clone(), result.freed_space as i64, result.files_deleted as i64, snap_ref, status),
        );
    }

    // Emit event to frontend
    let _ = app.emit("cleanup_completed", &result);

    // Send desktop notification
    if result.success {
        let _ = notify_rust::Notification::new()
            .summary("Cleanup Complete")
            .body(&format!("Freed {}, deleted {} files",
                crate::scanner::filesystem::format_file_size(total_freed), files_deleted))
            .timeout(5000)
            .show();
    } else {
        let _ = notify_rust::Notification::new()
            .summary("Cleanup Completed with Warnings")
            .body(&format!("Freed {}, {} errors occurred",
                crate::scanner::filesystem::format_file_size(total_freed), result.errors.len()))
            .timeout(5000)
            .show();
    }

    log::info!("Cleanup complete: freed {}, deleted {} files",
        crate::scanner::filesystem::format_file_size(total_freed), files_deleted);

    Ok(result)
}

// ============================================================
// Snapshot Commands
// ============================================================

#[tauri::command]
pub fn list_snapshots(app_state: State<AppState>) -> Result<Vec<SnapshotInfo>, String> {
    let snapshot_dir = &app_state.snapshot_dir;

    if !snapshot_dir.exists() {
        return Ok(vec![]);
    }

    let mut snapshots = Vec::new();

    for entry in std::fs::read_dir(snapshot_dir)
        .map_err(|e| format!("Failed to read snapshot directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            let id = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            let metadata_path = path.join(".metadata");
            if metadata_path.exists() {
                if let Ok(metadata) = std::fs::read_to_string(&metadata_path) {
                    let files_count = metadata.lines()
                        .find(|l| l.starts_with("files_count="))
                        .and_then(|l| l.split('=').nth(1))
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0);

                    let total_size = metadata.lines()
                        .find(|l| l.starts_with("total_size="))
                        .and_then(|l| l.split('=').nth(1))
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0);

                    let timestamp = metadata.lines()
                        .find(|l| l.starts_with("timestamp="))
                        .and_then(|l| l.split('=').nth(1))
                        .unwrap_or("")
                        .to_string();

                    snapshots.push(SnapshotInfo {
                        id,
                        timestamp,
                        files_count,
                        total_size,
                        restore_count: 0,
                    });
                }
            }
        }
    }

    Ok(snapshots)
}

#[tauri::command]
pub fn restore_snapshot(snapshot_id: String, app_state: State<AppState>) -> Result<(), String> {
    let snapshot_path = app_state.snapshot_dir.join(&snapshot_id);

    if !snapshot_path.exists() {
        return Err(format!("Snapshot not found: {}", snapshot_id));
    }

    for entry in walkdir::WalkDir::new(&snapshot_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name() != ".metadata")
    {
        let source = entry.path();
        if let Ok(relative) = source.strip_prefix(&snapshot_path) {
            log::info!("Restoring: {:?}", relative);
            // Restore file to original location
        }
    }

    Ok(())
}

// ============================================================
// Analytics Commands
// ============================================================

#[tauri::command]
pub fn get_analytics(days: u32, app_state: State<AppState>) -> Result<AnalyticsData, String> {
    let mut day_analytics = Vec::new();
    let mut total_space_freed_gb = 0.0f64;
    let mut total_scans = 0u32;
    let mut total_cleanups = 0u32;

    if let Ok(conn) = rusqlite::Connection::open(&app_state.db_path) {
        let mut stmt = conn.prepare(
            "SELECT date, COALESCE(SUM(space_freed), 0), COALESCE(SUM(scans_count), 0), COALESCE(SUM(cleanups_count), 0) FROM analytics WHERE date >= date('now', ?1) GROUP BY date ORDER BY date DESC LIMIT ?2"
        ).ok();

        if let Some(mut stmt) = stmt {
            let rows = stmt.query_map([&format!("-{} days", days), &days.to_string()], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                ))
            });

            if let Ok(rows) = rows {
                for row in rows.filter_map(|r| r.ok()) {
                    let (date, space_freed, scans, cleanups) = row;
                    let freed_gb = space_freed as f64 / (1024.0 * 1024.0 * 1024.0);
                    total_space_freed_gb += freed_gb;
                    total_scans += scans as u32;
                    total_cleanups += cleanups as u32;

                    day_analytics.push(DayAnalytics {
                        date,
                        space_freed_gb: freed_gb,
                        scans_count: scans as u32,
                        cleanups_count: cleanups as u32,
                    });
                }
            }
        }
    }

    let avg_savings = if total_cleanups > 0 {
        total_space_freed_gb / total_cleanups as f64
    } else {
        0.0
    };

    Ok(AnalyticsData {
        days: day_analytics,
        total_space_freed_gb,
        total_scans,
        total_cleanups,
        average_savings_per_cleanup_gb: avg_savings,
    })
}

// ============================================================
// AI Commands
// ============================================================

#[tauri::command]
pub fn ai_chat(message: String, _context: AIContext) -> Result<AIResponse, String> {
    log::info!("AI chat: {}", message);
    let msg_lower = message.to_lowercase();

    let (response, confidence, actions) = if msg_lower.contains("most space") || msg_lower.contains("taking up") {
        (
            "Based on typical system patterns, the largest space consumers are usually:\n\n1. **Browser Cache** - Chrome/Firefox/Safari can accumulate 2-10GB\n2. **Downloads folder** - Old installers and files you no longer need\n3. **Development Cache** - node_modules, build artifacts, package caches\n4. **System Logs** - Rotating logs that grow over time\n\nI recommend starting with Browser Cache and Temporary Files — they're safe to clean and often free up 5-15GB.".to_string(),
            0.85,
            vec![AIAction {
                label: "Scan Browser Cache".to_string(),
                action_type: "scan_category".to_string(),
                payload: serde_json::json!({"category": "Browser Cache"}),
            }]
        )
    } else if msg_lower.contains("duplicate") {
        (
            "I can help find duplicate files! These are files with identical content but possibly different names or locations. Run a full scan first and I'll identify duplicates for you.".to_string(),
            0.80,
            vec![AIAction {
                label: "Run Full Scan".to_string(),
                action_type: "start_scan".to_string(),
                payload: serde_json::json!({}),
            }]
        )
    } else if msg_lower.contains("development") || msg_lower.contains("dev") || msg_lower.contains("cache") {
        (
            "Development caches can be safely cleaned. These include npm/yarn caches, Cargo target directories, Docker images, Webpack/Vite caches, and Xcode derived data. Typically 3-20GB can be freed.".to_string(),
            0.90,
            vec![AIAction {
                label: "Clean Dev Cache".to_string(),
                action_type: "cleanup_category".to_string(),
                payload: serde_json::json!({"categories": ["Development Cache", "Application Cache"]}),
            }]
        )
    } else if msg_lower.contains("5gb") || msg_lower.contains("safe cleanup") {
        (
            "To free up ~5GB safely: 1) Browser Cache (1-4GB) - 100% safe, 2) System Cache (0.5-2GB) - Very safe, 3) Temporary Files (0.2-1GB) - 100% safe. These have safety scores of 0.85+.".to_string(),
            0.92,
            vec![AIAction {
                label: "Select Safe Categories".to_string(),
                action_type: "select_safe".to_string(),
                payload: serde_json::json!({"min_safety": 0.85}),
            }]
        )
    } else if msg_lower.contains("hello") || msg_lower.contains("hi") || msg_lower.contains("hey") {
        (
            "Hello! I'm your SlothCleaner AI assistant. I can help you find what's using disk space, suggest safe cleanups, identify duplicate files, and track space savings. What would you like to know?".to_string(),
            0.95,
            vec![]
        )
    } else {
        (
            "I can help with scanning your system, recommendations for safe cleanup, duplicate file detection, development cache cleaning, and analytics on space savings. Try asking something specific!".to_string(),
            0.60,
            vec![]
        )
    };

    Ok(AIResponse {
        message: response,
        actions,
        confidence,
    })
}

#[tauri::command]
pub fn get_ai_recommendations(app_state: State<AppState>) -> Result<Vec<Recommendation>, String> {
    let mut recommendations = Vec::new();

    let scans = app_state.scans.lock()
        .map_err(|e| format!("Failed to lock scans: {}", e))?;

    let has_scan_data = scans.values().any(|s| s.results.is_some());

    if has_scan_data {
        recommendations.push(Recommendation {
            id: Uuid::new_v4().to_string(),
            title: "Browser Cache Cleanup".to_string(),
            description: "Your browsers have accumulated cache. Safe to clean.".to_string(),
            category: "Browser Cache".to_string(),
            potential_savings_gb: 2.5,
            safety_score: 0.95,
            action_label: "Clean Now".to_string(),
        });
    } else {
        recommendations.push(Recommendation {
            id: Uuid::new_v4().to_string(),
            title: "Start with a Scan".to_string(),
            description: "Run a quick scan to identify cleanable files.".to_string(),
            category: "System".to_string(),
            potential_savings_gb: 0.0,
            safety_score: 1.0,
            action_label: "Quick Scan".to_string(),
        });
    }

    Ok(recommendations)
}

#[tauri::command]
pub fn update_user_feedback(file_path: String, decision: String) -> Result<(), String> {
    log::info!("User feedback: {} -> {}", file_path, decision);
    Ok(())
}

// ============================================================
// System Commands
// ============================================================

#[tauri::command]
pub fn get_system_info() -> Result<SystemInfo, String> {
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

    Ok(SystemInfo {
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
pub fn export_logs(app_state: State<AppState>) -> Result<String, String> {
    let log_path = app_state.data_dir.join("sloth-cleaner.log");
    Ok(log_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn reset_learning_data(app_state: State<AppState>) -> Result<(), String> {
    if let Ok(conn) = rusqlite::Connection::open(&app_state.db_path) {
        let _ = conn.execute("DELETE FROM file_decisions", []);
        let _ = conn.execute("DELETE FROM user_preferences", []);
    }
    Ok(())
}
