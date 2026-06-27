//   Copyright 2024 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::{Duration, Instant};

/// Per-transaction metrics.
#[derive(Debug, Clone, Serialize)]
pub struct TxMetrics {
    pub tx_id: String,
    pub construction_time_ms: u64,
    pub broadcast_to_mempool_ms: Option<u64>,
    pub broadcast_to_confirmed_ms: Option<u64>,
    pub fee_paid: u64,
    pub outcome: TxOutcome,
}

#[derive(Debug, Clone, Serialize)]
pub enum TxOutcome {
    Success,
    Rejected(String),
    Timeout,
    Stalled(String),
}

/// Per-scenario metrics.
#[derive(Debug, Clone, Serialize)]
pub struct ScenarioMetrics {
    pub scenario: String,
    pub mode: String,
    pub wall_clock_ms: u64,
    pub total_fees: u64,
    pub success_count: u32,
    pub failure_count: u32,
    pub balance_delta: i64,
    pub utxo_count: u32,
    pub height_start: u64,
    pub height_end: u64,
    pub peak_rss_mb: f64,
    pub peak_cpu_percent: f64,
    pub blocks_per_sec: Option<f64>,
    pub outputs_found: Option<u32>,
    pub tx_metrics: Vec<TxMetrics>,
    pub started_at: DateTime<Utc>,
    pub completed_at: DateTime<Utc>,
}

impl ScenarioMetrics {
    pub fn new(scenario: &str, mode: &str) -> Self {
        Self {
            scenario: scenario.to_string(),
            mode: mode.to_string(),
            wall_clock_ms: 0,
            total_fees: 0,
            success_count: 0,
            failure_count: 0,
            balance_delta: 0,
            utxo_count: 0,
            height_start: 0,
            height_end: 0,
            peak_rss_mb: 0.0,
            peak_cpu_percent: 0.0,
            blocks_per_sec: None,
            outputs_found: None,
            tx_metrics: Vec::new(),
            started_at: Utc::now(),
            completed_at: Utc::now(),
        }
    }
}

/// Timer utility for wall-clock measurement.
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn start() -> Self { Self { start: Instant::now() } }
    pub fn elapsed_ms(&self) -> u64 { self.start.elapsed().as_millis() as u64 }
}
