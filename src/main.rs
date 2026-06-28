//   Copyright 2024 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use clap::Parser;
use std::path::PathBuf;

mod config;
mod harness;
mod metrics;
mod modes;
mod scenarios;
mod report;

#[derive(Parser, Debug)]
#[command(name = "tari-wallet-benchmarks")]
#[command(about = "Reproducible performance test harness for minotari wallet modes")]
struct Cli {
    /// Path to configuration file
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,

    /// Wallet mode to benchmark: old, new, payment-processor, or all
    #[arg(short, long, default_value = "all")]
    mode: String,

    /// Specific scenario to run (B0, S0-S7) or all
    #[arg(short, long, default_value = "all")]
    scenario: String,

    /// Output file for the result profile (JSON)
    #[arg(short, long, default_value = "results/baseline.json")]
    output: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    log::info!("Loading configuration from {:?}", cli.config);
    let cfg = config::BenchConfig::load(&cli.config)?;

    log::info!("Initializing harness...");
    let mut harness = harness::BenchHarness::new(cfg).await?;

    let modes = match cli.mode.as_str() {
        "old"               => vec![modes::WalletMode::Old],
        "new"               => vec![modes::WalletMode::New],
        "payment-processor" => vec![modes::WalletMode::PaymentProcessor],
        "all" => vec![
            modes::WalletMode::Old,
            modes::WalletMode::New,
            modes::WalletMode::PaymentProcessor,
        ],
        unknown => {
            eprintln!(
                "ERROR: unknown mode {:?}. Valid options: old, new, payment-processor, all",
                unknown
            );
            std::process::exit(1);
        }
    };

    for mode in &modes {
        log::info!("Running benchmarks for {:?} mode", mode);
        harness.run_scenarios(mode, &cli.scenario).await?;
    }

    log::info!("Generating result profile...");
    harness.write_report(&cli.output)?;

    log::info!("Benchmark complete. Results written to {:?}", cli.output);
    Ok(())
}
