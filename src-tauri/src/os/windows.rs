/// Windows-specific functionality
use std::path::Path;

/// Create VSS (Volume Shadow Copy) snapshot for rollback (Windows only)
pub fn create_vss_snapshot(path: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // Use vssadmin or PowerShell for VSS snapshots
        let output = std::process::Command::new("powershell")
            .arg("-Command")
            .arg("Checkpoint-Computer -Description 'SlothCleaner' -RestorePointType MODIFY_SETTINGS")
            .output()
            .map_err(|e| format!("Failed to create VSS snapshot: {}", e))?;
        
        if output.status.success() {
            log::info!("VSS snapshot created");
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("VSS snapshot failed: {}", stderr))
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        let _ = path;
        Err("VSS snapshots only available on Windows".to_string())
    }
}

/// Get Windows system paths
pub fn get_system_paths() -> Vec<String> {
    vec![
        "C:\\Windows".to_string(),
        "C:\\Program Files".to_string(),
        "C:\\Program Files (x86)".to_string(),
        "C:\\ProgramData".to_string(),
    ]
}

/// Get Windows temp folder paths
pub fn get_temp_paths() -> Vec<String> {
    vec![
        "C:\\Windows\\Temp".to_string(),
        "%TEMP%".to_string(),
        "%TMP%".to_string(),
    ]
}

/// Clear Windows DNS cache
pub fn flush_dns_cache() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let output = std::process::Command::new("ipconfig")
            .arg("/flushdns")
            .output()
            .map_err(|e| format!("Failed to flush DNS cache: {}", e))?;
        
        if output.status.success() {
            Ok(())
        } else {
            Err("Failed to flush DNS cache".to_string())
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Err("DNS cache flush only available on Windows".to_string())
    }
}
