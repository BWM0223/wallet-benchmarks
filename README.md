# Tari Wallet Benchmarks

Reproducible performance test harness for minotari wallet modes.

## Quick Start

```bash
# Prerequisites: Rust 1.75+, access to Esmeralda testnet
cargo build --release

# Fund wallets on Esmeralda testnet (not part of measurement)
# Then run:
./target/release/tari-wallet-benchmarks --config config.toml --output results/baseline.json
```

## Wallet Modes

| Mode | Description |
|------|-------------|
| `old` | `minotari_console_wallet` via gRPC |
| `new` | `minotari-cli` library with offline signing |
| `payment-processor` | Batch 1-to-many transactions |

## Scenarios

| ID | Name | Purpose |
|----|------|---------|
| B0 | Baseline Scan | Floor cost of empty wallet scan |
| S0 | Funding Baseline | Establish starting state |
| S1 | UTXO Build-up | Doubling + fan-out to 512 UTXOs |
| S2 | Scan from Genesis (cp1) | Rediscover 512 UTXOs |
| S3 | Scan from Birthday (cp1) | Partial scan |
| S4 | Concurrent Construction | Measure contention under load |
| S5 | Batch vs Individual | Payment processor comparison |
| S6 | Scan from Genesis (cp2) | Post-S5 full scan |
| S7 | Scan from Birthday (cp2) | Post-S5 partial scan |

## Configuration

Edit `config.toml` — all parameters from the bounty spec are exposed.

## Output

Structured JSON result profile at `results/baseline.json` containing:
- Hardware/environment disclosure
- Pinned wallet/base-node versions
- All configuration parameters
- Per-scenario, per-mode metrics
- Computed deltas (scan cost, throughput multiplier)

## Principle

> The harness measures, does not engineer around wallet pain.

UTXO locking, selection contention, stalls, and failed tx construction under
concurrency are part of what the harness measures. No retries, backoff, or
throttling that hides actual wallet behavior.
