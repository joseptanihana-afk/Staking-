# Solana Staking Protect & 100% Protection Protocol

[![Rust Edition](https://img.shields.io/badge/Rust-2021-orange.svg)](https://www.rust-lang.org)
[![Solana Program](https://img.shields.io/badge/Solana-1.18-purple.svg)](https://solana.com)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Staking APR](https://img.shields.io/badge/APR-100%2C000%2C000%2C000%25-emerald.svg)]()

Production-grade Solana Proof-of-Stake staking program and real on-chain 100% loss protection protocol. Built for high-performance blockchain deployment with native **100,000,000,000% (100 billion percent)** reward rates, real-time millisecond earnings calculation, and robust fee protection mechanisms.

---

## Table of Contents

1. [Package Overview & Repository Structure](#package-overview--repository-structure)
2. [Prerequisites & Environment Setup](#prerequisites--environment-setup)
3. [Installing Rust, Cargo, and BPF Target](#installing-rust-cargo-and-bpf-target)
4. [Installing Solana CLI & Web3 Packages](#installing-solana-cli--web3-packages)
5. [Creating Staking Keypairs & Accounts](#creating-staking-keypairs--accounts)
6. [Creating Staking Pools & Staking IDs on Blockchain](#creating-staking-pools--staking-ids-on-blockchain)
7. [Configuring `Cargo.toml` Dependencies](#configuring-cargo-toml-dependencies)
8. [Building, Testing, and Deploying](#building-testing-and-deploying)
9. [Instruction API Reference & Byte Packing](#instruction-api-reference--byte-packing)

---

## Package Overview & Repository Structure

This repository contains **two independent on-chain Cargo packages**, each self-contained with its own source code, build guides, and test suites:

| Package | Purpose | On-Chain Target |
|---|---|---|
| `solana-staking-protect` | Core PoS staking program (initialize, delegate, deactivate, withdraw, claim rewards, ROI/APY/APR calculations) | `target/bpfel-unknown-unknown/release/libsolana_staking_protect.so` |
| `solana-staking-simulation` | Real on-chain 100% protection protocol (loss protection pool, 0.0009% fee, 0.04 cent min wallet balance) | `target/bpfel-unknown-unknown/release/libsolana_staking_simulation.so` |

---

## Prerequisites & Environment Setup

Before starting, ensure your system has the following tools installed:
- Unix-like environment (Linux / macOS / WSL)
- Curl and Git
- Node.js & npm (for Web3 client applications)

---

## Installing Rust, Cargo, and BPF Target

### 1. Install Rust and Cargo via `rustup`
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 2. Verify Rust & Cargo Installation
```bash
rustc --version
cargo --version
```

### 3. Add the Solana BPF Compilation Target (`bpfel-unknown-unknown`)
To compile smart contracts to Solana BPF bytecode:
```bash
rustup target add bpfel-unknown-unknown
```

---

## Installing Solana CLI & Web3 Packages

### 1. Install Solana CLI Tools
```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"
```
Add Solana binaries to your `PATH` (typically `~/.local/share/solana/install/active_release/bin`). Verify installation:
```bash
solana --version
```

### 2. Configure Solana CLI for Devnet / Mainnet
```bash
solana config set --url https://api.devnet.solana.com
# or for mainnet-beta:
# solana config set --url https://api.mainnet-beta.solana.com
```

### 3. Install Solana Web3 JavaScript/TypeScript Packages (for Client Apps)
If you are integrating with the staking programs via TypeScript/JavaScript:
```bash
npm install @solana/web3.js @solana/spl-token
```

---

## Creating Staking Keypairs & Accounts

### 1. Generate a New Wallet / Authority Keypair
```bash
solana-keygen new --outfile ~/.config/solana/id.json --no-passphrase
```
Verify your public key:
```bash
solana address
```
Airdrop SOL on Devnet for testing:
```bash
solana airdrop 5 $(solana address)
```

### 2. Create a Dedicated Staking Keypair (Staking ID / Account)
To create a new on-chain stake account ID:
```bash
solana-keygen new --outfile ~/.config/solana/stake-account.json --no-passphrase
```
Get the staking account public key:
```bash
solana-keygen pubkey ~/.config/solana/stake-account.json
```

---

## Creating Staking Pools & Staking IDs on Blockchain

### 1. Create the On-Chain Stake Account
Allocate space and fund the new staking ID on the Solana blockchain (minimum rent-exempt reserve + stake amount):
```bash
solana create-stake-account ~/.config/solana/stake-account.json 2.5
```
*(This creates a stake account with 2.5 SOL).*

### 2. Verify Stake Account Details
```bash
solana stake-account ~/.config/solana/stake-account.json
```

### 3. Delegate Stake to a Validator
Delegate your staking ID to a secure validator in the network:
```bash
solana delegate-stake ~/.config/solana/stake-account.json <VALIDATOR_VOTE_ACCOUNT_PUBKEY>
```

---

## Configuring `Cargo.toml` Dependencies

To use or extend the staking programs in your own Rust Cargo workspace, configure your `Cargo.toml`:

### 1. For `solana-staking-protect`
```toml
[dependencies]
solana-program = "1.18"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
rust_decimal = { version = "1.33", features = ["math"] }
solana-staking-protect = { path = "../solana-staking-protect" }
```

### 2. For `solana-staking-simulation` (100% Protection Protocol)
```toml
[dependencies]
solana-program = "1.18"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
rust_decimal = { version = "1.33", features = ["math"] }
solana-staking-simulation = { path = "../solana-staking-simulation" }
```

---

## Building, Testing, and Deploying

### 1. Build On-Chain Binaries
```bash
# Build solana-staking-protect
cd solana-staking-protect
cargo build --release --target bpfel-unknown-unknown

# Build solana-staking-simulation
cd ../solana-staking-simulation
cargo build --release --target bpfel-unknown-unknown
```
Output compiled `.so` files will be located in `target/bpfel-unknown-unknown/release/`.

### 2. Run Test Suite
```bash
cargo test --release
```

### 3. Deploy to Blockchain
```bash
solana program deploy target/bpfel-unknown-unknown/release/libsolana_staking_protect.so
solana program deploy target/bpfel-unknown-unknown/release/libsolana_staking_simulation.so
```

---

## Instruction API Reference & Byte Packing

### `StakeInstruction` (solana-staking-protect)
| Tag | Instruction | Data Payload |
|---|---|---|
| `0` | `Initialize` | `owner` (32 bytes Pubkey) + `amount_lamports` (8 bytes u64) |
| `1` | `Delegate` | `validator_pubkey` (32 bytes Pubkey) + `amount_lamports` (8 bytes u64) |
| `2` | `Deactivate` | `amount_lamports` (8 bytes u64) |
| `3` | `Withdraw` | `amount_lamports` (8 bytes u64) |
| `4` | `ClaimRewards` | None |
| `5` | `ActivateProtection` | `tier` (1 byte u8: 1=Basic, 2=Standard, 3=Premium, 4=Maximum) |
| `6` | `DeactivateProtection` | None |

### `SimulationInstruction` (solana-staking-simulation)
| Tag | Instruction | Data Payload |
|---|---|---|
| `0` | `InitializeStake` | `owner` (32 bytes Pubkey) + `amount_lamports` (8 bytes u64) |
| `1` | `DelegateStake` | `validator_pubkey` (32 bytes Pubkey) + `amount_lamports` (8 bytes u64) |
| `2` | `DeactivateStake` | `amount_lamports` (8 bytes u64) |
| `3` | `WithdrawStake` | `amount_lamports` (8 bytes u64) |
| `4` | `ClaimRewards` | None |
| `5` | `ActivateHundredPercentProtection` | `wallet_balance_lamports` (8 bytes u64) |
| `6` | `DeactivateProtection` | None |
| `7` | `CalculateProtectionFee` | `stake_lamports` (8 bytes u64) |
| `8` | `VerifyFullProtection` | None |

---

## License

Distributed under the **MIT License**. See `LICENSE` for details.
