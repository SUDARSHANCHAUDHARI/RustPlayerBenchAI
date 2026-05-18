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
