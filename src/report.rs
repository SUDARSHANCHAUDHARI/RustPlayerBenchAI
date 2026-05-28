use crate::bench::BenchResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchReport {
    pub device: String,
    pub duration_secs: u64,
    pub avg_cpu_percent: f64,
    pub avg_memory_mb: f64,
    pub sample_count: usize,
    pub verdict: Verdict,
    pub thresholds: Thresholds,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Thresholds {
    pub cpu_warn: f64,
    pub cpu_fail: f64,
    pub memory_warn_mb: f64,
    pub memory_fail_mb: f64,
}

impl Default for Thresholds {
    fn default() -> Self {
        Self {
            cpu_warn: 70.0,
            cpu_fail: 90.0,
            memory_warn_mb: 600.0,
            memory_fail_mb: 800.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Verdict {
    Pass,
    Warn,
    Fail,
}

impl std::fmt::Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verdict::Pass => write!(f, "PASS"),
            Verdict::Warn => write!(f, "WARN"),
            Verdict::Fail => write!(f, "FAIL"),
        }
    }
}

pub fn build(device: &str, duration: u64, result: &BenchResult) -> BenchReport {
    build_with_thresholds(device, duration, result, Thresholds::default())
}

pub fn build_with_thresholds(
    device: &str,
    duration: u64,
    result: &BenchResult,
    thresholds: Thresholds,
) -> BenchReport {
    let verdict = if result.avg_cpu_percent > thresholds.cpu_fail
        || result.avg_memory_mb > thresholds.memory_fail_mb
    {
        Verdict::Fail
    } else if result.avg_cpu_percent > thresholds.cpu_warn
        || result.avg_memory_mb > thresholds.memory_warn_mb
    {
        Verdict::Warn
    } else {
        Verdict::Pass
    };

    BenchReport {
        device: device.to_string(),
        duration_secs: duration,
        avg_cpu_percent: result.avg_cpu_percent,
        avg_memory_mb: result.avg_memory_mb,
        sample_count: result.sample_count,
        verdict,
        thresholds,
    }
}
