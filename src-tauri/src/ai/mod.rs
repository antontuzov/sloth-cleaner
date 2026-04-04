pub mod model;
pub mod trainer;
pub mod recommender;

use serde::{Deserialize, Serialize};

/// Initialize the AI model (lazy loading)
pub fn init_model(_app_handle: &tauri::AppHandle) -> Result<(), String> {
    // Model is loaded lazily when first needed
    log::info!("AI model initialization requested (lazy load)");
    Ok(())
}
