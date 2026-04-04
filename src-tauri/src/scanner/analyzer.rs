use std::collections::HashMap;
use super::categorizer::FileCategory;

/// Analyze scan results and generate statistics
pub struct Analyzer {
    category_stats: HashMap<FileCategory, CategoryStats>,
}

struct CategoryStats {
    total_size: u64,
    file_count: u64,
    average_size: f64,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            category_stats: HashMap::new(),
        }
    }
    
    /// Add a file to the analysis
    pub fn add_file(&mut self, category: FileCategory, size: u64) {
        let stats = self.category_stats.entry(category).or_insert(CategoryStats {
            total_size: 0,
            file_count: 0,
            average_size: 0.0,
        });
        
        stats.total_size += size;
        stats.file_count += 1;
        stats.average_size = stats.total_size as f64 / stats.file_count as f64;
    }
    
    /// Get total size across all categories
    pub fn total_size(&self) -> u64 {
        self.category_stats.values().map(|s| s.total_size).sum()
    }
    
    /// Get total file count
    pub fn total_files(&self) -> u64 {
        self.category_stats.values().map(|s| s.file_count).sum()
    }
    
    /// Get category breakdown
    pub fn category_breakdown(&self) -> Vec<(FileCategory, u64, u64)> {
        self.category_stats
            .iter()
            .map(|(cat, stats)| (cat.clone(), stats.total_size, stats.file_count))
            .collect()
    }
    
    /// Get top N categories by size
    pub fn top_categories(&self, n: usize) -> Vec<(FileCategory, u64)> {
        let mut categories: Vec<_> = self.category_stats
            .iter()
            .map(|(cat, stats)| (cat.clone(), stats.total_size))
            .collect();
        
        categories.sort_by(|a, b| b.1.cmp(&a.1));
        categories.into_iter().take(n).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_analyzer() {
        let mut analyzer = Analyzer::new();
        
        analyzer.add_file(FileCategory::Logs, 1000);
        analyzer.add_file(FileCategory::Logs, 2000);
        analyzer.add_file(FileCategory::Temporary, 500);
        
        assert_eq!(analyzer.total_size(), 3500);
        assert_eq!(analyzer.total_files(), 3);
        
        let top = analyzer.top_categories(1);
        assert_eq!(top[0].0, FileCategory::Logs);
        assert_eq!(top[0].1, 3000);
    }
}
