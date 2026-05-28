mod cli;
mod output;

use anyhow::Result;
use benchrun::{bench, report};
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run {
            device,
            duration,
            json,
            cpu_warn,
            cpu_fail,
            memory_warn,
            memory_fail,
        } => {
            let result = bench::run(&device, duration)?;
            let thresholds = report::Thresholds {
                cpu_warn,
                cpu_fail,
                memory_warn_mb: memory_warn,
                memory_fail_mb: memory_fail,
            };
            let report = report::build_with_thresholds(&device, duration, &result, thresholds);
            if json {
                output::json::print(&report)?;
            } else {
                output::terminal::print(&report);
            }
        }
    }
    Ok(())
}
