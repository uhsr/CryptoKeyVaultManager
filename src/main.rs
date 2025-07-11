// src/main.rs
/*
 * Main executable for CryptoKeyVaultManager
 */

use clap::Parser;
use cryptokeyvaultmanager::{Result, run};

#[derive(Parser)]
#[command(version, about = "CryptoKeyVaultManager - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
