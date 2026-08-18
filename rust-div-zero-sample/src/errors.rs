use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathError {
    DivisionByZero { numerator: i32 },
    LengthMismatch { numerators: usize, denominators: usize },
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero { numerator } => {
                write!(f, "division by zero attempted with numerator={}", numerator)
            }
            MathError::LengthMismatch { numerators, denominators } => {
                write!(
                    f,
                    "input length mismatch: numerators={} denominators={}",
                    numerators, denominators
                )
            }
        }
    }
}

impl std::error::Error for MathError {}
