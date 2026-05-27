# Architecture

RustPlayerBenchAI is a CLI benchmark runner for quick CPU and memory checks on digital signage player profiles.

## Goals

- Provide repeatable local benchmark-style output.
- Keep device profile logic separate from output rendering.
- Support terminal and JSON output.
- Keep the tool lightweight enough for solo maintenance.

## Module Layout

| Module | Responsibility |
| --- | --- |
| `src/cli.rs` | CLI command and benchmark options |
| `src/bench/` | Device profiles and sample generation |
| `src/report.rs` | Benchmark report model |
| `src/output/` | Terminal and JSON rendering |

## Data Flow

1. The CLI receives a device profile and duration.
2. Benchmark logic creates samples for CPU and memory.
3. The report computes averages, sample count, and verdict.
4. The renderer prints terminal or JSON output.

## Design Notes

- Device profiles should be explicit and test-covered.
- JSON output should stay stable for automation.
- The current model is synthetic and should not claim hardware-level precision.
- Future real-device collection should be added behind a separate module.

## Release Assumptions

- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and `cargo package` pass before release.
- GitHub Actions are intentionally not used in this repo.
