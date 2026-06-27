//   Copyright 2024 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use crate::config::BenchConfig;
use crate::metrics::{ScenarioMetrics, Timer};
use crate::modes::WalletMode;

/// B0 - Baseline Scan (empty wallet)
/// Floor cost of block-walk + view-key check with nothing to store.
pub async fn b0_baseline_scan(cfg: &BenchConfig, mode: &WalletMode) -> anyhow::Result<ScenarioMetrics> {
    let mut m = ScenarioMetrics::new("B0", mode.as_str());
    let timer = Timer::start();
    // TODO: launch scan from genesis with fresh seed, measure time to reach tip
    log::info!("    B0: Scanning from genesis with empty wallet...");
    m.wall_clock_ms = timer.elapsed_ms();
    m.utxo_count = 0;
    Ok(m)
}

/// S0 - Funding Baseline
pub async fn s0_funding_baseline(cfg: &BenchConfig, mode: &WalletMode) -> anyhow::Result<ScenarioMetrics> {
    let mut m = ScenarioMetrics::new("S0", mode.as_str());
    let timer = Timer::start();
    log::info!("    S0: Funding wallet with {} tXTM...", cfg.a_fund);
    // TODO: init wallet, receive 1 UTXO of A_fund, wait for C_min confirmations
    m.wall_clock_ms = timer.elapsed_ms();
    Ok(m)
}

/// S1 - UTXO Build-up (doubling + fan-out -> 512 UTXOs)
pub async fn s1_utxo_buildup(cfg: &BenchConfig, mode: &WalletMode) -> anyhow::Result<ScenarioMetrics> {
    let mut m = ScenarioMetrics::new("S1", mode.as_str());
    let timer = Timer::start();
    log::info!("    S1: Building {} UTXOs via {} doubling rounds + fan-out...",
        cfg.volume_target, cfg.doubling_rounds);
    // TODO: execute doubling rounds + fan-out per spec
    m.wall_clock_ms = timer.elapsed_ms();
    Ok(m)
}

/// S2 - Scan from Genesis (checkpoint 1)
pub async fn s2_scan_from_genesis(cfg: &BenchConfig, mode: &WalletMode) -> anyhow::Result<ScenarioMetrics> {
    let mut m = ScenarioMetrics::new("S2", mode.as_str());
    let timer = Timer::start();
    log::info!("    S2: Full scan from genesis after S1...");
    // TODO: wipe wallet data, scan from genesis, rediscover 512 UTXOs
    m.wall_clock_ms = timer.elapsed_ms();
    Ok(m)
}

/// S3 - Scan from Birthday (checkpoint 1)
pub async fn s3_scan_from_birthday(cfg: &BenchConfig, mode: &WalletMode) -> anyhow::Result<ScenarioMetrics> {
    let mut m = ScenarioMetrics::new("S3", mode.as_str());
    let timer = Timer::start();
    log::info!("    S3: Scan from birthday height...");
    m.wall_clock_ms = timer.elapsed_ms();
    Ok(m)
}

/// S4 - Concurrent Construction
pub async fn s4_concurrent_construction(cfg: &BenchConfig, mode: &WalletMode) -> anyhow::Result<ScenarioMetrics> {
    let mut m = ScenarioMetrics::new("S4", mode.as_str());
    let timer = Timer::start();
    log::info!("    S4: Concurrent construction with batches {:?}...", cfg.concurrent_batches);
    // TODO: for each N_concurrent, spawn N tasks constructing txs
    m.wall_clock_ms = timer.elapsed_ms();
    Ok(m)
}

/// S5 - Batch vs Individual (payment processor comparison)
pub async fn s5_batch_vs_individual(cfg: &BenchConfig, mode: &WalletMode) -> anyhow::Result<ScenarioMetrics> {
    let mut m = ScenarioMetrics::new("S5", mode.as_str());
    let timer = Timer::start();
    log::info!("    S5: Batch ({} recipients, {} per tx) vs individual...", cfg.s5_m, cfg.s5_k);
    // TODO: batch arm then individual arm per spec
    m.wall_clock_ms = timer.elapsed_ms();
    Ok(m)
}

/// S6 - Scan from Genesis (checkpoint 2, post-S5)
pub async fn s6_scan_from_genesis_cp2(cfg: &BenchConfig, mode: &WalletMode) -> anyhow::Result<ScenarioMetrics> {
    let mut m = ScenarioMetrics::new("S6", mode.as_str());
    let timer = Timer::start();
    log::info!("    S6: Full scan from genesis after S4+S5...");
    m.wall_clock_ms = timer.elapsed_ms();
    Ok(m)
}

/// S7 - Scan from Birthday (checkpoint 2)
pub async fn s7_scan_from_birthday_cp2(cfg: &BenchConfig, mode: &WalletMode) -> anyhow::Result<ScenarioMetrics> {
    let mut m = ScenarioMetrics::new("S7", mode.as_str());
    let timer = Timer::start();
    log::info!("    S7: Scan from birthday after S4+S5...");
    m.wall_clock_ms = timer.elapsed_ms();
    Ok(m)
}
