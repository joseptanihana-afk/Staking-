//! Instruction processor for the staking program

pub mod stake_processor;

pub use stake_processor::process_instruction;

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    stake_processor::process_instruction(program_id, accounts, instruction_data)
}
