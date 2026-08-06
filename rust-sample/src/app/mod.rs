pub mod config;

use crate::domain::engine::{analyze_records, process_records};
use crate::domain::models::{AnalysisReport, DataRecord};
use crate::io::repository::InMemoryRepository;

pub fn run(config: config::AppConfig) -> Result<AnalysisReport, String> {
    config.validate()?;

    let mut repo = InMemoryRepository::new();
    let records = generate_records(config.items, config.seed);
    repo.store(records.clone())?;

    let processed = process_records(&records, &config.mode)?;
    let report = analyze_records(&processed, &config.mode);
    Ok(report)
}

fn generate_records(count: usize, seed: u64) -> Vec<DataRecord> {
    let mut out = Vec::with_capacity(count);
    let mut x = seed.max(1);
    for idx in 0..count {
        x = lcg_next(x);
        let value = ((x % 10_000) as f64) / 37.0;
        let category = match idx % 4 {
            0 => "alpha",
            1 => "beta",
            2 => "gamma",
            _ => "delta",
        };

        let mut tags = Vec::new();
        if value > 100.0 {
            tags.push("high".to_string());
        }
        if value < 30.0 {
            tags.push("low".to_string());
        }
        if idx % 3 == 0 {
            tags.push("periodic".to_string());
        }

        out.push(DataRecord {
            id: idx as u64 + 1,
            category: category.to_string(),
            value,
            tags,
            active: idx % 2 == 0,
        });
    }
    out
}

fn lcg_next(s: u64) -> u64 {
    s.wrapping_mul(6364136223846793005).wrapping_add(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_count_matches() {
        let v = generate_records(10, 7);
        assert_eq!(v.len(), 10);
    }

    #[test]
    fn run_ok_for_normal() {
        let cfg = config::AppConfig {
            mode: "normal".to_string(),
            items: 12,
            seed: 11,
        };

        let report = run(cfg).unwrap();
        assert!(report.total_records >= 12);
    }
}
