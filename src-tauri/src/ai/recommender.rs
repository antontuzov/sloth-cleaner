use std::collections::HashMap;

/// Association rule for recommendations
#[derive(Debug, Clone)]
pub struct AssociationRule {
    pub antecedent: Vec<String>,
    pub consequent: Vec<String>,
    pub confidence: f32,
}

/// Recommendation engine using association rule mining
pub struct RecommendationEngine {
    rules: Vec<AssociationRule>,
    frequency_map: HashMap<String, u32>,
}

impl RecommendationEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            rules: Vec::new(),
            frequency_map: HashMap::new(),
        };
        
        // Initialize with some default rules
        engine.init_default_rules();
        
        engine
    }
    
    /// Initialize default association rules
    fn init_default_rules(&mut self) {
        // Users who cleaned browser cache often clean downloads
        self.rules.push(AssociationRule {
            antecedent: vec!["Browser Cache".to_string()],
            consequent: vec!["Downloads".to_string()],
            confidence: 0.65,
        });
        
        // Users who cleaned logs often clean temp files
        self.rules.push(AssociationRule {
            antecedent: vec!["Logs".to_string()],
            consequent: vec!["Temporary Files".to_string()],
            confidence: 0.75,
        });
        
        // Users who cleaned dev cache often clean node_modules
        self.rules.push(AssociationRule {
            antecedent: vec!["Development Cache".to_string()],
            consequent: vec!["Application Cache".to_string()],
            confidence: 0.80,
        });
    }
    
    /// Suggest categories to clean based on what was already cleaned
    pub fn suggest(&self, cleaned_categories: &[String]) -> Vec<String> {
        let mut suggestions: HashMap<String, f32> = HashMap::new();
        
        for rule in &self.rules {
            // Check if all antecedent categories are in cleaned_categories
            if rule.antecedent.iter().all(|c| cleaned_categories.contains(c)) {
                for consequent in &rule.consequent {
                    let score = suggestions.entry(consequent.clone()).or_insert(0.0);
                    *score += rule.confidence;
                }
            }
        }
        
        // Sort by score and return
        let mut sorted: Vec<_> = suggestions.into_iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        sorted.into_iter().map(|(cat, _)| cat).collect()
    }
    
    /// Record a category cleanup to update frequency
    pub fn record_cleanup(&mut self, category: &str) {
        *self.frequency_map.entry(category.to_string()).or_insert(0) += 1;
    }
    
    /// Get most frequently cleaned categories
    pub fn top_categories(&self, n: usize) -> Vec<(String, u32)> {
        let mut categories: Vec<_> = self.frequency_map.iter()
            .map(|(cat, count)| (cat.clone(), *count))
            .collect();
        
        categories.sort_by(|a, b| b.1.cmp(&a.1));
        categories.into_iter().take(n).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_suggestions() {
        let engine = RecommendationEngine::new();
        let suggestions = engine.suggest(&vec!["Browser Cache".to_string()]);
        
        // Should suggest Downloads based on default rule
        assert!(suggestions.iter().any(|s| s == "Downloads"));
    }
    
    #[test]
    fn test_no_suggestions_for_empty() {
        let engine = RecommendationEngine::new();
        let suggestions = engine.suggest(&vec![]);
        
        // No antecedent matched, so no suggestions
        assert!(suggestions.is_empty());
    }
}
