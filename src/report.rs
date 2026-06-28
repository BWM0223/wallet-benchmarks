//   Copyright 2024 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use serde::Serialize;
use sysinfo::System;
use crate::config::BenchConfig;
use crate::metrics::ScenarioMetrics;

#[derive(Debug, Serialize)]
pub struct ResultProfile {
    pub environment: Environment,
    pub config: ConfigSnapshot,
    pub scenarios: Vec<ScenarioMetrics>,
    pub computed_deltas: ComputedDeltas,
}

#[derive(Debug, Serialize)]
pub struct Environment {
    pub cpu_model: String,
    pub ram_gb: f64,
    pub os: String,
    pub disk_type: String,
    pub base_node_path: String,
    pub wallet_versions: WalletVersions,
}

#[derive(Debug, Serialize)]
pub struct WalletVersions {
    pub old_wallet: String,
    pub new_wallet: String,
    pub base_node: String,
}

#[derive(Debug, Serialize)]
pub struct ConfigSnapshot {
    pub a_fund: u64,
    pub c_min: u32,
    pub volume_target: u32,
    pub doubling_rounds: u32,
    pub fanout_outputs_per_tx: u32,
    pub concurrent_batches: Vec<u32>,
    pub s4_t_budget_secs: u64,
    pub s5_m: u32,
    pub s5_k: u32,
    pub fee_rate: u64,
}

#[derive(Debug, Serialize)]
pub struct ComputedDeltas {
    /// T_scan(S2) - T_scan(B0): cost of discovering 512 UTXOs
    pub scan_cost_s2_vs_b0_ms: Option<i64>,
    /// T_scan(S6) - T_scan(S2): cost of scanning S4+S5 history
    pub scan_cost_s6_vs_s2_ms: Option<i64>,
    /// T_scan(S6) / T_scan(B0): slowdown factor
    pub scan_slowdown_factor: Option<f64>,
    /// S5 throughput multiplier: T_individual / T_batch
    pub s5_throughput_multiplier: Option<f64>,
}

impl ResultProfile {
    pub fn from_metrics(cfg: &BenchConfig, scenarios: &[ScenarioMetrics]) -> anyhow::Result<Self> {
        let mut sys = System::new_all();
        sys.refresh_all();
        let cpu = sys.cpus().first().map(|c| c.brand().to_string()).unwrap_or_default();
        let ram = sys.total_memory() as f64 / 1_073_741_824.0;

        // Compute deltas — use mode-specific lookups so multi-mode runs
        // compare apples-to-apples. B0/S2/S6 come from the "new" mode
        // (or whichever is first), S5 batch vs individual uses explicit modes.
        let b0 = scenarios.iter().find(|s| s.scenario == "B0" && s.mode.contains("new"))
            .or_else(|| scenarios.iter().find(|s| s.scenario == "B0"));
        let s2 = scenarios.iter().find(|s| s.scenario == "S2" && s.mode.contains("new"))
            .or_else(|| scenarios.iter().find(|s| s.scenario == "S2"));
        let s6 = scenarios.iter().find(|s| s.scenario == "S6" && s.mode.contains("new"))
            .or_else(|| scenarios.iter().find(|s| s.scenario == "S6"));
        let s5_batch = scenarios.iter().find(|s| s.scenario == "S5" && s.mode.contains("payment"));
        let s5_indiv = scenarios.iter().find(|s| s.scenario == "S5" && s.mode.contains("new"));

        let deltas = ComputedDeltas {
            scan_cost_s2_vs_b0_ms: match (s2, b0) {
                (Some(a), Some(b)) => Some(a.wall_clock_ms as i64 - b.wall_clock_ms as i64),
                _ => None,
            },
            scan_cost_s6_vs_s2_ms: match (s6, s2) {
                (Some(a), Some(b)) => Some(a.wall_clock_ms as i64 - b.wall_clock_ms as i64),
                _ => None,
            },
            scan_slowdown_factor: match (s6, b0) {
                (Some(a), Some(b)) if b.wall_clock_ms > 0 =>
                    Some(a.wall_clock_ms as f64 / b.wall_clock_ms as f64),
                _ => None,
            },
            s5_throughput_multiplier: match (s5_indiv, s5_batch) {
                (Some(i), Some(b)) if b.wall_clock_ms > 0 =>
                    Some(i.wall_clock_ms as f64 / b.wall_clock_ms as f64),
                _ => None,
            },
        };

        Ok(Self {
            environment: Environment {
                cpu_model: cpu,
                ram_gb: ram,
                os: std::env::consts::OS.to_string(),
                disk_type: "unknown".to_string(),
                base_node_path: cfg.base_node_grpc.clone(),
                wallet_versions: WalletVersions {
                    old_wallet: cfg.old_wallet_binary.clone(),
                    new_wallet: cfg.new_wallet_binary.clone(),
                    base_node: "see config".to_string(),
                },
            },
            config: ConfigSnapshot {
                a_fund: cfg.a_fund,
                c_min: cfg.c_min,
                volume_target: cfg.volume_target,
                doubling_rounds: cfg.doubling_rounds,
                fanout_outputs_per_tx: cfg.fanout_outputs_per_tx,
                concurrent_batches: cfg.concurrent_batches.clone(),
                s4_t_budget_secs: cfg.s4_t_budget_secs,
                s5_m: cfg.s5_m,
                s5_k: cfg.s5_k,
                fee_rate: cfg.fee_rate,
            },
            scenarios: scenarios.to_vec(),
            computed_deltas: deltas,
        })
    }
}
