pub mod filesystem;
pub mod categorizer;
pub mod analyzer;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResults {
    pub scan_id: String,
    pub timestamp: String,
    pub total_size: u64,
    pub file_count: u64,
    pub duration_ms: u64,
    pub categories: Vec<CategoryResults>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryResults {
    pub category: String,
    pub size: u64,
    pub file_count: u64,
    pub files: Vec<crate::commands::FileInfo>,
}
