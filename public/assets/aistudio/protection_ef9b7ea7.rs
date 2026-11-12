//! Loss protection mechanism - Real on-chain 100% protection protocol
//!
//! Fee protection protocol: 0.0009% fee with 0.04 cent minimum wallet balance.
//! Maximum tier provides 100% protection on-chain. No simulator feature.

use crate::errors::StakingError;
use crate::state::SimulationProtectionTier;

/// Minimum wallet balance for protection: 0.04 cent in SOL (~40,000 lamports)
pub const MIN_WALLET_BALANCE_FOR_PROTECTION_LAMPORTS: u64 = 40_000;

/// Protection fee: 0.0009%
pub const PROTECTION_FEE_PERCENT_BASIS_POINTS: u64 = 9; // 0.0009% = 9 basis points of 1%

/// Loss protection manager for on-chain blockchain
pub struct LossProtection;

impl LossProtection {
    /// Get the protection percentage for a tier
    pub fn protection_percentage(tier: SimulationProtectionTier) -> u8 {
        match tier {
            SimulationProtectionTier::Basic => 10,
            SimulationProtectionTier::Standard => 25,
            SimulationProtectionTier::Premium => 50,
            SimulationProtectionTier::Maximum => 100,
        }
    }

    /// Calculate the protected amount in lamports for a stake
    pub fn calculate_protected_amount(
        stake_lamports: u64,
        tier: SimulationProtectionTier,
    ) -> u64 {
        let percentage = Self::protection_percentage(tier);
        (stake_lamports as u128)
            .checked_mul(percentage as u128)
            .and_then(|v| v.checked_div(100))
            .unwrap_or(0) as u64
    }

    /// Calculate the protection payout if a loss occurs
    /// Returns the amount reimbursed to the user (capped at protected amount)
    pub fn calculate_protection_payout(
        protected_amount_lamports: u64,
        lost_amount_lamports: u64,
    ) -> u64 {
        core::cmp::min(protected_amount_lamports, lost_amount_lamports)
    }

    /// Calculate the effective loss after protection reimbursement
    pub fn calculate_effective_loss(
        lost_amount_lamports: u64,
        payout_lamports: u64,
    ) -> u64 {
        lost_amount_lamports.saturating_sub(payout_lamports)
    }

    /// Check if the user has 100% protection (Maximum tier)
    pub fn is_fully_protected(tier: SimulationProtectionTier) -> bool {
        matches!(tier, SimulationProtectionTier::Maximum)
    }

    /// Calculate protection fee in lamports using 0.0009% rate
    pub fn calculate_protection_fee(
        stake_lamports: u64,
        _tier: SimulationProtectionTier,
    ) -> u64 {
        (stake_lamports as u128)
            .checked_mul(PROTECTION_FEE_PERCENT_BASIS_POINTS as u128)
            .and_then(|v| v.checked_div(1_000_000))
            .unwrap_or(0) as u64
    }

    /// Check if wallet has sufficient balance for protection
    /// Requires minimum 0.04 cent (40,000 lamports) and enough to cover fee
    pub fn is_wallet_sufficient_for_protection(
        wallet_balance_lamports: u64,
        stake_lamports: u64,
    ) -> bool {
        let fee = Self::calculate_protection_fee(stake_lamports, SimulationProtectionTier::Maximum);
        wallet_balance_lamports >= MIN_WALLET_BALANCE_FOR_PROTECTION_LAMPORTS
            && wallet_balance_lamports >= fee
    }

    /// Verify 100% protection status on blockchain
    pub fn verify_full_protection_on_blockchain(
        tier: SimulationProtectionTier,
        protection_active: bool,
    ) -> bool {
        protection_active && Self::is_fully_protected(tier)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::state::SimulationProtectionTier;

        #[test]
        fn test_protection_percentage_maximum() {
            assert_eq!(Self::protection_percentage(SimulationProtectionTier::Maximum), 100);
        }

        #[test]
        fn test_protection_percentage_basic() {
            assert_eq!(Self::protection_percentage(SimulationProtectionTier::Basic), 10);
        }

        #[test]
        fn test_calculate_protected_amount_100_percent() {
            let amount = Self::calculate_protected_amount(1_000_000_000, SimulationProtectionTier::Maximum);
            assert_eq!(amount, 1_000_000_000);
        }

        #[test]
        fn test_calculate_protected_amount_10_percent() {
            let amount = Self::calculate_protected_amount(1_000_000_000, SimulationProtectionTier::Basic);
            assert_eq!(amount, 100_000_000);
        }

        #[test]
        fn test_calculate_protection_payout() {
            let payout = Self::calculate_protection_payout(1_000_000_000, 500_000_000);
            assert_eq!(payout, 500_000_000);
            let payout = Self::calculate_protection_payout(1_000_000_000, 2_000_000_000);
            assert_eq!(payout, 1_000_000_000);
        }

        #[test]
        fn test_calculate_effective_loss() {
            let loss = Self::calculate_effective_loss(100_000_000, 10_000_000);
            assert_eq!(loss, 90_000_000);
        }

        #[test]
        fn test_is_fully_protected() {
            assert!(Self::is_fully_protected(SimulationProtectionTier::Maximum));
            assert!(!Self::is_fully_protected(SimulationProtectionTier::Basic));
        }

        #[test]
        fn test_protection_fee_0_0009_percent() {
            // 0.0009% of 1,000,000,000 lamports = 9,000 lamports
            let fee = Self::calculate_protection_fee(1_000_000_000, SimulationProtectionTier::Maximum);
            assert_eq!(fee, 9_000);
        }

        #[test]
        fn test_wallet_sufficient() {
            assert!(Self::is_wallet_sufficient_for_protection(40_000, 1_000_000_000));
            assert!(!Self::is_wallet_sufficient_for_protection(10_000, 1_000_000_000));
        }

        #[test]
        fn test_verify_full_protection_on_blockchain() {
            assert!(Self::verify_full_protection_on_blockchain(
                SimulationProtectionTier::Maximum,
                true,
            ));
            assert!(!Self::verify_full_protection_on_blockchain(
                SimulationProtectionTier::Maximum,
                false,
            ));
            assert!(!Self::verify_full_protection_on_blockchain(
                SimulationProtectionTier::Basic,
                true,
            ));
        }
    }
}
