use crate::errors::NullLikeError;
use std::ptr;

/// Intentionally unsafe: panics when value is None.
pub fn panic_on_none(value: Option<String>) -> usize {
    value.unwrap().len()
}

/// Direct null pointer dereference demonstration.
///
/// WARNING: This is undefined behavior and is intentionally included only
/// for educational/reproduction purposes.
pub fn direct_null_deref() -> i32 {
    let null_ptr: *const i32 = ptr::null();
    unsafe {
        let value: i32 = *null_ptr;
        println!("value = {}", value);
        value
    }
}

/// Safe variant: converts None into an explicit error.
pub fn safe_require_value(value: Option<String>, index: usize) -> Result<usize, NullLikeError> {
    match value {
        Some(v) => Ok(v.len()),
        None => Err(NullLikeError::MissingValue { index }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_require_value_ok() {
        let n = safe_require_value(Some("abc".to_string()), 0).unwrap();
        assert_eq!(n, 3);
    }

    #[test]
    fn safe_require_value_err() {
        let err = safe_require_value(None, 4).unwrap_err();
        assert_eq!(err, NullLikeError::MissingValue { index: 4 });
    }

    #[test]
    #[should_panic]
    fn panic_on_none_panics() {
        let _ = panic_on_none(None);
    }
}
