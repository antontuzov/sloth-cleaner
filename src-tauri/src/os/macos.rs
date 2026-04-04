/// macOS-specific functionality
use std::path::Path;

/// Create APFS snapshot for rollback (macOS only)
pub fn create_apfs_snapshot(path: &Path) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        // Use tmutil for Time Machine snapshots if available
        let output = std::process::Command::new("tmutil")
            .arg("snapshot")
            .output()
            .map_err(|e| format!("Failed to run tmutil: {}", e))?;
        
        if output.status.success() {
            log::info!("APFS snapshot created");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("tmutil failed: {}", stderr))
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        let _ = path;
        Err("APFS snapshots only available on macOS".to_string())
    }
}

/// Get macOS system cache paths
pub fn get_system_cache_paths() -> Vec<String> {
    vec![
        "~/Library/Caches".to_string(),
        "/Library/Caches".to_string(),
        "/System/Library/Caches".to_string(),
    ]
}

/// Get common application cache paths on macOS
pub fn get_app_cache_paths() -> Vec<String> {
    vec![
        "~/Library/Application Support".to_string(),
        "~/Library/Preferences".to_string(),
        "~/Library/Logs".to_string(),
        "~/Library/Saved Application State".to_string(),
    ]
}

/// Clear DNS cache (macOS)
pub fn flush_dns_cache() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("sudo")
            .arg("killall")
            .arg("-HUP")
            .arg("mDNSResponder")
            .output()
            .map_err(|e| format!("Failed to flush DNS cache: {}", e))?;
        
        if output.status.success() {
            Ok(())
        } else {
            Err("Failed to flush DNS cache".to_string())
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        Err("DNS cache flush only available on macOS".to_string())
    }
}
