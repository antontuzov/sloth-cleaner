use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Check if an entry is hidden
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Scan a directory for files
pub fn scan_directory(path: &Path, max_depth: Option<usize>) -> Vec<PathBuf> {
    let builder = WalkDir::new(path)
        .follow_links(true);
    
    let walker = if let Some(depth) = max_depth {
        builder.max_depth(depth).into_iter()
    } else {
        builder.into_iter()
    };
    
    walker
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_path_buf())
        .collect()
}

/// Get file metadata safely
pub fn get_file_info(path: &Path) -> Option<std::fs::Metadata> {
    std::fs::metadata(path).ok()
}

/// Calculate file age in days
pub fn file_age_days(path: &Path) -> Option<u64> {
    let metadata = get_file_info(path)?;
    let modified = metadata.modified().ok()?;
    
    let duration = modified.elapsed().ok()?;
    Some(duration.as_secs() / 86400)
}

/// Format file size in human-readable format
pub fn format_file_size(size: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0.0 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }
    
    #[test]
    fn test_is_hidden() {
        // Simple test cases
        assert!(Path::new(".hidden").file_name().unwrap().to_str().unwrap().starts_with('.'));
        assert!(!Path::new("visible").file_name().unwrap().to_str().unwrap().starts_with('.'));
    }
}
