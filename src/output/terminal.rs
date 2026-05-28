use benchrun::report::{BenchReport, Verdict};
use colored::Colorize;

pub fn print(report: &BenchReport) {
    println!("\n{}", "BenchRun Report".bold().underline());
    println!("{} {}", "Device:".bold(), report.device);
    println!("{} {}s", "Duration:".bold(), report.duration_secs);
    println!("{} {}", "Samples:".bold(), report.sample_count);
    println!("{} {:.1}%", "Avg CPU:".bold(), report.avg_cpu_percent);
    println!("{} {:.0} MB", "Avg Memory:".bold(), report.avg_memory_mb);
    println!(
        "{} CPU warn {:.1}%, fail {:.1}%; memory warn {:.0} MB, fail {:.0} MB",
        "Thresholds:".bold(),
        report.thresholds.cpu_warn,
        report.thresholds.cpu_fail,
        report.thresholds.memory_warn_mb,
        report.thresholds.memory_fail_mb
    );

    let verdict_colored = match report.verdict {
        Verdict::Pass => "PASS".green().bold(),
        Verdict::Warn => "WARN".yellow().bold(),
        Verdict::Fail => "FAIL".red().bold(),
    };
    println!("{} {}", "Verdict:".bold(), verdict_colored);
    println!();
}
