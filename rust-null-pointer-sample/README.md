# Rust Null Pointer Sample

This sample demonstrates Rust patterns for null-like failures, including a direct raw-pointer null dereference.

## What it includes

1. **Option panic path** via `Option::unwrap()` on `None` (`panic_on_none`).
2. **Direct null pointer dereference** in `unsafe` (`direct_null_deref`) using `*const i32` and `ptr::null()`.
3. **Safe path** via `Result` and custom error (`safe_require_value`).
4. Batch processing that compares unsafe and safe handling.

## Important warning

`direct_null_deref` intentionally performs undefined behavior by dereferencing a null raw pointer.
Running it may crash the process (segmentation fault / access violation), panic, or show other unpredictable behavior depending on platform/optimizer.

## Run

```bash
cargo run --manifest-path rust-null-pointer-sample/Cargo.toml -- --mode safe
cargo run --manifest-path rust-null-pointer-sample/Cargo.toml -- --mode panic
cargo run --manifest-path rust-null-pointer-sample/Cargo.toml -- --mode null-deref
```

## Test

```bash
cargo test --manifest-path rust-null-pointer-sample/Cargo.toml
```
