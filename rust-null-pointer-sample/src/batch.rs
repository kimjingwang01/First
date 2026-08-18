use crate::null_like::{panic_on_none, safe_require_value};

#[derive(Debug, Clone)]
pub struct SafeBatchReport {
    pub lengths: Vec<usize>,
    pub errors: Vec<String>,
    pub success_count: usize,
    pub failure_count: usize,
}

pub fn run_panic_mode(values: &[Option<String>]) -> Vec<usize> {
    let mut out = Vec::new();
    for v in values {
        out.push(panic_on_none(v.clone()));
    }
    out
}

pub fn run_safe_mode(values: &[Option<String>]) -> SafeBatchReport {
    let mut lengths = Vec::new();
    let mut errors = Vec::new();

    for (idx, v) in values.iter().enumerate() {
        match safe_require_value(v.clone(), idx) {
            Ok(n) => lengths.push(n),
            Err(e) => errors.push(e.to_string()),
        }
    }

    let success_count = lengths.len();
    let failure_count = errors.len();

    SafeBatchReport {
        lengths,
        errors,
        success_count,
        failure_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_mode_collects_missing_values() {
        let values = vec![Some("x".to_string()), None, Some("yz".to_string())];
        let r = run_safe_mode(&values);
        assert_eq!(r.success_count, 2);
        assert_eq!(r.failure_count, 1);
    }

    #[test]
    #[should_panic]
    fn panic_mode_panics_on_none() {
        let values = vec![Some("x".to_string()), None];
        let _ = run_panic_mode(&values);
    }
}
