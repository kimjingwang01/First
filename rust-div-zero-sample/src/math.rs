use crate::errors::MathError;

/// Deliberately unsafe example: panics when denominator == 0.
pub fn naive_divide(numerator: i32, denominator: i32) -> i32 {
    numerator / denominator
}

/// Safe variant that avoids panic and returns Result.
pub fn safe_divide(numerator: i32, denominator: i32) -> Result<i32, MathError> {
    if denominator == 0 {
        return Err(MathError::DivisionByZero { numerator });
    }
    Ok(numerator / denominator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_divide_ok() {
        assert_eq!(safe_divide(10, 2).unwrap(), 5);
    }

    #[test]
    fn safe_divide_err_on_zero() {
        let err = safe_divide(10, 0).unwrap_err();
        assert_eq!(err, MathError::DivisionByZero { numerator: 10 });
    }

    #[test]
    #[should_panic]
    fn naive_divide_panics_on_zero() {
        let _ = naive_divide(10, 0);
    }
}
