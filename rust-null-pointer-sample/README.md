# Rust Null Pointer Sample

This sample demonstrates Rust patterns often used to model "null pointer" style failures safely.

## What it includes

1. **Panic path** via `Option::unwrap()` on `None` (`panic_on_none`) to reproduce a null-like runtime failure.
2. **Safe path** via `Result` and custom error (`safe_require_value`).
3. Batch processing that compares unsafe and safe handling.

## Run

```bash
cargo run --manifest-path rust-null-pointer-sample/Cargo.toml -- --mode safe
cargo run --manifest-path rust-null-pointer-sample/Cargo.toml -- --mode panic
```

## Test

```bash
cargo test --manifest-path rust-null-pointer-sample/Cargo.toml
```
