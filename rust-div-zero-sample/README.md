# Rust Division by Zero Sample

This project demonstrates:

1. A **panic-causing division by zero** path (`naive_divide`) to reproduce the bug.
2. A **safe division** path (`safe_divide`) returning `Result`.
3. Batch processing examples showing both unsafe and safe strategies.

## Run

```bash
cargo run --manifest-path rust-div-zero-sample/Cargo.toml -- --mode safe
cargo run --manifest-path rust-div-zero-sample/Cargo.toml -- --mode panic
```

## Test

```bash
cargo test --manifest-path rust-div-zero-sample/Cargo.toml
```
