//! Stake account state for on-chain program

use crate::state::{ProtectionConfig, ProtectionTier};
use solana_program::pubkey::Pubkey;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{Pack, IsInitialized};
use serde::{Deserialize, Serialize};

/// State of a stake account
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StakeState {
    Uninitialized,
    Staking,
    Deactivating,
    Deactivated,
    Closed,
}

/// A stake account representing a user's staked SOL on-chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeAccount {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub validator: Option<Pubkey>,
    pub stake_lamports: u64,
    pub rewards_lamports: u64,
    pub state: u8,
    pub activation_epoch: Option<u64>,
    pub deactivation_epoch: Option<u64>,
    pub lockup_epochs: u64,
    pub protection_active: bool,
    pub protection_tier: Option<u8>,
    pub protected_amount_lamports: u64,
}

impl StakeAccount {
    pub fn new(owner: Pubkey, amount_lamports: u64) -> Self {
        Self {
            is_initialized: true,
            owner,
            validator: None,
            stake_lamports: amount_lamports,
            rewards_lamports: 0,
            state: StakeState::Uninitialized as u8,
            activation_epoch: None,
            deactivation_epoch: None,
            lockup_epochs: crate::state::DEFAULT_LOCKUP_EPOCHS,
            protection_active: false,
            protection_tier: None,
            protected_amount_lamports: 0,
        }
    }

    pub fn state_enum(&self) -> StakeState {
        match self.state {
            0 => StakeState::Uninitialized,
            1 => StakeState::Staking,
            2 => StakeState::Deactivating,
            3 => StakeState::Deactivated,
            4 => StakeState::Closed,
            _ => StakeState::Uninitialized,
        }
    }

    pub fn total_value(&self) -> u64 {
        self.stake_lamports.saturating_add(self.rewards_lamports)
    }

    pub fn is_staking(&self) -> bool {
        self.state == StakeState::Staking as u8
    }

    pub fn can_claim_rewards(&self) -> bool {
        self.rewards_lamports > 0
    }
}
