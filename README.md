# 🦀 Euler Solutions in Rust

This project implements 20+ mathematical and algorithmic challenges in idiomatic Rust using modular architecture, tests, and optional benchmarking.

## Structure

- `src/problems/problemNN.rs` – Each contains:

  - The original problem description (in doc comment)
  - A single `solve_NN()` function
  - Five targeted unit tests

- `main.rs` – Collects all solutions for bulk execution or CLI-driven runs

## Run a Problem

```bash
cargo run            # runs all
cargo test           # runs tests
cargo bench          # (optional)
```
