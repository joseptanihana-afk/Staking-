//! ROI / APY / APR / FuturesROI reward calculations for the simulation protection protocol
//!
//! Supports the native staking rate, futures ROI, and per-millisecond earnings
//! calculations that are used both on-chain and in simulation tooling.

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

/// Minimum reward rate
pub const MIN_REWARD_RATE: Decimal = Decimal::new(0, 0);

/// Native staking rate: 100,000,000,000% (100 billion percent)
/// Proof-of-staking native rate for real earning staking per milli-second
pub const NATIVE_STAKING_APR: Decimal = Decimal::new(100_000_000_000, 0);

/// Futures ROI rate (100 billion percent)
pub const FUTURES_ROI_RATE_PERCENT: Decimal = Decimal::new(100_000_000_000, 0);

/// Real earning reward rate per millisecond
pub const REAL_EARNING_PER_MILLISECOND_PPM: Decimal = Decimal::new(9, 5); // 0.0000099% per millisecond

/// Maximum reward rate
pub const MAX_REWARD_RATE: Decimal = Decimal::new(100_000_000_000, 0);

/// Default reward calculation method
pub enum RewardCalculationMethod {
    NativeStaking,
    FuturesROI,
    Hybrid,
}

/// Calculate ROI for a given stake
pub fn calculate_roi(
    stake_lamports: u64,
    reward_rate: Decimal,
    period_seconds: u64,
) -> Result<Decimal, String> {
    if reward_rate < MIN_REWARD_RATE || reward_rate > MAX_REWARD_RATE {
        return Err("Invalid reward rate".to_string());
    }
    if stake_lamports == 0 {
        return Err("Stake amount must be greater than zero".to_string());
    }

    let rate_per_period = reward_rate.checked_div(Decimal::new(period_seconds as u32, 0))
        .ok_or("Invalid reward rate")?;
    Ok(rate_per_period)
}

/// Calculate APY (annual percentage yield)
pub fn calculate_apy(reward_rate: Decimal, compounding_periods: u32) -> Result<Decimal, String> {
    if compounding_periods == 0 {
        return Err("Compounding periods must be greater than zero".to_string());
    }
    let one = Decimal::new(1, 0);
    let compounded = one + reward_rate.checked_div(Decimal::new(compounding_periods as u32, 0))
        .ok_or("Invalid reward rate")?;
    let apy = compounded.powv(compounding_periods) - one;
    Ok(apy)
}

/// Calculate APR (annual percentage rate)
pub fn calculate_apr(reward_rate: Decimal) -> Result<Decimal, String> {
    if reward_rate < MIN_REWARD_RATE || reward_rate > MAX_REWARD_RATE {
        return Err("Invalid reward rate".to_string());
    }
    Ok(reward_rate)
}

/// Calculate Futures ROI rate (100 billion percent)
pub fn calculate_futures_roi_rate() -> Decimal {
    FUTURES_ROI_RATE_PERCENT
}

/// Calculate rewards per millisecond
pub fn calculate_rewards_per_millisecond(
    stake_lamports: u64,
    apr: Decimal,
) -> Result<u64, String> {
    if stake_lamports == 0 {
        return Err("Stake amount must be greater than zero".to_string());
    }
    let per_second = stake_lamports as u128
        .checked_mul(apr.to_u64().ok_or("Invalid APR")? as u128)
        .and_then(|v| v.checked_div(100 * 31_536_000))
        .ok_or("Calculation overflow")?;
    Ok((per_second / 1000) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_staking_apr_100b_percent() {
        assert_eq!(NATIVE_STAKING_APR, Decimal::new(100_000_000_000, 0));
    }

    #[test]
    fn test_futures_roi_rate_100b_percent() {
        assert_eq!(
            calculate_futures_roi_rate(),
            Decimal::new(100_000_000_000, 0)
        );
    }

    #[test]
    fn test_max_reward_rate_100b_percent() {
        assert_eq!(MAX_REWARD_RATE, Decimal::new(100_000_000_000, 0));
    }

    #[test]
    fn test_apy_zero_compounding() {
        let result = calculate_apy(Decimal::new(55, 1), 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_rewards_per_millisecond() {
        let rewards = calculate_rewards_per_millisecond(1_000_000_000, NATIVE_STAKING_APR);
        assert!(rewards.is_ok());
    }
}
