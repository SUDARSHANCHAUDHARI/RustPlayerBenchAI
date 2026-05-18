use anyhow::Result;
use crate::report::BenchReport;

pub fn print(report: &BenchReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}
