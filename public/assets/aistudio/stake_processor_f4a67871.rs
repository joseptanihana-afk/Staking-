//! Instruction processing logic for the staking program

use crate::errors::StakingProtectError;
use crate::instructions::StakeInstruction;
use crate::state::{StakeAccount, StakeState, MAX_STAKE_LAMPORTS, MIN_STAKE_LAMPORTS};
use crate::protection::{self, ProtectionTier};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::ProgramError,
    pubkey::Pubkey,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StakeInstruction::unpack(instruction_data).map_err(|e| {
        msg!("Invalid instruction data: {:?}", e);
        e
    })?;

    match instruction {
        StakeInstruction::Initialize { owner, amount_lamports } => {
            msg!("Instruction: Initialize");
            process_initialize_stake(program_id, accounts, owner, amount_lamports)
        }
        StakeInstruction::Delegate { validator_pubkey, amount_lamports } => {
            msg!("Instruction: Delegate");
            process_delegate_stake(program_id, accounts, validator_pubkey, amount_lamports)
        }
        StakeInstruction::Deactivate { amount_lamports } => {
            msg!("Instruction: Deactivate");
            process_deactivate_stake(program_id, accounts, amount_lamports)
        }
        StakeInstruction::Withdraw { amount_lamports } => {
            msg!("Instruction: Withdraw");
            process_withdraw_stake(program_id, accounts, amount_lamports)
        }
        StakeInstruction::ClaimRewards => {
            msg!("Instruction: ClaimRewards");
            process_claim_rewards(program_id, accounts)
        }
        StakeInstruction::ActivateProtection { tier } => {
            msg!("Instruction: ActivateProtection");
            process_activate_protection(program_id, accounts, tier)
        }
        StakeInstruction::DeactivateProtection => {
            msg!("Instruction: DeactivateProtection");
            process_deactivate_protection(program_id, accounts)
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
    let stake_account_info = next_account_info(account_info_iter)?;

    if amount_lamports < MIN_STAKE_LAMPORTS {
        return Err(StakingProtectError::StakeAmountTooLow(MIN_STAKE_LAMPORTS).into());
    }
    if amount_lamports > MAX_STAKE_LAMPORTS {
        return Err(StakingProtectError::StakeAmountTooHigh(MAX_STAKE_LAMPORTS).into());
    }

    let mut stake_account = StakeAccount::new(owner, amount_lamports);
    stake_account.state = StakeState::Staking as u8;
    stake_account.pack_into_slice(&mut stake_account_info.data.borrow_mut());

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

    let mut stake_account = StakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if stake_account.state != StakeState::Staking as u8 {
        return Err(StakingProtectError::InvalidStateTransition.into());
    }

    stake_account.validator = Some(validator_pubkey);
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

    let mut stake_account = StakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if stake_account.state != StakeState::Staking as u8 {
        return Err(StakingProtectError::InvalidStateTransition.into());
    }

    stake_account.state = StakeState::Deactivating as u8;
    stake_account.deactivation_epoch = Some(1);
    stake_account.pack_into_slice(&mut stake_account_info.data.borrow_mut());

    msg!("Deactivate stake");
    Ok(())
}

fn process_withdraw_stake(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _amount_lamports: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account_info = next_account_info(account_info_iter)?;

    let mut stake_account = StakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if stake_account.state_enum() != StakeState::Deactivated {
        return Err(StakingProtectError::InvalidStateTransition.into());
    }

    stake_account.state = StakeState::Closed as u8;
    stake_account.pack_into_slice(&mut stake_account_info.data.borrow_mut());

    msg!("Withdraw stake");
    Ok(())
}

fn process_claim_rewards(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account_info = next_account_info(account_info_iter)?;

    let stake_account = StakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if !stake_account.can_claim_rewards() {
        return Err(StakingProtectError::InvalidInstructionData.into());
    }

    msg!("Rewards claimed: {} lamports", stake_account.rewards_lamports);
    Ok(())
}

fn process_activate_protection(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    tier: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account_info = next_account_info(account_info_iter)?;

    let mut stake_account = StakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if stake_account.protection_active {
        return Err(StakingProtectError::ProtectionAlreadyActive.into());
    }

    let protection_tier = ProtectionTier::from_u8(tier)
        .ok_or(StakingProtectError::InvalidProtectionTier)?;

    if !protection::ProtectionLogic::is_wallet_sufficient_for_protection(
        stake_account_info.lamports(),
        stake_account.stake_lamports,
    ) {
        return Err(StakingProtectError::InsufficientBalance.into());
    }

    let protected_amount = protection::ProtectionLogic::calculate_protected_amount(
        stake_account.stake_lamports,
        protection_tier,
    );

    stake_account.protection_active = true;
    stake_account.protection_tier = Some(protection_tier as u8);
    stake_account.protected_amount_lamports = protected_amount;
    stake_account.pack_into_slice(&mut stake_account_info.data.borrow_mut());

    msg!(
        "Protection activated: tier={}, protected={} lamports, fee={} lamports",
        protection_tier as u8,
        protected_amount,
        protection::ProtectionLogic::calculate_protection_fee(stake_account.stake_lamports, protection_tier)
    );
    Ok(())
}

fn process_deactivate_protection(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let stake_account_info = next_account_info(account_info_iter)?;

    let mut stake_account = StakeAccount::unpack_from_slice(&stake_account_info.data.borrow())?;

    if !stake_account.protection_active {
        return Err(StakingProtectError::ProtectionNotActive.into());
    }

    stake_account.protection_active = false;
    stake_account.protection_tier = None;
    stake_account.protected_amount_lamports = 0;
    stake_account.pack_into_slice(&mut stake_account_info.data.borrow_mut());

    msg!("Protection deactivated");
    Ok(())
}
