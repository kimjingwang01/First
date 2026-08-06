use std::cmp::Ordering;
use std::collections::HashMap;

use super::models::{AnalysisReport, DataRecord, ProcessedRecord, RiskLevel};

pub fn process_records(records: &[DataRecord], mode: &str) -> Result<Vec<ProcessedRecord>, String> {
    if records.is_empty() {
        return Err("no records".to_string());
    }

    let baseline = compute_baseline(records);
    let mut out = Vec::with_capacity(records.len());

    for rec in records {
        let norm = normalize_value(rec.value, baseline);
        let mut score = calculate_score(rec, norm, mode);
        let mut notes = Vec::new();

        if rec.tags.iter().any(|t| t == "high") {
            score *= 1.12;
            notes.push("high tag multiplier".to_string());
        }
        if rec.tags.iter().any(|t| t == "low") {
            score *= 0.91;
            notes.push("low tag dampener".to_string());
        }
        if rec.active {
            score += 3.5;
            notes.push("active boost".to_string());
        }

        let risk = classify_risk(score, mode);

        out.push(ProcessedRecord {
            id: rec.id,
            category: rec.category.clone(),
            raw_value: rec.value,
            normalized_value: norm,
            score,
            risk_level: risk,
            notes,
        });
    }

    Ok(out)
}

pub fn analyze_records(records: &[ProcessedRecord], mode: &str) -> AnalysisReport {
    let total = records.len();

    let mut sum = 0.0;
    let mut min_score = f64::INFINITY;
    let mut max_score = f64::NEG_INFINITY;

    let mut risk_counts: HashMap<String, usize> = HashMap::new();
    let mut category_counts: HashMap<String, usize> = HashMap::new();

    for r in records {
        sum += r.score;
        if r.score < min_score {
            min_score = r.score;
        }
        if r.score > max_score {
            max_score = r.score;
        }

        let risk_key = risk_to_str(&r.risk_level).to_string();
        *risk_counts.entry(risk_key).or_insert(0) += 1;
        *category_counts.entry(r.category.clone()).or_insert(0) += 1;
    }

    let avg = if total == 0 { 0.0 } else { sum / total as f64 };

    let mut top = records.to_vec();
    top.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));
    top.truncate(5);

    let mut notes = vec![
        format!("mode={} influences scoring curve", mode),
        "top_records are sorted by descending score".to_string(),
    ];
    if avg > 120.0 {
        notes.push("high average score detected".to_string());
    }

    AnalysisReport {
        mode: mode.to_string(),
        total_records: total,
        avg_score: avg,
        min_score,
        max_score,
        risk_counts,
        category_counts,
        top_records: top,
        notes,
    }
}

fn compute_baseline(records: &[DataRecord]) -> f64 {
    let sum: f64 = records.iter().map(|r| r.value).sum();
    let avg = if records.is_empty() { 1.0 } else { sum / records.len() as f64 };
    avg.max(1.0)
}

fn normalize_value(v: f64, baseline: f64) -> f64 {
    (v / baseline) * 100.0
}

fn calculate_score(rec: &DataRecord, normalized: f64, mode: &str) -> f64 {
    let category_factor = match rec.category.as_str() {
        "alpha" => 1.15,
        "beta" => 1.05,
        "gamma" => 0.95,
        "delta" => 0.88,
        _ => 1.0,
    };

    let mode_factor = match mode {
        "quick" => 0.8,
        "normal" => 1.0,
        "deep" => 1.25,
        _ => 1.0,
    };

    let wave = ((rec.id as f64) * 0.173).sin() * 7.0;

    (normalized * category_factor * mode_factor) + wave
}

fn classify_risk(score: f64, mode: &str) -> RiskLevel {
    let (a, b, c) = match mode {
        "quick" => (55.0, 90.0, 125.0),
        "normal" => (60.0, 100.0, 145.0),
        "deep" => (70.0, 115.0, 165.0),
        _ => (60.0, 100.0, 145.0),
    };

    if score < a {
        RiskLevel::Low
    } else if score < b {
        RiskLevel::Medium
    } else if score < c {
        RiskLevel::High
    } else {
        RiskLevel::Critical
    }
}

fn risk_to_str(level: &RiskLevel) -> &'static str {
    match level {
        RiskLevel::Low => "low",
        RiskLevel::Medium => "medium",
        RiskLevel::High => "high",
        RiskLevel::Critical => "critical",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<DataRecord> {
        vec![
            DataRecord { id: 1, category: "alpha".to_string(), value: 30.0, tags: vec![], active: true },
            DataRecord { id: 2, category: "beta".to_string(), value: 60.0, tags: vec!["high".to_string()], active: false },
            DataRecord { id: 3, category: "gamma".to_string(), value: 10.0, tags: vec!["low".to_string()], active: true },
            DataRecord { id: 4, category: "delta".to_string(), value: 90.0, tags: vec!["high".to_string(), "periodic".to_string()], active: false },
        ]
    }

    #[test]
    fn process_records_returns_same_count() {
        let input = sample();
        let out = process_records(&input, "normal").unwrap();
        assert_eq!(out.len(), input.len());
    }

    #[test]
    fn analyze_records_has_top_records() {
        let input = sample();
        let out = process_records(&input, "deep").unwrap();
        let report = analyze_records(&out, "deep");
        assert!(!report.top_records.is_empty());
    }
}
