use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NullLikeError {
    MissingValue { index: usize },
}

impl fmt::Display for NullLikeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NullLikeError::MissingValue { index } => {
                write!(f, "missing value at index {}", index)
            }
        }
    }
}

impl std::error::Error for NullLikeError {}
