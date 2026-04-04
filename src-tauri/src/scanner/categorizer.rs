use std::collections::HashMap;
use std::path::Path;

/// File categories for organizing scan results
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileCategory {
    SystemCache,
    ApplicationCache,
    Logs,
    Downloads,
    Trash,
    LargeFiles,
    Duplicates,
    Temporary,
    BrowserCache,
    DevelopmentCache,
    Thumbnails,
    Other,
}

impl FileCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SystemCache => "System Cache",
            Self::ApplicationCache => "Application Cache",
            Self::Logs => "Logs",
            Self::Downloads => "Downloads",
            Self::Trash => "Trash",
            Self::LargeFiles => "Large Files",
            Self::Duplicates => "Duplicates",
            Self::Temporary => "Temporary Files",
            Self::BrowserCache => "Browser Cache",
            Self::DevelopmentCache => "Development Cache",
            Self::Thumbnails => "Thumbnails",
            Self::Other => "Other",
        }
    }
}

/// Categorize a file based on its path and properties
pub fn categorize_file(path: &Path, size: u64) -> FileCategory {
    let path_str = path.to_string_lossy().to_lowercase();
    
    // Check for browser cache
    if is_browser_cache(path) {
        return FileCategory::BrowserCache;
    }
    
    // Check for development cache
    if is_development_cache(path) {
        return FileCategory::DevelopmentCache;
    }
    
    // Check for logs
    if is_log_file(path) {
        return FileCategory::Logs;
    }
    
    // Check for temporary files
    if is_temp_file(path) {
        return FileCategory::Temporary;
    }
    
    // Check for thumbnails
    if is_thumbnail(path) {
        return FileCategory::Thumbnails;
    }
    
    // Check for downloads
    if is_in_downloads(path) {
        return FileCategory::Downloads;
    }
    
    // Check for trash
    if is_in_trash(path) {
        return FileCategory::Trash;
    }
    
    // Large files (over 100MB)
    if size > 100_000_000 {
        return FileCategory::LargeFiles;
    }
    
    FileCategory::Other
}

/// Check if file is in browser cache directories
fn is_browser_cache(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    path_str.contains("/chrome/") || 
    path_str.contains("/firefox/") ||
    path_str.contains("/safari/") ||
    path_str.contains("/webkit/") ||
    path_str.contains("/browser/") && path_str.contains("/cache/")
}

/// Check if file is in development cache
fn is_development_cache(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    path_str.contains("/node_modules/.cache/") ||
    path_str.contains("/.next/cache/") ||
    path_str.contains("/.nuxt/cache/") ||
    path_str.contains("/.cache/") && path_str.contains("/webpack/") ||
    path_str.contains("/target/debug/.fingerprint/") ||
    path_str.contains("/.cargo/registry/cache/")
}

/// Check if file is a log file
fn is_log_file(path: &Path) -> bool {
    path.extension()
        .map(|ext| ext == "log")
        .unwrap_or(false) ||
    path.file_name()
        .map(|name| name.to_string_lossy().to_lowercase().ends_with(".log"))
        .unwrap_or(false)
}

/// Check if file is a temporary file
fn is_temp_file(path: &Path) -> bool {
    path.extension()
        .map(|ext| {
            matches!(
                ext.to_string_lossy().to_lowercase().as_str(),
                "tmp" | "temp" | "swp" | "bak" | "cache"
            )
        })
        .unwrap_or(false)
}

/// Check if file is a thumbnail
fn is_thumbnail(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    path_str.contains("/thumbnails/") ||
    path_str.contains("/.thumbnails/") ||
    path_str.contains("/.cache/thumbnails/")
}

/// Check if file is in downloads folder
fn is_in_downloads(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    path_str.contains("/downloads/") ||
    path_str.contains("/download/")
}

/// Check if file is in trash/recycle bin
fn is_in_trash(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    
    #[cfg(target_os = "macos")]
    {
        return path_str.contains("/.trash/") || path_str.contains("/trash/");
    }
    
    #[cfg(target_os = "windows")]
    {
        return path_str.contains("$recycle.bin") || path_str.contains("/recycler/");
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        return path_str.contains("/trash/") || path_str.contains("/.local/share/trash/");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_categorize_log_file() {
        let path = Path::new("/var/log/system.log");
        assert_eq!(categorize_file(path, 1000), FileCategory::Logs);
    }
    
    #[test]
    fn test_categorize_temp_file() {
        let path = Path::new("/tmp/file123.tmp");
        assert_eq!(categorize_file(path, 1000), FileCategory::Temporary);
    }
    
    #[test]
    fn test_categorize_large_file() {
        let path = Path::new("/Users/me/video.mp4");
        assert_eq!(categorize_file(path, 200_000_000), FileCategory::LargeFiles);
    }
}
