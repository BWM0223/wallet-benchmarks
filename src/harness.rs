//   Copyright 2024 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use std::path::Path;
use crate::config::BenchConfig;
use crate::metrics::ScenarioMetrics;
use crate::modes::WalletMode;
use crate::report::ResultProfile;
use crate::scenarios;

/// Orchestrates benchmark runs across all modes and scenarios.
pub struct BenchHarness {
    pub config: BenchConfig,
    pub results: Vec<ScenarioMetrics>,
}

impl BenchHarness {
    pub async fn new(config: BenchConfig) -> anyhow::Result<Self> {
        Ok(Self { config, results: Vec::new() })
    }

    /// Run the specified scenarios for the given wallet mode.
    pub async fn run_scenarios(&mut self, mode: &WalletMode, filter: &str) -> anyhow::Result<()> {
        let scenarios_to_run = match filter {
            "all" => vec!["B0","S0","S1","S2","S3","S4","S5","S6","S7"],
            s => vec![s],
        };

        for scenario_id in scenarios_to_run {
            log::info!("  Running scenario {} for mode {:?}", scenario_id, mode);
            let mut metrics = match scenario_id {
                "B0" => scenarios::b0_baseline_scan(&self.config, mode).await?,
                "S0" => scenarios::s0_funding_baseline(&self.config, mode).await?,
                "S1" => scenarios::s1_utxo_buildup(&self.config, mode).await?,
                "S2" => scenarios::s2_scan_from_genesis(&self.config, mode).await?,
                "S3" => scenarios::s3_scan_from_birthday(&self.config, mode).await?,
                "S4" => scenarios::s4_concurrent_construction(&self.config, mode).await?,
                "S5" => scenarios::s5_batch_vs_individual(&self.config, mode).await?,
                "S6" => scenarios::s6_scan_from_genesis_cp2(&self.config, mode).await?,
                "S7" => scenarios::s7_scan_from_birthday_cp2(&self.config, mode).await?,
                _ => {
                    log::warn!("Unknown scenario: {}", scenario_id);
                    continue;
                }
            };
            metrics.complete();
            self.results.push(metrics);
        }
        }
        Ok(())
    }

    /// Write the structured result profile to disk.
    pub fn write_report(&self, path: &Path) -> anyhow::Result<()> {
        let profile = ResultProfile::from_metrics(&self.config, &self.results)?;
        let json = serde_json::to_string_pretty(&profile)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, json)?;
        Ok(())
    }
}
