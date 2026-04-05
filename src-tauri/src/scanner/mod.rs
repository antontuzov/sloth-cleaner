pub mod filesystem;
pub mod categorizer;
pub mod analyzer;

// Re-export key types
pub use crate::types::{ScanResults, CategoryResults, FileInfo};
pub use filesystem::{
    scan_directory, scan_directory_multi, scan_multi_with_options,
    scan_with_options, scan_directory_parallel,
    find_large_files, find_duplicates, file_md5,
    get_cache_dirs, format_file_size, get_file_info, file_age_days,
    DEFAULT_LARGE_FILE_THRESHOLD,
};
