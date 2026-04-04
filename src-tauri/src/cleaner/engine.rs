use std::path::Path;
use trash;
use log;

/// Clean files safely using the trash system
pub struct CleanEngine {
    dry_run: bool,
    cleaned_files: Vec<String>,
    errors: Vec<String>,
    total_freed: u64,
}

impl CleanEngine {
    pub fn new(dry_run: bool) -> Self {
        Self {
            dry_run,
            cleaned_files: Vec::new(),
            errors: Vec::new(),
            total_freed: 0,
        }
    }
    
    /// Clean a single file
    pub fn clean_file(&mut self, path: &Path) -> Result<u64, String> {
        if self.dry_run {
            // In dry-run mode, just report what would be deleted
            let size = std::fs::metadata(path)
                .map_err(|e| format!("Failed to read metadata: {}", e))?
                .len();
            
            self.cleaned_files.push(path.to_string_lossy().to_string());
            self.total_freed += size;
            return Ok(size);
        }
        
        // Actually delete the file (move to trash)
        let size = std::fs::metadata(path)
            .map_err(|e| format!("Failed to read metadata: {}", e))?
            .len();
        
        trash::delete(path)
            .map_err(|e| format!("Failed to delete file: {}", e))?;
        
        self.cleaned_files.push(path.to_string_lossy().to_string());
        self.total_freed += size;
        
        Ok(size)
    }
    
    /// Get results
    pub fn get_results(self) -> (u64, Vec<String>, Vec<String>) {
        (self.total_freed, self.cleaned_files, self.errors)
    }
    
    /// Check if running in dry-run mode
    pub fn is_dry_run(&self) -> bool {
        self.dry_run
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dry_run_mode() {
        let engine = CleanEngine::new(true);
        assert!(engine.is_dry_run());
    }
    
    #[test]
    fn test_actual_run_mode() {
        let engine = CleanEngine::new(false);
        assert!(!engine.is_dry_run());
    }
}
