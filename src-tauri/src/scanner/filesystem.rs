use std::path::{Path, PathBuf};
use std::sync::Mutex;
use walkdir::WalkDir;
use rayon::prelude::*;

// ============================================================
// Exclude patterns
// ============================================================

/// Directories to always skip
fn should_skip_dir(name: &str) -> bool {
    matches!(
        name,
        ".Trash" | ".Spotlight-V100" | ".fseventsd" | ".DocumentRevisions-V100"
            | ".TemporaryItems" | ".vol" | "Network" | "Volumes" | "dev" | "proc" | "sys"
    )
}

/// File patterns to exclude
fn should_exclude_file(name: &str) -> bool {
    matches!(
        name,
        ".DS_Store" | ".localized" | "Icon\r" | ".AppleDouble" | ".LSOverride"
    )
}

/// Check if path matches any exclude pattern
fn matches_exclude_patterns(path: &Path, patterns: &[String]) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();
    patterns.iter().any(|p| path_str.contains(&p.to_lowercase()))
}

// ============================================================
// Incremental scanner (single-threaded callback)
// ============================================================

/// Scan a single directory incrementally with a callback
pub fn scan_directory<F>(path: &Path, max_depth: Option<usize>, on_file: F) -> usize
where
    F: FnMut(&Path),
{
    scan_with_options(path, max_depth, None, on_file)
}

/// Scan with configurable exclude patterns
pub fn scan_with_options<F>(
    path: &Path,
    max_depth: Option<usize>,
    extra_excludes: Option<&[String]>,
    mut on_file: F,
) -> usize
where
    F: FnMut(&Path),
{
    let mut builder = WalkDir::new(path)
        .follow_links(false)
        .min_depth(1);

    if let Some(depth) = max_depth {
        builder = builder.max_depth(depth);
    }

    let excludes = extra_excludes.unwrap_or(&[]);
    let mut count = 0usize;

    for entry in builder
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_str().unwrap_or("");
            !should_skip_dir(name) && !should_exclude_file(name) && !matches_exclude_patterns(e.path(), excludes)
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        on_file(entry.path());
        count += 1;
    }

    count
}

/// Scan multiple directories incrementally
pub fn scan_directory_multi<F>(
    paths: &[PathBuf],
    max_depth: Option<usize>,
    extra_excludes: Option<&[String]>,
    on_file: F,
) -> Result<usize, String>
where
    F: FnMut(&Path),
{
    scan_multi_with_options(paths, max_depth, extra_excludes, on_file)
}

pub fn scan_multi_with_options<F>(
    paths: &[PathBuf],
    max_depth: Option<usize>,
    extra_excludes: Option<&[String]>,
    mut on_file: F,
) -> Result<usize, String>
where
    F: FnMut(&Path),
{
    let mut total_count = 0;

    for path in paths {
        eprintln!("[FS] Scanning: {:?}", path);
        let count = scan_with_options(path, max_depth, extra_excludes, |p| {
            on_file(p);
        });
        eprintln!("[FS] Found {} files in {:?}", count, path);
        total_count += count;
    }

    Ok(total_count)
}

// ============================================================
// Parallel batch scanner (for duplicate detection)
// ============================================================

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
}

