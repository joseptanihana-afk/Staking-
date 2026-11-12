//! Error types for the Solana Staking Simulation Protection Protocol

use thiserror::Error;

/// All possible errors returned by the simulation protection program
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum StakingSimulationError {
    #[error("Invalid stake account state")]
    InvalidStakeAccountState,
    #[error("Insufficient SOL balance for staking")]
    InsufficientBalance,
    #[error("Stake amount below minimum: {0} lamports")]
    StakeAmountTooLow(u64),
    #[error("Stake amount exceeds maximum: {0} lamports")]
    StakeAmountTooHigh(u64),
    #[error("Staking period not yet started")]
    StakingPeriodNotStarted,
    #[error("Staking period already ended")]
    StakingPeriodEnded,
    #[error("Invalid reward rate: {0}")]
    InvalidRewardRate(String),
    #[error("Protection tier not found")]
    ProtectionTierNotFound,
    #[error("Loss protection already active")]
    ProtectionAlreadyActive,
    #[error("Loss protection not active")]
    ProtectionNotActive,
    #[error("Cannot withdraw: protection lock period active")]
    ProtectionLockActive,
    #[error("Arithmetic overflow")]
    ArithmeticOverflow,
    #[error("Arithmetic underflow")]
    ArithmeticUnderflow,
    #[error("Invalid protection configuration: {0}")]
    InvalidProtectionConfig(String),
    #[error("Unauthorized: signer is not the stake owner")]
    Unauthorized,
    #[error("Native staking account not found")]
    NativeStakingAccountNotFound,
    #[error("Reward calculation error: {0}")]
    RewardCalculationError(String),
    #[error("Invalid instruction data")]
    InvalidInstructionData,
    #[error("Account not initialized")]
    AccountNotInitialized,
    #[error("Account already initialized")]
    AccountAlreadyInitialized,
    #[error("Wallet balance insufficient for 100% protection")]
    WalletInsufficientForProtection,
    #[error("100% protection requires Maximum tier")]
    HundredPercentRequiresMaximumTier,
}

impl From<StakingSimulationError> for solana_program::program_error::ProgramError {
    fn from(err: StakingSimulationError) -> Self {
        solana_program::program_error::ProgramError::Custom(err as u32)
    }
}

/// Result type alias for the simulation protection program
pub type StakingSimulationResult<T> = core::result::Result<T, StakingSimulationError>;
