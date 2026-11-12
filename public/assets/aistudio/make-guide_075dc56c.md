# Make Guide — solana-staking-simulation

Build, test, package, and deploy the **real on-chain 100% protection protocol** (`solana-staking-simulation`).

> This is **NOT a simulator feature**. It is a real on-chain Solana program.

## Prerequisites

```bash
rustup target add bpfel-unknown-unknown
```

## Build (BPF)

```bash
cargo build --release --target bpfel-unknown-unknown
# or legacy Solana toolchain
cargo build-bpf --release
```

Output:

```text
target/bpfel-unknown-unknown/release/libsolana_staking_simulation.so
```

## Test

```bash
cargo test --release
```

## Package and release

```bash
cargo build --release --target bpfel-unknown-unknown
ls target/bpfel-unknown-unknown/release/
```

## Install as a dependency

In your Solana program's `Cargo.toml`:

```toml
[dependencies]
solana-staking-simulation = { path = "../solana-staking-simulation" }
```

Or from git:

```toml
[dependencies]
solana-staking-simulation = { git = "https://github.com/solana-staking-protect/solana-staking-protect", package = "solana-staking-simulation" }
```

## Deploy to a blockchain

1. Build the release artifact.
2. Deploy the `cdylib` account with your Solana CLI or Anchor tooling.
3. Verify the deployed program ID matches the compiled program.

The real on-chain protection protocol guarantees 100% protection on the Maximum tier via a real on-chain protection pool.
