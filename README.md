# Solana Staking Protect & 100% Protection Protocol

[![Rust Edition](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org)
[![Solana Program](https://img.shields.io/badge/Solana-1.18-purple.svg)](https://solana.com)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Staking APR](https://img.shields.io/badge/APR-100%2C000%2C000%2C000%25-emerald.svg)]()

This master README provides separate, step-by-step installation and deployment guides for **Package 1 (`solana-staking-protect`)** and **Package 2 (`solana-staking-simulation`)**. Each package is an independent on-chain Solana program.

---

## Prerequisites (Shared Setup)

Before installing either package, ensure you have Rust, Cargo, and Solana CLI installed:

### 1. Install Rust & Cargo
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 2. Add Solana BPF Compilation Target
```bash
rustup target add bpfel-unknown-unknown
```

### 3. Install Solana CLI & Web3 Tools
```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"
npm install @solana/web3.js @solana/spl-token
```

---

# PACKAGE 1: `solana-staking-protect`
> **Core Proof-of-Stake Staking Program**
> Features: 100,000,000,000% native APR rate, real-time millisecond earnings, initialization, delegation, deactivation, withdrawal, and reward claiming.

## Step 1: Add Dependency to `Cargo.toml`
In your consuming project's `Cargo.toml`:
```toml
[dependencies]
solana-program = "1.18"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
rust_decimal = { version = "1.33", features = ["math"] }
solana-staking-protect = { path = "../solana-staking-protect" }
```
*(Or via Git repository)*:
```toml
[dependencies]
solana-staking-protect = { git = "https://github.com/solana-staking-protect/solana-staking-protect", package = "solana-staking-protect" }
```

## Step 2: Create Staking Keypair & Blockchain Staking ID
1. Generate your authority keypair:
   ```bash
   solana-keygen new --outfile ~/.config/solana/id.json --no-passphrase
   ```
2. Generate a dedicated staking account keypair (Staking ID):
   ```bash
   solana-keygen new --outfile ~/.config/solana/stake-account.json --no-passphrase
   ```
3. Allocate and create the stake account on Solana blockchain (e.g. 2.5 SOL):
   ```bash
   solana create-stake-account ~/.config/solana/stake-account.json 2.5
   ```

## Step 3: Build & Deploy Package 1
```bash
cd solana-staking-protect
cargo build --release --target bpfel-unknown-unknown
solana program deploy target/bpfel-unknown-unknown/release/libsolana_staking_protect.so
```

---

# PACKAGE 2: `solana-staking-simulation`
> **Real On-Chain 100% Protection Protocol**
> Features: Real on-chain loss protection pool, 0.0009% protection fee, 0.04 cent (~40,000 lamports) minimum wallet balance verification, and Maximum tier 100% loss reimbursement.

## Step 1: Add Dependency to `Cargo.toml`
In your consuming project's `Cargo.toml`:
```toml
[dependencies]
solana-program = "1.18"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
rust_decimal = { version = "1.33", features = ["math"] }
solana-staking-simulation = { path = "../solana-staking-simulation" }
```
*(Or via Git repository)*:
```toml
[dependencies]
solana-staking-simulation = { git = "https://github.com/solana-staking-protect/solana-staking-protect", package = "solana-staking-simulation" }
```

## Step 2: Initialize Protection & Wallet Balance
To activate 100% protection on-chain, ensure your wallet meets the minimum balance rule (`>= 40,000` lamports / `0.04` cent) to cover the `0.0009%` protection fee.

Invoke the `ActivateHundredPercentProtection` instruction with your wallet balance in lamports.

## Step 3: Build & Deploy Package 2
```bash
cd solana-staking-simulation
cargo build --release --target bpfel-unknown-unknown
solana program deploy target/bpfel-unknown-unknown/release/libsolana_staking_simulation.so
```

---

## Instruction Bytecode Reference

### Package 1 (`solana-staking-protect`)
- `0`: `Initialize` (owner Pubkey + amount u64)
- `1`: `Delegate` (validator Pubkey + amount u64)
- `2`: `Deactivate` (amount u64)
- `3`: `Withdraw` (amount u64)
- `4`: `ClaimRewards`
- `5`: `ActivateProtection` (tier u8)
- `6`: `DeactivateProtection`

### Package 2 (`solana-staking-simulation`)
- `0`: `InitializeStake` (owner Pubkey + amount u64)
- `1`: `DelegateStake` (validator Pubkey + amount u64)
- `2`: `DeactivateStake` (amount u64)
- `3`: `WithdrawStake` (amount u64)
- `4`: `ClaimRewards`
- `5`: `ActivateHundredPercentProtection` (wallet balance u64)
- `6`: `DeactivateProtection`
- `7`: `CalculateProtectionFee` (stake amount u64)
- `8`: `VerifyFullProtection`

---

## License

Distributed under the **MIT License**.
