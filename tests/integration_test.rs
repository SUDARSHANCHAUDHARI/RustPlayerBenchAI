use assert_cmd::Command;
use benchrun::bench::{self, BenchResult};
use benchrun::report::{self, Verdict};
use predicates::str::contains;

// --- CLI tests ---

#[test]
fn test_help() {
    Command::cargo_bin("benchrun")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("benchmark runner"));
}

#[test]
fn test_run_subcommand_help() {
    Command::cargo_bin("benchrun")
        .unwrap()
        .args(["run", "--help"])
        .assert()
        .success()
        .stdout(contains("Device identifier"));
}

// --- Bench engine: device profiles ---

#[test]
fn test_scos_profile() {
    let result = bench::run("scos", 5).unwrap();
    assert_eq!(result.avg_cpu_percent, 45.0);
    assert_eq!(result.avg_memory_mb, 380.0);
    assert_eq!(result.sample_count, 5);
}

#[test]
fn test_brightsign_profile() {
    let result = bench::run("brightsign", 3).unwrap();
    assert_eq!(result.avg_cpu_percent, 30.0);
    assert_eq!(result.avg_memory_mb, 256.0);
}

#[test]
fn test_pi4_profile() {
    let result = bench::run("pi4", 2).unwrap();
    assert_eq!(result.avg_cpu_percent, 55.0);
    assert_eq!(result.avg_memory_mb, 512.0);
}

#[test]
fn test_unknown_device_uses_defaults() {
    let result = bench::run("unknown-device", 1).unwrap();
    assert_eq!(result.avg_cpu_percent, 40.0);
    assert_eq!(result.avg_memory_mb, 350.0);
}

#[test]
fn test_duration_sets_sample_count() {
    let result = bench::run("local", 7).unwrap();
    assert_eq!(result.sample_count, 7);
}

// --- Report verdict thresholds ---

#[test]
fn test_normal_load_is_pass() {
    let result = BenchResult { avg_cpu_percent: 40.0, avg_memory_mb: 300.0, sample_count: 5 };
    let rep = report::build("test", 5, &result);
    assert!(matches!(rep.verdict, Verdict::Pass));
}

#[test]
fn test_high_cpu_is_warn() {
    let result = BenchResult { avg_cpu_percent: 75.0, avg_memory_mb: 300.0, sample_count: 5 };
    let rep = report::build("test", 5, &result);
    assert!(matches!(rep.verdict, Verdict::Warn));
}

#[test]
fn test_high_memory_is_warn() {
    let result = BenchResult { avg_cpu_percent: 40.0, avg_memory_mb: 650.0, sample_count: 5 };
    let rep = report::build("test", 5, &result);
    assert!(matches!(rep.verdict, Verdict::Warn));
}

#[test]
fn test_critical_cpu_is_fail() {
    let result = BenchResult { avg_cpu_percent: 95.0, avg_memory_mb: 300.0, sample_count: 5 };
    let rep = report::build("test", 5, &result);
    assert!(matches!(rep.verdict, Verdict::Fail));
}

#[test]
fn test_critical_memory_is_fail() {
    let result = BenchResult { avg_cpu_percent: 40.0, avg_memory_mb: 850.0, sample_count: 5 };
    let rep = report::build("test", 5, &result);
    assert!(matches!(rep.verdict, Verdict::Fail));
}

// --- CLI integration ---

#[test]
fn test_cli_run_local_json() {
    Command::cargo_bin("benchrun")
        .unwrap()
        .args(["run", "--device", "local", "--duration", "1", "--json"])
        .assert()
        .success()
        .stdout(contains("avg_cpu_percent"));
}

#[test]
fn test_cli_run_scos_terminal() {
    Command::cargo_bin("benchrun")
        .unwrap()
        .args(["run", "--device", "scos", "--duration", "1"])
        .assert()
        .success()
        .stdout(contains("BenchRun Report"));
}
