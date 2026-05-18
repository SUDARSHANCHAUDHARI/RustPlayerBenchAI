use serde::{Deserialize, Serialize};
use crate::bench::BenchResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchReport {
    pub device: String,
    pub duration_secs: u64,
    pub avg_cpu_percent: f64,
    pub avg_memory_mb: f64,
    pub sample_count: usize,
    pub verdict: Verdict,
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
    let verdict = if result.avg_cpu_percent > 90.0 || result.avg_memory_mb > 800.0 {
        Verdict::Fail
    } else if result.avg_cpu_percent > 70.0 || result.avg_memory_mb > 600.0 {
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
    }
}
