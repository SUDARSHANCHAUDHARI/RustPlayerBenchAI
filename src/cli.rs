use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "benchrun",
    about = "Device benchmark runner for digital signage players",
    version
)]
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
        /// CPU percentage above this value produces WARN
        #[arg(long, default_value_t = 70.0)]
        cpu_warn: f64,
        /// CPU percentage above this value produces FAIL
        #[arg(long, default_value_t = 90.0)]
        cpu_fail: f64,
        /// Memory in MB above this value produces WARN
        #[arg(long, default_value_t = 600.0)]
        memory_warn: f64,
        /// Memory in MB above this value produces FAIL
        #[arg(long, default_value_t = 800.0)]
        memory_fail: f64,
    },
}
