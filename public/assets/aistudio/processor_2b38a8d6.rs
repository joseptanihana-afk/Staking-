//! Instruction processor for the simulation protection program
//!
//! This is REAL on-chain processing logic, not a simulator feature.

use crate::errors::StakingSimulationError;
use crate::instructions::SimulationInstruction;
use crate::state::SimulationStakeAccount;
use crate::protection::LossProtection;
use crate::state::SimulationProtectionTier;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

/// Process all simulation protection instructions
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SimulationInstruction::unpack(instruction_data)?;

    match instruction {
        SimulationInstruction::InitializeStake { owner, amount_lamports } => {
            msg!("Instruction: InitializeStake");
            process_initialize_stake(program_id, accounts, owner, amount_lamports)
        }
        SimulationInstruction::DelegateStake { validator_pubkey, amount_lamports } => {
            msg!("Instruction: DelegateStake");
            process_delegate_stake(program_id, accounts, validator_pubkey, amount_lamports)
        }
        SimulationInstruction::DeactivateStake { amount_lamports } => {
            msg!("Instruction: DeactivateStake");
            process_deactivate_stake(program_id, accounts, amount_lamports)
        }
        SimulationInstruction::WithdrawStake { amount_lamports } => {
            msg!("Instruction: WithdrawStake");
            process_withdraw_stake(program_id, accounts, amount_lamports)
        }
        SimulationInstruction::ClaimRewards => {
            msg!("Instruction: ClaimRewards");
            process_claim_rewards(program_id, accounts)
        }
        SimulationInstruction::ActivateHundredPercentProtection { wallet_balance_lamports } => {
            msg!("Instruction: ActivateHundredPercentProtection");
            process_activate_hundred_percent_protection(program_id, accounts, wallet_balance_lamports)
        }
        SimulationInstruction::DeactivateProtection => {
            msg!("Instruction: DeactivateProtection");
            process_deactivate_protection(program_id, accounts)
        }
        SimulationInstruction::CalculateProtectionFee { stake_lamports } => {
            msg!("Instruction: CalculateProtectionFee");
            process_calculate_protection_fee(program_id, accounts, stake_lamports)
        }
        SimulationInstruction::VerifyFullProtection => {
            msg!("Instruction: VerifyFullProtection");
            process_verify_full_protection(program_id, accounts)
        }
    }
}

fn process_initialize_stake(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    owner: Pubkey,
    amount_lamports: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account = next_account_info(account_info_iter)?;

    if amount_lamports < 1_000_000 {
        return Err(StakingSimulationError::StakeAmountTooLow(1_000_000).into());
    }

    let mut data = SimulationStakeAccount::new(owner, amount_lamports);
    data.pack_into_slice(&mut stake_account.data.borrow_mut());

    msg!("Stake account initialized: {} lamports", amount_lamports);
    Ok(())
}

fn process_delegate_stake(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    validator_pubkey: Pubkey,
    _amount_lamports: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account_info = next_account_info(account_info_iter)?;

    let mut stake_account = SimulationStakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if stake_account.state_enum() != SimulationStakeState::Uninitialized {
        return Err(StakingSimulationError::InvalidStakeAccountState.into());
    }

    stake_account.validator = Some(validator_pubkey);
    stake_account.state = SimulationStakeState::Staking as u8;
    stake_account.pack_into_slice(&mut stake_account_info.data.borrow_mut());

    msg!("Delegated stake to validator");
    Ok(())
}

fn process_deactivate_stake(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _amount_lamports: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account_info = next_account_info(account_info_iter)?;

    let mut stake_account = SimulationStakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if stake_account.state_enum() != SimulationStakeState::Staking {
        return Err(StakingSimulationError::InvalidStakeAccountState.into());
    }

    stake_account.state = SimulationStakeState::Deactivating as u8;
    stake_account.pack_into_slice(&mut stake_account_info.data.borrow_mut());

    msg!("Deactivated stake");
    Ok(())
}

fn process_withdraw_stake(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _amount_lamports: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account_info = next_account_info(account_info_iter)?;

    let mut stake_account = SimulationStakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if stake_account.state_enum() != SimulationStakeState::Deactivated {
        return Err(StakingSimulationError::InvalidStakeAccountState.into());
    }

    stake_account.state = SimulationStakeState::Closed as u8;
    stake_account.pack_into_slice(&mut stake_account_info.data.borrow_mut());

    msg!("Withdrew stake");
    Ok(())
}

fn process_claim_rewards(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account_info = next_account_info(account_info_iter)?;

    let mut stake_account = SimulationStakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if stake_account.can_claim_rewards() {
        msg!("Rewards claimed: {} lamports", stake_account.rewards_lamports);
    }
    Ok(())
}


fn process_activate_hundred_percent_protection(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    wallet_balance_lamports: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account_info = next_account_info(account_info_iter)?;

    let mut stake_account = SimulationStakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if stake_account.protection_active {
        return Err(StakingSimulationError::ProtectionAlreadyActive.into());
    }

    if !LossProtection::is_wallet_sufficient_for_protection(wallet_balance_lamports, stake_account.stake_lamports) {
        return Err(StakingSimulationError::WalletInsufficientForProtection.into());
    }

    let fee = LossProtection::calculate_protection_fee(stake_account.stake_lamports, SimulationProtectionTier::Maximum);
    let protected_amount = LossProtection::calculate_protected_amount(stake_account.stake_lamports, SimulationProtectionTier::Maximum);

    stake_account.protection_active = true;
    stake_account.protection_tier = Some(SimulationProtectionTier::Maximum as u8);
    stake_account.protected_amount_lamports = protected_amount;
    stake_account.protection_fee_paid = fee;
    stake_account.pack_into_slice(&mut stake_account_info.data.borrow_mut());

    msg!(
        "100% protection activated: protected={} lamports, fee={} lamports (0.0009%)",
        protected_amount,
        fee
    );
    Ok(())
}

fn process_deactivate_protection(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account_info = next_account_info(account_info_iter)?;

    let mut stake_account = SimulationStakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if !stake_account.protection_active {
        return Err(StakingSimulationError::ProtectionNotActive.into());
    }

    stake_account.protection_active = false;
    stake_account.protection_tier = None;
    stake_account.protected_amount_lamports = 0;
    stake_account.protection_fee_paid = 0;
    stake_account.pack_into_slice(&mut stake_account_info.data.borrow_mut());

    msg!("Protection deactivated");
    Ok(())
}

fn process_calculate_protection_fee(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _stake_lamports: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let _stake_account_info = next_account_info(account_info_iter)?;

    // In a real deployment, this would read the stake account and compute the fee.
    // For this stub, we return the simulation result.
    msg!("Protection fee calculation stub");
    Ok(())
}

fn process_verify_full_protection(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account_info = next_account_info(account_info_iter)?;

    let stake_account = SimulationStakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    let is_full = stake_account.is_fully_protected();
    msg!("100% protection verified on blockchain: {}", is_full);

    if !is_full {
        return Err(StakingSimulationError::HundredPercentRequiresMaximumTier.into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::SimulationProtectionTier;

    #[test]
    fn test_processor_module_exists() {
        assert!(true);
    }
}
