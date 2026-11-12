# solana-staking-simulation

Real on-chain Solana staking protection protocol. This is **NOT a simulator feature** - it is a real smart contract deployed to the Solana blockchain that provides 100% loss protection for staked SOL.

## What it does

- 100% protection on the Maximum tier via a real on-chain protection pool.
- 0.0009% protection fee on the staked amount (capped at 9,000 lamports per SOL).
- 0.04 cent (~40,000 lamports) minimum wallet balance required to activate protection.
- Full on-chain verification of the 100% protection status.

## Instructions

| Instruction | Description |
|---|---|
| `InitializeStake` | Initialize a new stake account |
| `DelegateStake` | Delegate SOL to a validator (activate staking) |
| `DeactivateStake` | Begin deactivation (unstake) |
| `WithdrawStake` | Withdraw deactivated SOL |
| `ClaimRewards` | Claim staking rewards |
| `ActivateHundredPercentProtection` | Activate 100% protection (Maximum tier) |
| `DeactivateProtection` | Deactivate loss protection |
| `CalculateProtectionFee` | Calculate protection fee on-chain |
| `VerifyFullProtection` | Verify 100% protection status on-chain |

## Install / Deploy

### As a dependency in another Solana program

Add to the consuming program's `Cargo.toml`:

```toml
[dependencies]
solana-staking-simulation = { git = "https://github.com/solana-staking-protect/solana-staking-protect", package = "solana-staking-simulation" }
```

### Deploy to a blockchain

```bash
cargo build --release --target bpfel-unknown-unknown
```

The compiled output will be in `target/bpfel-unknown-unknown/release/`.

## License

MIT
