pub mod model;
pub mod trainer;
pub mod recommender;

pub use model::PreferenceModel;
pub use trainer::Trainer;
pub use recommender::RecommendationEngine;

/// Initialize the AI model (lazy loading)
pub fn init_model(_app_handle: &tauri::AppHandle) -> Result<(), String> {
    log::info!("AI model initialization ready (lazy load)");
    Ok(())
}
