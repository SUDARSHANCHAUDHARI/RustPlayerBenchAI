use anyhow::Result;

pub struct BenchResult {
    pub avg_cpu_percent: f64,
    pub avg_memory_mb: f64,
    pub sample_count: usize,
}

pub fn run(device: &str, duration_secs: u64) -> Result<BenchResult> {
    // Simulate benchmark sampling — real impl would connect to device API
    let sample_count = duration_secs as usize;
    let avg_cpu_percent = simulate_cpu(device);
    let avg_memory_mb = simulate_memory(device);

    Ok(BenchResult {
        avg_cpu_percent,
        avg_memory_mb,
        sample_count,
    })
}

fn simulate_cpu(device: &str) -> f64 {
    match device {
        "scos" => 45.0,
        "brightsign" => 30.0,
        "pi4" => 55.0,
        _ => 40.0,
    }
}

fn simulate_memory(device: &str) -> f64 {
    match device {
        "scos" => 380.0,
        "brightsign" => 256.0,
        "pi4" => 512.0,
        _ => 350.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_known_devices_return_expected_profiles() {
        let scos = run("scos", 3).unwrap();
        assert_eq!(scos.avg_cpu_percent, 45.0);
        assert_eq!(scos.avg_memory_mb, 380.0);
        assert_eq!(scos.sample_count, 3);

        let bs = run("brightsign", 2).unwrap();
        assert_eq!(bs.avg_cpu_percent, 30.0);
        assert_eq!(bs.avg_memory_mb, 256.0);

        let pi = run("pi4", 1).unwrap();
        assert_eq!(pi.avg_cpu_percent, 55.0);
        assert_eq!(pi.avg_memory_mb, 512.0);
    }

    #[test]
    fn unknown_device_returns_defaults() {
        let result = run("unknown-device", 5).unwrap();
        assert_eq!(result.avg_cpu_percent, 40.0);
        assert_eq!(result.avg_memory_mb, 350.0);
        assert_eq!(result.sample_count, 5);
    }

    #[test]
    fn sample_count_matches_duration() {
        let result = run("pi4", 10).unwrap();
        assert_eq!(result.sample_count, 10);
    }
}
