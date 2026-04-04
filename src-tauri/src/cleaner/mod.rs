pub mod engine;
pub mod safety;
pub mod rollback;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanupResult {
    pub proposal_id: String,
    pub success: bool,
    pub freed_space: u64,
    pub files_deleted: u64,
    pub errors: Vec<String>,
    pub snapshot_id: Option<String>,
}
