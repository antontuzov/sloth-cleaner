use std::path::PathBuf;
use directories::ProjectDirs;

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub data_dir: PathBuf,
    pub snapshot_dir: PathBuf,
    pub log_dir: PathBuf,
    pub max_snapshot_size_mb: u64,
    pub enable_rollback: bool,
    pub dry_run_always: bool,
    pub min_file_age_days: u64,
}

impl AppConfig {
    pub fn new() -> Result<Self, String> {
        let proj_dirs = ProjectDirs::from("com", "slothcleaner", "SlothCleaner")
            .ok_or("Failed to get project directories".to_string())?;
        
        let data_dir = proj_dirs.data_dir().to_path_buf();
        let snapshot_dir = data_dir.join("snapshots");
        let log_dir = proj_dirs.data_local_dir().join("logs");
        
        Ok(Self {
            data_dir,
            snapshot_dir,
            log_dir,
            max_snapshot_size_mb: 500,
            enable_rollback: true,
            dry_run_always: false,
            min_file_age_days: 1,
        })
    }
    
    /// Ensure all directories exist
    pub fn ensure_dirs(&self) -> Result<(), String> {
        std::fs::create_dir_all(&self.data_dir)
            .map_err(|e| format!("Failed to create data dir: {}", e))?;
        
        std::fs::create_dir_all(&self.snapshot_dir)
            .map_err(|e| format!("Failed to create snapshot dir: {}", e))?;
        
        std::fs::create_dir_all(&self.log_dir)
            .map_err(|e| format!("Failed to create log dir: {}", e))?;
        
        Ok(())
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new().expect("Failed to create default config")
    }
}
