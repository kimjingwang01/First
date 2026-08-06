# Rust Sample Project

A medium-sized Rust sample with modular structure, CLI, domain logic, storage simulation, report generation, and tests.

## Structure

- `Cargo.toml`
- `src/main.rs`
- `src/lib.rs`
- `src/app/mod.rs`
- `src/app/config.rs`
- `src/domain/mod.rs`
- `src/domain/models.rs`
- `src/domain/engine.rs`
- `src/io/mod.rs`
- `src/io/repository.rs`

## Run

```bash
cargo run --manifest-path rust-sample/Cargo.toml -- --mode quick --items 10
```

## Test

```bash
cargo test --manifest-path rust-sample/Cargo.toml
```
