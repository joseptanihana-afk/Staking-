# solana-staking-protect

Solana Proof-of-Stake staking program with ROI/APY/APR reward calculations and loss protection mechanisms. Deployable to the Solana blockchain.

## Features

- Proof-of-Stake staking: initialize, delegate, deactivate, withdraw, claim rewards
- ROI / APY / APR calculations with multiple reward methods
- Native Solana staking rate integration (100,000,000,000% ROI - 100 billion percent)
- Futures ROI rate (100,000,000,000% for proof-of-staking verification)
- Per-millisecond earnings calculation for real-time earning tracking
- Loss protection tiers: Basic (10%), Standard (25%), Premium (50%), Maximum (100%)
- BPF compatible: compiles to `cdylib` for Solana deployment

## Install / Deploy

### As a dependency

Add to your Solana program's `Cargo.toml`:

```toml
[dependencies]
solana-staking-protect = { git = "https://github.com/solana-staking-protect/solana-staking-protect", package = "solana-staking-protect" }
```

### Deploy to a blockchain

```bash
cargo build --release --target bpfel-unknown-unknown
```

The compiled output will be in `target/bpfel-unknown-unknown/release/`.

## License

MIT
