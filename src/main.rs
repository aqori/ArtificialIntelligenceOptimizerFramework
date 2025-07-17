// src/main.rs
/*
 * Main executable for ArtificialIntelligenceOptimizerFramework
 */

use clap::Parser;
use artificialintelligenceoptimizerframework::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialIntelligenceOptimizerFramework - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
