use anyhow::{bail, Context, Result};
use std::hint::black_box;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

pub struct BenchResult {
    pub avg_cpu_percent: f64,
    pub avg_memory_mb: f64,
    pub sample_count: usize,
    /// true when the numbers were measured on this machine; false for reference profiles.
    pub measured: bool,
}

const FRAME_BUDGET: Duration = Duration::from_micros(16_667); // 60 fps
const FRAME_WIDTH: usize = 640;
const FRAME_HEIGHT: usize = 360;

pub fn run(device: &str, duration_secs: u64) -> Result<BenchResult> {
    if device == "local" {
        return measure_local(duration_secs);
    }
    // Reference values per device class, not measured from any device.
    Ok(BenchResult {
        avg_cpu_percent: reference_cpu(device),
        avg_memory_mb: reference_memory(device),
        sample_count: duration_secs as usize,
        measured: false,
    })
}

/// Renders simulated frames at 60 fps on this machine for `duration_secs` (at least 1).
/// CPU % is the share of wall time spent rendering; memory is this process's RSS,
/// sampled once per second. `sample_count` is the number of frames rendered.
fn measure_local(duration_secs: u64) -> Result<BenchResult> {
    let seconds = duration_secs.max(1);
    let mut frame = vec![0u32; FRAME_WIDTH * FRAME_HEIGHT];
    let mut frames = 0usize;
    let mut busy = Duration::ZERO;
    let mut memory_samples = Vec::with_capacity(seconds as usize);
    let start = Instant::now();

    for second in 1..=seconds {
        let second_end = start + Duration::from_secs(second);
        while Instant::now() < second_end {
            let frame_start = Instant::now();
            render_frame(&mut frame, frames as u32);
            let work = frame_start.elapsed();
            busy += work;
            frames += 1;
            if let Some(idle) = FRAME_BUDGET.checked_sub(work) {
                thread::sleep(idle.min(second_end.saturating_duration_since(Instant::now())));
            }
        }
        memory_samples.push(resident_memory_mb()?);
    }

    let elapsed = start.elapsed().as_secs_f64();
    Ok(BenchResult {
        avg_cpu_percent: (busy.as_secs_f64() / elapsed * 100.0).min(100.0),
        avg_memory_mb: memory_samples.iter().sum::<f64>() / memory_samples.len() as f64,
        sample_count: frames,
        measured: true,
    })
}

fn render_frame(frame: &mut [u32], tick: u32) {
    for (i, px) in frame.iter_mut().enumerate() {
        let x = (i % FRAME_WIDTH) as u32;
        let y = (i / FRAME_WIDTH) as u32;
        *px = (x.wrapping_add(tick) ^ y.wrapping_mul(31)).wrapping_mul(2_654_435_761);
    }
    black_box(frame);
}

/// Resident set size of this process in MB, read via `ps` (macOS and Linux).
fn resident_memory_mb() -> Result<f64> {
    let output = Command::new("ps")
        .args(["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .context("failed to run `ps` to read memory usage")?;
    if !output.status.success() {
        bail!("`ps` exited with {}", output.status);
    }
    let kb: f64 = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .context("unexpected `ps` rss output")?;
    Ok(kb / 1024.0)
}

fn reference_cpu(device: &str) -> f64 {
    match device {
        "scos" => 45.0,
        "brightsign" => 30.0,
        "pi4" => 55.0,
        _ => 40.0,
    }
}

fn reference_memory(device: &str) -> f64 {
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
        assert!(!scos.measured);

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
        assert!(!result.measured);
    }

    #[test]
    fn sample_count_matches_duration() {
        let result = run("pi4", 10).unwrap();
        assert_eq!(result.sample_count, 10);
    }

    #[test]
    fn local_is_measured_not_a_profile() {
        let result = run("local", 1).unwrap();
        assert!(result.measured);
        assert!(result.sample_count > 0);
        assert!(result.avg_cpu_percent > 0.0 && result.avg_cpu_percent <= 100.0);
        // The frame buffer alone is ~0.9 MB, so RSS must be above that.
        assert!(result.avg_memory_mb > 0.5);
        assert_ne!(
            (result.avg_cpu_percent, result.avg_memory_mb),
            (40.0, 350.0),
            "local must not fall back to the unknown-device constants"
        );
    }
}
