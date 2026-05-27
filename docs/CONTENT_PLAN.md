# Content Plan

## Positioning

RustPlayerBenchAI is a good honesty-first engineering story: benchmark reports are only useful when the tool clearly says what it measures and what it does not.

## Blog Post Queue

| Priority | Working Title | Feature Tie-In |
| --- | --- | --- |
| 1 | What a Useful Signage Player Benchmark Report Should Say | Current report model |
| 2 | Designing Pass, Warn, and Fail Thresholds for Device Checks | Configurable thresholds |
| 3 | From Synthetic Profiles to Real Device Metrics | Future host collection |

## Auto-Blog Prompt Seed

Write a practical blog post about building a Rust CLI for signage player benchmark reports. Be explicit that the current version uses profile-based CPU and memory samples. Include terminal output, JSON output, and a section on the future path toward real host metrics.

## Useful Examples

- `benchrun run --device scos --duration 5`
- `examples/scos-report.json`
- JSON report comparison idea.
