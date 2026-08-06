pub mod app;
pub mod domain;
pub mod io;

pub fn banner() -> &'static str {
    "Rust Sample Project: modular CLI application"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_contains_keyword() {
        assert!(banner().contains("Rust Sample"));
    }
}
