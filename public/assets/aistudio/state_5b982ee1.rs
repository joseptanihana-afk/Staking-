//! Simulation stake account state - Real on-chain state structures

use solana_program::pubkey::Pubkey;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{Pack, IsInitialized};
use serde::{Deserialize, Serialize};

//! State of a simulation stake account
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationStakeState {
    Uninitialized,
    Staking,
    Deactivating,
    Deactivated,
    Closed,
}

/// Protection tier for simulation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationProtectionTier {
    Basic = 1,
    Standard = 2,
    Premium = 3,
    Maximum = 4,
}

//! Simulation stake account state structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationStakeAccount {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub validator: Option<Pubkey>,
    pub stake_lamports: u64,
    pub rewards_lamports: u64,
    pub state: SimulationStakeState,
    pub activation_epoch: Option<u64>,
    pub deactivation_epoch: Option<u64>,
    pub lockup_epochs: u64,
    pub protection_active: bool,
    pub protection_tier: Option<SimulationProtectionTier>,
    pub protected_amount_lamports: u64,
    pub protection_fee_paid: u64,
}

impl Pack for SimulationStakeAccount {
    const LEN: usize = 1 + 32 + 33 + 8 + 8 + 1 + 9 + 9 + 8 + 1 + 2 + 8 + 8;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut offset = 0;
        dst[offset] = self.is_initialized as u8;
        offset += 1;
        dst[offset..offset + 32].copy_from_slice(self.owner.as_ref());
        offset += 32;
        match self.validator {
            Some(v) => {
                dst[offset] = 1;
                offset += 1;
                dst[offset..offset + 32].copy_from_slice(v.as_ref());
                offset += 32;
            }
            None => {
                dst[offset] = 0;
                offset += 1;
            }
        }
        dst[offset..offset + 8].copy_from_slice(&self.stake_lamports.to_le_bytes());
        offset += 8;
        dst[offset..offset + 8].copy_from_slice(&self.rewards_lamports.to_le_bytes());
        offset += 8;
        dst[offset] = self.state as u8;
        offset += 1;
        match self.activation_epoch {
            Some(e) => {
                dst[offset] = 1;
                offset += 1;
                dst[offset..offset + 8].copy_from_slice(&e.to_le_bytes());
                offset += 8;
            }
            None => {
                dst[offset] = 0;
                offset += 1;
            }
        }
        match self.deactivation_epoch {
            Some(e) => {
                dst[offset] = 1;
                offset += 1;
                dst[offset..offset + 8].copy_from_slice(&e.to_le_bytes());
                offset += 8;
            }
            None => {
                dst[offset] = 0;
                offset += 1;
            }
        }
        dst[offset..offset + 8].copy_from_slice(&self.lockup_epochs.to_le_bytes());
        offset += 8;
        dst[offset] = self.protection_active as u8;
        offset += 1;
        dst[offset] = self.protection_tier.map_or(0, |t| t as u8);
        offset += 1;
        dst[offset..offset + 8].copy_from_slice(&self.protected_amount_lamports.to_le_bytes());
        offset += 8;
        dst[offset..offset + 8].copy_from_slice(&self.protection_fee_paid.to_le_bytes());
    }
}

impl SimulationStakeAccount {
    pub fn new(owner: Pubkey, stake_lamports: u64) -> Self {
        Self {
            is_initialized: true,
            owner,
            validator: None,
            stake_lamports,
            rewards_lamports: 0,
            state: SimulationStakeState::Staking,
            activation_epoch: None,
            deactivation_epoch: None,
            lockup_epochs: 0,
            protection_active: false,
            protection_tier: None,
            protected_amount_lamports: 0,
            protection_fee_paid: 0,
        }
    }

    pub fn can_claim_rewards(&self) -> bool {
        self.rewards_lamports > 0 && self.state == SimulationStakeState::Deactivated
    }

    pub fn is_fully_protected(&self) -> bool {
        self.protection_active && self.protection_tier == Some(SimulationProtectionTier::Maximum)
    }

    pub fn protection_percentage(&self) -> u8 {
        self.protection_tier
            .map_or(0, |t| match t {
                SimulationProtectionTier::Basic => 10,
                SimulationProtectionTier::Standard => 25,
                SimulationProtectionTier::Premium => 50,
                SimulationProtectionTier::Maximum => 100,
            })
    }
}

impl IsInitialized for SimulationStakeAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
