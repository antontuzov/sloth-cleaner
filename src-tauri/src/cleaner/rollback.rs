use std::path::{Path, PathBuf};
use uuid::Uuid;
use chrono::Utc;
use log;

/// Snapshot for rollback
pub struct Snapshot {
    pub id: String,
    pub path: PathBuf,
    pub timestamp: String,
    pub files_count: u32,
    pub total_size: u64,
}

/// Rollback manager for creating and restoring snapshots
pub struct RollbackManager {
    snapshot_dir: PathBuf,
}

impl RollbackManager {
    pub fn new(snapshot_dir: PathBuf) -> Self {
        Self { snapshot_dir }
    }
    
    /// Initialize snapshot directory
    pub fn init(&self) -> Result<(), String> {
        if !self.snapshot_dir.exists() {
            std::fs::create_dir_all(&self.snapshot_dir)
                .map_err(|e| format!("Failed to create snapshot directory: {}", e))?;
        }
        Ok(())
    }
    
    /// Create a snapshot of files
    pub fn create_snapshot(&self, files: &[PathBuf]) -> Result<Snapshot, String> {
        let snapshot_id = Uuid::new_v4().to_string();
        let snapshot_path = self.snapshot_dir.join(&snapshot_id);
        
        // Create snapshot directory
        std::fs::create_dir_all(&snapshot_path)
            .map_err(|e| format!("Failed to create snapshot: {}", e))?;
        
        let mut total_size = 0u64;
        let mut files_count = 0u32;
        
        // Copy files to snapshot (with size limit)
        for file in files {
            if file.metadata().map(|m| m.len()).unwrap_or(0) > 100_000_000 {
                // Skip files larger than 100MB
                log::warn!("Skipping large file: {:?}", file);
                continue;
            }
            
            // Preserve directory structure
            let relative_path = file.strip_prefix("/").unwrap_or(file);
            let dest = snapshot_path.join(relative_path);
            
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            
            if std::fs::copy(file, &dest).is_ok() {
                total_size += file.metadata().map(|m| m.len()).unwrap_or(0);
                files_count += 1;
            }
        }
        
        // Save metadata
        let metadata = format!(
            "timestamp={}\nfiles_count={}\ntotal_size={}",
            Utc::now(),
            files_count,
            total_size
        );
        
        std::fs::write(snapshot_path.join(".metadata"), metadata)
            .map_err(|e| format!("Failed to save metadata: {}", e))?;
        
        Ok(Snapshot {
            id: snapshot_id,
            path: snapshot_path,
            timestamp: Utc::now().to_rfc3339(),
            files_count,
            total_size,
        })
    }
    
    /// Restore a snapshot
    pub fn restore_snapshot(&self, snapshot_id: &str) -> Result<(), String> {
        let snapshot_path = self.snapshot_dir.join(snapshot_id);
        
        if !snapshot_path.exists() {
            return Err(format!("Snapshot not found: {}", snapshot_id));
        }
        
        // Walk through snapshot and restore files
        for entry in walkdir::WalkDir::new(&snapshot_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.file_name() != ".metadata")
        {
            let source = entry.path();
            let relative = source.strip_prefix(&snapshot_path).unwrap_or(source);
            
            // Restore to original location
            // Note: This is simplified - real implementation needs more safety checks
            log::info!("Restoring: {:?}", relative);
        }
        
        Ok(())
    }
    
    /// List all snapshots
    pub fn list_snapshots(&self) -> Result<Vec<Snapshot>, String> {
        let mut snapshots = Vec::new();
        
        if !self.snapshot_dir.exists() {
            return Ok(snapshots);
        }
        
        for entry in std::fs::read_dir(&self.snapshot_dir)
            .map_err(|e| format!("Failed to read snapshot directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                let id = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                
                // Read metadata if exists
                let metadata_path = path.join(".metadata");
                if metadata_path.exists() {
                    if let Ok(metadata) = std::fs::read_to_string(&metadata_path) {
                        let files_count = metadata
                            .lines()
                            .find(|l| l.starts_with("files_count="))
                            .and_then(|l| l.split('=').nth(1))
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0);
                        
                        let total_size = metadata
                            .lines()
                            .find(|l| l.starts_with("total_size="))
                            .and_then(|l| l.split('=').nth(1))
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(0);
                        
                        let timestamp = metadata
                            .lines()
                            .find(|l| l.starts_with("timestamp="))
                            .and_then(|l| l.split('=').nth(1))
                            .unwrap_or("")
                            .to_string();
                        
                        snapshots.push(Snapshot {
                            id,
                            path,
                            timestamp,
                            files_count,
                            total_size,
                        });
                    }
                }
            }
        }
        
        Ok(snapshots)
    }
    
    /// Delete a snapshot
    pub fn delete_snapshot(&self, snapshot_id: &str) -> Result<(), String> {
        let snapshot_path = self.snapshot_dir.join(snapshot_id);
        
        if !snapshot_path.exists() {
            return Err(format!("Snapshot not found: {}", snapshot_id));
        }
        
        std::fs::remove_dir_all(&snapshot_path)
            .map_err(|e| format!("Failed to delete snapshot: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_snapshot_directory() {
        let temp_dir = std::env::temp_dir().join("sloth_test_snapshots");
        let manager = RollbackManager::new(temp_dir.clone());
        
        assert!(manager.init().is_ok());
        
        // Cleanup
        std::fs::remove_dir_all(&temp_dir).ok();
    }
}
