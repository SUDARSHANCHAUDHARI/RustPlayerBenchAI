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
        } => {
            let result = bench::run(&device, duration)?;
            let report = report::build(&device, duration, &result);
            if json {
                output::json::print(&report)?;
            } else {
                output::terminal::print(&report);
            }
        }
    }
    Ok(())
}
