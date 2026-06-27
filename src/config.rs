//   Copyright 2024 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use serde::Deserialize;
use std::path::Path;

/// All tunable benchmark parameters as specified in the bounty.
#[derive(Debug, Clone, Deserialize)]
pub struct BenchConfig {
    /// Single funding UTXO per mode (default: 10,000 tXTM)
    #[serde(default = "default_a_fund")]
    pub a_fund: u64,
    /// Confirmation depth (default: 3)
    #[serde(default = "default_c_min")]
    pub c_min: u32,
    /// Target UTXO count after S1 (default: 512)
    #[serde(default = "default_volume_target")]
    pub volume_target: u32,
    /// Doubling rounds (default: 6) -> 1->64 UTXOs
    #[serde(default = "default_doubling_rounds")]
    pub doubling_rounds: u32,
    /// Outputs per fan-out tx (default: 8)
    #[serde(default = "default_fanout_outputs")]
    pub fanout_outputs_per_tx: u32,
    /// Concurrent batch sizes for S4 (default: [8,16,32,64,128])
    #[serde(default = "default_concurrent_batches")]
    pub concurrent_batches: Vec<u32>,
    /// S4 time budget per batch in seconds (default: 900 = 15 min)
    #[serde(default = "default_s4_budget")]
    pub s4_t_budget_secs: u64,
    /// S5 total recipients (default: 100)
    #[serde(default = "default_s5_m")]
    pub s5_m: u32,
    /// S5 outputs per batch tx (default: 10)
    #[serde(default = "default_s5_k")]
    pub s5_k: u32,
    /// Explicit fee rate
    #[serde(default = "default_fee_rate")]
    pub fee_rate: u64,
    /// Base node gRPC address
    #[serde(default = "default_base_node")]
    pub base_node_grpc: String,
    /// Old wallet binary path
    #[serde(default)]
    pub old_wallet_binary: String,
    /// New wallet (minotari-cli) binary path
    #[serde(default)]
    pub new_wallet_binary: String,
    /// Wallet data directory
    #[serde(default = "default_data_dir")]
    pub data_dir: String,
}

fn default_a_fund() -> u64 { 10_000_000_000 } // 10,000 tXTM in microtari
fn default_c_min() -> u32 { 3 }
fn default_volume_target() -> u32 { 512 }
fn default_doubling_rounds() -> u32 { 6 }
fn default_fanout_outputs() -> u32 { 8 }
fn default_concurrent_batches() -> Vec<u32> { vec![8, 16, 32, 64, 128] }
fn default_s4_budget() -> u64 { 900 }
fn default_s5_m() -> u32 { 100 }
fn default_s5_k() -> u32 { 10 }
fn default_fee_rate() -> u64 { 25 }
fn default_base_node() -> String { "http://127.0.0.1:18142".into() }
fn default_data_dir() -> String { "./wallet-data".into() }

impl BenchConfig {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let cfg: Self = toml::from_str(&content)?;
        Ok(cfg)
    }
}
