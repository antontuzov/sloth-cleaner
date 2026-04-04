use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Safety levels for file deletion
#[derive(Debug, Clone, PartialEq)]
pub enum SafetyLevel {
    Safe,
    Warning(WarningReason),
    Blocked(BlockReason),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WarningReason {
    RecentlyModified,
    LowAIScore,
    UnknownFileType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockReason {
    SystemProtected,
    FileInUse,
    UserProtected,
    TooRecent,
}

pub type SafetyResult = Result<(), SafetyLevel>;

/// Safety engine to protect important files
pub struct SafetyEngine {
    system_paths: HashSet<PathBuf>,
    protected_extensions: HashSet<String>,
    active_processes: HashSet<String>,
    user_protected_paths: HashSet<PathBuf>,
    min_file_age_days: u64,
}

impl SafetyEngine {
    pub fn new() -> Self {
        let mut system = Self {
            system_paths: HashSet::new(),
            protected_extensions: HashSet::new(),
            active_processes: HashSet::new(),
            user_protected_paths: HashSet::new(),
            min_file_age_days: 1,
        };
        
        // Initialize with default protected paths
        system.init_system_paths();
        system.init_protected_extensions();
        
        system
    }
    
    /// Initialize OS-specific system paths
    fn init_system_paths(&mut self) {
        #[cfg(target_os = "macos")]
        {
            let protected = vec![
                "/System",
                "/Library",
                "/usr",
                "/bin",
                "/sbin",
                "/var",
                "/etc",
            ];
            for path in protected {
                self.system_paths.insert(PathBuf::from(path));
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            let protected = vec![
                "C:\\Windows",
                "C:\\Program Files",
                "C:\\Program Files (x86)",
                "C:\\ProgramData",
            ];
            for path in protected {
                self.system_paths.insert(PathBuf::from(path));
            }
        }
    }
    
    /// Initialize protected file extensions
    fn init_protected_extensions(&mut self) {
        let extensions = vec![
            "exe", "dll", "sys", "drv",
            "app", "framework", "kext",
            "ini", "cfg", "conf",
        ];
        
        for ext in extensions {
            self.protected_extensions.insert(ext.to_string());
        }
    }
    
    /// Check if a file is safe to delete
    pub fn is_safe_to_delete(&self, path: &Path) -> SafetyLevel {
        // Layer 1: System protection
        if self.is_system_path(path) {
            return SafetyLevel::Blocked(BlockReason::SystemProtected);
        }
        
        // Layer 2: Protected extensions
        if self.has_protected_extension(path) {
            return SafetyLevel::Blocked(BlockReason::SystemProtected);
        }
        
        // Layer 3: Active process check (simplified)
        if self.is_file_in_use(path) {
            return SafetyLevel::Blocked(BlockReason::FileInUse);
        }
        
        // Layer 4: User protected paths
        if self.is_user_protected(path) {
            return SafetyLevel::Blocked(BlockReason::UserProtected);
        }
        
        // Layer 5: Age check
        if self.is_too_recent(path) {
            return SafetyLevel::Warning(WarningReason::RecentlyModified);
        }
        
        SafetyLevel::Safe
    }
    
    /// Check if path is in system directories
    fn is_system_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.system_paths.iter().any(|sp| path_str.starts_with(sp.to_string_lossy().as_ref()))
    }
    
    /// Check if file has protected extension
    fn has_protected_extension(&self, path: &Path) -> bool {
        path.extension()
            .map(|ext| self.protected_extensions.contains(&ext.to_string_lossy().to_lowercase()))
            .unwrap_or(false)
    }
    
    /// Check if file is currently in use (simplified)
    fn is_file_in_use(&self, path: &Path) -> bool {
        // Try to open file exclusively - if it fails, might be in use
        std::fs::OpenOptions::new()
            .write(true)
            .open(path)
            .map(|_| false)
            .unwrap_or(true)
    }
    
    /// Check if path is user-protected
    fn is_user_protected(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.user_protected_paths.iter().any(|pp| path_str.starts_with(pp.to_string_lossy().as_ref()))
    }
    
    /// Check if file is too recent
    fn is_too_recent(&self, path: &Path) -> bool {
        if let Ok(metadata) = std::fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.elapsed() {
                    let days = duration.as_secs() / 86400;
                    return days < self.min_file_age_days;
                }
            }
        }
        false
    }
    
    /// Add user-protected path
    pub fn add_user_protected_path(&mut self, path: PathBuf) {
        self.user_protected_paths.insert(path);
    }
    
    /// Remove user-protected path
    pub fn remove_user_protected_path(&mut self, path: &Path) {
        self.user_protected_paths.remove(path);
    }
    
    /// Set minimum file age
    pub fn set_min_file_age_days(&mut self, days: u64) {
        self.min_file_age_days = days;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_path_protection() {
        let engine = SafetyEngine::new();
        
        #[cfg(target_os = "macos")]
        {
            let result = engine.is_safe_to_delete(Path::new("/System/Library/test.txt"));
            assert!(matches!(result, SafetyLevel::Blocked(BlockReason::SystemProtected)));
        }
    }
    
    #[test]
    fn test_protected_extension() {
        let engine = SafetyEngine::new();
        let result = engine.is_safe_to_delete(Path::new("/tmp/test.exe"));
        assert!(matches!(result, SafetyLevel::Blocked(BlockReason::SystemProtected)));
    }
}
