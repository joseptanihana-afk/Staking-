//! Error types for the Solana Staking program
//!
use thiserror::Error;

//! All possible errors returned by the staking program
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum StakingError {
    #[error("Invalid instruction data")]
    InvalidInstructionData,
    #[error("Account not initialized")]
    AccountNotInitialized,
    #[error("Account already initialized")]
    AccountAlreadyInitialized,
    #[error("Unauthorized: signer is not the stake owner")]
    Unauthorized,
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Stake amount below minimum: {0} lamports")]
    StakeAmountTooLow(u64),
    #[error("Stake amount exceeds maximum: {0} lamports")]
    StakeAmountTooHigh(u64),
    #[error("Invalid protection tier")]
    InvalidProtectionTier,
    #[error("Protection already active")]
    ProtectionAlreadyActive,
    #[error("Protection not active")]
    ProtectionNotActive,
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
    #[error("Arithmetic underflow")]
    ArithmeticUnderflow,
    #[error("Invalid state transition")]
    InvalidStateTransition,
}

//! Result type alias for the staking program
pub type StakingResult<T> = core::result::Result<T, StakingError>;

impl From<StakingError> for solana_program::program_error::ProgramError {
    fn from(err: StakingError) -> Self {
        solana_program::program_error::ProgramError::Custom(err as u32)
    }
}
