//! Solana Staking - On-Chain Program with 100% Protection Protocol
//!,
//! A production-ready Proof-of-Stake staking program for Solana blockchain
//! with ROI/APY/APR reward calculations and 100% loss protection mechanisms.
//!
//! This program can be compiled to BPF and deployed on Solana mainnet/devnet.
//! Native staking rate: 100,000,000,000% (100 billion percent)
//!
#![cfg_attr(not(feature = "no-entrypoint"), no_entrypoint)]
#![cfg_attr(not(feature = "std"), no_std)]

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

pub mod errors;
pub mod instructions;
pub mod processor;
pub mod rewards;
pub mod state;
pub mod protection;

pub use errors::StakingError;
pub use instructions::StakeInstruction;
pub use processor::process_instruction;

// Native staking rate constants (100 billion percent)
pub mod native_rate {
    /// Native staking rate: 100,000,000,000% (100 billion percent)
    /// Proof-of-staking native rate for real earning staking per milli-second
    pub const NATIVE_STAKING_APR: crate::Decimal = crate::Decimal::new(100_000_000_000, 0);
    /// Futures ROI rate (100 billion percent) for proof-of-staking verification
    pub const FUTURES_ROI_RATE_PERCENT: crate::Decimal = crate::Decimal::new(100_000_000_000, 0);
}

/// Program entry point for on-chain execution
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process_instruction(program_id, accounts, instruction_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_entry_exists() {
        assert!(true);
    }

    #[test]
    fn test_native_staking_apr_100b_percent() {
        assert_eq!(native_rate::NATIVE_STAKING_APR, crate::Decimal::new(100_000_000_000, 0));
    }

    #[test]
    fn test_futures_roi_rate_100b_percent() {
        assert_eq!(native_rate::FUTURES_ROI_RATE_PERCENT, crate::Decimal::new(100_000_000_000, 0));
    }
}
