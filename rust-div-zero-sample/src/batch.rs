use crate::errors::MathError;
use crate::math::{naive_divide, safe_divide};

#[derive(Debug, Clone)]
pub struct SafeBatchReport {
    pub values: Vec<i32>,
    pub errors: Vec<String>,
    pub success_count: usize,
    pub failure_count: usize,
}

pub fn run_panic_mode(numerators: &[i32], denominators: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    let len = numerators.len().min(denominators.len());
    for i in 0..len {
        // Intentionally unsafe for demonstration.
        out.push(naive_divide(numerators[i], denominators[i]));
    }
    out
}

pub fn run_safe_mode(numerators: &[i32], denominators: &[i32]) -> SafeBatchReport {
    let mut values = Vec::new();
    let mut errors = Vec::new();

    if numerators.len() != denominators.len() {
        let err = MathError::LengthMismatch {
            numerators: numerators.len(),
            denominators: denominators.len(),
        };
        errors.push(err.to_string());
    }

    let len = numerators.len().min(denominators.len());
    for i in 0..len {
        match safe_divide(numerators[i], denominators[i]) {
            Ok(v) => values.push(v),
            Err(e) => errors.push(format!("index {}: {}", i, e)),
        }
    }

    let success_count = values.len();
    let failure_count = errors.len();

    SafeBatchReport {
        values,
        errors,
        success_count,
        failure_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_mode_collects_errors() {
        let ns = vec![10, 20, 30];
        let ds = vec![2, 0, 5];

        let r = run_safe_mode(&ns, &ds);
        assert_eq!(r.success_count, 2);
        assert_eq!(r.failure_count, 1);
    }

    #[test]
    #[should_panic]
    fn panic_mode_panics_when_zero_exists() {
        let ns = vec![10, 20, 30];
        let ds = vec![2, 0, 5];
        let _ = run_panic_mode(&ns, &ds);
    }
}
