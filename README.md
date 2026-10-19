# RustPlayerBenchAI

[![crates.io](https://img.shields.io/crates/v/playerbenchai?logo=rust)](https://crates.io/crates/playerbenchai)
[![Downloads](https://img.shields.io/crates/d/playerbenchai?logo=rust)](https://crates.io/crates/playerbenchai)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)

> A Rust CLI benchmark reporter for digital signage players — measured locally, reference values per device class.

**RustPlayerBenchAI** (installed as the `playerbenchai` command) measures CPU and memory on
the local machine, or reports fixed reference values for known device classes, then maps
those results into `PASS`, `WARN`, or `FAIL` verdicts. Every report says which one it is.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Included Example](#included-example)
- [Supported Device Profiles](#supported-device-profiles)
- [Verdict Thresholds](#verdict-thresholds)
- [JSON Use Cases](#json-use-cases)
- [Development](#development)
- [Project Structure](#project-structure)
- [Documentation](#documentation)
- [Release Status](#release-status)
- [License](#license)
- [About](#about)

## Overview

Digital signage fleets often mix player types with different performance envelopes.
RustPlayerBenchAI gives you a simple, scriptable way to compare expected device profiles,
validate thresholds, and produce consistent JSON or terminal benchmark summaries.

## Features

- `local` (the default) measures this machine: it renders simulated 640x360 frames at
  60 fps for the given duration and times them with `std::time::Instant`.
- `scos`, `brightsign`, `pi4`, and unknown devices return reference values (not measured).
- Labels every report with its source: `measured` or `reference values (not measured)`.
- Calculates CPU, memory, and sample count.
- Produces `PASS`, `WARN`, and `FAIL` verdicts from CPU and memory thresholds.
- Supports terminal output for humans and JSON output for automation.
- Includes integration tests for profiles, thresholds, duration behavior, and CLI output.

## Installation

### From crates.io (recommended)

```bash
cargo install playerbenchai
```

### From source

```bash
git clone https://github.com/SUDARSHANCHAUDHARI/RustPlayerBenchAI.git
cd RustPlayerBenchAI
cargo build --release
```

The binary is created at:

```bash
target/release/playerbenchai
```

Optional local install from a source checkout:

```bash
cargo install --path .
```

## Usage

```bash
# Measure this machine for 10 seconds (the default `local` device)
playerbenchai run

# Show reference values for a known device class (not measured)
playerbenchai run --device scos
playerbenchai run --device brightsign
playerbenchai run --device pi4

# Set duration in seconds
playerbenchai run --device scos --duration 30

# Emit JSON
playerbenchai run --device scos --json

# Tune pass/warn/fail thresholds for your environment
playerbenchai run --device scos --cpu-warn 60 --cpu-fail 85 --memory-warn 500 --memory-fail 700
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
  "source": "reference values (not measured)",
  "verdict": "Pass"
}
```

## Supported Device Profiles

`local` is the only measured device. CPU is the share of wall time spent rendering frames
(one core). Memory is this process's resident set size, read with `ps` once per second.
Sample count is the number of frames rendered.

The other devices are reference values (not measured). No device is contacted.

| Device | CPU (reference) | Memory (reference) |
|---|---:|---:|
| `scos` | 45% | 380 MB |
| `brightsign` | 30% | 256 MB |
| `pi4` | 55% | 512 MB |
| unknown | 40% | 350 MB |

## Verdict Thresholds

| Verdict | CPU | Memory |
|---|---:|---:|
| `PASS` | 70% or lower | 600 MB or lower |
| `WARN` | 70% to 90% | 600 MB to 800 MB |
| `FAIL` | Above 90% | Above 800 MB |

Thresholds can be customized per run with `--cpu-warn`, `--cpu-fail`, `--memory-warn`, and
`--memory-fail`. JSON output includes the thresholds used for that verdict.

## JSON Use Cases

The `--json` flag is useful for CI jobs, smoke-test dashboards, QA scripts, and fleet checks
where another tool needs to consume the benchmark result.

```bash
playerbenchai run --device pi4 --duration 60 --json > bench-report.json
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
  bench/          Local measurement and reference device values
  report.rs       Verdict and report output
tests/
  integration_test.rs
```

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Maintainer notes](docs/NOTES.md)
- [Content plan](docs/CONTENT_PLAN.md)

## Release Status

Current release: **`v1.1.1`**, published on [crates.io](https://crates.io/crates/playerbenchai).

Each release is verified with formatting, Clippy, tests, an optimized release build, and
`cargo package` before publishing.

## License

MIT — see [LICENSE](LICENSE).

---

## About

I'm Sudarshan Chaudhari, a Senior Quality Engineer, Test Automation specialist, and AI systems builder based in Bangkok, Thailand.

I have 13+ years of experience in software quality engineering, working across SaaS, fintech, gaming, web, mobile, cloud, and digital signage platforms. My background combines hands-on test automation with QA leadership, test strategy, CI/CD, release quality, production investigation, and cross-platform validation.

Alongside my professional QA career, I run [SudarshanTechLabs](https://sudarshantechlabs.com/), my independent engineering and product lab where I design, build, test, and ship software across Android, web, AI, cybersecurity, developer tooling, and cross-platform applications.

### What I work on

- ⚙️ **Quality Engineering & Test Automation** — Playwright, Selenium, Cypress, Appium, API testing, automation frameworks, end-to-end testing, CI/CD, release gates, GitHub Actions, risk-based testing, and production validation
- 🤖 **AI Systems & Automation** — AI agents, multi-agent orchestration, MCP servers, AI-assisted QA, prompt tooling, developer workflows, automation systems, and Claude Code plugins
- 📱 **Mobile & Cross-Platform Applications** — Android applications built with Kotlin and Jetpack Compose, Google Play releases, automated build and publishing pipelines, and cross-platform development spanning iOS, web, Windows, and macOS
- 🌐 **Web Applications & Platforms** — Full-stack applications using Next.js, TypeScript, Firebase, Cloudflare, REST APIs, and modern web infrastructure
- 🛠️ **Developer Tooling & CLI Engineering** — Rust, Python, TypeScript, CLI utilities, multi-repository tooling, build automation, release tooling, and engineering productivity systems
- 🛡️ **Cybersecurity & Observability** — Threat detection, log analysis, security auditing, vulnerability assessment, monitoring, and security-focused developer tools
- 📺 **Digital Signage & Device Platforms** — Content validation, playback testing, device compatibility, production investigation, monitoring, and QA across diverse hardware and operating-system environments

My work sits at the intersection of quality engineering, automation, AI, and software development. I approach products with a QA mindset from the beginning: understanding failure modes, designing for testability, automating repetitive work, and building release confidence into the engineering process.

Through SudarshanTechLabs, I also build products and tools from idea to production, covering architecture, development, testing, CI/CD, release automation, monitoring, and ongoing maintenance.

🌐 [sudarshantechlabs.com](https://sudarshantechlabs.com/) · 💼 [LinkedIn](https://linkedin.com/in/sudarshan-chaudhari) · 🐙 [GitHub](https://github.com/SUDARSHANCHAUDHARI) · ✉️ [sunny.sudarshan@gmail.com](mailto:sunny.sudarshan@gmail.com)
