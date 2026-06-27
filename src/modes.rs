//   Copyright 2024 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

/// Wallet modes under test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletMode {
    /// Old wallet (minotari_console_wallet) via gRPC
    Old,
    /// New wallet (minotari-cli library) with offline signing
    New,
    /// Payment processor (batch 1-to-many via minotari-cli)
    PaymentProcessor,
}

impl WalletMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Old => "old_wallet",
            Self::New => "new_wallet",
            Self::PaymentProcessor => "payment_processor",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::Old, Self::New, Self::PaymentProcessor]
    }
}

impl std::fmt::Display for WalletMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
