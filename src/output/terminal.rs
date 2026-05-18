use colored::Colorize;
use crate::report::{BenchReport, Verdict};

pub fn print(report: &BenchReport) {
    println!("\n{}", "BenchRun Report".bold().underline());
    println!("{} {}", "Device:".bold(), report.device);
    println!("{} {}s", "Duration:".bold(), report.duration_secs);
    println!("{} {}", "Samples:".bold(), report.sample_count);
    println!("{} {:.1}%", "Avg CPU:".bold(), report.avg_cpu_percent);
    println!("{} {:.0} MB", "Avg Memory:".bold(), report.avg_memory_mb);

    let verdict_colored = match report.verdict {
        Verdict::Pass => "PASS".green().bold(),
        Verdict::Warn => "WARN".yellow().bold(),
        Verdict::Fail => "FAIL".red().bold(),
    };
    println!("{} {}", "Verdict:".bold(), verdict_colored);
    println!();
}
