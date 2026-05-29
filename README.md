# RustPlayerBenchAI

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)

RustPlayerBenchAI is a Rust CLI benchmark reporter for digital signage player profiles. It produces deterministic CPU and memory benchmark reports for known device classes, then maps those results into `PASS`, `WARN`, or `FAIL` verdicts.

## Why This Exists

Digital signage fleets often mix player types with different performance envelopes. RustPlayerBenchAI gives you a simple, scriptable way to compare expected device profiles, validate thresholds, and produce consistent JSON or terminal benchmark summaries.

## Features

- Runs benchmark simulations for supported player profiles.
- Supports `local`, `scos`, `brightsign`, `pi4`, and unknown-device fallback profiles.
- Calculates CPU, memory, and sample count.
- Uses duration to control the number of generated samples.
- Produces `PASS`, `WARN`, and `FAIL` verdicts from CPU and memory thresholds.
- Supports terminal output for humans and JSON output for automation.
- Includes integration tests for profiles, thresholds, duration behavior, and CLI output.

## Installation

```bash
git clone https://github.com/SUDARSHANCHAUDHARI/RustPlayerBenchAI.git
cd RustPlayerBenchAI
cargo build --release
```

The binary is created at:

```bash
target/release/benchrun
```

Optional local install:

```bash
cargo install --path .
```

## Usage

```bash
# Run benchmark against the default local profile for 10 seconds
benchrun run

# Run a known device profile
benchrun run --device scos
benchrun run --device brightsign
benchrun run --device pi4

# Set duration in seconds
benchrun run --device scos --duration 30

# Emit JSON
benchrun run --device scos --json

# Tune pass/warn/fail thresholds for your environment
benchrun run --device scos --cpu-warn 60 --cpu-fail 85 --memory-warn 500 --memory-fail 700
```

## Included Example

This repository includes a checked-in sample report:

```bash
cat examples/scos-report.json
```

Real JSON output from the CLI:

```json
{
  "device": "scos",
  "duration_secs": 5,
  "avg_cpu_percent": 45.0,
  "avg_memory_mb": 380.0,
  "sample_count": 5,
  "verdict": "Pass"
}
```

## Supported Device Profiles

| Device | CPU Profile | Memory Profile |
|---|---:|---:|
| `scos` | 45% | 380 MB |
| `brightsign` | 30% | 256 MB |
| `pi4` | 55% | 512 MB |
| `local` or unknown | 40% | 350 MB |

## Verdict Thresholds

| Verdict | CPU | Memory |
|---|---:|---:|
| `PASS` | 70% or lower | 600 MB or lower |
| `WARN` | 70% to 90% | 600 MB to 800 MB |
| `FAIL` | Above 90% | Above 800 MB |

Thresholds can be customized per run with `--cpu-warn`, `--cpu-fail`, `--memory-warn`, and `--memory-fail`. JSON output includes the thresholds used for that verdict.

## JSON Use Cases

The `--json` flag is useful for CI jobs, smoke-test dashboards, QA scripts, and fleet checks where another tool needs to consume the benchmark result.

```bash
benchrun run --device pi4 --duration 60 --json > bench-report.json
```

## Development

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release
```

Run these checks locally before publishing changes.

## Project Structure

```text
src/
  cli.rs          Command-line interface
  bench/          Device profile and benchmark calculation
  report.rs       Verdict and report output
tests/
  integration_test.rs
```

## Project Docs

- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Maintainer notes](docs/NOTES.md)
- [Content plan](docs/CONTENT_PLAN.md)

## Release Status

Current production release: `v1.1.0`

The `v1.1.0` release was verified with formatting, clippy, tests, optimized release build, and `cargo package`.

## License

MIT. See [LICENSE](LICENSE).

## Developer

Built by [Sudarshan Chaudhari](https://github.com/SUDARSHANCHAUDHARI) under SudarshanTechLabs.
