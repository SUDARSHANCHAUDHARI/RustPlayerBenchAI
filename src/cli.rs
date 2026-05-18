use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "benchrun", about = "Device benchmark runner for digital signage players", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run benchmark against a device
    Run {
        /// Device identifier (e.g. scos, brightsign, pi4)
        #[arg(long, default_value = "local")]
        device: String,
        /// Duration in seconds
        #[arg(long, default_value = "10")]
        duration: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}
