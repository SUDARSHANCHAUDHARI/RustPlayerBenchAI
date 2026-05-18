# RustPlayerBenchAI — Claude Code Context

## Purpose
Rust CLI benchmark runner for digital signage player devices.
Measures CPU, memory, FPS and detects performance regressions.

## Type
Rust CLI (benchrun)

## Stack
- Language: Rust (stable)
- CLI: clap
- Serialization: serde + serde_json
- Errors: anyhow + thiserror
- Terminal: colored

## Commands
cargo run -- run --device scos --duration 30
cargo run -- run --device local --json
cargo test
cargo clippy
cargo fmt
cargo build --release

## GitHub Repo
https://github.com/SUDARSHANCHAUDHARI/RustPlayerBenchAI
