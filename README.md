# RustPlayerBenchAI

Rust CLI benchmark runner for digital signage player devices. Measures CPU and memory usage and produces Pass/Warn/Fail verdicts.

## Install

```bash
cargo build --release
# binary at target/release/benchrun
```

## Usage

```bash
# Run benchmark against a device (default: local, 10s)
benchrun run

# Specific device
benchrun run --device scos
benchrun run --device brightsign
benchrun run --device pi4

# Custom duration
benchrun run --device scos --duration 30

# JSON output
benchrun run --device scos --json
```

## Supported devices

| Device | CPU profile | Memory profile |
|---|---|---|
| `scos` | 45% | 380 MB |
| `brightsign` | 30% | 256 MB |
| `pi4` | 55% | 512 MB |
| `local` / unknown | 40% | 350 MB |

## Verdict thresholds

| Verdict | CPU | Memory |
|---|---|---|
| `PASS` | ≤ 70% | ≤ 600 MB |
| `WARN` | 70–90% | 600–800 MB |
| `FAIL` | > 90% | > 800 MB |

## Test

```bash
cargo test
```

14 integration tests — device profiles, verdict thresholds, CLI.

## Stack

Rust · clap · serde · colored · chrono · anyhow
