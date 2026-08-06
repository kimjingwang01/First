use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DataRecord {
    pub id: u64,
    pub category: String,
    pub value: f64,
    pub tags: Vec<String>,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct ProcessedRecord {
    pub id: u64,
    pub category: String,
    pub raw_value: f64,
    pub normalized_value: f64,
    pub score: f64,
    pub risk_level: RiskLevel,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct AnalysisReport {
    pub mode: String,
    pub total_records: usize,
    pub avg_score: f64,
    pub min_score: f64,
    pub max_score: f64,
    pub risk_counts: HashMap<String, usize>,
    pub category_counts: HashMap<String, usize>,
    pub top_records: Vec<ProcessedRecord>,
    pub notes: Vec<String>,
}

impl AnalysisReport {
    pub fn render_text(&self) -> String {
        let mut s = String::new();
        s.push_str("=== Analysis Report ===\n");
        s.push_str(&format!("mode: {}\n", self.mode));
        s.push_str(&format!("total_records: {}\n", self.total_records));
        s.push_str(&format!("avg_score: {:.4}\n", self.avg_score));
        s.push_str(&format!("min_score: {:.4}\n", self.min_score));
        s.push_str(&format!("max_score: {:.4}\n", self.max_score));
        s.push_str("risk_counts:\n");
        for (k, v) in &self.risk_counts {
            s.push_str(&format!("  - {}: {}\n", k, v));
        }
        s.push_str("category_counts:\n");
        for (k, v) in &self.category_counts {
            s.push_str(&format!("  - {}: {}\n", k, v));
        }
        s.push_str("top_records:\n");
        for r in &self.top_records {
            s.push_str(&format!(
                "  - id={} cat={} score={:.3} risk={:?}\n",
                r.id, r.category, r.score, r.risk_level
            ));
        }
        if !self.notes.is_empty() {
            s.push_str("notes:\n");
            for n in &self.notes {
                s.push_str(&format!("  - {}\n", n));
            }
        }
        s
    }
}
