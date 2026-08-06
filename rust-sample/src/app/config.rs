#[derive(Debug, Clone)]
pub struct AppConfig {
    pub mode: String,
    pub items: usize,
    pub seed: u64,
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.items == 0 {
            return Err("items must be > 0".to_string());
        }
        if self.items > 100_000 {
            return Err("items too large".to_string());
        }
        match self.mode.as_str() {
            "quick" | "normal" | "deep" => Ok(()),
            _ => Err(format!("invalid mode: {}", self.mode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_ok() {
        let c = AppConfig { mode: "quick".to_string(), items: 1, seed: 1 };
        assert!(c.validate().is_ok());
    }

    #[test]
    fn validate_reject_zero() {
        let c = AppConfig { mode: "quick".to_string(), items: 0, seed: 1 };
        assert!(c.validate().is_err());
    }
}
