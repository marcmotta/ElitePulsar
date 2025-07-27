// src/main.rs
/*
 * Main executable for ElitePulsar
 */

use clap::Parser;
use elitepulsar::{Result, run};

#[derive(Parser)]
#[command(version, about = "ElitePulsar - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
