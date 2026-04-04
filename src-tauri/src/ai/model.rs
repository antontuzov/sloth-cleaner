use std::collections::HashMap;

/// Simple preference model for user learning (no external ML dependency)
pub struct PreferenceModel {
    category_weights: HashMap<String, f32>,
    file_age_preference: f32,
    size_threshold: u64,
}

impl PreferenceModel {
    pub fn new() -> Self {
        Self {
            category_weights: HashMap::new(),
            file_age_preference: 7.0,
            size_threshold: 1_000_000,
        }
    }
    
    /// Predict safety score for a file (0.0 to 1.0)
    pub fn predict_safety_score(
        &self,
        category: &str,
        file_age_days: u64,
        file_size: u64,
    ) -> f32 {
        let category_score = self.category_weights.get(category).copied().unwrap_or(0.5);
        let age_score = self.calculate_age_score(file_age_days);
        let size_score = self.calculate_size_score(file_size);
        
        (category_score * 0.5) + (age_score * 0.3) + (size_score * 0.2)
    }
    
    /// Calculate age-based score
    fn calculate_age_score(&self, age_days: u64) -> f32 {
        let age = age_days as f32;
        let pref = self.file_age_preference;
        
        if age >= pref * 2.0 {
            1.0 // Very safe
        } else if age >= pref {
            0.8
        } else if age >= pref * 0.5 {
            0.5
        } else {
            0.2 // Risky - very recent
        }
    }
    
    /// Calculate size-based score
    fn calculate_size_score(&self, size: u64) -> f32 {
        let size_mb = size as f64 / 1_000_000.0;
        let threshold_mb = self.size_threshold as f64 / 1_000_000.0;
        
        if size_mb > threshold_mb * 10.0 {
            0.4 // Large files - be cautious
        } else if size_mb > threshold_mb {
            0.6
        } else {
            1.0 // Small files are generally safe
        }
    }
    
    /// Update model with user feedback
    pub fn train(&mut self, category: &str, deleted: bool) {
        let weight = self.category_weights.entry(category.to_string()).or_insert(0.5);
        
        if deleted {
            // User deleted this category - it's probably safe
            *weight = (*weight * 0.9) + 0.1;
        } else {
            // User kept this category - it's probably important
            *weight = (*weight * 0.9) - 0.1;
        }
        
        // Clamp between 0.0 and 1.0
        *weight = weight.max(0.0).min(1.0);
    }
    
    /// Get category weights
    pub fn get_weights(&self) -> &HashMap<String, f32> {
        &self.category_weights
    }
    
    /// Reset model to defaults
    pub fn reset(&mut self) {
        self.category_weights.clear();
        self.file_age_preference = 7.0;
        self.size_threshold = 1_000_000;
    }
}

impl Default for PreferenceModel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_prediction() {
        let model = PreferenceModel::new();
        let score = model.predict_safety_score("Logs", 30, 1000);
        assert!(score > 0.5, "Old small log files should be safe to delete");
    }
    
    #[test]
    fn test_training_updates_weights() {
        let mut model = PreferenceModel::new();
        model.train("Logs", true);
        
        let weights = model.get_weights();
        assert!(weights.contains_key("Logs"));
        assert!(weights["Logs"] > 0.5);
    }
}
