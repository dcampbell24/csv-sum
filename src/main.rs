use clap::Parser;
use serde::Deserialize;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut reader = csv::Reader::from_path(args.path)?;
    let mut total = 0.0;

    for result in reader.deserialize() {
        let record: LineItem = result?;
        let _not_used = record.description;
        total += record.usd;
    }

    println!("Total (USD): {total}");
    Ok(())
}

/// Program to sum the first column of a CSV file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the CSV file
    #[arg(long)]
    path: String,
}

#[derive(Debug, Deserialize)]
struct LineItem {
    #[serde(rename = "USD")]
    usd: f64,
    description: String,
}
