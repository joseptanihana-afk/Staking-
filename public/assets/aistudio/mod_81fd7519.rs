//! Staking account state structures

pub mod stake_account;
pub mod protection_config;

pub use stake_account::StakeAccount;
pub use stake_account::StakeState;
pub use protection_config::ProtectionConfig;
pub use protection_config::ProtectionTier;

use solana_program::pubkey::Pubkey;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{Pack, IsInitialized};
use serde::{Deserialize, Serialize};

/// Maximum stake amount in lamports (1,000,000 SOL)
pub const MAX_STAKE_LAMPORTS: u64 = 1_000_000_000_000_000;
/// Minimum stake amount in lamports (0.001 SOL)
pub const MIN_STAKE_LAMPORTS: u64 = 1_000_000;
/// Default lock-up period in epochs
pub const DEFAULT_LOCKUP_EPOCHS: u64 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(MIN_STAKE_LAMPORTS, 1_000_000);
        assert_eq!(MAX_STAKE_LAMPORTS, 1_000_000_000_000_000);
    }
}
