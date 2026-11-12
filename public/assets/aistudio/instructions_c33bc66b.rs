//! Staking instruction definitions
//!
use solana_program::pubkey::Pubkey;
use serde::{Deserialize, Serialize};

//! All staking instructions supported by the program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StakeInstruction {
    Initialize {
        owner: Pubkey,
        amount_lamports: u64,
    },
    Delegate {
        validator_pubkey: Pubkey,
        amount_lamports: u64,
    },
    Deactivate {
        amount_lamports: u64,
    },
    Withdraw {
        amount_lamports: u64,
    },
    ClaimRewards,
    ActivateProtection {
        tier: u8,
    },
    DeactivateProtection,
}

impl StakeInstruction {
    /// Unpack instruction data from bytes
    pub fn unpack(input: &[u8]) -> Result<Self, StakingError> {
        if input.is_empty() {
            return Err(StakingError::InvalidInstructionData);
        }
        let tag = input[0];
        let rest = &input[1..];
        match tag {
            0 => {
                if rest.len() < 32 + 8 {
                    return Err(StakingError::InvalidInstructionData);
                }
                let owner = Pubkey::try_from(rest[..32].try_into().map_err(|_| StakingError::InvalidInstructionData)?)
                    .map_err(|_| StakingError::InvalidInstructionData)?;
                let amount = u64::from_le_bytes(rest[32..40].try_into().map_err(|_| StakingError::InvalidInstructionData)?);
                Ok(StakeInstruction::Initialize { owner, amount_lamports: amount })
            }
            1 => {
                if rest.len() < 32 + 8 {
                    return Err(StakingError::InvalidInstructionData);
                }
                let validator = Pubkey::try_from(rest[..32].try_into().map_err(|_| StakingError::InvalidInstructionData)?)
                    .map_err(|_| StakingError::InvalidInstructionData)?;
                let amount = u64::from_le_bytes(rest[32..40].try_into().map_err(|_| StakingError::InvalidInstructionData)?);
                Ok(StakeInstruction::Delegate { validator_pubkey: validator, amount_lamports: amount })
            }
            2 => {
                if rest.len() < 8 {
                    return Err(StakingError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[..8].try_into().map_err(|_| StakingError::InvalidInstructionData)?);
                Ok(StakeInstruction::Deactivate { amount_lamports: amount })
            }
            3 => {
                if rest.len() < 8 {
                    return Err(StakingError::InvalidInstructionData);
                }
                let amount = u64::from_le_bytes(rest[..8].try_into().map_err(|_| StakingError::InvalidInstructionData)?);
                Ok(StakeInstruction::Withdraw { amount_lamports: amount })
            }
            4 => Ok(StakeInstruction::ClaimRewards),
            5 => {
                if rest.is_empty() {
                    return Err(StakingError::InvalidInstructionData);
                }
                Ok(StakeInstruction::ActivateProtection { tier: rest[0] })
            }
            6 => Ok(StakeInstruction::DeactivateProtection),
            _ => Err(StakingError::InvalidInstructionData),
        }
    }

    /// Pack instruction into bytes
    pub fn pack(&self) -> Vec<u8> {
        let mut data = Vec::new();
        match self {
            StakeInstruction::Initialize { owner, amount_lamports } => {
                data.push(0);
                data.extend_from_slice(owner.as_ref());
                data.extend_from_slice(&amount_lamports.to_le_bytes());
            }
            StakeInstruction::Delegate { validator_pubkey, amount_lamports } => {
                data.push(1);
                data.extend_from_slice(validator_pubkey.as_ref());
                data.extend_from_slice(&amount_lamports.to_le_bytes());
            }
            StakeInstruction::Deactivate { amount_lamports } => {
                data.push(2);
                data.extend_from_slice(&amount_lamports.to_le_bytes());
            }
            StakeInstruction::Withdraw { amount_lamports } => {
                data.push(3);
                data.extend_from_slice(&amount_lamports.to_le_bytes());
            }
            StakeInstruction::ClaimRewards => {
                data.push(4);
            }
            StakeInstruction::ActivateProtection { tier } => {
                data.push(5);
                data.push(*tier);
            }
            StakeInstruction::DeactivateProtection => {
                data.push(6);
            }
        }
        data
    }
}