/// Scan and collect all file entries using rayon for parallelism
pub fn scan_directory_parallel(paths: &[PathBuf], max_depth: Option<usize>, extra_excludes: Option<&[String]>) -> Vec<FileEntry> {
    let excludes = extra_excludes.unwrap_or(&[]).to_vec();
    
    // First pass: collect paths per directory
    let mut dir_file_lists: Vec<Vec<PathBuf>> = Vec::new();
    
    for path in paths {
        let mut files = Vec::new();
        scan_with_options(path, max_depth, Some(excludes.as_slice()), |p| {
            files.push(p.to_path_buf());
        });
        dir_file_lists.push(files);
    }

    // Parallel metadata collection
    dir_file_lists.into_par_iter()
        .flat_map(|files| {
            files.into_par_iter()
                .filter_map(|path| {
                    std::fs::metadata(&path).ok().map(|m| FileEntry {
                        size: m.len(),
                        path,
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

// ============================================================
// Large file detection
// ============================================================

/// Default threshold for "large files" (100MB)
pub const DEFAULT_LARGE_FILE_THRESHOLD: u64 = 100_000_000;

/// Find files exceeding the size threshold
pub fn find_large_files(paths: &[PathBuf], threshold: u64) -> Vec<(PathBuf, u64)> {
    let threshold = if threshold > 0 { threshold } else { DEFAULT_LARGE_FILE_THRESHOLD };
    let entries = scan_directory_parallel(paths, Some(15), None);
    
    entries.into_iter()
        .filter(|e| e.size >= threshold)
        .map(|e| (e.path, e.size))
        .collect()
}

// ============================================================
// Duplicate file detection
// ============================================================

use std::collections::HashMap;
use std::io::Read;
use md5::{Md5, Digest};

/// Compute MD5 hash of a file (fast, good enough for dedup)
pub fn file_md5(path: &Path) -> Option<String> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut hasher = Md5::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer).ok()?;
        if bytes_read == 0 { break; }
        Digest::update(&mut hasher, &buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Some(hex::encode(result))
}

/// Find duplicate files by first grouping by size, then hashing
pub fn find_duplicates(paths: &[PathBuf], max_depth: Option<usize>) -> Vec<Vec<(PathBuf, u64)>> {
    let entries = scan_directory_parallel(paths, max_depth, None);
    
    // Group by size first (fast filter)
    let mut size_groups: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for entry in entries {
        if entry.size > 0 { // Skip empty files
            size_groups.entry(entry.size).or_default().push(entry.path);
        }
    }
    
    // Only hash files that share a size with other files
    let mut duplicates = Vec::new();
    for (_size, paths) in size_groups.into_iter().filter(|(_, p)| p.len() > 1) {
        let mut hash_groups: HashMap<String, Vec<(PathBuf, u64)>> = HashMap::new();
        
        for path in paths {
            if let Some(size) = std::fs::metadata(&path).ok().map(|m| m.len()) {
                if let Some(hash) = file_md5(&path) {
                    hash_groups.entry(hash).or_default().push((path, size));
                }
            }
        }
        
        for (_hash, group) in hash_groups.into_iter().filter(|(_, g)| g.len() > 1) {
            duplicates.push(group);
        }
    }
    
    duplicates
}

// ============================================================
// Helpers
// ============================================================

pub fn get_file_info(path: &Path) -> Option<std::fs::Metadata> {
    std::fs::metadata(path).ok()
}

pub fn file_age_days(path: &Path) -> Option<u64> {
    let metadata = get_file_info(path)?;
    let modified = metadata.modified().ok()?;
    let duration = modified.elapsed().ok()?;
    Some(duration.as_secs() / 86400)
}

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

/// Get specific cache directories to scan
pub fn get_cache_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(home) = dirs::home_dir() {
        #[cfg(target_os = "macos")]
        {
            dirs.push(home.join("Library/Caches"));
            dirs.push(home.join("Library/Logs"));
            dirs.push(home.join("Library/Application Support"));
            dirs.push(home.join("Library/Containers"));
            dirs.push(home.join("Library/Developer/Xcode/DerivedData"));
            dirs.push(home.join("Library/Saved Application State"));
            dirs.push(home.join("Downloads"));
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(local) = dirs::cache_dir() {
                dirs.push(local);
            }
            if let Some(local) = dirs::data_local_dir() {
                dirs.push(local.join("Temp"));
            }
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            dirs.push(home.join(".cache"));
            dirs.push(home.join(".local/share/Trash"));
        }
    }

    dirs.into_iter().filter(|d| d.exists()).collect()
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
    fn test_should_skip_dir() {
        assert!(should_skip_dir(".Trash"));
        assert!(should_skip_dir("dev"));
        assert!(!should_skip_dir("Library"));
        assert!(!should_skip_dir("Documents"));
    }

    #[test]
    fn test_should_exclude_file() {
        assert!(should_exclude_file(".DS_Store"));
        assert!(!should_exclude_file("document.pdf"));
    }
}
