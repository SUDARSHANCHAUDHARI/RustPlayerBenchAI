use assert_cmd::Command;
use predicates::str::contains;

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
fn test_run_local() {
    Command::cargo_bin("benchrun")
        .unwrap()
        .args(["run", "--device", "local", "--duration", "1"])
        .assert()
        .success()
        .stdout(contains("BenchRun Report"));
}
